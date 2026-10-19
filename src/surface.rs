// Copyright © 2026 1o. All rights reserved.

//! Every line `differences` answers to, declared once with simple_impl_derive
//! over xccute_contract: the `differences` program (a `CompositeShell` root),
//! its surfaces (`tree`, `count`, `self`: each a `CompositeSubCommand`) and
//! one `SimpleSubCommand` leaf per line, each naming its exit statuses (0
//! `same`, 1 `different`, 2 `refused`). A line under a surface is its leaf's
//! nested wrapper (simple_impl_derive 0.2.7, `RootableNestedCommand`), so it
//! keeps its leaf's setters, statuses and rules at every depth. The usage
//! screen is the root's help lines, generated from the same declarations.
//!
//! [`read`] reads the words after `differences` into the declared line
//! through the line's own setters and refuses a word no setter takes; the
//! line is then the authority. Two spellings are read as the line they mean:
//! `tree A B` is `tree compare A B`, `count A B` is `count compare A B`.

use simple_impl_derive::{CompositeShell, CompositeSubCommand, SimpleSubCommand};
use std::ffi::{OsStr, OsString};
use xccute_contract::{CommandHelp, CommandPreview, CommandValidationError, CommandValidationResult, CompositeShellCommand, RootableNestedCommand, ValidatedCommand};
use xccute_runtime::{ExitStatusDecision, ExitStatusPolicy, RuntimeExitStatus};

/// The `differences` program.
#[derive(CompositeShell, Debug, Clone, Copy, Default)]
#[shell(program = "differences")]
#[composite(surface = "tree", ty = TreeSurface)]
#[composite(surface = "count", ty = CountSurface)]
#[composite(command = "receipts", ty = Receipts)]
#[composite(command = "revisions", ty = Revisions, init = "task")]
#[composite(command = "changes", ty = Changes)]
#[composite(command = "readings", ty = Readings, init = "spec")]
#[composite(command = "tasks", ty = Tasks)]
#[composite(surface = "self", method = "self_", ty = SelfSurface)]
pub struct Differences;

/// `differences tree take|list|compare`.
#[derive(CompositeSubCommand, Debug, Clone, Copy, Default)]
#[subcommand(segment = "tree")]
#[composite(command = "take", ty = TreeTake, init = "point")]
#[composite(command = "list", ty = TreeList)]
#[composite(command = "compare", ty = TreeCompare, init = "a,b")]
pub struct TreeSurface;

/// `differences count compare|history`.
#[derive(CompositeSubCommand, Debug, Clone, Copy, Default)]
#[subcommand(segment = "count")]
#[composite(command = "compare", ty = CountCompare, init = "a,b")]
#[composite(command = "history", ty = CountHistory)]
pub struct CountSurface;

/// `differences self install|uninstall|status`.
#[derive(CompositeSubCommand, Debug, Clone, Copy, Default)]
#[subcommand(segment = "self")]
#[composite(command = "install", ty = SelfInstall)]
#[composite(command = "uninstall", ty = SelfUninstall)]
#[composite(command = "status", ty = SelfStatus)]
pub struct SelfSurface;

