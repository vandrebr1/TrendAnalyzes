use std::{
    io,
    path::Path,
    process::{Child, Command},
    thread,
    time::Duration,
};

fn start_service(workspace_root: &Path, package_dir: &str) -> io::Result<Child> {
    Command::new("cargo")
        .arg("run")
        .current_dir(workspace_root.join("src").join(package_dir))
        .spawn()
}

fn stop_service(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn main() -> io::Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut gateway = start_service(workspace_root, "api-gateway")?;
    let mut ai_service = match start_service(workspace_root, "ai_service") {
        Ok(child) => child,
        Err(error) => {
            stop_service(&mut gateway);
            return Err(error);
        }
    };

    println!("Started api-gateway on port 3000 and ai_service on port 3060");

    loop {
        if let Some(status) = gateway.try_wait()? {
            stop_service(&mut ai_service);
            return Err(io::Error::other(format!(
                "api-gateway exited with status {status}"
            )));
        }

        if let Some(status) = ai_service.try_wait()? {
            stop_service(&mut gateway);
            return Err(io::Error::other(format!(
                "ai_service exited with status {status}"
            )));
        }

        thread::sleep(Duration::from_millis(250));
    }
}