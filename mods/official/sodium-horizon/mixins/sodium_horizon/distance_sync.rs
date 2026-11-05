// Adapted from Horizon's MIT-licensed game.rs; extend ticket view distance.
fn patched(game: &mut Game<'_>) -> ::core::ops::ControlFlow<()> {
::core::ops::ControlFlow::Break((|| {
        let rule = game.ui.rules.bool("spectators_generate_chunks");
        let mut players: Vec<((i32, i32), bool)> = game.lan.as_ref().map_or_else(Vec::new, |h| {
            h.hub
                .guests
                .iter()
                .filter(|g| h.hub.here(g))
                .map(|g| (g.chunk(), g.spectator()))
                .collect()
        });
        if let Some(p) = &game.player {
            players.push((
                (
                    (p.position.x.floor() as i32) >> 4,
                    (p.position.z.floor() as i32) >> 4,
                ),
                game.spectator(),
            ));
        }
        // A spectator the rule skips is still a player: a game that has one
        // must not take the headless fixtures' tick-everywhere case below.
        let any_player = !players.is_empty();
        let sources = (!game.options.execution_role.is_client() || any_player).then(|| {
            horizon_client::ticket_levels::Sources {
                players: players
                    .into_iter()
                    .filter(|&(_, spectator)| game_mode::loads_chunks(rule, spectator))
                    .map(|(chunk, _)| chunk)
                    .collect(),
                forced: game.streaming.forced.iter().copied().collect(),
                // Filled from the `PORTAL` tickets by `update`.
                portal: Vec::new(),
                view_distance: game.settings.render_distance.clamp(2, 128),
                simulation_distance: game.settings.simulation_distance.max(2),
            }
        });
        game.tickets.update(sources);
    })())
}
