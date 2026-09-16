import { useRef, useState, type FormEvent } from "react";
import { parseAnalysis, toKeywords, type Analysis } from "../lib/analysis";
import { AnalysisError, requestAnalysis } from "../lib/api";
import { AnalysisReport } from "./AnalysisReport";

type Status = "idle" | "loading" | "error";

export function Analyzer() {
  const [input, setInput] = useState("");
  const [status, setStatus] = useState<Status>("idle");
  const [analysis, setAnalysis] = useState<Analysis | null>(null);
  const [raw, setRaw] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const pending = useRef<AbortController | null>(null);

  async function run(event: FormEvent) {
    event.preventDefault();

    const keywords = toKeywords(input);
    if (keywords.length === 0) {
      setStatus("error");
      setError("Name at least one keyword to analyze.");
      return;
    }

    pending.current?.abort();
    const controller = new AbortController();
    pending.current = controller;

    setStatus("loading");
    setError(null);

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
      setError(
        failure instanceof AnalysisError
          ? failure.message
          : "Something went wrong before the analysis started.",
      );
    } finally {
      if (pending.current === controller) pending.current = null;
    }
  }

  return (
    <>
      <form className="query" onSubmit={run}>
        <label className="query__label" htmlFor="keywords">
          Keywords
        </label>

        <div className="query__row">
          <input
            id="keywords"
            className="query__input"
            type="text"
            value={input}
            onChange={(event) => setInput(event.target.value)}
            placeholder="climate policy, COP30"
            aria-describedby="keywords-hint"
            autoComplete="off"
            spellCheck={false}
          />
          <button className="query__submit" type="submit" disabled={status === "loading"}>
            {status === "loading" ? "Reading" : "Analyze"}
          </button>
        </div>

        {/*
          The backend joins every keyword into a single Google News query rather
          than searching each one, so the hint says that outright — otherwise a
          comma reads as "analyze these separately".
        */}
        <p className="query__hint" id="keywords-hint">
          Separate them with commas. They combine into one news search, not one search each.
        </p>

        {status === "loading" && (
          <p className="query__note">
            Searching Google News, then reading the results in two passes. This takes a few
            seconds.
          </p>
        )}

        {status === "error" && error && (
          <p className="query__note query__note--error" role="alert">
            {error}
          </p>
        )}
      </form>

      <AnalysisReport analysis={analysis} raw={raw} status={status} />
    </>
  );
}
