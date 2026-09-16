import type { CSSProperties } from "react";

/**
 * The tally is the page's one device, and it reports real numbers only.
 *
 * The search asks Google News for at most `CAP` articles
 * (`search(&query, 20)` in ai_client.rs) and `split_article_batches` divides
 * whatever comes back with `div_ceil(2)`, so the first pass always takes the
 * larger half. The two groups below mirror that split exactly.
 *
 * Nothing here is an estimate. While the request is out the frontend has no
 * count to show, so the ticks sweep without claiming progress; the count only
 * appears once the backend has appended it.
 */
const CAP = 20;
const FIRST_PASS = Math.ceil(CAP / 2);

export type TallyState = "idle" | "reading" | "done";

interface TallyProps {
  /** Articles the backend reported reading, or null before a run finishes. */
  read: number | null;
  state: TallyState;
}

function caption(state: TallyState, read: number | null) {
  if (state === "reading") {
    return "Searching Google News, then reading both halves at once.";
  }

  if (state === "done") {
    // append_article_count always appends the count, so `read` being null here
    // means the line did not survive parsing. Say nothing about a number
    // rather than falling back to the idle wording, which would read as though
    // no run had happened.
    return read === null ? (
      "Read in two passes."
    ) : (
      <>
        <span className="tally__count">{read}</span>{" "}
        {read === 1 ? "article" : "articles"} read, out of the {CAP} the search asks for.
      </>
    );
  }

  return `Up to ${CAP} recent articles, read in two passes.`;
}

export function Tally({ read, state }: TallyProps) {
  const total = Math.min(Math.max(read ?? 0, 0), CAP);
  const firstRead = Math.ceil(total / 2);

  const groups = [
    { key: "first", size: FIRST_PASS, lit: firstRead },
    { key: "second", size: CAP - FIRST_PASS, lit: total - firstRead },
  ];

  return (
    <>
      {/* Hidden from assistive technology: the caption below states the count
          in words, so announcing twenty ticks would only repeat it. */}
      <div className={`tally tally--${state}`} aria-hidden="true">
        {groups.map((group) => (
          <div className="tally__group" key={group.key}>
            {Array.from({ length: group.size }, (_, index) => (
              <span
                key={index}
                className={index < group.lit ? "tick tick--read" : "tick"}
                style={{ "--i": index } as CSSProperties}
              />
            ))}
          </div>
        ))}
      </div>

      {/*
        The only announced status on the page. It is always in the DOM so that
        a screen reader picks up the change, and it covers both moments that
        used to be silent: the run starting, and the result arriving.
      */}
      <p className="tally__caption" role="status">
        {caption(state, read)}
      </p>
    </>
  );
}
