import { useRef, useState, type FormEvent } from "react";
import { parseAnalysis, toKeywords, type Analysis } from "../lib/analysis";
import { AnalysisError, requestAnalysis } from "../lib/api";
import { AnalysisReport } from "./AnalysisReport";

type Status = "idle" | "loading" | "error";

/**
 * The same sentence the gateway sends for a 400, so the two paths into an
 * empty request read identically wherever the check happens to run.
 */
const NO_SUBJECT = "Name at least one subject to read about.";

export function Analyzer() {
  const [input, setInput] = useState("");
  const [status, setStatus] = useState<Status>("idle");
  const [analysis, setAnalysis] = useState<Analysis | null>(null);
  const [raw, setRaw] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [detail, setDetail] = useState<string | null>(null);
  const pending = useRef<AbortController | null>(null);

  async function run(event: FormEvent) {
    event.preventDefault();

    // The interface calls these subjects and the wire field is `keywords`, so
    // the names diverge on purpose: the copy follows the reader, the code
    // follows the API.
    const keywords = toKeywords(input);
    if (keywords.length === 0) {
      setStatus("error");
      setError(NO_SUBJECT);
      setDetail(null);
      return;
    }

    pending.current?.abort();
    const controller = new AbortController();
    pending.current = controller;

    setStatus("loading");
    setError(null);
    setDetail(null);

    try {
      const response = await requestAnalysis(keywords, controller.signal);
      setRaw(response);
      setAnalysis(parseAnalysis(response));
      setStatus("idle");
    } catch (failure) {
      if (failure instanceof DOMException && failure.name === "AbortError") return;
      setAnalysis(null);
      setRaw(null);
      setStatus("error");

      if (failure instanceof AnalysisError) {
        setError(failure.message);
        setDetail(failure.detail);
      } else {
        setError("The answer arrived but could not be read as an analysis.");
        setDetail(failure instanceof Error ? failure.message : null);
      }
    } finally {
      if (pending.current === controller) pending.current = null;
    }
  }

  return (
    <>
      <form className="ask" id="ask" onSubmit={run}>
        {/* The page heading is the field's label: the sentence the reader
            completes is also the plainest description of what this does. */}
        <h1 className="ask__title">
          <label htmlFor="subject">What is being said about</label>
        </h1>

        <div className="ask__row">
          <input
            id="subject"
            className="ask__field"
            type="text"
            value={input}
            onChange={(event) => setInput(event.target.value)}
            placeholder="climate policy, COP30"
            aria-describedby="subject-hint"
            autoComplete="off"
            spellCheck={false}
          />
          <button className="ask__submit" type="submit" disabled={status === "loading"}>
            {status === "loading" ? "Reading" : "Read"}
          </button>
        </div>

        {/*
          The backend joins every subject into a single Google News query rather
          than searching each one, so the hint says that outright — otherwise a
          comma reads as "read these separately".
        */}
        <p className="ask__hint" id="subject-hint">
          Separate subjects with commas. They combine into one news search, not one search each.
        </p>

        {/*
          The failure path shows what the success path shows: the sentence a
          reader can act on, and, folded away, the message the server actually
          sent — which on a 502 is the only way to tell the two causes apart.
        */}
        {status === "error" && error && (
          <div className="alert" role="alert">
            <p>{error}</p>
            {detail && (
              <details className="alert__detail">
                <summary>What the server said</summary>
                <pre>{detail}</pre>
              </details>
            )}
          </div>
        )}
      </form>

      <AnalysisReport analysis={analysis} raw={raw} status={status} />
    </>
  );
}
