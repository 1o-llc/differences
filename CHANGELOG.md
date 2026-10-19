# Changelog

## 0.1.0 (2026-10-19)

The command line for every kind of history a project keeps. James asked for
"differences as the CLI" and "give me all": one program that shows any
history side by side. The form is `differences <kind> <a> <b>`. Every link is
pinned to 77440fe.

- The lines are declared once, with simple_impl_derive 0.2.7 over
  xccute_contract: the `differences` program ([src/surface.rs#L33](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L33)) and its
  surfaces `tree`, `count` and `self` ([src/surface.rs#L41](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L41),
  [src/surface.rs#L48](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L48), [src/surface.rs#L56](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L56)). There is one leaf per line,
  and each names the statuses `0 same`, `1 different` and `2 refused`. A
  line under a surface is its leaf's nested wrapper, so it keeps its leaf's
  setters, statuses and rules. The usage screen is the root's generated help
  lines.
- `read` ([src/surface.rs#L585](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L585)) reads the words through each line's own
  setters ([src/surface.rs#L558](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L558)). A word no setter takes is refused.
  `tree A B` reads as `tree compare A B`, and `count A B` as
  `count compare A B`.
- Each field's words are checked. `--with` is tree_history's `TreeWith`
  word ([src/surface.rs#L372](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L372)). `receipts` takes `--since N` or two
  receipts, not both ([src/surface.rs#L379](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L379)).
- The lines, answered by differences_kit ([src/cli.rs#L41](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/cli.rs#L41)):
  - `tree take POINT [--dir] [--level] [--exclude]... [--with]` runs `tree`
    as tree_history's declared line. It keeps the snapshot under
    `data/trees/` and the line's receipt in the project's journal.
  - `tree A B` and `tree list`.
  - `count A B [--records]` (the change records in 1o) and
    `count history [ROUTE]`.
  - `receipts --since N` and `receipts A B`.
  - `revisions TASK [--history DIR]`.
  - `changes JOB...`.
  - `readings SPEC [SEQ_A SEQ_B]`.
  - `tasks [--history DIR]`.

  The page goes to stdout and what was said about it to stderr
  ([src/cli.rs#L104](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/cli.rs#L104)).
- `self install|uninstall|status` ([src/install.rs#L38](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/install.rs#L38)) installs through
  `installation`, as custodian installs itself. It makes a plan first, then
  installs from the plan, then writes installation's receipt.
- Proven by `every_line_reads_back_from_its_own_words` ([src/surface.rs#L624](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L624)),
  `the_short_spellings_and_the_refusals` ([src/surface.rs#L653](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/src/surface.rs#L653)) and
  `two_tree_points_and_their_receipts` ([tests/cli.rs#L24](https://github.com/1o-llc/differences/blob/77440fe94086f5ae04dce73256966bdab7efafac/tests/cli.rs#L24)). The last runs
  the built program: two tree points, their comparison, the receipts the tree
  lines left, a point named twice refused.

Proof: `cargo test`, 3 tests.
