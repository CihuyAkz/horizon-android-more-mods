# Punchy

- Native mod ID: `punchy`
- Supplied HNMOD version: `0.1.2`
- Original archive: `punchy-rust-0.1.2.hnmod`
- Pinned Horizon revision: `81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045`
- Pinned horizon-mixins revision: `ddd9700b21997c95d95ac6ee5ecafe3830cf622a`
- Required hook API: `1`

## Port status

Integrated into the Android client as a statically bundled **preview**. The Punchy engine and animation data are compiled into the Android client; the port wires the first-person hand/item geometry, animation tick, keybinds, settings, and successful-use/mining triggers. It is not dynamically loaded from a runtime mods directory.

This is not complete Punchy renderer parity. Armour/body animation, every custom animation condition, and full Java/Fabric behaviour are not claimed as implemented. The Mods description identifies it as a preview; the short Mods title keeps the ON/OFF state visible.
