// The mod's row in Options > Mods.
fn modmenu_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "modmenu".into(),
        title: "Mod List 1.0.0".into(),
        description: "A Mods button on the title and pause screens, and this list with search, details and each mod's settings.".into(),
        keys: Vec::new(),
    });
}
