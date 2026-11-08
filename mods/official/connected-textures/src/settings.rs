// Android static adapter: settings page.
pub(super) fn run(mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client_render::native_connected_textures as ct;
    if mod_id == ct::ID {
        buttons.extend(ct::setting_labels());
    }
}