/// The tree at a point, taken through the declared `tree` line and kept under the point's name.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "take")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct TreeTake {
    /// The point's name: a plain word.
    #[builder(init_required, into)]
    #[arg(positional, each = plain)]
    pub point: OsString,
    /// The directory listed (the project root by default).
    #[builder(into)]
    #[arg(kv = "--dir", each = nonempty)]
    pub dir: Option<OsString>,
    /// How deep.
    #[builder(into)]
    #[arg(kv = "--level", each = number)]
    pub level: Option<OsString>,
    /// A pattern left out, one per `--exclude`.
    #[arg(kv = "--exclude", each = nonempty)]
    pub exclude: Vec<OsString>,
    /// `all`, `dirs`, `gitignore`, comma-separated: hidden files, directories only, what `.gitignore` ignores left out.
    #[builder(into)]
    #[arg(kv = "--with", each = with_word)]
    pub with: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// Every tree point kept, oldest first.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "list")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct TreeList {
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// The tree at two kept points, side by side: the paths each holds that the other does not.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "compare")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct TreeCompare {
    /// The earlier point.
    #[builder(init_required, into)]
    #[arg(positional, each = plain)]
    pub a: OsString,
    /// The later point.
    #[builder(init_required, into)]
    #[arg(positional, each = plain)]
    pub b: OsString,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// Two counts side by side, each named by its receipt number, job (`<job>:open`, `<job>:close`), commit or date.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "compare")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct CountCompare {
    /// The earlier count.
    #[builder(init_required, into)]
    #[arg(positional, each = nonempty)]
    pub a: OsString,
    /// The later count.
    #[builder(init_required, into)]
    #[arg(positional, each = nonempty)]
    pub b: OsString,
    /// The difference as change records too (`nodeplan.1o.diff.v1`, one per file).
    #[arg(flag = "--records")]
    pub records: bool,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// Every count of a route (all routes by default), newest first, each with its commit.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "history")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct CountHistory {
    /// `*`, `crate:<name>` (or a crate's name), `module:<path>`.
    #[builder(into)]
    #[arg(positional, each = nonempty)]
    pub route: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// The receipts since a receipt number, with the specs read again over moved code; or two receipts side by side.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "receipts")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
