// Adapted from Horizon's MIT-licensed ticket_levels.rs; extends the native range.
fn patched(s: &Sources) -> ::core::ops::ControlFlow<Map> {
::core::ops::ControlFlow::Break((|| {
    let (view, player_sim, reach) = reach(s);
    let boxes = s
        .players
        .iter()
        .map(|&p| (p, reach))
        .chain(s.forced.iter().map(|&f| (f, 1)))
        .chain(s.portal.iter().map(|&p| (p, PORTAL_RADIUS)));
    let Some((x0, z0, x1, z1)) = boxes.fold(None, |b, ((x, z), r)| {
        let (lo_x, lo_z, hi_x, hi_z) = (
            i64::from(x) - i64::from(r),
            i64::from(z) - i64::from(r),
            i64::from(x) + i64::from(r),
            i64::from(z) + i64::from(r),
        );
        Some(match b {
            None => (lo_x, lo_z, hi_x, hi_z),
            Some((a, b, c, d)) => (lo_x.min(a), lo_z.min(b), hi_x.max(c), hi_z.max(d)),
        })
    }) else {
        return Map::Sparse(FxHashMap::default());
    };
    let (width, depth) = ((x1 - x0 + 1) as u64, (z1 - z0 + 1) as u64);
    if width * depth > GRID_CELLS {
        return Map::Sparse(compute_sparse(s));
    }
    let (x0, z0, width, depth) = (x0 as i32, z0 as i32, width as u32, depth as u32);
    let mut cells = vec![NONE; (width * depth) as usize].into_boxed_slice();
    let mut lower = |x: i32, z: i32, loading: i32, sim: i32| {
        let i = ((z - z0) as u32 * width + (x - x0) as u32) as usize;
        let e = &mut cells[i];
        e.0 = e.0.min(loading.min(i32::from(NONE.0)) as u8);
        e.1 = e.1.min(sim.clamp(0, i32::from(NONE.1)) as u8);
    };
    for &(px, pz) in &s.players {
        for dz in -reach..=reach {
            for dx in -reach..=reach {
                let d = dx.abs().max(dz.abs());
                lower(
                    px + dx,
                    pz + dz,
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
                lower(fx + dx, fz + dz, level, level);
            }
        }
    }
    for &(cx, cz) in &s.portal {
        for dz in -PORTAL_RADIUS..=PORTAL_RADIUS {
            for dx in -PORTAL_RADIUS..=PORTAL_RADIUS {
                let level = PORTAL_LEVEL + dx.abs().max(dz.abs());
                lower(cx + dx, cz + dz, level, level);
            }
        }
    }
    Map::Grid {
        x0,
        z0,
        width,
        depth,
        cells,
    }
})())
}
