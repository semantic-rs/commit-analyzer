extern crate clog;

use clog::Clog;
use clog::error::Error;

#[derive(PartialEq,Eq,Debug)]
pub enum CommitType {
    Unknown,
    Major,
    Minor,
    Patch,
}

use CommitType::*;

pub fn analyze(commits: &[&str]) -> Result<CommitType,Error> {
    let mut commit_type = Unknown;

    let clog = try!(Clog::new());

    commits
        .iter()
        .map(|message| clog.parse_raw_commit(message))
        .all(|commit| {
            if commit.breaks.len() > 0 {
                commit_type = Major;
                return false;
            }

            if commit.commit_type == "feat" {
                commit_type = Minor;
            }

            if commit_type == Unknown && commit.commit_type == "fix" {
                commit_type = Patch;
            }

            return true;
        });

    Ok(commit_type)
}

#[test]
fn unknown_type() {
    let commits = vec!["This commit message has no type"];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}

#[test]
fn patch_commit() {
    let commits = vec!["fix: This commit fixes a bug"];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}

#[test]
fn minor_commit() {
    let commits = vec!["feat: This commit introduces a new feature"];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}

#[test]
fn major_commit() {
    let commits = vec!["feat: This commits breaks something\nBREAKING CHANGE: breaks things"];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}

#[test]
fn major_commit_multiple() {
    let commits = vec![
        "feat: This commits breaks something\r\n\r\nBREAKING CHANGE: breaks things",
        "fix: Simple fix"
    ];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}

#[test]
fn minor_commit_multiple() {
    let commits = vec![
        "feat: This commits introduces a new feature",
        "fix: Simple fix"
    ];
    assert_eq!(Unknown, analyze(&commits).unwrap());
}
