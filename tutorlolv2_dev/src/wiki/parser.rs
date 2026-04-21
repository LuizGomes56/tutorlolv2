use crate::wiki::model::{
    AbilityBlockRaw, AbilityFetchJob, AbilityTemplatePageRaw, ChampionTemplatePageRaw, FormulaKind,
    ParameterEntry, WikiLink,
};
use crate::{JsonRead, JsonWrite, wiki::node::NodePool};
use crate::{
    MayFail,
    client::{HttpClient, SaveTo},
    wiki::model::ChampionRaw,
};
use mlua::{Lua, LuaSerdeExt, Value};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use scraper::{Html, Selector};
use std::{collections::BTreeMap, process::Command};
use tokio::sync::OnceCell;
use tutorlolv2_gen::Key;

static NODE_POOL: OnceCell<NodePool> = OnceCell::const_new();

pub async fn node_pool() -> &'static NodePool {
    NODE_POOL
        .get_or_init(|| async {
            NodePool::new(5)
                .await
                .expect("[pool] Failed to initialize NodePool")
        })
        .await
}

impl HttpClient {
    pub fn node_fetch(&self, path: &str, module: &str) -> MayFail {
        if std::fs::exists(path)? {
            println!("[exists] {path}");
            return Ok(());
        }

        let url = format!("https://wiki.leagueoflegends.com/en-us/{module}");

        println!("[fetch] {url}");

        let js = format!(
            r#"
            const {{ writeFileSync }} = require("fs");

            (async function () {{
                try {{
                    const r = await fetch("{url}");
                    const t = await r.text();

                    console.log("[node] status:", r.status);
                    console.log("[node] final_url:", r.url);

                    if (!r.ok) {{
                        console.error("[node] request failed with status", r.status);
                        process.exit(1);
                    }}

                    writeFileSync("{path}", t, "utf8");
                }} catch (err) {{
                    console.error("[node] fetch error:", err);
                    process.exit(1);
                }}
            }})();
        "#
        );

        let output = Command::new("node")
            .arg("-e")
            .arg(js)
            .output()
            .map_err(|e| format!("Failed to execute node: {e:?}"))?;

        if !output.stdout.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        }

        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if !output.status.success() {
            return Err(format!(
                "Node exited with status {} while fetching {url}",
                output.status
            )
            .into());
        }

        if !std::fs::exists(path)? {
            return Err(
                format!("Node finished successfully, but file was not created: {path}").into(),
            );
        }

