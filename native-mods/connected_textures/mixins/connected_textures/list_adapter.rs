// Android static adapter: expose the renderer mod in Options > Mods.
pub(super) fn run(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_render::native_connected_textures as ct;
    mods.push(crate::game::native_hooks::NativeMod {
        id: ct::ID.into(),
        title: format!("{} {}", ct::NAME, ct::VERSION),
        description: ct::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