#[validate(with = "receipts_valid")]
pub struct Receipts {
    /// The earlier receipt.
    #[builder(into)]
    #[arg(positional, each = number)]
    pub a: Option<OsString>,
    /// The later receipt.
    #[builder(into)]
    #[arg(positional, each = number)]
    pub b: Option<OsString>,
    /// Every receipt after this number.
    #[builder(into)]
    #[arg(kv = "--since", each = number)]
    pub since: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// A task's revisions side by side (base, fix1, ...), and what each changed beside the one before it.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "revisions")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct Revisions {
    /// The task's directory name.
    #[builder(init_required, into)]
    #[arg(positional, each = plain)]
    pub task: OsString,
    /// Another history root than the project's (`data/history/`).
    #[builder(into)]
    #[arg(kv = "--history", each = nonempty)]
    pub history: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// The lines each job touched, side by side, and together.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "changes")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct Changes {
    /// The jobs, by id or name.
    #[arg(positional, each = plain)]
    pub jobs: Vec<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// Two readings of one spec side by side (receipt numbers; the last two by default).
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "readings")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct Readings {
    /// The spec's name.
    #[builder(init_required, into)]
    #[arg(positional, each = plain)]
    pub spec: OsString,
    /// The earlier reading's receipt.
    #[builder(into)]
    #[arg(positional, each = number)]
    pub a: Option<OsString>,
    /// The later reading's receipt.
    #[builder(into)]
    #[arg(positional, each = number)]
    pub b: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// Every task history, its revisions and its last result.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "tasks")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
pub struct Tasks {
    /// Another history root than the project's (`data/history/`).
    #[builder(into)]
    #[arg(kv = "--history", each = nonempty)]
    pub history: Option<OsString>,
    /// The project.
    #[builder(into)]
    #[arg(kv = "--root", each = nonempty)]
    pub root: Option<OsString>,
}

/// differences installs itself: a plan first, the install from the plan, installation's receipt.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "install")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
#[validate(with = "install_valid")]
pub struct SelfInstall {
    /// The program to install (the running one by default).
    #[builder(into)]
    #[arg(kv = "--from", each = nonempty)]
    pub from: Option<OsString>,
    #[builder(into)]
    #[arg(kv = "--bin-dir", each = nonempty)]
    pub bin_dir: Option<OsString>,
    #[builder(into)]
    #[arg(kv = "--state-dir", each = nonempty)]
    pub state_dir: Option<OsString>,
    /// The plan only: nothing written.
    #[arg(flag = "--dry-run")]
    pub dry_run: bool,
    #[arg(flag = "--force")]
    pub force: bool,
}

/// differences uninstalls itself, from installation's receipt.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "uninstall")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
#[validate(with = "uninstall_valid")]
pub struct SelfUninstall {
    #[builder(into)]
    #[arg(kv = "--bin-dir", each = nonempty)]
    pub bin_dir: Option<OsString>,
    #[builder(into)]
    #[arg(kv = "--state-dir", each = nonempty)]
    pub state_dir: Option<OsString>,
    #[arg(flag = "--force")]
    pub force: bool,
}

/// Whether the installed program is the one its receipt names.
#[allow(clippy::duplicated_attributes)]
#[derive(SimpleSubCommand, Debug, Clone, Default)]
#[subcommand(segment = "status")]
#[exit_status(code = 0, word = "same")]
#[exit_status(code = 1, word = "different")]
#[exit_status(code = 2, word = "refused")]
#[validate(with = "status_valid")]
pub struct SelfStatus {
    #[builder(into)]
    #[arg(kv = "--bin-dir", each = nonempty)]
    pub bin_dir: Option<OsString>,
    #[builder(into)]
    #[arg(kv = "--state-dir", each = nonempty)]
    pub state_dir: Option<OsString>,
}

// ---- the words each field takes ----

fn refuse(field: &str, msg: String) -> CommandValidationResult {
    Err(CommandValidationError::structural(msg).with_field(field))
}

fn nonempty(field: &str, word: &OsStr) -> CommandValidationResult {
    if word.is_empty() {
        return refuse(field, format!("{field} is empty"));
    }
    Ok(())
}

/// A word that never reads as an option.
fn plain(field: &str, word: &OsStr) -> CommandValidationResult {
    let t = word.to_string_lossy();
    if t.is_empty() || t.starts_with('-') {
        return refuse(field, format!("{t:?} is not a plain {field}"));
    }
    Ok(())
}

fn number(field: &str, word: &OsStr) -> CommandValidationResult {
    if word.to_str().and_then(|w| w.parse::<i64>().ok()).is_none() {
        return refuse(field, format!("{word:?} is not a number"));
    }
    Ok(())
}

/// The tree's option word: its names, comma-separated, checked as the word.
fn with_word(field: &str, word: &OsStr) -> CommandValidationResult {
    match tree_history::TreeWith::from_names(word.to_string_lossy().split(',').filter(|w| !w.is_empty())) {
        Ok(_) => Ok(()),
        Err(e) => refuse(field, e.to_string()),
    }
}

fn receipts_valid(line: &Receipts) -> CommandValidationResult {
    match (&line.a, &line.b, &line.since) {
        (Some(_), Some(_), None) | (None, None, Some(_)) => Ok(()),
        _ => refuse("since", "receipts --since N, or receipts A B".into()),
    }
}

fn dirs_valid(bin: &Option<OsString>, state: &Option<OsString>) -> CommandValidationResult {
    if bin.is_some() != state.is_some() {
        return refuse("bin_dir", "--bin-dir and --state-dir go together".into());
    }
    Ok(())
}

fn install_valid(line: &SelfInstall) -> CommandValidationResult {
    dirs_valid(&line.bin_dir, &line.state_dir)
}

fn uninstall_valid(line: &SelfUninstall) -> CommandValidationResult {
    dirs_valid(&line.bin_dir, &line.state_dir)
}

fn status_valid(line: &SelfStatus) -> CommandValidationResult {
    dirs_valid(&line.bin_dir, &line.state_dir)
}

// ---- reading words into a line ----

/// The words after the subcommand, taken by the line's setters: options first (anywhere), then positionals in order.
pub struct Words {
    rest: Vec<String>,
}

impl Words {
    pub fn new(args: &[String]) -> Words {
        Words { rest: args.to_vec() }
    }

    /// `--name` present: taken.
    pub fn flag(&mut self, name: &str) -> bool {
        match self.rest.iter().position(|w| w == name) {
            Some(i) => {
                self.rest.remove(i);
                true
            }
            None => false,
        }
    }

    /// `--name VALUE`: the value, taken; `--name` last with no value is refused.
    pub fn kv(&mut self, name: &str) -> Result<Option<OsString>, String> {
        match self.rest.iter().position(|w| w == name) {
            Some(i) if i + 1 < self.rest.len() => {
                let v = self.rest.remove(i + 1);
                self.rest.remove(i);
                Ok(Some(v.into()))
            }
            Some(_) => Err(format!("{name} takes a value")),
            None => Ok(None),
        }
    }

    /// Every `--name VALUE`, in order.
    pub fn kvs(&mut self, name: &str) -> Result<Vec<OsString>, String> {
        let mut out = Vec::new();
        while let Some(v) = self.kv(name)? {
            out.push(v);
        }
        Ok(out)
    }

    /// The next word that is not an option.
    pub fn positional(&mut self) -> Option<OsString> {
        let i = self.rest.iter().position(|w| !w.starts_with("--") || w == "-")?;
        Some(self.rest.remove(i).into())
    }

    /// Every remaining word that is not an option.
    pub fn positionals(&mut self) -> Vec<OsString> {
        let mut out = Vec::new();
        while let Some(p) = self.positional() {
            out.push(p);
        }
        out
    }

    fn need(&mut self, what: &str, line: &str) -> Result<OsString, String> {
        self.positional().ok_or_else(|| format!("differences {line} names its {what}"))
    }

    /// Every word was taken, or the first one no setter takes is refused.
    pub fn done(self, line: &str) -> Result<(), String> {
        match self.rest.first() {
            None => Ok(()),
            Some(w) => Err(format!("`{w}` is not a word of `differences {line}`")),
        }
    }
}

/// A leaf as the full line: directly under the root, or under a surface (its nested wrapper).
macro_rules! full {
    ($l:expr, root) => {
        $l.clone().rooted::<Differences>()
    };
    ($l:expr, $surface:ident) => {
        <_ as RootableNestedCommand<Differences, $surface>>::nested($l.clone(), $surface)
    };
}

macro_rules! lines {
    ($($variant:ident($ty:ident) = [$($word:literal),+] in $under:ident => $read:expr;)*) => {
        /// A declared line, read from the words.
        #[derive(Debug, Clone)]
        pub enum Line { $($variant($ty),)* }

        /// Every line's place (its words after `differences`), in order.
        pub const PATHS: &[&str] = &[$(concat!($($word, " "),+),)*];

        /// Every surface the root has, by its segment.
        pub const SURFACES: &[&str] = &["tree", "count", "self"];

        impl Line {
            /// The words that name the line: `tree compare`.
            pub fn path(&self) -> &'static str {
                match self { $(Line::$variant(_) => concat!($($word, " "),+).trim_end(),)* }
            }

            /// The words after the program, as the line writes them.
            pub fn argv(&self) -> Vec<OsString> {
                match self { $(Line::$variant(l) => full!(l, $under).argv(),)* }
            }

            /// `differences tree compare a b`.
            pub fn display(&self) -> String {
                match self { $(Line::$variant(l) => full!(l, $under).build_display(),)* }
            }

            /// The line's own rules.
            pub fn validate(&self) -> CommandValidationResult {
                match self { $(Line::$variant(l) => full!(l, $under).validate(),)* }
            }

            /// The preview xccute makes of it: data, no side effect.
            pub fn preview(&self) -> CommandPreview {
                match self { $(Line::$variant(l) => full!(l, $under).preview(),)* }
            }

            /// What xccute decides after a status, by the statuses the line names.
            pub fn decide(&self, code: i32) -> ExitStatusDecision {
                match self {
                    $(Line::$variant(l) => {
                        let l = full!(l, $under);
                        let accepted = l.ok_codes().contains(&code);
                        ExitStatusPolicy::of(&l).decide(RuntimeExitStatus::new(Some(code), accepted))
                    })*
                }
            }

            /// The word the line names for a status.
            pub fn status_word(&self, code: i32) -> &'static str {
                match self { $(Line::$variant(l) => full!(l, $under).exit_status_of(Some(code)).map_or("unnamed", |s| s.word),)* }
            }
        }

        fn read_path(path: &str, rest: &[String]) -> Result<Line, String> {
            match path {
                $(p if p == concat!($($word, " "),+).trim_end() => {
                    let mut w = Words::new(rest);
                    let read: fn(&mut Words) -> Result<$ty, String> = $read;
                    let line = read(&mut w)?;
                    w.done(p)?;
                    Ok(Line::$variant(line))
                })*
                other => Err(format!("`{other}`; the lines are: {}", PATHS.iter().map(|p| p.trim_end()).collect::<Vec<_>>().join(" · "))),
            }
        }
    };
}

