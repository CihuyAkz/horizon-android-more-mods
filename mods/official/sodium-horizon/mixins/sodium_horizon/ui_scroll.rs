// SPDX-License-Identifier: MIT
fn ui_scroll(game:&mut crate::game::Game<'_>,amount:f32)->std::ops::ControlFlow<Result<()>> {
    if crate::game::sodium_horizon_ui::scroll(game,amount) { std::ops::ControlFlow::Break(Ok(())) } else { std::ops::ControlFlow::Continue(()) }
}
