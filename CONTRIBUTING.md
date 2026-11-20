# Contributing to Ferrite

Thanks for helping. Ferrite is a fork of [Horizon](https://github.com/ludovic111/horizon): Minecraft: Java Edition
26.3 rewritten in Rust. The one rule that shapes everything is that **it plays like vanilla**: first exactly like it,
then better where players gain from it ([vanilla first, then better](#vanilla-first-then-better)).

This repository holds the game and the launcher, plus the tools, tests and docs that build and check them. Report bugs
and ask questions in [issues](https://github.com/arcbaseproject/ferrite/issues).

**Fix it upstream when you can.** Ferrite keeps Horizon's code, crate names (`horizon-*`), environment variables
(`HORIZON_*`) and the name of its top layer ("Horizon", [below](#vanilla-first-then-better)) so upstream changes merge
cleanly. A fix or feature that isn't specific to Ferrite belongs in Horizon first, following
[its contributing guide](https://github.com/ludovic111/horizon/blob/main/CONTRIBUTING.md); Ferrite picks it up on the
next merge. Open it here when it concerns only Ferrite: its branding, its launcher changes, or work Horizon won't take.

## Build it

- **Prerequisites:** Rust (stable), Python 3.11+, JDK 25, git 2.38 or newer, and on Linux the ALSA and udev headers
  (`libasound2-dev libudev-dev`; Void Linux: `base-devel pkg-config alsa-lib-devel eudev-libudev-devel`).
- **Resources:** Ferrite never downloads the game; it prepares its resources from your own copy of Minecraft 26.3, as
  described in [Build from source](docs/native-client.md#run-from-a-checkout).
- **Commands:**

  ```sh
  cargo run --release -p horizon-prepare -- --prepare --server-jar ~/Downloads/server.jar
  cargo run --release -p horizon-game
  ```

- **Launcher:** its own Cargo workspace in `launcher/` (`cd launcher && cargo run`; on NixOS, `nix-shell` first).

## Vanilla first, then better

Ferrite gets there in two stages. First it plays exactly like vanilla; that is what parity work is for. Then it
improves on vanilla where players gain from it, mostly speed: FPS, TPS, loading, memory.

Wherever vanilla has a feature, vanilla is the specification, down to the order of random calls. "Parity" here means
Ferrite gives the same results as the pinned 26.3 client and server for the same inputs, checked by a **harness**: a
`tools/test_*_parity.py` script that runs a small Java oracle against the real game and a Rust probe against Ferrite,
and compares their outputs.

- **Read the vanilla code before writing Ferrite's.** `tools/rea_vanilla.py` queries the pinned game bytecode:
  `python3 tools/rea_vanilla.py method net.minecraft.world.level.block.DispenserBlock dispenseFrom`.
  Say in your pull request which vanilla classes and methods you followed.
- **One version: 26.3.** Parity work targets 26.3 until its parity is good, redstone first. Don't open pull requests
  that move Ferrite to a newer Minecraft version for now; to join servers on a newer version, put a Via proxy
  (ViaVersion, ViaProxy) in front of them.
- **Ports of vanilla behaviour come with a harness**, new or extended, so the next change can't silently break
  them. [The parity workflow](docs/parity-workflow.md) shows how harnesses run; copy a nearby one. Where vanilla
  has nothing to compare (Horizon's own features, performance work, refactors), no harness is needed: one Rust
  test of the changed behaviour, or the numbers before and after.
- **Horizon's own features** (the launcher, the new menus) follow [the design system](docs/design-system.md) and are
  off by default wherever they would change the vanilla look.
- **Three layers: engine, platform, Horizon.**
  - Vanilla code never depends on the platform (mods, shader packs, HDR, the settings system) or on Horizon
    (its extras, cosmetics, the shop, official servers, telemetry).
  - The platform never depends on Horizon.
  - Horizon uses only the platform, never the engine: it will live in its own repository.

  [docs/layers.md](docs/layers.md) says where a new feature goes; CI checks the dependencies.
- **Everything players run is Rust.** Python and Java are fine for dev tools and harness drivers in `tools/`, never
  for player-facing code.
- **No game files in the repo.** Never commit game files, decompiled source or prepared resources (`.local/`):
  `tools/check_public_tree.py` rejects them (`git config core.hooksPath .githooks` runs it before every commit).
  The pre-push hook checks the full history reachable from the branches and tags being pushed. Run
  `python3 tools/check_public_tree.py --history` to audit the history of all local refs.

### When it doesn't have to be 1:1

Parity is about what players and the vanilla game see, not about copying vanilla's internals. Performance work (a
multithreaded server tick, rendering, chunk loading, memory) doesn't have to keep vanilla's internal order or its
exact random calls. It has to work together with normal Minecraft:

- **Vanilla clients and servers still work with it.** A vanilla 26.3 client plays on a Ferrite server, Ferrite joins
  vanilla servers, and saves and worlds open in both, as before.
- **The game still plays the same.** Farms, redstone, mobs and drops work the way players know from vanilla. A
  difference may only show where no player can rely on it, such as which of two mobs ticks first or exactly which
  random number a mob gets.
- **Vanilla's slow paths aren't copied.** When vanilla spends a lot of FPS or TPS on something that can reach the
  same result faster, Ferrite does it faster. Copying a performance problem isn't parity.
- **Say what changes.** The pull request names the vanilla behaviour it no longer reproduces exactly, why players
  won't notice it or why it's worth it, and how it was checked: numbers before and after, and the harnesses that
  still pass. A change players could notice, such as a different tick order on a busy server, goes behind a setting.

## Vendor libraries and Java mods

- **Upscalers and latency tools.** AMD FSR (MIT) may go in the repository.
  NVIDIA's DLSS, Reflex and any other library under an NVIDIA license may not: they ship only as an optional native mod
  that downloads NVIDIA's files itself, on the player's machine. No NVIDIA binaries, headers or SDK files in the
  repository.
- **The JVM bridge** (running Java mods) is only ever a separate, optional download, never installed or on by
  default. The game may offer to turn it on when it finds a Java mod; the player decides. It is promoted out of beta
  only after MSPT and RAM benchmarks against the Java client with the same mods ([#13914](https://github.com/horizon-rust/horizon/issues/13914)).

## Licence

Ferrite is under the GNU Lesser General Public License, version 3 or any later version ([LICENSE](LICENSE)), like
Horizon; the copyright holders are in [NOTICE](NOTICE). The LGPL adds permissions to the GNU GPL version 3, whose text
is in [LICENSES/GPL-3.0.txt](LICENSES/GPL-3.0.txt). What you contribute is under the same licence. Code merged into
Horizon before its move from MIT stays available under the MIT licence ([LICENSES/MIT.txt](LICENSES/MIT.txt)).

Code taken from another project must be under a licence the LGPL-3.0 accepts, such as MIT, BSD, ISC, zlib,
Apache-2.0, MPL-2.0, LGPL-3.0 or LGPL-2.1-or-later. It keeps its copyright line and licence notice, and gets an entry
in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Code under the GPL or the AGPL can't go in: it would put all of
Ferrite under that licence. If you aren't sure, ask in an issue first.

## Before asking for review

Run the self-check and paste the block it prints into your pull request:

```sh
python3 tools/pr_ready.py --base origin/main
```

It looks at what your branch changes and runs what matters: rustfmt, clippy with warnings denied, the tests of the
crates you touched (and the launcher's), generated files, `Cargo.lock` and the Python tests. It runs the vanilla
harnesses only when your branch changes a harness, an oracle, a probe or the runner; otherwise it lists the ones your
change can affect. `--harnesses` runs them anyway (it needs the vanilla data in `.local/`). It exits non-zero when
something fails.

For a quick check while you work, `python3 tools/check_changed.py` runs rustfmt, clippy and tests for only the crates
your branch changed and the crates that depend on them (`--dry-run` lists the commands).

### Translations

Most translations were written by machine and stay marked unreviewed until someone who speaks the language checks
them ([docs/translations.md](docs/translations.md)). Never mark machine-written text as reviewed. Translation keys
stay upstream's English; the launcher swaps "Horizon" for "Ferrite" when it shows them.

## Pull requests

- **Open an issue first** for anything bigger than a small fix, so nobody does the same work twice.
- **One change per pull request**, under about 3000 lines of code; split bigger work into pieces that each make sense
  alone. What counts: lines added plus lines removed, outside Markdown (`.md`) files and test files (any `tests/` or
  `test/` folder, and `test_*.py`). Tests inside a source file (`#[cfg(test)]` modules) and lines moved unchanged
  don't count. pr_ready counts it the same way and warns when a pull request is over.
- **Fill in the template**: what and why, the vanilla reference, the pr_ready block, screenshots for anything you
  can see.
- **Title: `kind(area): what changes`**, for example `parity(mobs): a ghast's fireball uses its ExplosionPower` or
  `perf(launcher): download the game's files in parallel`. The kind is `parity`, `bug`, `perf`, `feature`, `extra`
  (off by default, not in vanilla), `tooling` or `docs`.
- **Keep upstream merges easy.** Don't rename `horizon-*` crates, `HORIZON_*` variables, data folders or translation
  keys; rebrand only what players see.
