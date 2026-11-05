// SPDX-License-Identifier: MIT
fn ui_mouse(game:&mut crate::game::Game<'_>,pressed:bool,right:bool)->std::ops::ControlFlow<Result<()>> {
    if crate::game::sodium_horizon_ui::mouse(game,pressed,right,false) { std::ops::ControlFlow::Break(Ok(())) } else { std::ops::ControlFlow::Continue(()) }
}
