// The Mods page's button opens the mod list; the page itself stays out.
fn modmenu_options_page(page: Page) -> core::ops::ControlFlow<()> {
    if page == Page::Mods {
        modmenu_options::OPEN.store(true, std::sync::atomic::Ordering::Relaxed);
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
