// The pause screen's icon row, one row lower under Mod Menu's button.
fn modmenu_icons(mut icons: [(f32, f32, &'static str); 4]) -> [(f32, f32, &'static str); 4] {
    for i in icons.iter_mut() {
        i.1 += 24.0;
    }
    icons
}
