# speedupy-native: instructions for agents

Read this before touching the repository. `README.md` says what the package is
and how to build it; `docs/native-hashes.md` in `dew-uff/speedupy` is the
design this repository implements.

## Working conventions

The same as `dew-uff/speedupy`'s `AGENTS.md`, in short:

- Everything versioned is in English. Documents say "the maintainer", never a
  person's name.
- One task, one issue, one branch `issue_<N>`, one pull request. Commit
  messages start with `Issue(#N): `. Pull requests are integrated with a merge
  commit, never a squash or a rebase.
- Do not commit on your own: leave the change in the working tree and show the
  diff and the result of each check. Commit, push and the pull request are the
  dev's, or yours when the dev says so.
- Do what the task says and nothing more; anything else found on the way goes
  to a new issue.
- Three failed attempts at the same goal: stop, explain, recommend.

## What never changes without a decision

- **The vendored sources** under `vendor/`: upstream files copied unchanged,
  at the versions the measurements of the SpeeduPy papers used (`NOTICE`
  names each repository, version and commit). They are never edited. A new
  upstream version is a new copy plus a `NOTICE` update; it changes hash
  values, so it is a maintainers' decision, at least a minor version, written
  in the release notes with before and after values of one input.
- **`tests/test_vectors.py`.** Generated, never edited by hand. It changes only
  with the previous item, by running `tools/gen_vectors.py` against the
  original `.so` libraries built from the new commit.
- **The Python API** (`a5hash_128_64`, `museair_bfast_64`, `meowhash_32`,
  `HAS_MEOWHASH`, `PLATFORM`): SpeeduPy's hash classes call it by name.

## Definition of done

- `maturin develop --release` builds on the dev's machine.
- `pytest -q` is green: every reference value reproduced.
- `cargo fmt --check` is clean.
- CI is green at the tip of the branch: `ci-ok` is the one job that says so.
  It fails when any wheel failed to build or to pass the tests on its own
  platform (Linux x86_64 and aarch64, Windows x64, macOS Intel and arm64, and
  the sdist).

## Traps

- **Vendored files are not ours.** A fix that seems needed in `vendor/` goes
  upstream, or into the wrappers in `csrc/` and the glue in `src/`; the copies
  stay byte for byte what `NOTICE` says they are (`diff` against the upstream
  commit proves it).
- **MuseAir has no manifest of its own here.** `crates/museair/Cargo.toml` is
  a bridge manifest whose `[lib] path` points at the vendored `src/lib.rs`
  (upstream v0.4, which forbids unsafe code; the extension calls
  `bfast::hash` directly and needs no C export). The upstream manifest is kept
  as `vendor/museair/Cargo.toml.upstream` for its license and feature list.
- **MeowHash is x86_64 only.** `build.rs` compiles its wrapper only there and
  sets `cfg(has_meowhash)`; `HAS_MEOWHASH` also checks AES-NI at run time. A
  test that needs MeowHash skips where `HAS_MEOWHASH` is false, never fails.
- **abi3.** The extension uses the stable ABI from Python 3.10, so PyO3's
  `PyBuffer` is unavailable; the functions take `bytes` (`&[u8]`), which is
  what SpeeduPy passes. Do not widen them to the buffer protocol without
  raising the ABI floor.
- **The tests import the installed wheel**, not `python/`: run them after
  `maturin develop`, or after installing a built wheel, never with `python/` on
  `sys.path`.
