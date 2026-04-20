#!/usr/bin/env bash
# =============================================================================
# build.sh — NovaForge top-level build script
#
# Builds every crate in this repository:
#   1. The Fyrox engine workspace (all crates under the root Cargo.toml)
#   2. The NovaForge game project (novaforge/ sub-workspace)
#
# Usage:
#   ./build.sh           # debug build of everything
#   ./build.sh --release # release build of everything
#   ./build.sh --game    # build only the NovaForge game (debug)
#   ./build.sh --engine  # build only the Fyrox engine workspace (debug)
#   ./build.sh --test    # run all tests in both workspaces
#   ./build.sh --help    # show this message
#
# Prerequisites:
#   - Rust toolchain >= 1.87 (install via https://rustup.rs)
#   - System libraries required by Fyrox (see README.md for your platform)
# =============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NOVAFORGE_DIR="$REPO_ROOT/novaforge"

PROFILE_FLAG=""
TEST_MODE=false
BUILD_ENGINE=true
BUILD_GAME=true

# ---------------------------------------------------------------------------
# Parse arguments
# ---------------------------------------------------------------------------
for arg in "$@"; do
    case "$arg" in
        --release)
            PROFILE_FLAG="--release"
            ;;
        --game)
            BUILD_ENGINE=false
            BUILD_GAME=true
            ;;
        --engine)
            BUILD_ENGINE=true
            BUILD_GAME=false
            ;;
        --test)
            TEST_MODE=true
            ;;
        --help|-h)
            sed -n '2,20p' "$0"
            exit 0
            ;;
        *)
            echo "Unknown argument: $arg" >&2
            echo "Run '$0 --help' for usage." >&2
            exit 1
            ;;
    esac
done

echo "============================================================"
echo "  NovaForge Build Script"
echo "  Repo root : $REPO_ROOT"
echo "  Profile   : ${PROFILE_FLAG:-(debug)}"
echo "  Test mode : $TEST_MODE"
echo "============================================================"

# ---------------------------------------------------------------------------
# Helper: run cargo in a specific directory
# ---------------------------------------------------------------------------
cargo_run() {
    local dir="$1"
    shift
    echo ""
    echo ">>> cargo $* (in $dir)"
    (cd "$dir" && cargo "$@")
}

# ---------------------------------------------------------------------------
# Build / test the Fyrox engine workspace
# ---------------------------------------------------------------------------
if $BUILD_ENGINE; then
    echo ""
    echo "------------------------------------------------------------"
    echo "  [1/2] Fyrox Engine Workspace"
    echo "------------------------------------------------------------"

    if $TEST_MODE; then
        cargo_run "$REPO_ROOT" test $PROFILE_FLAG --workspace
    else
        cargo_run "$REPO_ROOT" build $PROFILE_FLAG --workspace
    fi
fi

# ---------------------------------------------------------------------------
# Build / test the NovaForge game sub-workspace
# ---------------------------------------------------------------------------
if $BUILD_GAME; then
    echo ""
    echo "------------------------------------------------------------"
    echo "  [2/2] NovaForge Game Project"
    echo "------------------------------------------------------------"

    if $TEST_MODE; then
        cargo_run "$NOVAFORGE_DIR" test $PROFILE_FLAG --workspace
    else
        cargo_run "$NOVAFORGE_DIR" build $PROFILE_FLAG --workspace
    fi
fi

echo ""
echo "============================================================"
echo "  Build complete!"
echo ""
echo "  To run the game:"
echo "    cd novaforge"
echo "    cargo run --package executor --release"
echo "============================================================"
