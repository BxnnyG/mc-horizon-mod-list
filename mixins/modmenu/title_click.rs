// A click on the title screen's "Mods" button.
fn modmenu_title_click(
    title: &Title,
    x: f32,
    y: f32,
    size: (f32, f32),
) -> core::ops::ControlFlow<(Action, bool)> {
    let r = modmenu_title::rect(size.0, size.1);
    let main = matches!(title.mode, Mode::Main) && title.rules.borrow().is_none();
    if main && x >= r[0] && x < r[0] + r[2] && y >= r[1] && y < r[1] + r[3] {
        modmenu_title::OPEN.store(true, std::sync::atomic::Ordering::Relaxed);
        core::ops::ControlFlow::Break((Action::None, true))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
