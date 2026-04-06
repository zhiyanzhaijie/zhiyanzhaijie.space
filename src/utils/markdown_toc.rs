use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: usize,
}

fn parse_markdown_heading(line: &str) -> Option<(usize, String)> {
    let mut level = 0;
    for ch in line.chars() {
        if ch == '#' {
            level += 1;
        } else {
            break;
        }
    }

    if !(1..=6).contains(&level) {
        return None;
    }

    let rest = line[level..].trim_start();
    if rest.is_empty() {
        return None;
    }

    let title = rest.trim_end_matches('#').trim().to_string();
    if title.is_empty() {
        return None;
    }

    Some((level, title))
}

fn slugify_title(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in title.chars() {
        if ch.is_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if ch.is_whitespace() || ch == '-' || ch == '_' {
            if !last_was_dash {
                slug.push('-');
                last_was_dash = true;
            }
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "section".to_string()
    } else {
        slug
    }
}

fn unique_id(base: &str, counts: &mut HashMap<String, usize>) -> String {
    let count = counts.entry(base.to_string()).or_insert(0);
    if *count == 0 {
        *count += 1;
        base.to_string()
    } else {
        let id = format!("{}-{}", base, *count);
        *count += 1;
        id
    }
}

pub fn inject_heading_anchors_and_collect_toc(markdown: &str) -> (String, Vec<TocItem>) {
    let mut in_code_block = false;
    let mut lines_out = Vec::new();
    let mut toc_items = Vec::new();
    let mut id_counts = HashMap::new();

    for line in markdown.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            lines_out.push(line.to_string());
            continue;
        }

        if !in_code_block {
            if let Some((level, title)) = parse_markdown_heading(trimmed) {
                let base = slugify_title(&title);
                let id = unique_id(&base, &mut id_counts);

                lines_out.push(format!(r#"<span id="{}"></span>"#, id));
                lines_out.push(line.to_string());

                if (2..=4).contains(&level) {
                    toc_items.push(TocItem { id, title, level });
                }

                continue;
            }
        }

        lines_out.push(line.to_string());
    }

    (lines_out.join("\n"), toc_items)
}

pub fn collect_toc_items(markdown: &str) -> Vec<TocItem> {
    let (_, toc) = inject_heading_anchors_and_collect_toc(markdown);
    toc
}
