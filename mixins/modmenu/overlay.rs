// The mod list, over the screen it was opened from.
fn modmenu_overlay(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::modmenu::draw(game, batch);
}
