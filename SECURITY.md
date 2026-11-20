# Security

## Reporting a vulnerability

Report it privately through GitHub: the repository's **Security** tab, then **Report a vulnerability**
([direct link](https://github.com/arcbaseproject/ferrite/security/advisories/new)). Don't open a public issue for
it.

Include what is affected (game, launcher, dedicated server), the version or commit, and how to reproduce it. You will
get an answer in the advisory thread; once a fix is out, the advisory is published with credit to you unless you ask
otherwise.

If the problem is in code Ferrite shares with [Horizon](https://github.com/ludovic111/horizon) unchanged, report it
to Horizon too, following its own process, so both get fixed.

## Supported versions

Only the newest release gets security fixes: the launcher updates itself and its game to it.

## What Ferrite does differently from Horizon

- **Its own signed releases.** The launcher updates itself and installs game builds only from
  [arcbaseproject/ferrite's releases](https://github.com/arcbaseproject/ferrite/releases), signed with Ferrite's own
  key (`crates/horizon-release-sign/ferrite-release-keys.txt`); a Horizon release or key is never accepted for it.
  The private key lives only in the repository's `release` environment secret `RELEASE_SIGNING_KEY`.
- **No telemetry, bug reports or launcher agents.** The launcher starts the game with telemetry and the heartbeat
  off, and has no AI-agent or MCP control surface.
- **Unsigned local builds.** A build imported with `--trust-unsigned` skips the signature check: only import builds
  you made yourself.

[docs/security.md](docs/security.md) describes Horizon's own pipeline (agents, CI, signed releases), most of which
Ferrite doesn't run.
