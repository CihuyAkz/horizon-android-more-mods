fn smartf3_hud(
    game: &mut crate::game::Game<'_>,
    size: [u32; 2],
) -> ::core::ops::ControlFlow<HudBatch> {
    game.smartf3_hud(size).map_or(
        ::core::ops::ControlFlow::Continue(()),
        ::core::ops::ControlFlow::Break,
    )
}
