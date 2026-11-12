fn sliders_punchy(
    game: &crate::game::Game<'_>,
    mod_id: &str,
    sliders: &mut Vec<horizon_client_render::mod_list::NativeSlider>,
) {
    if mod_id == "punchy" {
        crate::game::punchy::settings_sliders(game, sliders);
    }
}
