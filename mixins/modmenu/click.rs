// A click on the mods icon in the pause screen's row opens the mod list.
fn modmenu_click(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<Result<()>> {
    let [x, y] = crate::game::modmenu::pause_icon();
    if !ui::hovered(game.cursor, x, y, 20.0, 20.0) {
        return core::ops::ControlFlow::Continue(());
    }
    game.audio.play_button_click();
    crate::game::modmenu::open(game);
    core::ops::ControlFlow::Break(Ok(()))
}
