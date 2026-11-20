<h1 align="center">Ferrite</h1>

<p align="center">
  <b>Minecraft: Java Edition 26.3, rewritten in Rust.</b><br>
  It opens your worlds, generates the same terrain and plays by the same rules, without Java.
</p>

Ferrite is a fork of [Horizon](https://github.com/ludovic111/horizon), by Ludovic Marie, the Horizon Team and its
contributors. It is not an official Horizon build, and Horizon's team does not support it.

## How Ferrite differs from Horizon

- **Its own updates.** The launcher updates itself, and installs game builds, from Ferrite's signed releases on
  [arcbaseproject/ferrite](https://github.com/arcbaseproject/ferrite/releases), never from upstream Horizon.
- **A leaner launcher.** No telemetry, bug reports, AI agents, shop, cosmetics wardrobe or server hosting.
- **Same game.** Internal crate names, environment variables (`HORIZON_*`) and file formats stay Horizon's, so
  upstream changes merge cleanly and your Horizon data keeps working. Ferrite's own game files and launcher data go
  in separate Ferrite folders; existing Horizon data stays where it is.

## What you need

- **Your own copy of Minecraft: Java Edition.** Ferrite ships no Minecraft files. It builds them from the game you
  bought, so install version 26.3 and start it once.
- **Windows, macOS or Linux:** Windows on x64 or ARM64, macOS 12 or later, Linux on x86-64. On Linux, install your
  GPU's Vulkan driver (for AMD, Mesa's RADV); without one the game falls back to OpenGL, which is slower.
- **To build it:** a Rust toolchain, plus `libasound2-dev libudev-dev` on Linux.

## Build and play

Prepare the Minecraft 26.3 resources first, following [the source build instructions](CONTRIBUTING.md#build-it).
Then, from this checkout:

```sh
./run.sh               # build and start the game
./run.sh --launcher    # open the launcher
```

For the fastest build on your own machine, `tools/build-runtime.sh` builds with thin LTO and `target-cpu=native`.

To use your local build with launcher-managed instances and Microsoft sign-in, install a compatible base version in
the launcher, then run:

```sh
./run.sh --launcher import-build "$PWD/target/fast/horizon-client" --trust-unsigned
```

The import makes Play start your local executable. On Windows, use `horizon-client.exe`.

## Docs

[Build guide](docs/native-client.md#run-from-a-checkout) · [Contributing](CONTRIBUTING.md) ·
[Architecture](docs/architecture.md) · [Benchmarks](docs/benchmark.md) · [Mods](docs/mods.md) ·
[All docs](docs/README.md)

## Licence

Ferrite is released under the [GNU LGPL, version 3 or later](LICENSE), the same licence as Horizon; copyright holders are in [NOTICE](NOTICE). Code merged
into Horizon before its move from MIT stays available under the [MIT licence](LICENSES/MIT.txt). Dependencies keep
their own licences; see the [notices](THIRD_PARTY_NOTICES.md).

Horizon was founded by [Ludovic Marie](https://lu4ovic.com) and built by the Horizon Team and
[its contributors](https://github.com/horizon-rust/horizon/graphs/contributors). Support it on
[GitHub Sponsors](https://github.com/sponsors/ludovic111).

<sub>NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT. Minecraft is a
trademark of Mojang Synergies AB; its code and assets belong to Mojang and Microsoft under the
[Minecraft EULA](https://www.minecraft.net/en-us/eula).</sub>
