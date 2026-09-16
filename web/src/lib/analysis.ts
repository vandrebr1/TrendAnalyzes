/**
 * The backend returns one plain-text blob with three fixed headings followed by
 * an article count. See CLAUDE.md: those headings are an end-to-end contract
 * shared by prompts.rs, append_article_count, and this parser.
 */
export interface Analysis {
  mainTopics: string[];
  recurringThemes: string[];
  dominantNarrative: string;
  articlesAnalyzed: number | null;
}

type Section = "topics" | "themes" | "narrative";

const HEADINGS: Record<string, Section> = {
  "main topics": "topics",
  "recurring themes": "themes",
  "dominant narrative": "narrative",
};

const BULLET = /^[-*•]\s*/;
const ARTICLE_COUNT = /^\**\s*articles analyzed\s*\**\s*:\s*(\d+)/i;

/** Strips Markdown emphasis and the trailing colon so headings match loosely. */
function asHeadingKey(line: string): string {
  return line
    .replace(/\*/g, "")
    .replace(/:\s*$/, "")
    .trim()
    .toLowerCase();
}

export function parseAnalysis(raw: string): Analysis {
  const mainTopics: string[] = [];
  const recurringThemes: string[] = [];
  const narrative: string[] = [];
  let articlesAnalyzed: number | null = null;
  let current: Section | null = null;

  for (const line of raw.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    const count = trimmed.match(ARTICLE_COUNT);
    if (count) {
      articlesAnalyzed = Number(count[1]);
      current = null;
      continue;
    }

    const heading = HEADINGS[asHeadingKey(trimmed)];
    if (heading) {
      current = heading;
      continue;
    }

    if (!current) continue;

    const content = trimmed.replace(BULLET, "").trim();
    if (!content) continue;

    if (current === "topics") mainTopics.push(content);
    else if (current === "themes") recurringThemes.push(content);
    else narrative.push(content);
  }

  return {
    mainTopics,
    recurringThemes,
    dominantNarrative: narrative.join(" ").trim(),
    articlesAnalyzed,
  };
}

/** Splits the comma-separated field into the array the API expects. */
export function toKeywords(input: string): string[] {
  return input
    .split(",")
    .map((keyword) => keyword.trim())
    .filter((keyword) => keyword.length > 0);
}
