// Adapted from Horizon's MIT-licensed ticket_levels.rs; extends the native range.
fn patched(s: &Sources) -> ::core::ops::ControlFlow<FxHashMap<(i32, i32), (u8, u8)>> {
::core::ops::ControlFlow::Break((|| {
    let (view, player_sim, reach) = reach(s);
    let mut levels: FxHashMap<(i32, i32), (u8, u8)> = FxHashMap::default();
    let mut lower = |key, loading: i32, sim: i32| {
        let e = levels.entry(key).or_insert(NONE);
        e.0 = e.0.min(loading.min(i32::from(NONE.0)) as u8);
        e.1 = e.1.min(sim.clamp(0, i32::from(NONE.1)) as u8);
    };
    for &(px, pz) in &s.players {
        for dz in -reach..=reach {
            for dx in -reach..=reach {
                let d = dx.abs().max(dz.abs());
                lower(
                    (px + dx, pz + dz),
                    i32::from(ENTITY_TICKING) + (d - view).max(0),
                    player_sim + d,
                );
            }
        }
    }
    for &(fx, fz) in &s.forced {
        for dz in -1i32..=1 {
            for dx in -1i32..=1 {
                let level = i32::from(ENTITY_TICKING) + dx.abs().max(dz.abs());
                lower((fx + dx, fz + dz), level, level);
            }
        }
    }
    for &(cx, cz) in &s.portal {
        for dz in -PORTAL_RADIUS..=PORTAL_RADIUS {
            for dx in -PORTAL_RADIUS..=PORTAL_RADIUS {
                let level = PORTAL_LEVEL + dx.abs().max(dz.abs());
                lower((cx + dx, cz + dz), level, level);
            }
        }
    }
    levels.retain(|_, l| *l != NONE);
    levels
})())
}
