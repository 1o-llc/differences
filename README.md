# differences

Every kind of history a project keeps, side by side, on the command line: `differences <kind> <a> <b>`.

```text
differences tree take POINT [--dir DIR] [--level N] [--exclude PATTERN]... [--with all,dirs,gitignore]
differences tree A B                       the tree at two points (tree compare A B)
differences tree list
differences count A B [--records]          two counts: a receipt number, a job, <job>:open, <job>:close, a commit, a date
differences count history [ROUTE]
differences receipts --since N             the receipts after N, and the specs read again over moved code
differences receipts A B                   two receipts, field by field
differences revisions TASK [--history DIR] a task's revisions side by side, and what each fix changed
differences changes JOB...                 the lines each job touched
differences readings SPEC [SEQ_A SEQ_B]    two readings of one spec (the last two by default)
differences tasks [--history DIR]          every task history
differences self install|uninstall|status
```

Every line takes `--root DIR`: the project, `.custodian/` there or above (the current directory by default). The
page goes to stdout, what was said about it to stderr. Exit status: 0 the same (or done), 1 different, 2 refused.

Each line is declared once with simple_impl_derive over xccute_contract (the `differences` program, its `tree`,
`count` and `self` surfaces, one leaf per line naming its statuses); the words are read into the line through its
own setters, and a word no setter takes is refused. `tree take` runs `tree` itself as a declared line (tree_history)
and keeps its receipt in the project's journal; every other line only reads. The answers are differences_kit's.

## Install

`cargo build --release`, then `target/release/differences self install`: a plan first (every path decided, nothing
written), the install from the plan (copied, its SHA-256 verified, renamed into place, installation's receipt
written); `differences self status` says whether the installed file is still the one its receipt names.

## Releasing

Bump `version` in Cargo.toml, add a dated `## <version> (YYYY-MM-DD)` entry to CHANGELOG.md, and push to
`main`. CI (.github/workflows/ci.yml) runs the tests on stable and Rust 1.89 and clippy, then publishes that
version to crates.io through trusted publishing; a version already on crates.io publishes nothing.

## Ownership

Developed and published by James Hunt. The repository is held by the 1o organization
(https://github.com/1o-llc). Assignment of the copyright to 1o is pending documentation. The license texts
(LICENSE-MIT, LICENSE-APACHE) are left as they were found. Every file new in 0.1.0 carries the notice
Copyright © 2026 1o. All rights reserved.

## License

MIT OR Apache-2.0
