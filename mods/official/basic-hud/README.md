# The Basic HUD 0.5.0 — Android static port

Upstream project: https://github.com/repositorykreml1n/the-basic-hud
Upstream mod page: https://playhorizon.dev/mods/basic_hud
Upstream author: repositorykreml1n
License: MIT (see `LICENSE`).

This folder preserves the user-supplied 0.5.0 Rust source. The Android client
statically compiles it under `target_os = "android"`; it is not loaded from a
`.hnmod` file at runtime.

## Android integration

The Android adapter wires the module to the existing client hooks for:

- Options > Mods registration, settings, key binding and HUD editor input;
- HUD drawing, frame/tick updates and the enhanced Tab list;
- totem-use counts and remote-player health observations;
- recent pickup tracking, including the verified multiplayer packet handoff;
- spawn/remove cleanup for per-entity health cache entries.

The original HNMOD bridge for private `Cooldowns` fields is replaced with two
read-only accessors in `horizon-items::Cooldowns`. The native allocator/thread
callback injection is intentionally omitted: its helper is specific to the
Windows GNU thread-detach path and is not part of the Android runtime.

The upstream bundle pins Horizon `68ac6f647e7022b2c696e81863d7cface28c9416`
and horizon-mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. The Android
port adapts those hooks to the source tree in this archive; that does not prove
binary compatibility with other Horizon nightlies.

The static source checks can validate file paths and hook routing. Rust/Android
compilation and on-device runtime testing have not yet been run in this editing
environment because the Rust toolchain and Android SDK/NDK are unavailable.
