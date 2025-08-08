// Build script for mandel.rs build

use chrono::{TimeZone, Utc};
use semver::{BuildMetadata, Prerelease, Version};
use std::{env, path::Path, process::Command};

struct GitInfo {
    branch_name: String,
    head_id: String,
    clean_workspace: bool,
}

fn get_git_info() -> Option<GitInfo> {
    let repo_path = env::var("CARGO_MANIFEST_DIR").ok()?;
    let repo = gix::discover(Path::new(&repo_path)).ok()?;
    let head = repo.head().ok()?;

    // Get the branch name
    let branch_name = head.referent_name()?.shorten().to_string();

    // Get the shortened commit id
    let raw_head_id = head.id()?;
    let head_id = raw_head_id.shorten_or_id().to_string();

    // Check whether the workspace is clean
    // Using git command line tool as the gix API is completely un-understandable…
    let clean_workspace = Command::new("git")
        .args(["diff", "--quiet"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
        && Command::new("git")
            .args(["diff", "--cached", "--quiet"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

    Some(GitInfo {
        branch_name,
        head_id,
        clean_workspace,
    })
}

fn get_full_version(git_info: GitInfo, date: &str) -> Option<String> {
    let branch = git_info.branch_name.as_str();
    let raw_version = env::var("CARGO_PKG_VERSION").ok()?;
    if branch == "master" || branch == "main" {
        Some(raw_version)
    } else {
        let v_version = Version::parse(raw_version.as_str()).ok()?;
        let mut pre = String::new();
        if branch != "next" {
            pre.push_str(branch);
            pre.push('-');
        }
        pre.push_str(date);
        pre.push('-');
        pre.push_str(&git_info.head_id);
        if !git_info.clean_workspace {
            pre.push('-');
            pre.push('X');
        }
        Some(
            Version {
                major: v_version.major,
                minor: v_version.minor,
                patch: v_version.patch + 1,
                pre: Prerelease::new(pre.as_str()).ok()?,
                build: BuildMetadata::EMPTY,
            }
            .to_string(),
        )
    }
}

fn main() {
    let now = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap(),
        Err(_) => Utc::now(),
    };
    let date_string = now.format("%Y%m%d%H%M%S").to_string();
    let full_version = get_git_info()
        .and_then(|i| get_full_version(i, &date_string))
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo::rustc-env=MANDEL_FULL_VERSION={}", full_version);

    // Tell Cargo to re-run if .git/HEAD changes (branch switches)
    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rerun-if-changed=.git/refs");
}

// end of file
