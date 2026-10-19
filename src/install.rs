// Copyright © 2026 1o. All rights reserved.

//! `differences self install|uninstall|status`: differences installs itself
//! through `installation`, as custodian does: a plan first (every path
//! decided, nothing written), the install from the plan (copied, its SHA-256
//! verified, renamed into place, installation's receipt written), and the
//! uninstall from the receipt. Never `cargo install --path`.

use installation::{Installer, Scope};
use std::path::PathBuf;

pub const BIN: &str = "differences";

/// What a `self` line asks: its action and its options.
#[derive(Debug, Clone, Default)]
pub struct SelfAsk {
    pub action: &'static str,
    pub from: Option<String>,
    pub bin_dir: Option<String>,
    pub state_dir: Option<String>,
    pub dry_run: bool,
    pub force: bool,
}

fn installer(l: &SelfAsk) -> Result<Installer, String> {
    match (l.bin_dir.clone(), l.state_dir.clone()) {
        (Some(b), Some(st)) => Installer::user_in(BIN, PathBuf::from(b), PathBuf::from(st)),
        _ => Installer::new(BIN, Scope::User),
    }
    .map_err(|e| e.to_string())
}

fn short(sha: &str) -> &str {
    &sha[..sha.len().min(16)]
}

/// The line answered: what it printed, and its exit code (0 done, 1 the installed file no longer matches its receipt).
pub fn self_line(l: &SelfAsk) -> Result<(Vec<String>, i32), String> {
    let inst = installer(l)?;
    let mut said = Vec::new();
    let mut code = 0;
    match l.action {
        "install" => {
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            let from = l.from.clone().map(PathBuf::from).unwrap_or(exe);
            let from = from.canonicalize().map_err(|e| format!("{}: {e}", from.display()))?;
            let plan = inst.plan(BIN, &from).map_err(|e| e.to_string())?;
            said.push(plan.describe());
            if l.dry_run {
                said.push(format!("planned: {} → {} (--dry-run: nothing written)", plan.source.display(), plan.destination.display()));
                return Ok((said, 0));
            }
            let receipt = inst.install(&plan).map_err(|e| e.to_string())?;
            said.push(format!("installed {} → {} (sha256 {})", receipt.source.display(), receipt.destination.display(), short(&receipt.sha256)));
            said.push(format!("installation's receipt: {}", plan.receipt.display()));
            if !std::env::var_os("PATH").is_some_and(|p| std::env::split_paths(&p).any(|d| d == inst.bin_dir())) {
                said.push(format!("{} is not on PATH in this shell", inst.bin_dir().display()));
            }
        }
        "uninstall" => match inst.uninstall(BIN, l.force).map_err(|e| e.to_string())? {
            Some(r) => said.push(format!("uninstalled {} (the receipt from {})", r.destination.display(), r.installed_at_unix)),
            None => said.push(format!("nothing to uninstall: no receipt for {BIN} in {}", inst.state_dir().display())),
        },
        _ => match inst.receipt(BIN).map_err(|e| e.to_string())? {
            Some(r) => {
                let now = installation::sha256_file(&r.destination).ok();
                let same = now.as_deref() == Some(r.sha256.as_str());
                said.push(format!("installed: {} sha256 {} ({})", r.destination.display(), short(&r.sha256), if same { "the file matches its receipt" } else { "the file no longer matches its receipt" }));
                if !same {
                    code = 1;
                }
            }
            None => {
                said.push(format!("not installed: no receipt for {BIN} in {}", inst.state_dir().display()));
                code = 1;
            }
        },
    }
    Ok((said, code))
}
