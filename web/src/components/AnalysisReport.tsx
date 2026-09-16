import type { Analysis } from "../lib/analysis";
import { Tally } from "./Tally";

interface AnalysisReportProps {
  analysis: Analysis | null;
  raw: string | null;
  status: "idle" | "loading" | "error";
}

/**
 * Before a run, only the tally is on the page: the question and the instrument
 * that will answer it. The sections are not drawn as empty scaffolding, so the
 * arrival of the narrative is the page's one moment rather than a fill-in.
 */
export function AnalysisReport({ analysis, raw, status }: AnalysisReportProps) {
  const reading = status === "loading";
  const filled = analysis !== null && !reading;

  // append_article_count checks only that the three heading lines exist, not
  // that anything sits under them, so a response with bare headings passes the
  // backend's own 502 check and arrives here empty. Without this floor the page
  // would show an article count and no answer, which reads exactly like a run
  // that never happened.
  const barren =
    filled &&
    analysis.mainTopics.length === 0 &&
    analysis.recurringThemes.length === 0 &&
    analysis.dominantNarrative.length === 0;

  return (
    <section className="reading" aria-busy={reading}>
      <Tally
        state={reading ? "reading" : filled ? "done" : "idle"}
        read={filled ? analysis.articlesAnalyzed : null}
      />

      {barren && (
        <div className="alert" role="alert">
          <p>
            The articles were found, but the reading came back with its three headings and nothing
            under them. Try running it again.
          </p>
        </div>
      )}

      {filled && analysis.mainTopics.length > 0 && (
        <div className="zone">
          <h2 className="zone__label">Main topics</h2>
          <ul className="findings">
            {analysis.mainTopics.map((topic, index) => (
              <li key={`topic-${index}`}>{topic}</li>
            ))}
          </ul>
        </div>
      )}

      {filled && analysis.recurringThemes.length > 0 && (
        <div className="zone">
          <h2 className="zone__label">Recurring themes</h2>
          <ul className="findings">
            {analysis.recurringThemes.map((theme, index) => (
              <li key={`theme-${index}`}>{theme}</li>
            ))}
          </ul>
        </div>
      )}

      {/* The one block that breaks the label grid, because it is the answer
          the other two build towards. */}
      {filled && analysis.dominantNarrative && (
        <div className="answer">
          <h2 className="answer__label">Dominant narrative</h2>
          <p className="answer__text">{analysis.dominantNarrative}</p>
        </div>
      )}

      {/*
        The sections above are parsed out of one plain-text blob, and anything
        the model writes outside the three known headings is discarded. This
        shows the answer exactly as it arrived — both so the result can be
        checked against its source, and so nothing the model said is lost.
      */}
      {filled && raw && (
        <details className="source">
          <summary className="source__summary">Show the response as it arrived</summary>
          <pre className="source__raw">{raw}</pre>
        </details>
      )}
    </section>
  );
}
