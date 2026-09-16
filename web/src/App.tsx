import { Analyzer } from "./components/Analyzer";

export default function App() {
  return (
    <>
      <a className="skip" href="#ask">
        Skip to the search
      </a>

      <header className="bar">
        <span className="bar__name">Trend Analyzes</span>
        <a className="bar__link" href="#how">
          How it works
        </a>
      </header>

      <main className="page">
        <Analyzer />

        <section className="how" id="how">
          <div className="zone">
            <h2 className="zone__label">Where the articles come from</h2>
            <p>
              Every run makes one Google News query from the subjects you name and reads up to
              twenty results. The order is Google's, not chronological. Nothing is stored and
              nothing is cached, so each run reflects what that search returns at that moment.
            </p>
          </div>

          <div className="zone">
            <h2 className="zone__label">Why it reads them twice</h2>
            <p>
              The articles are split in half and read in parallel, so no single pass has to hold all
              of them at once. A third pass merges the two readings, giving more weight to whatever
              turned up in both.
            </p>
          </div>

          <div className="zone">
            <h2 className="zone__label">What it does not do</h2>
            <p>
              There is no statistical model here and no trend score. This is a language model
              reading current coverage and reporting what it finds. Treat the result as a reading of
              the news, not a measurement of it.
            </p>
          </div>
        </section>
      </main>
    </>
  );
}
