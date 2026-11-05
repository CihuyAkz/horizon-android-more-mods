// Adapted from Horizon's MIT-licensed ticket_levels.rs; extends the native range.
fn patched(s: &Sources) -> ::core::ops::ControlFlow<Option<(i32, i32, i32, i32)>> {
::core::ops::ControlFlow::Break((|| {
    let sim = s.simulation_distance.clamp(0, 128);
    let boxes = s
        .players
        .iter()
        .map(|&p| (p, sim))
        .chain(s.forced.iter().map(|&f| (f, 0)))
        .chain(s.portal.iter().map(|&p| (p, PORTAL_RADIUS)));
    boxes.fold(None, |b, ((x, z), r)| {
        let (lo_x, lo_z, hi_x, hi_z) = (x - r, z - r, x + r, z + r);
        Some(match b {
            None => (lo_x, lo_z, hi_x, hi_z),
            Some((a, b, c, d)) => (lo_x.min(a), lo_z.min(b), hi_x.max(c), hi_z.max(d)),
        })
    })
})())
}