lines! {
    TreeTake(TreeTake) = ["tree", "take"] in TreeSurface => |w| Ok(TreeTake { dir: w.kv("--dir")?, level: w.kv("--level")?, exclude: w.kvs("--exclude")?, with: w.kv("--with")?, root: w.kv("--root")?, point: w.need("point", "tree take")? });
    TreeList(TreeList) = ["tree", "list"] in TreeSurface => |w| Ok(TreeList { root: w.kv("--root")? });
    TreeCompare(TreeCompare) = ["tree", "compare"] in TreeSurface => |w| { let root = w.kv("--root")?; Ok(TreeCompare { a: w.need("two points: tree A B", "tree compare")?, b: w.need("two points: tree A B", "tree compare")?, root }) };
    CountCompare(CountCompare) = ["count", "compare"] in CountSurface => |w| { let (records, root) = (w.flag("--records"), w.kv("--root")?); Ok(CountCompare { a: w.need("two counts: count A B", "count compare")?, b: w.need("two counts: count A B", "count compare")?, records, root }) };
    CountHistory(CountHistory) = ["count", "history"] in CountSurface => |w| Ok(CountHistory { root: w.kv("--root")?, route: w.positional() });
    Receipts(Receipts) = ["receipts"] in root => |w| Ok(Receipts { since: w.kv("--since")?, root: w.kv("--root")?, a: w.positional(), b: w.positional() });
    Revisions(Revisions) = ["revisions"] in root => |w| Ok(Revisions { history: w.kv("--history")?, root: w.kv("--root")?, task: w.need("task", "revisions")? });
    Changes(Changes) = ["changes"] in root => |w| Ok(Changes { root: w.kv("--root")?, jobs: w.positionals() });
    Readings(Readings) = ["readings"] in root => |w| Ok(Readings { root: w.kv("--root")?, spec: w.need("spec", "readings")?, a: w.positional(), b: w.positional() });
    Tasks(Tasks) = ["tasks"] in root => |w| Ok(Tasks { history: w.kv("--history")?, root: w.kv("--root")? });
    SelfInstall(SelfInstall) = ["self", "install"] in SelfSurface => |w| Ok(SelfInstall { from: w.kv("--from")?, bin_dir: w.kv("--bin-dir")?, state_dir: w.kv("--state-dir")?, dry_run: w.flag("--dry-run"), force: w.flag("--force") });
    SelfUninstall(SelfUninstall) = ["self", "uninstall"] in SelfSurface => |w| Ok(SelfUninstall { bin_dir: w.kv("--bin-dir")?, state_dir: w.kv("--state-dir")?, force: w.flag("--force") });
    SelfStatus(SelfStatus) = ["self", "status"] in SelfSurface => |w| Ok(SelfStatus { bin_dir: w.kv("--bin-dir")?, state_dir: w.kv("--state-dir")? });
}

