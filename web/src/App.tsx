import { Analyzer } from "./components/Analyzer";

export default function App() {
  return (
    <>
      <header className="masthead">
        <span className="masthead__name">Trend Analyzes</span>
        <a className="masthead__link" href="#method">
          How it works
        </a>
      </header>

      <main>
        <section className="hero">
          <h1 className="hero__title">Read twenty articles at once.</h1>
          <p className="hero__lede">
            Name a subject. Trend Analyzes pulls the twenty most recent news articles about it and
            reports the topics, themes and narrative they share.
          </p>
        </section>

        <Analyzer />

        <section className="method" id="method">
          <div className="method__block">
            <h2 className="method__heading">Where the articles come from</h2>
            <p>
              Every run makes one Google News query from your keywords and takes the twenty most
              recent results. Nothing is archived and nothing is cached, so you get what is being
              published now.
            </p>
          </div>

          <div className="method__block">
            <h2 className="method__heading">Why it reads them twice</h2>
            <p>
              The twenty articles are split in half and summarized in parallel, so no single pass
              has to hold all of them at once. A third pass compares the two summaries and keeps
              what appears in both.
            </p>
          </div>

          <div className="method__block">
            <h2 className="method__heading">What it does not do</h2>
            <p>
              There is no statistical model here and no trend score. This is a language model
              reading current coverage and reporting what it finds. Treat the result as a reading
              of the news, not a measurement of it.
            </p>
          </div>
        </section>
      </main>

      <footer className="colophon">
        <p>
          Built on Rust and Axum. Articles from Google News. The HTTP API behind this page is
          documented at <a href="/swagger">/swagger</a>.
        </p>
      </footer>
    </>
  );
}
