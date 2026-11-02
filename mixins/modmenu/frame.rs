// Once a frame: the title screen's button asks for the list.
fn modmenu_frame(game: &mut crate::game::Game<'_>) {
    crate::game::modmenu::frame(game);
}
