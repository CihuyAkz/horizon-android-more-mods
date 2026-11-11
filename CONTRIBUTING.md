# Contributing to Starling

Thanks for helping. Starling is the Android port of [Horizon](https://github.com/ludovic111/horizon), Minecraft: Java Edition 26.3 rewritten in Rust. This page is the short version of how we work.

## Where a change goes

- **The game itself** (gameplay, world generation, rendering shared with the desktop, menus) belongs upstream, in Horizon, where it is checked against vanilla. Follow [Horizon's contributing guide](https://github.com/ludovic111/horizon/blob/main/CONTRIBUTING.md): vanilla is the specification, and gameplay changes come with a parity harness.
- **The Android app** belongs here: the packaging (`packaging/android/`), the app's entry (`crates/horizon-client/src/android_lib.rs`), touch controls (`touch.rs`, `game/touch.rs`), the Starling folder and imports (`import.rs`, `game/imports.rs`, `game/files_offer.rs`) and anything that only matters on a phone.

**Mod-related pull requests go to [arismoko/horizon](https://github.com/arismoko/horizon)**, the fork that carries the
mod API and reviews and merges that work. That covers the mod API, native hooks and events for mods, native mods,
content mods (`.hmod`), mixins, wasm mods, `cargo horizon-mod`, `horizon-mods`, mod builds and the registry, and
their docs ([mods](docs/mods.md), [native mods](docs/native-mods.md)). Open the pull request there from your usual
branch. Mod-related pull requests opened here are closed with a pointer to it. Engine extension points that Horizon's
own crates register (the [layer split](docs/layers.md)), resource and shader pack formats, and gameplay fixes stay
here.

## Build it

You need Rust (stable) with the `aarch64-linux-android` target, [cargo-ndk](https://github.com/bbqsrc/cargo-ndk), the Android SDK (build-tools 35.0.0, platform 36, NDK r28c), a JDK 25 and Python 3.10+.

<<<<<<< HEAD
```sh
python packaging/android/build_apk.py            # build the library, then the APK
python packaging/android/build_apk.py --install  # and install it over adb
```
||||||| c98a51a664
  ```sh
  cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar
  cargo run --release -p horizon-client
  ```
=======
  ```sh
  cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar
  cargo run --release -p horizon-game
  ```
>>>>>>> arismoko/main

The APK carries no game files. On its first start the app signs in, downloads them from Mojang for an account that owns the game and prepares them on the phone; to skip that while testing, prepare the resources from your own copy of Minecraft 26.3 as described in [Build from source](README.md#build-from-source), zip them as a `.starling` file and open it with the app. The setup's hand-drawn skin comes from `python packaging/lowbudget/make.py` (Pillow): run it after changing `hand.py`, `make.py` or `strings.json`, and commit `pack/` with them.

## Test on a phone

<<<<<<< HEAD
Say what you checked on a real device, and on which one. A `horizon-args.txt` in the app's own files adds command-line arguments and environment variables at launch (one per line, `NAME=value` for the environment); with `--control-socket` and `HORIZON_CONTROL_PORT`, `horizon-ctl` drives the game over `adb forward`. Empty the file afterwards so the app starts normally.
||||||| c98a51a664
Wherever vanilla has a feature, vanilla is the specification, down to the order of random calls. "Parity" here means
Horizon gives the same results as the pinned 26.3 client and server for the same inputs, checked by a **harness**: a
`tools/test_*_parity.py` script that runs a small Java oracle against the real game and a Rust probe against Horizon,
and compares their outputs.

- **Read the vanilla code before writing Horizon's.** `tools/rea_vanilla.py` queries the pinned game bytecode:
  `python3 tools/rea_vanilla.py method net.minecraft.world.level.block.DispenserBlock dispenseFrom`.
  Say in your pull request which vanilla classes and methods you followed.
- **Gameplay changes come with a harness**, new or extended, so the next change can't silently break them.
  [The parity workflow](docs/parity-workflow.md) shows how harnesses run; copy a nearby one.
- **Horizon's own features** (the launcher, the new menus) follow [the design system](docs/design-system.md) and are
  off by default wherever they would change the vanilla look.
- **Everything players run is Rust.** Python and Java are fine for dev tools and harness drivers in `tools/`, never
  for player-facing code.
- **No game files in the repo.** Never commit game files, decompiled source or prepared resources (`.local/`):
  `tools/check_public_tree.py` rejects them (`git config core.hooksPath .githooks` runs it before every commit).
  The pre-push hook checks the full history reachable from the branches and tags being pushed. Run
  `python3 tools/check_public_tree.py --history` to audit the history of all local refs.
=======
Wherever vanilla has a feature, vanilla is the specification, down to the order of random calls. "Parity" here means
Horizon gives the same results as the pinned 26.3 client and server for the same inputs, checked by a **harness**: a
`tools/test_*_parity.py` script that runs a small Java oracle against the real game and a Rust probe against Horizon,
and compares their outputs.

- **Read the vanilla code before writing Horizon's.** `tools/rea_vanilla.py` queries the pinned game bytecode:
  `python3 tools/rea_vanilla.py method net.minecraft.world.level.block.DispenserBlock dispenseFrom`.
  Say in your pull request which vanilla classes and methods you followed.
- **One version: 26.3.** Parity work targets 26.3 until its parity is good, redstone first. Don't open pull requests
  that move Horizon to a newer Minecraft version for now; to join servers on a newer version, put a Via proxy
  (ViaVersion, ViaProxy) in front of them.
- **Gameplay changes come with a harness**, new or extended, so the next change can't silently break them.
  [The parity workflow](docs/parity-workflow.md) shows how harnesses run; copy a nearby one.
- **Horizon's own features** (the launcher, the new menus) follow [the design system](docs/design-system.md) and are
  off by default wherever they would change the vanilla look.
- **Three layers: engine, platform, Horizon.** Vanilla code never depends on the platform (mods, shader packs,
  Horizon-only settings) or on Horizon (cosmetics, the shop, official servers, telemetry), and the platform never
  depends on Horizon. [docs/layers.md](docs/layers.md) says where a new feature goes; CI checks the dependencies.
- **Everything players run is Rust.** Python and Java are fine for dev tools and harness drivers in `tools/`, never
  for player-facing code.
- **No game files in the repo.** Never commit game files, decompiled source or prepared resources (`.local/`):
  `tools/check_public_tree.py` rejects them (`git config core.hooksPath .githooks` runs it before every commit).
  The pre-push hook checks the full history reachable from the branches and tags being pushed. Run
  `python3 tools/check_public_tree.py --history` to audit the history of all local refs.
>>>>>>> arismoko/main

## Only original code, or code whose license allows it

Starling is MIT, as Horizon is. Its code is written from Minecraft 26.3 itself and from public documentation. Another project's source may also be read and used, **only when that project's license allows it**:

<<<<<<< HEAD
- **Allowed:** MIT, MIT-0, BSD-2-Clause, BSD-3-Clause, 0BSD, ISC, zlib, Apache-2.0, Unlicense, and the CC0-1.0 public-domain dedication. That code may be read, copied or adapted.
- **Java's own library is GPL too.** OpenJDK's source (`java.lang.Math`, `StrictMath`, `FdLibm`, `HashMap`, the sorts, `Double.toString`, HotSpot's intrinsics...) is GPL-2.0, so it is off limits like the rest. Reproduce Java's behaviour from its API documentation (the Javadoc) and from what a JVM returns, or from a permissive original such as Sun's fdlibm C sources (see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)); see #6741.
- **Everything else is off limits, even to read:** GPL, LGPL (Iris), AGPL, MPL, EPL, Polyform (Sodium), the Creative Commons licenses (CC BY, CC BY-SA, CC BY-NC and the others; CC0 is a public-domain dedication, not one of them), custom or "source available" licenses, all rights reserved (OptiFine), and any project with no license or an unclear one. Don't read, copy, translate or paraphrase their source, and don't send code an AI tool wrote from it. Being able to download a mod or a pack does not give you the right to read its files.
- **Check the license yourself.** Read the LICENSE file at the commit you use and the header of each file you take: some repositories mix licenses, and a file under another license is off limits. A license that changed: the one at your commit counts. In doubt, don't: ask in an issue first.
- **Say it in the pull request:** the project, a link to the commit, its license, and what you read or copied.
- **Copied or adapted code** keeps its copyright line and license notice at the top of the file, and gets an entry in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Apache-2.0 code also keeps its NOTICE text and says what was changed.
||||||| c98a51a664
- **Allowed:** MIT, MIT-0, BSD-2-Clause, BSD-3-Clause, 0BSD, ISC, zlib, Apache-2.0, Unlicense, and the CC0-1.0
  public-domain dedication. That code may be read, copied or adapted.
- **Off limits, even to read:** GPL, LGPL (Iris), AGPL, MPL, EPL, Polyform (Sodium), the Creative Commons licenses
  (CC BY, CC BY-SA, CC BY-NC and the others; CC0 is a public-domain dedication, not one of them), custom or
  "source available" licenses, all rights reserved (OptiFine), and any project with no license or an unclear one.
  Don't read, copy, translate or paraphrase their source, and don't send code an AI tool wrote from it. Being able
  to download a mod or a pack does not give you the right to read its files.
- **Java's own library is GPL too.** OpenJDK's source (`java.lang.Math`, `StrictMath`, `FdLibm`, `HashMap`, the
  sorts, `Double.toString`, HotSpot's intrinsics...) is GPL-2.0, so it is off limits like the rest. Reproduce Java's
  behaviour from its API documentation (the Javadoc) and from what a JVM returns, or from a permissive original such
  as Sun's fdlibm C sources (see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)); see #6741.
- **Check the license yourself.** Read the LICENSE file at the commit you use and the header of each file you take:
  some repositories mix licenses, and a file under another license is off limits. A license that changed: the one at
  your commit counts. In doubt, don't: ask in #contributors first.
- **Say it in the pull request:** the project, a link to the commit, its license, and what you read or copied. A
  pull request that used another project's source without saying so is closed.
- **Copied or adapted code** keeps its copyright line and license notice at the top of the file, and gets an entry
  in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Apache-2.0 code also keeps its NOTICE text and says what
  was changed.
=======
- **Allowed:** MIT, MIT-0, BSD-2-Clause, BSD-3-Clause, 0BSD, ISC, zlib, Apache-2.0, Unlicense, and the CC0-1.0
  public-domain dedication. That code may be read, copied or adapted.
- **Off limits, even to read:** GPL, LGPL (Iris), AGPL, MPL, EPL, Polyform (Sodium), the Creative Commons licenses
  (CC BY, CC BY-SA, CC BY-NC and the others; CC0 is a public-domain dedication, not one of them), custom or
  "source available" licenses, all rights reserved (OptiFine), and any project with no license or an unclear one.
  Don't read, copy, translate or paraphrase their source, and don't send code an AI tool wrote from it. Being able
  to download a mod or a pack does not give you the right to read its files.
- **Java's own library is GPL too.** OpenJDK's source (`java.lang.Math`, `StrictMath`, `FdLibm`, `HashMap`, the
  sorts, `Double.toString`, HotSpot's intrinsics...) is GPL-2.0, so it is off limits like the rest. Reproduce Java's
  behaviour from its API documentation (the Javadoc) and from what a JVM returns, or from a permissive original such
  as Sun's fdlibm C sources (see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)); see horizon-rust/horizon#6741.
- **Check the license yourself.** Read the LICENSE file at the commit you use and the header of each file you take:
  some repositories mix licenses, and a file under another license is off limits. A license that changed: the one at
  your commit counts. In doubt, don't: ask in #contributors first.
- **Say it in the pull request:** the project, a link to the commit, its license, and what you read or copied. A
  pull request that used another project's source without saying so is closed.
- **Copied or adapted code** keeps its copyright line and license notice at the top of the file, and gets an entry
  in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Apache-2.0 code also keeps its NOTICE text and says what
  was changed.
>>>>>>> arismoko/main

<<<<<<< HEAD
User documentation and file formats are fine to read, and packs and mods are fine to run and observe, whatever their license.
||||||| c98a51a664
User documentation and file formats are fine to read, and packs and mods are fine to run and observe, whatever their
license. Shader packs are a clean-room rewrite of the pack format, produced from a written description of it (see
#1561); they carry no foreign source, and the rules above still apply to anything you read while working on them.
=======
User documentation and file formats are fine to read, and packs and mods are fine to run and observe, whatever their
license. Shader packs are a clean-room rewrite of the pack format, produced from a written description of it (see
#1561): the [specification and its provenance log](docs/cleanroom/README.md) are public. They carry no foreign
source, and the rules above still apply to anything you read while working on them. A shader-pack pull request
names its sources: the sections of that specification, the documentation pages and the packs it ran.

## Vendor libraries and Java mods

- **Upscalers and latency tools.** AMD FSR (MIT) may go in the repository, like any code under an allowed license.
  NVIDIA's DLSS, Reflex and any other library under an NVIDIA license may not: they ship only as an optional native mod
  that downloads NVIDIA's files itself, on the player's machine. No NVIDIA binaries, headers or SDK files in the
  repository.
- **The JVM bridge** (running Java mods) is only ever a separate, optional download, never installed or on by
  default. The game may offer to turn it on when it finds a Java mod; the player decides. Promotion out of beta requires MSPT and RAM benchmarks against Java with the same mods ([horizon-rust/horizon#13914](https://github.com/horizon-rust/horizon/issues/13914)).
>>>>>>> arismoko/main

## Before a pull request

```sh
git config core.hooksPath .githooks
python3 tools/pr_ready.py
```

It runs rustfmt, clippy with warnings denied, the tests of the crates you touched, the Python tests and the public-tree check. Keep pull requests small, and write commit messages in English.

For the game workspace, Clippy checks all targets of the touched crates and their transitive workspace dependents, including examples. The dependency scope includes optional, platform, build and development dependencies. Tests run library, binary, integration and doc tests without building unrelated example executables. Shared Cargo configuration and toolchain changes select the whole workspace; CI also keeps its workspace-wide checks.

<<<<<<< HEAD
Never commit game files, decompiled source or prepared resources (`.local/`): `tools/check_public_tree.py` rejects them (`git config core.hooksPath .githooks` runs it before every commit). The pre-push hook checks the full history reachable from the branches and tags being pushed; `python3 tools/check_public_tree.py --history` audits the history of all local refs.
||||||| c98a51a664
## Claim before you start

Work is tracked in GitHub issues, and an issue that is assigned is taken (so is one with the `in progress` label:
claimed on Discord with Kira's `/claim`): pick another, or ask its holder in the thread. Before you start, comment
**`/claim`** on the issue; you are assigned at once, no role needed. No issue for your work yet? Open one (a line is
enough), then claim it. `/unclaim` gives it back. Issues labelled `tracker`, `proposal` or `not claimable` aren't
claimed whole: claim one of the issues a tracker lists, or open one for the part you take.

A claim with no open pull request linking the issue (`Fixes #123` in the description, or a draft) is released after
5 days, with a comment: open a draft early. You can hold 8 open issues at a time, not counting the ones your
pull requests already fix. Agents claim the same way.

When your pull request changes the same functions as another open one, or closes the same issue, a bot comment names
it (edited on every push, never repeated): talk to its author in the thread before you both finish the same work.

## Pull requests

- **Small.** One change per pull request, under about 500 lines of code where you can; split bigger work into
  pieces that each make sense alone. Small pull requests get reviewed and merged the same day.
- **Fill in the template**: what and why, the vanilla reference, the pr_ready block, screenshots for anything you
  can see.
- **Tick "Allow edits by maintainers".** When something small is left (a lint, a rebase, a nit) we push the fixup
  ourselves instead of sending it back to you; only design questions come back.
- **Once we've pushed to your branch, merge main into it; don't rebase or force-push.** A rebase or force-push
  replaces our commits with yours and erases the fixes we added (it happened on #5188 and #5203). Tell your agent
  too: `git merge origin/main`, never `git rebase` or `git push --force` on a branch a maintainer has touched. If
  our fixes get erased anyway, we finish the work in our own pull request that keeps your commits and credits you.
- **Tick its kind** in the template: `parity` (Horizon now does what vanilla does), `bug` (a crash or breakage),
  `perf`, `feature` (new for players), `extra` (Horizon-only, off by default), `tooling` or `docs`. A workflow sets
  that label, and area labels (`launcher`, `mobs`, `worldgen`, `rendering`...) from the paths you change.
  `label:parity` is the list of real progress toward vanilla.
- **Release labels** (we set them if you can't): `feature` also puts it under "New" in the release notes,
  `skip-release` when players see no change (tooling, docs, tests), nothing for a fix. `release-now` is for crash
  fixes that shouldn't wait. pr_ready suggests one.

Who reviews and merges: the Horizon Team, on CI for ordinary pull requests; risky ones (saves, network, world
generation, the launcher's updates) wait for a local proof first ([merging](docs/release-flow.md#merging)).

Every merge to main is released automatically ([release flow](docs/release-flow.md)), so main always stays playable.
How agents, checks, releases and signed updates keep this safe: [security](docs/security.md).

## Parity checks on forks

The `verify` check (formatting, lints, tests) runs on every pull request. The `parity` check runs the affected
harnesses on a maintainer's machine, which holds the game files, so it never runs a fork's code on its own: a
maintainer reads your change, then adds the `run-parity` label, which runs it once on your current commit. After
new pushes, we add it again.

## Talk to us

[Discord](https://discord.gg/playhorizon) for questions and ideas, [issues](https://github.com/horizon-rust/horizon/issues)
for bugs. To take something on, `/claim` its issue ([above](#claim-before-you-start)).
=======
### Translations

Most translations are machine-written and remain unreviewed until a speaker checks them. See [docs/translations.md](docs/translations.md). Never mark machine-written text as reviewed.

## Claim before you start

Work is tracked in GitHub issues, and an issue that is assigned is taken (so is one with the `in progress` label:
claimed on Discord with Kira's `/claim`): pick another, or ask its holder in the thread. Before you start, comment
**`/claim`** on the issue; you are assigned at once, no role needed. No issue for your work yet? Open one (a line is
enough), then claim it. `/unclaim` gives it back. Issues labelled `tracker`, `proposal` or `not claimable` aren't
claimed whole: claim one of the issues a tracker lists, or open one for the part you take.

A claim with no open pull request linking the issue (`Fixes #123` in the description, or a draft) is released after
5 days, with a comment: open a draft early. Agents claim the same way.

You can hold at most 5 claimed issues that no open pull request of yours links. A sixth `/claim` is refused with a
comment that gives the count; opening a pull request for one of them, or `/unclaim` on one, frees a slot. The
repository's collaborators, and the trusted contributors listed in the repository variable `CLAIM_LIMIT_EXEMPT`, are
not limited.

When your pull request changes the same functions as another open one, or closes the same issue, a bot comment names
it (edited on every push, never repeated): talk to its author in the thread before you both finish the same work.

## Pull requests

- **One change.** One concern per pull request (a hook, a handle, a feature or a fix), at whatever size it
  needs. There's no line limit; split by concern, never by size, and don't bundle unrelated changes.
- **Fill in the template**: what and why, the vanilla reference, the pr_ready block, screenshots for anything you
  can see.
- **Tick "Allow edits by maintainers".** When something small is left (a lint, a rebase, a nit) we push the fixup
  ourselves instead of sending it back to you; only design questions come back.
- **Once we've pushed to your branch, merge main into it; don't rebase or force-push.** A rebase or force-push
  replaces our commits with yours and erases the fixes we added (it happened on #5188 and #5203). Tell your agent
  too: `git merge origin/main`, never `git rebase` or `git push --force` on a branch a maintainer has touched. If
  our fixes get erased anyway, we finish the work in our own pull request that keeps your commits and credits you.
- **Tick its kind** in the template: `parity` (Horizon now does what vanilla does), `bug` (a crash or breakage),
  `perf`, `feature` (new for players), `extra` (Horizon-only, off by default), `tooling` or `docs`. A workflow sets
  that label, and area labels (`launcher`, `mobs`, `worldgen`, `rendering`...) from the paths you change.
  `label:parity` is the list of real progress toward vanilla.
- **Release labels** (we set them if you can't): `feature` also puts it under "New" in the release notes,
  `skip-release` when players see no change (tooling, docs, tests), nothing for a fix. `release-now` is for crash
  fixes that shouldn't wait. pr_ready suggests one.

Who reviews and merges: the Horizon Team, on CI for ordinary pull requests; risky ones (saves, network, world
generation, the launcher's updates) wait for a local proof first ([merging](docs/release-flow.md#merging)).

Every merge to main is released automatically ([release flow](docs/release-flow.md)), so main always stays playable.
How agents, checks, releases and signed updates keep this safe: [security](docs/security.md).

### Large pull requests

500 lines stays the default. A pull request that can't be split below it starts from a spec:

1. **Open an issue with the spec** before the code: what changes, the files it touches, the behaviour players and
   other code will see, and the tests or harnesses that prove it.
2. **A maintainer approves the spec** in the issue. Code written before that may have to change or wait.
3. **The pull request links the approved issue** (`Spec: #123` in the description) and stays within it; a change of
   plan goes back to the issue first.
4. **An agent check compares the code with the spec**, run by the maintainers' agents: anything the spec doesn't
   cover, or that it promises and the code doesn't do, comes back to you.
5. **Then the normal review**, as for any pull request.

## Parity checks on forks

The `verify` check (formatting, lints, tests) runs on every pull request. The `parity` check runs the affected
harnesses on a maintainer's machine, which holds the game files, so it never runs a fork's code on its own: a
maintainer reads your change, then adds the `run-parity` label, which runs it once on your current commit. After
new pushes, we add it again.

## Talk to us

[Discord](https://discord.gg/playhorizon) for questions and ideas, [issues](https://github.com/horizon-rust/horizon/issues)
for bugs. To take something on, `/claim` its issue ([above](#claim-before-you-start)).

How Horizon is run, and how people join the Team or lead a group: [GOVERNANCE.md](GOVERNANCE.md).
>>>>>>> arismoko/main
