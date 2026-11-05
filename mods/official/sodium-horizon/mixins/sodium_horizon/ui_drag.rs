// SPDX-License-Identifier: MIT
fn ui_drag(game:&mut crate::game::Game<'_>)->std::ops::ControlFlow<Result<()>> {
    if crate::game::sodium_horizon_ui::mouse(game,true,false,true) { std::ops::ControlFlow::Break(Ok(())) } else { std::ops::ControlFlow::Continue(()) }
}
