use std::io::{IsTerminal, Write as _};
use std::process::Command;

use anyhow::{Context, Result};

use crate::config::{self, Config};
use crate::llm;

fn git(args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .output()
        .context("failed to run git")?;
    if !out.status.success() {
        anyhow::bail!("git {} failed", args.join(" "));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn git_apply(args: &[&str]) -> Result<()> {
    let st = Command::new("git")
        .args(args)
        .status()
        .context("failed to run git")?;
    if !st.success() {
        anyhow::bail!("git {} failed", args.join(" "));
    }
    Ok(())
}

pub fn split_message(msg: &str) -> (String, String) {
    let mut lines = msg.lines();
    let subject = lines.next().unwrap_or("").trim().to_string();
    let body: String = lines.collect::<Vec<_>>().join("\n").trim().to_string();
    (subject, body)
}

pub async fn run(apply: bool, staged: bool, unstaged: bool, model: Option<String>) -> Result<()> {
    if staged && unstaged {
        anyhow::bail!("--staged and --unstaged are mutually exclusive");
    }
    let scope_args: Vec<&str> = if staged {
        vec!["--cached"]
    } else if unstaged {
        vec![]
    } else {
        vec!["HEAD"]
    };
    let diff_args: Vec<&str> = ["diff"].into_iter().chain(scope_args.clone()).collect();
    let stat_args: Vec<&str> = ["diff", "--stat"].into_iter().chain(scope_args).collect();
    let status = git(&["status", "--short"])?;
    if status.trim().is_empty() {
        println!("working tree clean, nothing to commit.");
        return Ok(());
    }
    let (stat, diff) = match (git(&stat_args), git(&diff_args)) {
        (Ok(stat), Ok(diff)) => (stat, diff),
        // fresh repos have no HEAD to diff against, fall back to staged plus unstaged diffs
        _ => (
            git(&["diff", "--cached", "--stat"])? + &git(&["diff", "--stat"])?,
            git(&["diff", "--cached"])? + &git(&["diff"])?,
        ),
    };
    let untracked = git(&["ls-files", "--others", "--exclude-standard"])?;
    let user = format!(
        "Branch status:\n{status}\nDiff stat:\n{stat}\nUntracked files:\n{untracked}\nDiff content:\n{}",
        config::truncate(&diff, config::max_diff_chars())
    );
    let cfg = Config::load(model.as_deref())?;
    let system = format!(
        "You write git commit messages. Write the message in {}. Reply with the subject line first, \
        then a blank line, then the body. Subject: imperative mood, lowercase start, no period, max 50 chars, \
        format <type>(<scope>): <subject>. Type must be one of feat, fix, refactor, perf, chore, docs, test, \
        style, build, ci. Body: one bullet per change area, each bullet must name the concrete files, modules, \
        flags, or env vars involved and explain WHY the change was made. Be specific and detailed; never write \
        generic bullets without file or symbol references. No code fences.",
        cfg.commit_lang
    );
    let msg = llm::chat(&cfg, &system, &user).await?;
    println!("{msg}");
    if apply {
        do_commit(&msg)?;
    } else if std::io::stdin().is_terminal() {
        eprint!("commit with this message? [y/N] ");
        std::io::stderr().flush().ok();
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .context("failed to read answer")?;
        if confirmed(&answer) {
            do_commit(&msg)?;
        } else {
            eprintln!("aborted.");
        }
    }
    Ok(())
}

fn confirmed(answer: &str) -> bool {
    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}

fn do_commit(msg: &str) -> Result<()> {
    let (subject, body) = split_message(msg);
    if subject.is_empty() {
        anyhow::bail!("model returned an empty message, aborting");
    }
    if body.is_empty() {
        git_apply(&["commit", "-m", &subject])?;
    } else {
        git_apply(&["commit", "-m", &subject, "-m", &body])?;
    }
    eprintln!("committed.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_subject_body() {
        let (s, b) = split_message(
            "feat(tr): support pipe input\n\n- pipe composes well\n- covers combined use",
        );
        assert_eq!(s, "feat(tr): support pipe input");
        assert!(b.contains("pipe"));
    }

    #[test]
    fn confirm_answers() {
        assert!(confirmed("y"));
        assert!(confirmed("Y\n"));
        assert!(confirmed("yes"));
        assert!(!confirmed(""));
        assert!(!confirmed("n"));
        assert!(!confirmed("yeah"));
    }

    #[test]
    fn split_subject_only() {
        let (s, b) = split_message("fix(commit): handle empty diff");
        assert_eq!(s, "fix(commit): handle empty diff");
        assert!(b.is_empty());
    }
}
