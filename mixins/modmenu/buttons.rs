// The pause screen's buttons with Mod Menu's "Mods" (action 40) under
// Advancements and Statistics, and the rows below it one row lower.
fn modmenu_buttons(
    mut buttons: Vec<(f32, f32, f32, &'static str, u8, bool)>,
) -> Vec<(f32, f32, f32, &'static str, u8, bool)> {
    let Some(&(x, top, ..)) = buttons.first() else {
        return buttons;
    };
    // Return to Game sits at y0 + 50.
    let y0 = top - 50.0;
    for b in buttons.iter_mut() {
        if b.1 >= y0 + 122.0 {
            b.1 += 24.0;
        }
    }
    buttons.push((x, y0 + 98.0, 204.0, "Mods", 40, true));
    buttons
}
