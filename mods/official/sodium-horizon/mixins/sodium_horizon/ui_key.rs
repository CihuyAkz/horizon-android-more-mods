// SPDX-License-Identifier: MIT
fn ui_key(game:&mut crate::game::Game<'_>,key:winit::keyboard::KeyCode,text:Option<&str>)->std::ops::ControlFlow<Result<()>> {
    if crate::game::sodium_horizon_ui::key(game,key,text) { std::ops::ControlFlow::Break(Ok(())) } else { std::ops::ControlFlow::Continue(()) }
}
