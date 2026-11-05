// Android static adapter: expose the mod in Options > Mods.
pub(super) fn run(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::android_bundled::accurate_block_placement as abp;
    mods.push(crate::game::native_hooks::NativeMod {
        id: abp::ID.into(),
        title: format!("Accurate Block Placement {}", abp::VERSION),
        description: abp::DESCRIPTION.into(),
        keys: vec![[
            abp::KEY.into(),
            "Toggle accurate placement".into(),
            abp::DEFAULT_KEY.into(),
        ]],
    });
}
