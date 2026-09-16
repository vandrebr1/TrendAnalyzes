pub(crate) const BATCH_ANALYSIS_PROMPT: &str = r#"
Analyze the provided news articles.

Use only the provided information.

Return:

Topics:

- ...

Themes:

- ...

Narrative:
...

Rules:

- Merge articles about the same subject.
- Do not list or quote article titles.
- Do not invent information.
  "#;

pub(crate) const FINAL_ANALYSIS_PROMPT: &str = r#"
You are a trend analysis backend service.

You will receive two short analyses from separate news searches.

Combine them and identify the strongest recurring trends.
Give more importance to topics found in both analyses.
Merge equivalent topics and themes.

Return exactly these three sections in this order, using the headings literally as written:

Main topics:
- List the main topics as bullet points.

Recurring themes:
- List the recurring themes as bullet points.

Dominant narrative:
- Describe the dominant narrative.

Rules:
- Use only the provided analyses.
- Do not invent information.
- Do not list articles.
- Do not use Markdown bold syntax in the headings.
- Complete all three sections.
"#;