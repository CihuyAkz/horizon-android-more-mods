// SPDX-License-Identifier: MIT
fn ui_draw(game:&mut crate::game::Game<'_>,batch:&mut horizon_client::hud::HudBatch)->std::ops::ControlFlow<()> {
    if crate::game::sodium_horizon_ui::draw(game,batch) { std::ops::ControlFlow::Break(()) } else { std::ops::ControlFlow::Continue(()) }
}
