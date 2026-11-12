fn slider_punchy(game: &crate::game::Game<'_>, mod_id: &str, slider: usize, value: f32, finished: bool) {
    if mod_id == "punchy" {
        crate::game::punchy::slider(game, slider, value, finished);
    }
}
