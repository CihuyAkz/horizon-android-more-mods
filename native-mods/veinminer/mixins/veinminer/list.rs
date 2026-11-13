fn official_veinminer_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "veinminer".into(),
        title: "Vein Miner 0.3.2".into(),
        description: "Mine connected blocks in an ore or log vein on the server. Use /veinminer to check or change your per-player setting.".into(),
        keys: Vec::new(),
    });
}
