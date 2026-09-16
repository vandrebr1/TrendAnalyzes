/**
 * Talks to api-gateway, which is reached differently in each mode:
 *
 * - Development: Vite proxies /api to localhost:3000 and strips the prefix
 *   (see vite.config.ts), so the browser sees a single origin.
 * - Production: the gateway serves this bundle itself, so the API is already
 *   same-origin and the routes are mounted at the root — no prefix at all.
 *
 * Getting this wrong is quiet rather than loud: a request to a path the gateway
 * does not route falls through to the static handler and comes back as
 * index.html with status 200, which then fails to parse as JSON. That case is
 * caught below and named, because the HTTP status gives no hint of it.
 *
 * VITE_API_BASE_URL overrides both, for deploying the frontend separately.
 */
const BASE_URL = import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? "/api" : "");

interface GatewayError {
  message?: string;
}

/**
 * Failures the interface can explain. `detail` carries the backend's own
 * message when there is one; the error block renders it behind a disclosure,
 * because on a 502 it is the only thing that separates an unreachable service
 * from an empty news search.
 */
export class AnalysisError extends Error {
  readonly detail: string | null;

  constructor(message: string, detail: string | null = null) {
    super(message);
    this.name = "AnalysisError";
    this.detail = detail;
  }
}

async function readGatewayMessage(response: Response): Promise<string | null> {
  try {
    const body = (await response.json()) as GatewayError;
    return typeof body.message === "string" ? body.message : null;
  } catch {
    return null;
  }
}

function explain(status: number, detail: string | null): AnalysisError {
  // Unreachable through the current gateway: ai_chat_proxy maps every error
  // from call_ai_service to 502, including ai_service's own 400. Kept because
  // it costs nothing and is correct the day the gateway starts passing the
  // upstream status through — but do not read its presence as evidence that a
  // 400 can arrive today.
  if (status === 400) {
    return new AnalysisError("Name at least one subject to read about.", detail);
  }

  if (detail?.includes("missing configuration")) {
    return new AnalysisError(
      "The server is missing its credentials. Check that AI_SERVICE_TOKEN matches in both services and that the ai_service keys are set.",
      detail,
    );
  }

  if (status === 502) {
    return new AnalysisError(
      "The reading service could not finish. It may be unreachable, or the news search returned nothing usable for these subjects.",
      detail,
    );
  }

  return new AnalysisError(`The server responded with ${status}.`, detail);
}

export async function requestAnalysis(keywords: string[], signal?: AbortSignal): Promise<string> {
  let response: Response;

  try {
    response = await fetch(`${BASE_URL}/ai/chat`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ keywords }),
      signal,
    });
  } catch (error) {
    if (error instanceof DOMException && error.name === "AbortError") throw error;
    throw new AnalysisError(
      "No answer from the gateway. Start it with cargo run and try again.",
      error instanceof Error ? error.message : null,
    );
  }

  if (!response.ok) {
    throw explain(response.status, await readGatewayMessage(response));
  }

  let body: { response?: unknown };

  try {
    body = (await response.json()) as { response?: unknown };
  } catch (error) {
    // A 200 that is not JSON means the SPA fallback answered instead of the
    // API, so the request went to a path the gateway does not route.
    throw new AnalysisError(
      "The server sent a page instead of a reading, which means the request went to a path it does not serve. If this is a built bundle, check that VITE_API_BASE_URL is unset.",
      error instanceof Error ? error.message : null,
    );
  }

  if (typeof body.response !== "string" || body.response.trim().length === 0) {
    throw new AnalysisError("The server returned an empty reading. Try running it again.");
  }

  return body.response;
}
