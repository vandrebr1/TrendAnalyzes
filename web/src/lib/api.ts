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
 * index.html with status 200, which then fails to parse as JSON.
 *
 * VITE_API_BASE_URL overrides both, for deploying the frontend separately.
 */
const BASE_URL = import.meta.env.VITE_API_BASE_URL ?? (import.meta.env.DEV ? "/api" : "");

interface GatewayError {
  message?: string;
}

/**
 * Failures the interface can explain. `detail` carries the backend's own
 * message when there is one, for the cases an operator needs to see verbatim.
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
  if (status === 400) {
    return new AnalysisError("Add at least one subject before running an analysis.", detail);
  }

  if (detail?.includes("missing configuration")) {
    return new AnalysisError(
      "The server is missing its credentials. Check that AI_SERVICE_TOKEN matches in both services and that the ai_service keys are set.",
      detail,
    );
  }

  if (status === 502) {
    return new AnalysisError(
      "The analysis service could not complete the request. It may be unreachable, or the news search returned nothing usable for these subjects.",
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

  const body = (await response.json()) as { response?: unknown };

  if (typeof body.response !== "string" || body.response.trim().length === 0) {
    throw new AnalysisError("The server returned an empty analysis. Try running it again.");
  }

  return body.response;
}
