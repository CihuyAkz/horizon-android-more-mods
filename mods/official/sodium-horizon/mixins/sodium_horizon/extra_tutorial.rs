// SPDX-License-Identifier: MIT
fn extra_tutorial(game:&mut crate::game::Game<'_>)->std::ops::ControlFlow<()> {

    if !horizon_client_render::sodium_extra_filters::enabled(13) {
        game.tutorial_toasts.clear();
        std::ops::ControlFlow::Break(())
    } else {std::ops::ControlFlow::Continue(())}

}