/// The usage screen's lines: the root's help lines, generated from the declarations.
pub fn usage_lines() -> Vec<xccute_contract::HelpLine> {
    Differences::help_lines()
}

/// The commands a surface holds.
fn commands_of(surface: &str) -> Vec<&'static str> {
    PATHS.iter().filter_map(|p| p.trim_end().split_once(' ').filter(|(s, _)| *s == surface).map(|(_, c)| c)).collect()
}

/// The words after `differences`, as a declared line, or why not.
pub fn read(args: &[OsString]) -> Result<Line, String> {
    let words: Vec<String> = args.iter().map(|a| a.to_str().map(str::to_owned)).collect::<Option<Vec<_>>>().ok_or("a word is not UTF-8")?;
    let (seg, rest) = words.split_first().ok_or("no subcommand")?;
    let line = if SURFACES.contains(&seg.as_str()) {
        let commands = commands_of(seg);
        match rest.first() {
            Some(c) if commands.contains(&c.as_str()) => read_path(&format!("{seg} {c}"), &rest[1..])?,
            // `tree A B` and `count A B`: the comparison they mean
            Some(_) if seg == "tree" || seg == "count" => read_path(&format!("{seg} compare"), rest)?,
            Some(c) => return Err(format!("`differences {seg} {c}`: {seg} takes {}", commands.join(", "))),
            None => return Err(format!("`differences {seg}` names its action: {}", commands.join(", "))),
        }
    } else {
        read_path(seg, rest)?
    };
    line.validate().map_err(|e| e.to_string())?;
    Ok(line)
}

