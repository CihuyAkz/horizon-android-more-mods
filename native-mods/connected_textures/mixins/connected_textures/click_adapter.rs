// Android static adapter: persist a settings-page click to config/connected_textures.txt.
pub(super) fn run(mod_id: &str, button: usize) {
    use horizon_client_render::native_connected_textures as ct;
    if mod_id == ct::ID {
        ct::setting_click(button);
    }
}
