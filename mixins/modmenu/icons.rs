// The pause screen's icon row, moved left to make room for the mods icon,
// which goes after the last of them.
fn modmenu_icons(mut icons: [(f32, f32, &'static str); 4]) -> [(f32, f32, &'static str); 4] {
    for i in icons.iter_mut() {
        i.0 -= 12.0;
    }
    crate::game::modmenu::set_pause_icon(icons[3].0 + 24.0, icons[3].1);
    icons
}
