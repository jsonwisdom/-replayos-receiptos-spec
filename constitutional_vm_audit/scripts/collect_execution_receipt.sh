#!/usr/bin/env bash
set -uo pipefail

# ReplayOS Constitutional VM execution-receipt collector.
# Produces raw evidence without claiming PASS unless the targeted cargo test exits 0.

ROOT_DIR="$(git rev-parse --show-toplevel)"
CRATE_DIR="${ROOT_DIR}/constitutional_vm_audit"
RUN_ID="$(date -u +%Y%m%dT%H%M%SZ)"
RECEIPT_ROOT="${RECEIPT_ROOT:-${TMPDIR:-/tmp}/constitutional_vm_audit_receipts}"
OUT_DIR="${RECEIPT_ROOT}/${RUN_ID}"

# Capture repository state before creating any evidence artifacts.
cd "${ROOT_DIR}"
COMMIT_SHA="$(git rev-parse HEAD)"
BRANCH="$(git branch --show-current)"
WORKTREE_STATUS="$(git status --porcelain=v1)"
GIT_DIFF_STAT="$(git diff --stat)"
GIT_DIFF_CACHED_STAT="$(git diff --cached --stat)"

mkdir -p "${OUT_DIR}"

record() {
  local name="$1"
  shift
  {
    printf '$'
    printf ' %q' "$@"
    printf '\n'
    "$@"
  } >"${OUT_DIR}/${name}.txt" 2>&1
  return 0
}

printf '%s\n' "${COMMIT_SHA}" >"${OUT_DIR}/commit_sha.txt"
printf '%s\n' "${BRANCH}" >"${OUT_DIR}/branch.txt"
printf '%s' "${WORKTREE_STATUS}" >"${OUT_DIR}/git_status_porcelain.txt"
printf '%s' "${GIT_DIFF_STAT}" >"${OUT_DIR}/git_diff_stat.txt"
printf '%s' "${GIT_DIFF_CACHED_STAT}" >"${OUT_DIR}/git_diff_cached_stat.txt"

record rustc_version rustc --version
record cargo_version cargo --version
record rustc_verbose rustc -vV

cd "${CRATE_DIR}"
cp Cargo.lock "${OUT_DIR}/Cargo.lock" 2>/dev/null || true
record cargo_tree cargo tree
record cargo_fetch_verbose cargo fetch --verbose
pwd >"${OUT_DIR}/working_directory.txt"

# Secrets are intentionally excluded. The allowlist is explicit and replay-oriented.
{
  env | LC_ALL=C sort | grep -E '^(CI|GITHUB_|RUNNER_|RUST|CARGO|HOSTNAME|HOME|PATH|SHELL|USER|LANG|LC_)=' || true
} >"${OUT_DIR}/environment_allowlist.txt"

TEST_COMMAND=(cargo test -- --nocapture auditlayer_is_append_only)
printf '%q ' "${TEST_COMMAND[@]}" >"${OUT_DIR}/test_command.txt"
printf '\n' >>"${OUT_DIR}/test_command.txt"

set +e
"${TEST_COMMAND[@]}" >"${OUT_DIR}/test.stdout.log" 2>"${OUT_DIR}/test.stderr.log"
TEST_EXIT_CODE=$?
set -e
printf '%s\n' "${TEST_EXIT_CODE}" >"${OUT_DIR}/test_exit_code.txt"

cat >"${OUT_DIR}/receipt_manifest.json" <<EOF
{
  "schema": "ReplayOSExecutionReceipt",
  "version": "1.0",
  "run_id": "${RUN_ID}",
  "commit_sha": "${COMMIT_SHA}",
  "branch": "${BRANCH}",
  "test_command": "cargo test -- --nocapture auditlayer_is_append_only",
  "test_exit_code": ${TEST_EXIT_CODE},
  "worktree_clean": $(if [[ -z "${WORKTREE_STATUS}" ]]; then echo true; else echo false; fi),
  "status": "$(if [[ ${TEST_EXIT_CODE} -eq 0 && -z "${WORKTREE_STATUS}" ]]; then echo EXECUTION_VERIFIED; else echo EXECUTION_NOT_VERIFIED; fi)"
}
EOF

# Deterministic per-file commitments. The hash manifest does not hash itself.
(
  cd "${OUT_DIR}"
  find . -maxdepth 1 -type f \
    ! -name 'receipt_bundle.sha256' \
    ! -name 'receipt_bundle_manifest_hash.txt' \
    -print0 \
    | LC_ALL=C sort -z \
    | xargs -0 sha256sum
) >"${OUT_DIR}/receipt_bundle.sha256"

BUNDLE_HASH="$(sha256sum "${OUT_DIR}/receipt_bundle.sha256" | awk '{print $1}')"
printf '%s\n' "${BUNDLE_HASH}" >"${OUT_DIR}/receipt_bundle_manifest_hash.txt"

printf 'Receipt directory: %s\n' "${OUT_DIR}"
printf 'Commit: %s\n' "${COMMIT_SHA}"
printf 'Test exit code: %s\n' "${TEST_EXIT_CODE}"
printf 'Bundle manifest hash: %s\n' "${BUNDLE_HASH}"

if [[ -n "${WORKTREE_STATUS}" ]]; then
  printf 'FAIL-CLOSED: worktree was not clean before receipt collection.\n' >&2
  exit 2
fi

exit "${TEST_EXIT_CODE}"
