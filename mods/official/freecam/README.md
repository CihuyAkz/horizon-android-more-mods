# Freecam

- Native mod ID: `freecam`
- Supplied HNMOD version: `0.1.0`
- Original archive: `freecam-0.1.0.hnmod`
- Pinned Horizon revision: `af55a734865da4f1b11836164a979ff06a1f0449`
- Pinned horizon-mixins revision: `4342a0bfbe71d4d14185528d64e496d6022f16a1`
- Required hook API: `1`

## Android port status

Freecam is statically wired into the Android game build and appears in Options > Mods. Its toggle/reset keys, movement input, detached camera, camera turning, player visibility, optional player-freeze gate, settings, toast HUD, and clean disable/reset path are connected to the Android game hooks. It starts disabled; enable it explicitly in Options > Mods. The default toggle binding remains F4 and can be changed in the mod's key settings.

This is a partial port, not feature-complete parity with the original mod. The helper runtime documents remaining gaps: collision modes beyond `ignoreAll`, hand rendering, submersion fog, outline, control-player mode, full brightness, and server restrictions are not ported. Touch-only activation has not been verified; an external keyboard or remapped key may be needed on devices without a usable F4 binding. Android compilation and in-game camera movement have not yet been verified in this environment.
