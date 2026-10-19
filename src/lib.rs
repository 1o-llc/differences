// Copyright © 2026 1o. All rights reserved.

//! `differences`: every kind of history a project keeps, side by side, on
//! the command line. Each line is declared once ([`surface`]) as an xccute
//! line under the `differences` program; [`cli`] reads the words into that
//! line and answers it through differences_kit; [`install`] installs the
//! program through `installation`.
//!
//! ```text
//! differences tree take POINT [--dir DIR] [--level N] [--exclude PATTERN]... [--with all,dirs,gitignore]
//! differences tree A B                       (tree compare A B)
//! differences tree list
//! differences count A B [--records]          (count compare A B: receipt, job, <job>:open, commit, date)
//! differences count history [ROUTE]
//! differences receipts --since N | differences receipts A B
//! differences revisions TASK [--history DIR]
//! differences changes JOB...
//! differences readings SPEC [SEQ_A SEQ_B]
//! differences tasks [--history DIR]
//! differences self install|uninstall|status
//! ```
//!
//! Every line takes `--root DIR` (the project, `.custodian/` there or above;
//! the current directory by default). Exit status: 0 the same (or done), 1
//! different, 2 refused.

pub mod cli;
pub mod install;
pub mod surface;