        Ok(())
    }

    pub async fn build_wiki_abilities(&self) -> MayFail {
        self.download_wiki_abilities().await?;
        self.parse_wiki_abilities()?;
        Ok(())
    }

    pub fn parse_ability_template_page_raw(
        html: &str,
        champion_name: &str,
        slot: Key,
        variant: usize,
        ability_name: &str,
    ) -> MayFail<AbilityTemplatePageRaw> {
        todo!()
    }

    pub fn parse_wiki_abilities(&self) -> MayFail {
        std::fs::create_dir_all("cache/templates/abilities")?;

        let jobs = Self::build_ability_jobs()?;

        for job in jobs {
            if !std::fs::exists(&job.html_path)? {
                println!("[skip] missing html {}", job.html_path);
                continue;
            }

            let html = std::fs::read_to_string(&job.html_path)?;

            let parsed = Self::parse_ability_template_page_raw(
                &html,
                &job.champion_name,
                job.slot,
                job.variant,
                &job.ability_name,
            )?;

            parsed.into_file(&job.json_path)?;
            println!("[parsed] {}", job.json_path);
        }

        Ok(())
    }

    pub async fn download_wiki_abilities(&self) -> MayFail {
        std::fs::create_dir_all("cache/wiki/abilities")?;
        std::fs::create_dir_all("cache/templates/abilities")?;

        let jobs = Self::build_ability_jobs()?;

        for job in jobs.into_iter() {
            if std::fs::exists(&job.html_path)? {
                println!("[exists] {}", job.html_path);
                return Ok(());
            }

            let url = format!("https://wiki.leagueoflegends.com/en-us/{}", job.module);
            println!(
                "[fetch] {} {} {}",
                job.apiname,
                job.slot.as_char(),
                job.ability_name
            );

            let result = node_pool()
                .await
                .fetch(url.clone(), job.html_path.clone())
                .await
                .map_err(|e| format!("[error] Failed to poll {url}: {e:?}"))?;

            if !result.ok {
                return Err(format!(
                    "failed to fetch {}: {}",
                    job.module,
                    result.error.unwrap_or_else(|| "unknown error".into())
                )
                .into());
            }
        }

        Ok(())
    }

    pub fn wiki_cache(&self) -> MayFail<String> {
        let path = "./cache/wiki/champions.html";

        self.node_fetch(path, "Module:ChampionData/data")?;

        let text = std::fs::read_to_string(path)?;
        let html = Html::parse_document(&text);

        let pre_selector = Selector::parse("pre.mw-code.mw-script")
            .map_err(|e| format!("Failed to parse <pre> selector: {e:?}"))?;

        let pre = html
            .select(&pre_selector)
            .next()
            .ok_or("Failed to select <pre> tag")?
            .text()
            .collect::<String>();

        std::fs::write("cache/wiki/champions_lua.txt", &pre)?;

        Ok(pre)
    }

    pub fn build_ability_jobs() -> MayFail<Vec<AbilityFetchJob>> {
        let champions = BTreeMap::<String, ChampionRaw>::from_file("cache/wiki/champions.json")?;
        let mut out = Vec::new();

        for (champion_name, raw) in champions {
            let groups = [
                (Key::P, &raw.skill_i),
                (Key::Q, &raw.skill_q),
                (Key::W, &raw.skill_w),
                (Key::E, &raw.skill_e),
                (Key::R, &raw.skill_r),
            ];

            for (slot, skills) in groups {
                for (variant, ability_name) in skills.iter().enumerate() {
                    let module = format!(
                        "Template:Data_{}/{}",
                        wiki_title(&champion_name),
                        wiki_title(ability_name),
                    );

                    let path = format!(
                        "cache/wiki/abilities/{}_{}_{}.html",
                        raw.apiname,
                        slot.as_char(),
                        variant
                    );

                    out.push(AbilityFetchJob {
                        champion_name: champion_name.clone(),
                        slot,
                        variant,
                        ability_name: ability_name.clone(),
                        module,
                        apiname: raw.apiname.clone(),
                        html_path: path,
                        json_path: format!(
                            "cache/templates/abilities/{}_{}_{}.json",
                            raw.apiname,
                            slot.as_char(),
                            variant
                        ),
                    });
                }
            }
        }

        Ok(out)
    }

    pub async fn download_wiki_champions(&self) -> MayFail {
        let champions = BTreeMap::<String, ChampionRaw>::from_file("cache/wiki/champions.json")?;

        for (name, raw) in champions
            .into_iter()
            .filter(|(name, _)| name != "Mega Gnar")
        {
            println!("[download] {name}");
            let ChampionRaw { apiname, .. } = raw;

            let path = format!("cache/wiki/champions/{apiname}.html");
            let url = format!("https://wiki.leagueoflegends.com/en-us/Template:Data_{name}");
            let job_result = node_pool()
                .await
                .fetch(url, path)
                .await
                .map_err(|e| format!("[error] Failed to poll {name}: {e:?}"))?;

            println!("[download] {name} result: {job_result:?}");
        }

        Ok(())
    }
}

