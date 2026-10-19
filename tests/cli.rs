// Copyright © 2026 1o. All rights reserved.

//! The program on a small project: two tree points taken and compared, the
//! receipts the tree lines left, the task histories, and a refusal.

use std::path::PathBuf;
use std::process::Command;

fn project() -> PathBuf {
    let d = std::env::temp_dir().join(format!("differences-cli-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(d.join(".custodian/config")).unwrap();
    std::fs::write(d.join(".custodian/config/custodian.toml"), "mode = \"plain\"\n").unwrap();
    std::fs::create_dir_all(d.join("src")).unwrap();
    std::fs::write(d.join("src/lib.rs"), "").unwrap();
    d
}

fn run(root: &PathBuf, words: &[&str]) -> (i32, String, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_differences")).args(words).current_dir(root).output().unwrap();
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned())
}

#[test]
fn two_tree_points_and_their_receipts() {
    if !Command::new("tree").arg("--version").output().is_ok_and(|o| o.status.success()) {
        eprintln!("tree is not on this machine; skipped");
        return;
    }
    let root = project();
    let (code, out, err) = run(&root, &["tree", "take", "one", "--exclude", ".custodian"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with(".\n`-- src\n    `-- lib.rs\n"), "{out}");
    assert!(err.contains("journal: run #1 tree"), "{err}");
    std::fs::write(root.join("src/two.rs"), "").unwrap();
    assert_eq!(run(&root, &["tree", "take", "two", "--exclude", ".custodian"]).0, 0);
    let (code, out, err) = run(&root, &["tree", "one", "two"]);
    assert_eq!(code, 1, "different: {err}");
    assert!(out.contains("| | one | two |") && out.contains("+ src/two.rs"), "{out}");
    assert!(err.contains("differences tree compare one two (different)"), "{err}");
    let (code, out, _) = run(&root, &["receipts", "--since", "0"]);
    assert_eq!(code, 1);
    assert!(out.contains("| #2 |") && out.contains("| tree |"), "{out}");
    let (code, _, err) = run(&root, &["tree", "one", "one"]);
    assert_eq!(code, 0, "{err}");
    let (code, _, err) = run(&root, &["tree", "take", "one"]);
    assert_eq!(code, 2, "a point is named once: {err}");
    let (code, _, err) = run(&root, &["moods"]);
    assert_eq!(code, 2);
    assert!(err.contains("the lines are: tree take"), "{err}");
}
