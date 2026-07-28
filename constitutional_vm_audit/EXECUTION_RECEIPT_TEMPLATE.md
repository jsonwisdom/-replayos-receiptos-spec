# ReplayOS Constitutional VM — Execution Receipt Template

**Status:** REQUIRED BEFORE `PR_DRAFT → PR_READY_FOR_REVIEW`

No screenshot, summary, or narrative substitute is admissible. Raw files produced by `scripts/collect_execution_receipt.sh` are the evidence source.

## 1. Commit Binding

- Commit SHA: `<40-hex SHA>`
- Branch: `<branch>`
- Worktree clean: `<true|false>`
- `git status --porcelain=v1`: `git_status_porcelain.txt`
- `git diff --stat`: `git_diff_stat.txt`
- `git diff --cached --stat`: `git_diff_cached_stat.txt`

## 2. Toolchain Provenance

- `rustc --version`: `rustc_version.txt`
- `cargo --version`: `cargo_version.txt`
- `rustc -vV`: `rustc_verbose.txt`

The host triple is taken from `rustc_verbose.txt`. Target triples beyond the host MUST be separately recorded when used.

## 3. Dependency Resolution

- Lockfile: `Cargo.lock`
- Dependency graph: `cargo_tree.txt`
- Registry/vendor resolution: `cargo_fetch_verbose.txt`
- Vendor artifact hash, when applicable: `<SHA-256 and artifact identity>`

A missing lockfile or incomplete dependency graph makes the receipt inadmissible.

## 4. Invocation

- Exact command: `test_command.txt`
- Working directory: `working_directory.txt`
- Non-secret environment allowlist: `environment_allowlist.txt`

Secrets MUST NOT be collected. Any additional environment variable needed for replay MUST be disclosed by name and its non-secret value.

## 5. Raw Test Evidence

- stdout: `test.stdout.log`
- stderr: `test.stderr.log`
- Exit code: `test_exit_code.txt`

Required command:

```bash
cargo test -- --nocapture auditlayer_is_append_only
```

A qualifying green execution requires exit code `0` and a clean worktree.

## 6. Receipt Integrity

- Machine manifest: `receipt_manifest.json`
- Per-file SHA-256 list: `receipt_bundle.sha256`
- SHA-256 of the hash manifest: `receipt_bundle_manifest_hash.txt`

The receipt directory MUST be preserved without editing. Any correction creates a new run directory and a new receipt identity.

## 7. Reviewer Countersignature

Reviewer identity: `<real GitHub/human identity>`

Statement:

> I independently verified the execution receipt for commit `<SHA>` and confirm that the recorded test results are complete, authentic, and bound to the stated environment and dependency graph.

Countersignature text SHA-256: `<SHA-256>`

Reviewer assignment and countersignature occur only after an admissible green receipt exists.

## 8. Promotion Directive

Promotion is authorized only when all statements are true:

- [ ] Tested SHA equals the current PR head SHA.
- [ ] Worktree was clean.
- [ ] Dependency evidence is complete.
- [ ] Required test exited `0`.
- [ ] Raw stdout and stderr are preserved.
- [ ] Receipt hashes verify.
- [ ] Reviewer countersignature is recorded.

Then, and only then:

```text
SOURCE_VERIFIED
EXECUTION_VERIFIED
PR_READY_FOR_REVIEW
MERGE_PENDING_COUNTERSIGNATURE
```

Until then:

```text
SOURCE_VERIFIED
EXECUTION_RECEIPT_MISSING
PR_DRAFT
MERGE_BLOCKED
```