/// A field as a string.
pub fn s(v: &OsString) -> String {
    v.to_string_lossy().into_owned()
}

/// An optional field as a string.
pub fn so(v: &Option<OsString>) -> Option<String> {
    v.as_ref().map(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(line: &str) -> Vec<OsString> {
        line.split_whitespace().map(OsString::from).collect()
    }

    /// Each line, written by its builder, reads back to the same words.
    #[test]
    fn every_line_reads_back_from_its_own_words() {
        let lines: Vec<Vec<OsString>> = vec![
            Differences::tree().take("base").dir("/tmp/x").level("2").exclude(["target", ".git"]).with("all,gitignore").root("/tmp/p").argv(),
            Differences::tree().list().root("/tmp/p").argv(),
            Differences::tree().compare("a", "b").root("/tmp/p").argv(),
            Differences::count().compare("19", "32").records().root("/tmp/p").argv(),
            Differences::count().history().route("*").argv(),
            Differences::receipts().since("90").argv(),
            Differences::receipts().a("73").b("77").argv(),
            Differences::revisions("first-reading").history("/tmp/h").argv(),
            Differences::changes().jobs(["j-1", "j-2"]).argv(),
            Differences::readings("env").a("3").b("9").argv(),
            Differences::tasks().history("/tmp/h").argv(),
            Differences::self_().install().from("target/release/differences").bin_dir("b").state_dir("s").dry_run().force().argv(),
            Differences::self_().uninstall().force().argv(),
            Differences::self_().status().argv(),
        ];
        let mut paths: Vec<&str> = lines.iter().map(|l| read(l).unwrap_or_else(|e| panic!("{l:?}: {e}")).path()).collect();
        paths.sort();
        paths.dedup();
        assert_eq!(paths.len(), PATHS.len(), "every line");
        for argv in lines {
            let back = read(&argv).unwrap_or_else(|e| panic!("{argv:?}: {e}"));
            assert_eq!(back.argv(), argv);
        }
        assert_eq!(usage_lines().len(), PATHS.len());
    }

    #[test]
    fn the_short_spellings_and_the_refusals() {
        assert_eq!(read(&words("tree base fix1")).unwrap().display(), "differences tree compare base fix1");
        assert_eq!(read(&words("count 19 32 --records")).unwrap().display(), "differences count compare 19 32 --records");
        assert!(read(&words("receipts --since x")).is_err());
        assert!(read(&words("receipts 3")).is_err(), "one receipt is not two");
        assert!(read(&words("receipts 3 4 --since 2")).is_err());
        assert!(read(&words("tree take p --with loud")).unwrap_err().contains("loud"));
        assert!(read(&words("tree take ../p")).is_ok(), "the point's name is the kit's to refuse");
        assert!(read(&words("self install --bin-dir b")).is_err());
        assert!(read(&words("moods")).unwrap_err().contains("the lines are: tree take"));
        assert!(read(&words("readings env --loud")).unwrap_err().contains("`--loud` is not a word of `differences readings`"));
        let l = read(&words("tasks")).unwrap();
        assert_eq!((l.status_word(0), l.status_word(1), l.status_word(2)), ("same", "different", "refused"));
    }
}
