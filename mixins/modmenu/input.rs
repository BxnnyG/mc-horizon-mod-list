// The window's input, while the mod list is open.
fn modmenu_input(
    game: &mut crate::game::Game<'_>,
    event: &winit::event::WindowEvent,
) -> core::ops::ControlFlow<()> {
    if crate::game::modmenu::input(game, event) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
