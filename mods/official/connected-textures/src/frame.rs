// Android static adapter: load settings and invalidate chunks on changes.
pub(super) fn run(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::native_connected_textures as ct;
    if !ct::loaded() {
        ct::load(crate::game::Game::native_config_dir().ok());
    }
    ct::set_active(game.native_on(ct::ID));
    if ct::take_remesh() {
        game.all_sections_changed();
    }
}
