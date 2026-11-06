# Sodium Horizon

- Native mod ID: `sodium_horizon`
- Supplied HNMOD version: `0.3.1`
- Original archive: `sodium_horizon-0.3.1.hnmod`
- Pinned Horizon revision: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Pinned horizon-mixins revision: `4342a0bfbe71d4d14185528d64e496d6022f16a1`
- Required hook API: `1`

## Android port status

A partial Android runtime port is connected to the terrain mesher. When enabled in Options > Mods, it switches section face-visibility resolution to the included row/bitset resolver. The resolver is disabled by default and the original Horizon visibility path remains active until the player enables the mod.

This is not a complete Sodium Horizon port. The supplied settings screen, render-distance controls, extra particles/animations/sprites/frames/paintings/stands/stars/toasts/tutorials, and related UI helpers are not wired into Android. The optimized visibility path has source-level parity checks against the existing mesher reference, but Android compilation, device performance, and visual regression testing have not yet been run in this environment.
