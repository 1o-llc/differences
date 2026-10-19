// Copyright © 2026 1o. All rights reserved.

//! `differences`: the words after it read into a declared line, answered, and said.

fn main() {
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    std::process::exit(differences::cli::main_argv(&args));
}
