// Copyright © 2026 1o. All rights reserved.

//! The words after `differences` read into a declared line ([`crate::surface`]),
//! answered through differences_kit, and said: the page on stdout, what was
//! said about it on stderr, the exit status the line names (0 `same`, 1
//! `different`, 2 `refused`).

use crate::install::{self_line, SelfAsk};
use crate::surface::{read, s, so, usage_lines, Line};
use differences_kit::{Difference, Project};
use std::ffi::OsString;
use std::path::PathBuf;
use tree_history::TreeSpec;

fn project(root: &Option<OsString>) -> Result<Project, String> {
    let start = so(root).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    Project::find(&start).map_err(|e| e.to_string())
}

fn num(v: &Option<OsString>) -> Option<i64> {
    so(v).and_then(|v| v.parse().ok())
}

/// The usage screen.
pub fn usage() -> String {
    let mut out = String::from("differences: every kind of history a project keeps, side by side\n\n");
    for h in usage_lines() {
        out.push_str(&format!("  {}\n", h.words));
    }
    out.push_str("\n`tree A B` is `tree compare A B`; `count A B` is `count compare A B`. Every line takes --root DIR (.custodian there or above).\nexit status: 0 the same (or done), 1 different, 2 refused.\n");
    out
}

/// A line answered: the difference, or what a `self` line said with its code.
pub enum Answer {
    Difference(Difference),
    Said(Vec<String>, i32),
}

/// The line answered.
pub fn answer(line: &Line) -> Result<Answer, String> {
    let kit = |e: differences_kit::Error| e.to_string();
    let d = match line {
        Line::TreeTake(l) => {
            let p = project(&l.root)?;
            let mut spec = TreeSpec::new(so(&l.dir).map(PathBuf::from).unwrap_or_else(|| p.root.clone()));
            spec.level = num(&l.level).map(|n| n.max(0) as u32);
            spec.exclude = l.exclude.iter().map(s).collect();
            spec.with = so(&l.with).map(|w| w.split(',').filter(|x| !x.is_empty()).map(str::to_string).collect()).unwrap_or_default();
            let (snap, receipt) = differences_kit::tree_take(&p, &spec, &s(&l.point)).map_err(kit)?;
            Difference {
                kind: "tree".into(),
                sides: vec![snap.point.clone()],
                page: snap.output.clone(),
                said: vec![format!("tree {}: {} directories, {} files, digest {} ({})", snap.point, snap.directories, snap.files, snap.digest, snap.line), format!("journal: run #{} tree, receipt {}", receipt.seq, receipt.outputs.join(", "))],
                differs: false,
                records: Vec::new(),
            }
        }
        Line::TreeList(l) => differences_kit::trees(&project(&l.root)?).map_err(kit)?,
        Line::TreeCompare(l) => differences_kit::tree(&project(&l.root)?, &s(&l.a), &s(&l.b)).map_err(kit)?,
        Line::CountCompare(l) => {
            let mut d = differences_kit::count(&project(&l.root)?, &s(&l.a), &s(&l.b)).map_err(kit)?;
            if l.records {
                d.page.push_str("\n## change records\n\n");
                for r in &d.records {
                    d.page.push_str(&r.render());
                }
            }
            d
        }
        Line::CountHistory(l) => differences_kit::count_history(&project(&l.root)?, so(&l.route).as_deref()).map_err(kit)?,
        Line::Receipts(l) => {
            let p = project(&l.root)?;
            match (num(&l.a), num(&l.b), num(&l.since)) {
                (Some(a), Some(b), _) => differences_kit::receipts_pair(&p, a, b).map_err(kit)?,
                (_, _, Some(n)) => differences_kit::receipts_since(&p, n).map_err(kit)?,
                _ => return Err("receipts --since N, or receipts A B".into()),
            }
        }
        Line::Revisions(l) => {
            let history = so(&l.history).map(PathBuf::from);
            let p = if history.is_some() && l.root.is_none() { None } else { Some(project(&l.root)?) };
            differences_kit::revisions(p.as_ref(), history.as_deref(), &s(&l.task)).map_err(kit)?
        }
        Line::Changes(l) => {
            let jobs: Vec<String> = l.jobs.iter().map(s).collect();
            differences_kit::changes(&project(&l.root)?, &jobs.iter().map(String::as_str).collect::<Vec<_>>()).map_err(kit)?
        }
        Line::Readings(l) => differences_kit::readings(&project(&l.root)?, &s(&l.spec), num(&l.a), num(&l.b)).map_err(kit)?,
        Line::Tasks(l) => {
            let history = so(&l.history).map(PathBuf::from);
            let p = if history.is_some() && l.root.is_none() { None } else { Some(project(&l.root)?) };
            differences_kit::tasks(p.as_ref(), history.as_deref()).map_err(kit)?
        }
        Line::SelfInstall(l) => return self_line(&SelfAsk { action: "install", from: so(&l.from), bin_dir: so(&l.bin_dir), state_dir: so(&l.state_dir), dry_run: l.dry_run, force: l.force }).map(|(s, c)| Answer::Said(s, c)),
        Line::SelfUninstall(l) => return self_line(&SelfAsk { action: "uninstall", bin_dir: so(&l.bin_dir), state_dir: so(&l.state_dir), force: l.force, ..Default::default() }).map(|(s, c)| Answer::Said(s, c)),
        Line::SelfStatus(l) => return self_line(&SelfAsk { action: "status", bin_dir: so(&l.bin_dir), state_dir: so(&l.state_dir), ..Default::default() }).map(|(s, c)| Answer::Said(s, c)),
    };
    Ok(Answer::Difference(d))
}

/// The program: the words, the line, its answer, its status.
pub fn main_argv(args: &[OsString]) -> i32 {
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") && args.len() == 1 {
        print!("{}", usage());
        return if args.is_empty() { 2 } else { 0 };
    }
    let line = match read(args) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("differences: refused: {e}");
            return 2;
        }
    };
    match answer(&line) {
        Ok(Answer::Difference(d)) => {
            print!("{}", d.page);
            for s in &d.said {
                eprintln!("differences: {s}");
            }
            let code = i32::from(d.differs);
            eprintln!("differences: {} ({})", line.display(), line.status_word(code));
            code
        }
        Ok(Answer::Said(said, code)) => {
            for s in &said {
                println!("{s}");
            }
            code
        }
        Err(e) => {
            eprintln!("differences: refused: {e}");
            eprintln!("differences: {} ({})", line.display(), line.status_word(2));
            2
        }
    }
}
