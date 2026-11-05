// SPDX-License-Identifier: MIT
fn ui_text(game:&mut crate::game::Game<'_>,text:&str)->std::ops::ControlFlow<Result<()>> {
    if crate::game::sodium_horizon_ui::type_text(game,text) {std::ops::ControlFlow::Break(Ok(()))}
    else {std::ops::ControlFlow::Continue(())}
}
