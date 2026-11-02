// The main title screen is showing: the "Mods" button goes on it.
fn modmenu_title_draw(title: &Title) {
    let main = matches!(title.mode, Mode::Main) && title.rules.borrow().is_none();
    modmenu_title::MAIN.store(main, std::sync::atomic::Ordering::Relaxed);
}
