// A click on Mod Menu's "Mods" button opens the mod list.
fn modmenu_click(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<Result<()>> {
    let (w, h) = {
        let size = game
            .renderer
            .as_ref()
            .map_or([1, 1], |r| [r.config.width, r.config.height]);
        let b = HudBatch::new(size);
        (b.width, b.height)
    };
    let links = !game.server_links().is_empty();
    let hit = pause_buttons(w, h, game.external_server(), links)
        .into_iter()
        .any(|b| b.4 == 40 && ui::hovered(game.cursor, b.0, b.1, b.2, 20.0));
    if !hit {
        return core::ops::ControlFlow::Continue(());
    }
    game.audio.play_button_click();
    crate::game::modmenu::open(game);
    core::ops::ControlFlow::Break(Ok(()))
}
