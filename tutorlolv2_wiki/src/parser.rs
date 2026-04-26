use crate::{client::MayFail, selector};
use scraper::Html;
use std::collections::BTreeMap;

pub fn get_cells(html: &Html) -> MayFail<BTreeMap<String, String>> {
    let mut cells = BTreeMap::new();

    for tr in html.select(&selector("table.article-table > tbody > tr")?) {
        let tds = tr.select(&selector("td")?).collect::<Vec<_>>();

        if tds.len() == 3 {
            let first = tds.first().ok_or("Failed to find parameter")?;
            let key = first
                .select(&selector("code")?)
                .next()
                .map(|element| element.text())
                .unwrap_or(first.text())
                .collect::<String>();

            let value = tds.get(1).ok_or("Failed to get value")?;
            let text = value.text().collect::<String>();

            if text.trim().is_empty() {
                continue;
            }

            cells.insert(key.clone() + "_raw", value.inner_html());
            cells.insert(key, text.trim().to_string());
        }
    }

    Ok(cells)
}

pub fn parse_lua(text: &str) -> MayFail<String> {
    Ok(Html::parse_document(&text)
        .select(&selector("pre.mw-code.mw-script")?)
        .next()
        .ok_or("Failed to select <pre> tag")?
        .text()
        .collect::<String>()
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n"))
}
