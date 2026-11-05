// SPDX-License-Identifier: MIT
fn extra_toasts(game:&mut crate::game::Game<'_>)->() {

    let advance=horizon_client_render::sodium_extra_filters::enabled(11);
    let recipes=horizon_client_render::sodium_extra_filters::enabled(12);
    if !advance || !recipes {game.adv.toasts.retain(|t| if t.is_recipe(){recipes}else{advance});}

}
