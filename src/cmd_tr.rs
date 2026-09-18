use std::io::{IsTerminal, Read};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::llm;

pub fn contains_cjk(s: &str) -> bool {
    s.chars().any(|c| matches!(c,
        '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{3040}'..='\u{30FF}' | '\u{AC00}'..='\u{D7AF}'
    ))
}

pub fn default_target(input: &str) -> &'static str {
    if contains_cjk(input) { "en" } else { "zh" }
}

fn lang_name(code: &str) -> &str {
    match code.to_lowercase().as_str() {
        "zh" | "cn" | "chinese" => "Chinese",
        "en" | "english" => "English",
        "ja" | "jp" | "japanese" => "Japanese",
        "ko" | "korean" => "Korean",
        "fr" | "french" => "French",
        "de" | "german" => "German",
        "es" | "spanish" => "Spanish",
        "ru" | "russian" => "Russian",
        _ => code,
    }
}

fn read_input(text: Vec<String>) -> Result<String> {
    let joined = text.join(" ");
    if !joined.trim().is_empty() {
        return Ok(joined);
    }
    if std::io::stdin().is_terminal() {
        anyhow::bail!("no input: pass text as arguments or pipe it via stdin");
    }
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .context("failed to read piped stdin")?;
    if buf.trim().is_empty() {
        anyhow::bail!("no input: pass text as arguments or pipe it via stdin");
    }
    Ok(buf)
}

pub async fn run(
    text: Vec<String>,
    to: Option<String>,
    from: Option<String>,
    model: Option<String>,
) -> Result<()> {
    let input = read_input(text)?;
    let cfg = Config::load(model.as_deref())?;
    let target = to
        .or(cfg.tr_to.clone())
        .unwrap_or_else(|| default_target(&input).to_string());
    let source = from.or(cfg.tr_from.clone());
    let system = "You are a translator. Output only the translation, no explanations.";
    let user = match source {
        Some(f) => format!(
            "Translate the following text from {} to {}. Keep formatting:\n{}",
            lang_name(&f),
            lang_name(&target),
            input
        ),
        None => format!(
            "Translate the following text to {}. Keep formatting:\n{}",
            lang_name(&target),
            input
        ),
    };
    let out = llm::chat(&cfg, system, &user).await?;
    println!("{out}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_direction() {
        assert_eq!(default_target("hello"), "zh");
        assert_eq!(default_target("\u{4f60}\u{597d}"), "en");
        assert_eq!(default_target("hello \u{4f60}\u{597d}"), "en");
    }

    #[test]
    fn lang_names() {
        assert_eq!(lang_name("en"), "English");
        assert_eq!(lang_name("ZH"), "Chinese");
        assert_eq!(lang_name("xx"), "xx");
    }
}
