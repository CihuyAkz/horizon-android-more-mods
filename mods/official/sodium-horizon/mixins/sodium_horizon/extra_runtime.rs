// SPDX-License-Identifier: MIT
// Original render-only filters. No lighting, world simulation or network changes.
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

// Bits 0..5: water, lava, fire, portal, other blocks, sculk animations.
// 6: particles; 7: named mod particles; 8..10: frames, stands, paintings.
// 11..13: advancement, recipe and tutorial toasts; 14: coordinates.
static ENABLED: AtomicU64 = AtomicU64::new(u64::MAX & !(1 << 14));
static PARTICLES: [AtomicU64; 8] = [const { AtomicU64::new(u64::MAX) }; 8];
static LAYERS: RwLock<Vec<u8>> = RwLock::new(Vec::new());

pub fn enabled(bit: usize) -> bool {
    ENABLED.load(Ordering::Relaxed) & (1 << bit) != 0
}
pub fn configure(mask: u64, disabled_particles: &[usize]) {
    for word in &PARTICLES {
        word.store(u64::MAX, Ordering::Relaxed);
    }
    for &id in disabled_particles {
        if let Some(word) = PARTICLES.get(id / 64) {
            word.fetch_and(!(1 << (id % 64)), Ordering::Relaxed);
        }
    }
    ENABLED.store(mask, Ordering::Relaxed);
}
pub fn particle_allowed(id: usize, named: bool) -> bool {
    enabled(6)
        && if named {
            enabled(7)
        } else {
            PARTICLES
                .get(id / 64)
                .is_none_or(|word| word.load(Ordering::Relaxed) & (1 << (id % 64)) != 0)
        }
}
pub fn category(name: &str) -> u8 {
    let path = name.strip_prefix("minecraft:").unwrap_or(name);
    if path.starts_with("block/water_") {
        0
    } else if path.starts_with("block/lava_") {
        1
    } else if path.starts_with("block/fire_") || path.starts_with("block/soul_fire_") {
        2
    } else if path == "block/nether_portal" || path == "block/end_portal" {
        3
    } else if path.starts_with("block/sculk") {
        5
    } else {
        4
    }
}
pub fn register_layer(layer: u32, name: &str) {
    let Ok(mut layers) = LAYERS.write() else {
        return;
    };
    // A malformed pack must not request an unbounded table allocation.
    if layer >= 1_048_576 {
        return;
    }
    let length = layers.len().max(layer as usize + 1);
    layers.resize(length, 4);
    layers[layer as usize] = category(name);
}
pub fn animation_allowed(layer: u32) -> bool {
    let mask = ENABLED.load(Ordering::Relaxed);
    if mask & 63 == 63 {
        return true;
    }
    let category = LAYERS
        .read()
        .ok()
        .and_then(|layers| layers.get(layer as usize).copied())
        .unwrap_or(4);
    mask & (1 << category) != 0
}
