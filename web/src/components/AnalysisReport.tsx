import type { Analysis } from "../lib/analysis";

interface AnalysisReportProps {
  analysis: Analysis | null;
  raw: string | null;
  status: "idle" | "loading" | "error";
}

/**
 * The three sections are always on the page. Before a run they are empty rules
 * of decreasing length — the shape of the answer, shown as a promise. That
 * narrowing is the one structural idea the page is built around, so it must be
 * visible on first paint, not only after a result arrives.
 */
const SECTIONS = [
  { key: "topics", label: "Main topics", width: "wide" },
  { key: "themes", label: "Recurring themes", width: "medium" },
  { key: "narrative", label: "Dominant narrative", width: "narrow" },
] as const;

function Rule({ width }: { width: (typeof SECTIONS)[number]["width"] }) {
  return <div className={`rule rule--${width}`} aria-hidden="true" />;
}

export function AnalysisReport({ analysis, raw, status }: AnalysisReportProps) {
  const filled = analysis !== null && status !== "loading";

  return (
    <section
      className={`report report--${status}`}
      aria-live="polite"
      aria-busy={status === "loading"}
    >
      {SECTIONS.map((section) => {
        const items =
          section.key === "topics"
            ? analysis?.mainTopics
            : section.key === "themes"
              ? analysis?.recurringThemes
              : null;

        return (
          <div key={section.key} className={`report__section report__section--${section.width}`}>
            <h2 className="report__heading">{section.label}</h2>

            {!filled && <Rule width={section.width} />}

            {filled && section.key === "narrative" && (
              <p className="report__narrative">{analysis.dominantNarrative}</p>
            )}

            {filled && items && (
              <ul className="report__list">
                {items.map((item, index) => (
                  <li key={`${section.key}-${index}`}>{item}</li>
                ))}
              </ul>
            )}
          </div>
        );
      })}

      {filled && analysis.articlesAnalyzed !== null && (
        <p className="report__provenance">
          Read across {analysis.articlesAnalyzed} articles returned by Google News.
        </p>
      )}

      {/*
        The sections above are parsed out of one plain-text blob, and anything
        the model writes outside the three known headings is discarded. This
        shows the answer exactly as it arrived — both so the result can be
        checked against its source, and so nothing the model said is lost.
      */}
      {filled && raw && (
        <details className="disclosure">
          <summary className="disclosure__summary">Show the response as it arrived</summary>
          <pre className="disclosure__raw">{raw}</pre>
        </details>
      )}
    </section>
  );
}