fn file_safe(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn parse_champions_lua() -> MayFail {
    let src = std::fs::read_to_string("cache/wiki/champions_lua.txt")?;

    let lua = Lua::new();

    let cleaned = src
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n");

    let value = lua.load(&cleaned).eval::<Value>()?;
    lua.from_value::<BTreeMap<String, ChampionRaw>>(value)?
        .into_file("cache/wiki/champions.json")
}

pub fn parse_champion_templates() -> MayFail {
    std::fs::read_dir("cache/wiki/champions")
        .map_err(|e| format!("[error] Unable to read directory path in fn [parallel_read]: {e:?}"))?
        .filter_map(Result::ok)
        .par_bridge()
        .into_par_iter()
        .try_for_each(|entry| {
            let Ok(file_name) = entry.file_name().into_string() else {
                panic!("[error] Failed to get file name for entry: {entry:?}");
            };

            println!("[parallel] Processing {file_name:?}");

            let Ok(html) = std::fs::read_to_string(entry.path()) else {
                panic!("[error] Failed to read file bytes for entry: {entry:?}");
            };

            let name = file_name.trim_end_matches(".html");

            match parse_champion_template_page(&html, name) {
                Err(e) => Err(format!("[error] Failed to parse {name}: {e:?}")),
                Ok(data) => data
                    .into_file(format!("cache/wiki/templates/champions/{name}.json"))
                    .map_err(|e| format!("[error] Failed to serialize template for {name}: {e:?}")),
            }
        })
        .map_err(|e| format!("Failed to run tasks in parallel: {e:?}").into())
}

pub fn parse_champion_template_page(
    html: &str,
    champion_name: &str,
) -> MayFail<ChampionTemplatePageRaw> {
    let doc = Html::parse_document(html);

    let title_sel = Selector::parse("h1")?;
    let table_sel = Selector::parse("table")?;
    let tr_sel = Selector::parse("tr")?;
    let th_sel = Selector::parse("th")?;
    let td_sel = Selector::parse("td")?;
    let a_sel = Selector::parse("a")?;

    let page_title = doc
        .select(&title_sel)
        .next()
        .map(|x| x.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let mut parameters_by_section: BTreeMap<String, BTreeMap<String, ParameterEntry>> =
        BTreeMap::new();

    let mut current_section = String::from("Uncategorized");

    for table in doc.select(&table_sel) {
        let rows = table.select(&tr_sel).collect::<Vec<_>>();

        if rows.is_empty() {
            continue;
        }

        let header_text = rows[0]
            .text()
            .collect::<String>()
            .replace('\n', " ")
            .trim()
            .to_string();

        if !header_text.contains("Parameter") || !header_text.contains("Value") {
            continue;
        }

        for row in rows.into_iter().skip(1) {
            let ths: Vec<_> = row.select(&th_sel).collect();
            let tds: Vec<_> = row.select(&td_sel).collect();

            let mut cells = Vec::new();

            for cell in ths.iter().chain(tds.iter()) {
                let text = cell.text().collect::<String>().trim().to_string();
                let html = cell.inner_html();

                let links = cell
                    .select(&a_sel)
                    .filter_map(|a| {
                        let href = a.value().attr("href")?.to_string();
                        let text = a.text().collect::<String>().trim().to_string();
                        Some(WikiLink { text, href })
                    })
                    .collect::<Vec<_>>();

                if !text.is_empty() || !html.trim().is_empty() {
                    cells.push((text, html, links));
                }
            }

            if cells.len() == 1 {
                current_section = cells[0].0.clone();
                parameters_by_section
                    .entry(current_section.clone())
                    .or_default();
                continue;
            }

            if cells.len() >= 3 {
                let parameter = cells[0].0.clone();
                let value_text = cells[1].0.clone();
                let value_html = cells[1].1.clone();
                let description_text = cells[2].0.clone();
                let description_html = cells[2].1.clone();

                let mut links = Vec::new();
                links.extend(cells[0].2.clone());
                links.extend(cells[1].2.clone());
                links.extend(cells[2].2.clone());

                let entry = ParameterEntry {
                    parameter: parameter.clone(),
                    value_text,
                    value_html,
                    description_text,
                    description_html,
                    links,
                };

                parameters_by_section
                    .entry(current_section.clone())
                    .or_default()
                    .insert(parameter, entry);
            }
        }
    }

    Ok(ChampionTemplatePageRaw {
        champion_name: champion_name.to_string(),
        page_title,
        pages: Vec::new(),
        categories: Vec::new(),
        ability_subpages: Vec::new(),
        parameters_by_section,
    })
}

fn wiki_title(s: &str) -> String {
    s.replace(' ', "_")
}

pub fn extract_ability_blocks(raw: &AbilityTemplatePageRaw) -> Vec<AbilityBlockRaw> {
    let mut out = Vec::new();

    for suffix in std::iter::once("".to_string()).chain((0..=10).map(|i| i.to_string())) {
        let desc_key = format!("description{suffix}");
        let lvl_key = format!("leveling{suffix}");

        let description = raw.parameters.get(&desc_key).map(|x| x.value_text.clone());
        let leveling_text = raw.parameters.get(&lvl_key).map(|x| x.value_text.clone());
        let leveling_html = raw.parameters.get(&lvl_key).map(|x| x.value_html.clone());

        if description.is_some() || leveling_text.is_some() || leveling_html.is_some() {
            out.push(AbilityBlockRaw {
                suffix,
                description,
                leveling_text,
                leveling_html,
            });
        }
    }

    out
}

pub fn classify_formula_kind(label: &str) -> FormulaKind {
    let x = label.to_ascii_lowercase();

    if x.contains("damage") {
        FormulaKind::Damage
    } else if x.contains("shield") {
        FormulaKind::Shield
    } else if x.contains("heal") {
        FormulaKind::Heal
    } else if x.contains("cost") {
        FormulaKind::Cost
    } else if x.contains("cooldown") {
        FormulaKind::Cooldown
    } else {
        FormulaKind::Other
    }
}
