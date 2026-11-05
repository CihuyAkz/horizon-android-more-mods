// Adapted from Horizon's MIT-licensed ticket_levels.rs; extends the native range.
fn patched(s: &Sources) -> ::core::ops::ControlFlow<(i32, i32, i32)> {
::core::ops::ControlFlow::Break((|| {
    let view = s.view_distance.clamp(0, 128);
    let simulation = s.simulation_distance.clamp(0, 128);
    // PLAYER_LOADING within the view distance, PLAYER_SIMULATION at the
    // player's chunk (`getPlayerTicketLevel`: never below 0).
    let player_sim = i32::from(ENTITY_TICKING) - simulation;
    let reach = (view + 1).max(i32::from(BLOCK_TICKING) - player_sim);
    (view, player_sim, reach)
})())
}
