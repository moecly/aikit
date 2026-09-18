use anyhow::{Context, Result};

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";
const DEFAULT_MODEL: &str = "gpt-4o-mini";
const DEFAULT_COMMIT_LANG: &str = "English";
const MAX_DIFF_CHARS: usize = 12_000;

pub fn max_diff_chars() -> usize {
    MAX_DIFF_CHARS
}

fn env_nonempty(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn mask(key: &str) -> String {
    let tail: String = key
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("****{}", tail)
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub tr_to: Option<String>,
    pub tr_from: Option<String>,
    pub commit_lang: String,
}

impl Config {
    fn base_url() -> (String, &'static str) {
        match env_nonempty("AIKIT_BASE_URL") {
            Some(v) => (v.trim_end_matches('/').to_string(), "env"),
            None => (DEFAULT_BASE_URL.to_string(), "default"),
        }
    }

    fn model_of(model_override: Option<&str>) -> (String, &'static str) {
        if let Some(m) = model_override {
            return (m.to_string(), "flag");
        }
        match env_nonempty("AIKIT_MODEL") {
            Some(v) => (v, "env"),
            None => (DEFAULT_MODEL.to_string(), "default"),
        }
    }

    fn commit_lang() -> (String, &'static str) {
        match env_nonempty("AIKIT_COMMIT_LANG") {
            Some(v) => (v, "env"),
            None => (DEFAULT_COMMIT_LANG.to_string(), "default"),
        }
    }

    pub fn load(model_override: Option<&str>) -> Result<Self> {
        let (base_url, _) = Self::base_url();
        let api_key = env_nonempty("AIKIT_API_KEY")
            .context("AIKIT_API_KEY is not set; write your key into .env.secrets and retry")?;
        let (model, _) = Self::model_of(model_override);
        let tr_to = env_nonempty("AIKIT_TR_TO");
        let tr_from = env_nonempty("AIKIT_TR_FROM");
        let (commit_lang, _) = Self::commit_lang();
        Ok(Self {
            base_url,
            api_key,
            model,
            tr_to,
            tr_from,
            commit_lang,
        })
    }

    pub fn summarize() -> Vec<(String, String)> {
        let (base_url, base_src) = Self::base_url();
        let (model, model_src) = Self::model_of(None);
        let (commit_lang, commit_src) = Self::commit_lang();
        let api_key = match env_nonempty("AIKIT_API_KEY") {
            Some(k) => format!("{} (set)", mask(&k)),
            None => "not set (AIKIT_API_KEY)".to_string(),
        };
        let tr_to = env_nonempty("AIKIT_TR_TO")
            .map(|v| format!("{v} (env)"))
            .unwrap_or_else(|| "auto Chinese-English (default)".to_string());
        let tr_from = env_nonempty("AIKIT_TR_FROM")
            .map(|v| format!("{v} (env)"))
            .unwrap_or_else(|| "auto-detect (default)".to_string());
        vec![
            ("base_url".to_string(), format!("{base_url} ({base_src})")),
            ("model".to_string(), format!("{model} ({model_src})")),
            ("api_key".to_string(), api_key),
            ("tr_to".to_string(), tr_to),
            ("tr_from".to_string(), tr_from),
            (
                "commit_lang".to_string(),
                format!("{commit_lang} ({commit_src})"),
            ),
        ]
    }
}

pub fn truncate(s: &str, limit: usize) -> String {
    if s.len() <= limit {
        return s.to_string();
    }
    let mut end = limit;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...\n[truncated, {} chars total]", &s[..end], s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_short_key() {
        assert_eq!(mask("abc"), "****");
    }

    #[test]
    fn mask_long_key() {
        assert_eq!(mask("sk-1234567890"), "****7890");
    }

    #[test]
    fn truncate_ascii() {
        assert_eq!(truncate("abcdef", 4), "abcd...\n[truncated, 6 chars total]");
        assert_eq!(truncate("ab", 4), "ab");
    }

    #[test]
    fn truncate_keeps_char_boundary() {
        let s = "hello";
        let out = truncate(s, 5);
        assert_eq!(out, "hello");
    }

    #[test]
    fn summarize_has_all_keys() {
        let keys: Vec<String> = Config::summarize().into_iter().map(|(k, _)| k).collect();
        assert_eq!(
            keys,
            vec![
                "base_url",
                "model",
                "api_key",
                "tr_to",
                "tr_from",
                "commit_lang"
            ]
        );
    }
}
