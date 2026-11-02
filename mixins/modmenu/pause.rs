// Under the mod list, the pause screen isn't drawn.
fn modmenu_pause() -> core::ops::ControlFlow<()> {
    if crate::game::modmenu::is_open() {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
