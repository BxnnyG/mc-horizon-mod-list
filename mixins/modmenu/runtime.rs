// Mod List for Horizon: the mods in a list on the left with a search box,
// the selected one's details on the right, with its settings and a switch.
// An independent screen laid out as Mod Menu lays its own out. This file is the
// module the bundle adds to the game (`crate::game::modmenu`).
use super::Game;
use horizon_client::hud::HudBatch;
use horizon_client::options_ui::{Event, Page};
use std::cell::RefCell;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::PhysicalKey;

const ROW: f32 = 36.0;
type R = [f32; 4];

#[derive(Default)]
struct State {
    open: bool,
    selected: Option<String>,
    scroll: f32,
    search: String,
    /// A mod's settings are open: the list comes back when they close.
    reopen: bool,
    /// The filter row is open; sorted Z-A.
    filters: bool,
    descending: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

fn inside(r: R, p: (f32, f32)) -> bool {
    p.0 >= r[0] && p.0 < r[0] + r[2] && p.1 >= r[1] && p.1 < r[1] + r[3]
}

/// Whether the list is showing.
pub(super) fn is_open() -> bool {
    with(|s| s.open)
}

pub(super) fn open(game: &mut Game<'_>) {
    game.list_native_mods();
    with(|s| {
        s.open = true;
        s.scroll = 0.0;
    });
}

pub(super) fn frame(game: &mut Game<'_>) {
    use std::sync::atomic::Ordering::Relaxed;
    let asked = horizon_client::title::modmenu_title::OPEN.swap(false, Relaxed)
        || horizon_client::options_ui::modmenu_options::OPEN.swap(false, Relaxed);
    let closed = with(|s| s.reopen) && game.screen != super::Screen::Options;
    if closed {
        with(|s| s.reopen = false);
    }
    // Back on the pause or title screen; a mod's settings may also have gone
    // on into the game (the maps' Change Position).
    let back = closed && (game.title.is_some() || game.screen == super::Screen::Paused);
    if asked || back {
        open(game);
    }
}

/// `Name 1.0` as name and version.
fn split_title(title: &str) -> (&str, &str) {
    match title.rsplit_once(' ') {
        Some((n, v)) if v.chars().next().is_some_and(|c| c.is_ascii_digit()) => (n, v),
        _ => (title, ""),
    }
}

/// The mods shown, by name: their places in the game's list.
fn entries(game: &Game<'_>) -> Vec<usize> {
    let query = with(|s| s.search.to_lowercase());
    let mods = &game.settings.mods.installed;
    let mut list: Vec<usize> = (0..mods.len())
        .filter(|&i| {
            query.is_empty()
                || mods[i].title.to_lowercase().contains(&query)
                || mods[i].description.to_lowercase().contains(&query)
        })
        .collect();
    list.sort_by_key(|&i| mods[i].title.to_lowercase());
    if with(|s| s.descending) {
        list.reverse();
    }
    list
}

/// What Horizon doesn't keep about a mod: who made it, and its page.
fn credits(id: &str) -> (&'static str, &'static str) {
    const ME: &str = "BxnnyG";
    match id {
        "appleskin" => ("soufi", "https://github.com/soufied/AppleSkin-Horizon"),
        "jei" => ("enderliker", "https://github.com/enderliker/horizon-jei"),
        "lookat" => ("d3grando", "https://github.com/d3grando/lookat"),
        "smartf3" => ("d3grando", "https://github.com/d3grando/smartf3"),
        "minimap" | "zoom" => ("Horizon", "https://github.com/horizon-rust/horizon-mods"),
        "veinminer" => ("levzzz5154", "https://github.com/levzzz5154/horizon-veinminer"),
        "xmaps" => (ME, ""),
        "ipn" => (ME, ""),
        "zoomify" => (ME, "https://github.com/BxnnyG/mc-horizon-zoom"),
        "gammautils" => (ME, "https://github.com/BxnnyG/mc-horizon-gamma"),
        "chathistory" => (ME, "https://github.com/BxnnyG/mc-horizon-chat-history"),
        "closet" => (ME, "https://github.com/BxnnyG/mc-horizon-sleep-monsters"),
        "modmenu" => (ME, "https://github.com/BxnnyG/mc-horizon-mod-list"),
        "modcontrols" => (ME, "https://github.com/BxnnyG/mc-horizon-mod-keys"),
        _ => ("Unknown", ""),
    }
}

/// The screen's parts, placed as Mod Menu places them.
struct Layout {
    pane: f32,
    right: f32,
    search: R,
    filters: R,
    sort: R,
    list: R,
    description: R,
    configure: R,
    website: R,
    toggle: R,
    folder: R,
    done: R,
}
const DETAILS_Y: f32 = 48.0;

fn layout(w: f32, h: f32) -> Layout {
    let pane = (w / 2.0).floor() - 8.0;
    let right = w - pane;
    let top = DETAILS_Y + 19.0;
    let search_w = pane - 32.0 - 22.0;
    let search_x = ((pane - search_w) / 2.0 - 11.0).floor();
    let half = ((pane / 2.0 - 2.0).min(200.0)).floor();
    Layout {
        pane,
        right,
        search: [search_x, 22.0, search_w, 20.0],
        filters: [search_x + search_w + 2.0, 22.0, 20.0, 20.0],
        sort: [pane - 92.0, 45.0, 88.0, 20.0],
        list: [0.0, top, pane, h - top - 36.0],
        description: [right, DETAILS_Y + 60.0, pane, h - DETAILS_Y - 96.0],
        configure: [w - 24.0, DETAILS_Y, 20.0, 20.0],
        website: [right + (pane / 2.0 - 2.0 - half) / 2.0, DETAILS_Y + 36.0, half, 20.0],
        toggle: [right + pane / 2.0 + 2.0 + (pane / 2.0 - 2.0 - half) / 2.0, DETAILS_Y + 36.0, half, 20.0],
        folder: [(w / 2.0).floor() - 154.0, h - 28.0, 150.0, 20.0],
        done: [(w / 2.0).floor() + 4.0, h - 28.0, 150.0, 20.0],
    }
}

fn gui(game: &Game<'_>) -> (f32, f32, f32) {
    game.renderer.as_ref().map_or((1.0, 1.0, 1.0), |r| {
        let b = HudBatch::new([r.config.width, r.config.height]);
        (b.width, b.height, b.scale)
    })
}

/// Words broken into lines at most `width` wide.
fn wrap(game: &Game<'_>, text: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let next = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
        if game.native_text_width(&next) > width && !line.is_empty() {
            lines.push(std::mem::replace(&mut line, word.to_owned()));
        } else {
            line = next;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// `text` cut to `width`, with dots when it was longer.
fn cut(game: &Game<'_>, text: &str, width: f32) -> String {
    if game.native_text_width(text) <= width {
        return text.to_owned();
    }
    let mut out = text.to_owned();
    while !out.is_empty() && game.native_text_width(&format!("{out}...")) > width {
        out.pop();
    }
    format!("{}...", out.trim_end())
}

/// The item that stands for a mod, as Horizon's mods carry no pictures.
fn icon_item(id: &str) -> Option<&'static str> {
    Some(match id {
        "appleskin" => "minecraft:golden_apple",
        "chathistory" => "minecraft:writable_book",
        "closet" => "minecraft:red_bed",
        "gammautils" => "minecraft:glowstone",
        "inventory_profiles" | "ipn" => "minecraft:chest",
        "jei" => "minecraft:crafting_table",
        "lookat" => "minecraft:ender_eye",
        "modcontrols" => "minecraft:lever",
        "modmenu" => "minecraft:bookshelf",
        "smartf3" => "minecraft:comparator",
        "veinminer" => "minecraft:diamond_pickaxe",
        "xmaps" => "minecraft:filled_map",
        "zoomify" => "minecraft:spyglass",
        _ => return None,
    })
}

/// A mod's picture: its item, or its initials on a colour taken from its id.
fn draw_icon(game: &Game<'_>, batch: &mut HudBatch, id: &str, name: &str, r: R, on: bool) {
    if let Some(item) = icon_item(id) {
        game.native_fill(batch, r, if on { 0x3090_90A0 } else { 0x3040_4040 });
        let side = r[2] * 0.8;
        let at = [r[0] + (r[2] - side) / 2.0, r[1] + (r[3] - side) / 2.0];
        if game.native_item(batch, item, at, side) {
            return;
        }
    }
    let hash = id.bytes().fold(2166136261u32, |h, b| (h ^ u32::from(b)).wrapping_mul(16777619));
    let color = 0xFF00_0000 | (hash & 0x7F_7F7F) | 0x30_3030;
    game.native_fill(batch, r, if on { color } else { 0xFF40_4040 });
    let mut words = name.split_whitespace();
    let initials: String = match (words.next(), words.next()) {
        (Some(a), Some(b)) => a.chars().take(1).chain(b.chars().take(1)).collect(),
        (Some(a), None) => a.chars().take(2).collect(),
        _ => "?".into(),
    };
    let start = batch.vertices.len();
    let tw = game.native_text_width(&initials);
    game.native_text(batch, &initials, [r[0], r[1]], 0xFF_FFFF, true);
    // The letters fill the picture.
    let k = (r[2] * 0.7 / tw.max(1.0)).min(r[3] * 0.6 / 8.0);
    batch.scale_since(start, r[0], r[1], k, k);
    let (dx, dy) = ((r[2] - tw * k) / 2.0, (r[3] - 8.0 * k) / 2.0);
    for v in &mut batch.vertices[start..] {
        v.pos[0] += dx;
        v.pos[1] += dy;
    }
}

fn button(game: &Game<'_>, batch: &mut HudBatch, r: R, text: &str) {
    horizon_client::ui::button(
        batch,
        &game.gui.atlas,
        r[0],
        r[1],
        r[2],
        r[3],
        &horizon_client::text::Text::literal(text),
        true,
        inside(r, game.cursor),
    );
}

/// Mod Menu's gear button's picture.
fn gear(game: &Game<'_>, batch: &mut HudBatch, r: R, color: u32) {
    let (cx, cy) = (r[0] + r[2] / 2.0, r[1] + r[3] / 2.0);
    let atlas = &game.gui.atlas;
    let square = |radius: f32, turn: f32| -> [[f32; 2]; 4] {
        std::array::from_fn(|i| {
            let a = turn + i as f32 * std::f32::consts::FRAC_PI_2;
            [cx + radius * a.cos(), cy + radius * a.sin()]
        })
    };
    batch.fill_quad(atlas, square(6.5, 0.0), color);
    batch.fill_quad(atlas, square(6.5, std::f32::consts::FRAC_PI_4), color);
    batch.fill_quad(atlas, square(2.8, std::f32::consts::FRAC_PI_4), horizon_client::hud::rgba(70, 70, 70, 255));
}

pub(super) fn draw(game: &mut Game<'_>, batch: &mut HudBatch) {
    use horizon_client::title::modmenu_title as t;
    let main = t::MAIN.swap(false, std::sync::atomic::Ordering::Relaxed);
    if !with(|s| s.open) {
        if main {
            let r = t::rect(batch.width, batch.height);
            button(game, batch, r, "Mods");
        }
        return;
    }
    let (w, h) = (batch.width, batch.height);
    let l = layout(w, h);
    // The pause screen makes way (`pause.rs`); the others are covered.
    if game.screen != super::Screen::Paused {
        game.native_fill(batch, [0.0, 0.0, w, h], 0xFF10_1014);
    }
    horizon_client::ui::menu_background(batch, &game.gui.atlas, 0.0, 0.0, w, h, true);
    let title = "Mods";
    let tw = game.native_text_width(title);
    game.native_text(batch, title, [(l.pane - tw) / 2.0, 8.0], 0xFF_FFFF, true);
    // The search box and the filter button beside it.
    let (search, filters) = with(|s| (s.search.clone(), s.filters));
    horizon_client::ui::nine_slice(
        batch,
        &game.gui.atlas,
        "gui/sprites/widget/text_field_highlighted",
        l.search[0],
        l.search[1],
        l.search[2],
        l.search[3],
        u32::MAX,
    );
    let at = [l.search[0] + 4.0, l.search[1] + 6.0];
    if search.is_empty() {
        game.native_text(batch, "Search mods", at, 0x80_8080, false);
    } else {
        game.native_text(batch, &format!("{search}_"), at, 0xE0_E0E0, true);
    }
    button(game, batch, l.filters, "");
    for (i, width) in [12.0, 8.0, 4.0].iter().enumerate() {
        let x = l.filters[0] + (20.0 - width) / 2.0;
        game.native_fill(batch, [x, l.filters[1] + 6.0 + i as f32 * 3.0, *width, 2.0], 0xFFE0_E0E0);
    }
    let list = entries(game);
    let count = format!("Showing {} mods", list.len());
    game.native_text(batch, &count, [l.search[0], 52.0], 0xFF_FFFF, true);
    if filters {
        let label = if with(|s| s.descending) { "Sort: Z-A" } else { "Sort: A-Z" };
        button(game, batch, l.sort, label);
    }
    // The list.
    game.native_fill(batch, l.list, 0x6000_0000);
    let selected = with(|s| s.selected.clone());
    let chosen = selected
        .and_then(|id| list.iter().copied().find(|&i| game.settings.mods.installed[i].id == id))
        .or(list.first().copied());
    let scroll = with(|s| s.scroll);
    let start = batch.vertices.len();
    for (n, &i) in list.iter().enumerate() {
        let y = l.list[1] + 4.0 + n as f32 * ROW - scroll;
        if y + ROW < l.list[1] || y > l.list[1] + l.list[3] {
            continue;
        }
        let row = [l.list[0] + 2.0, y - 2.0, l.list[2] - 8.0, ROW];
        let (id, title, description) = {
            let m = &game.settings.mods.installed[i];
            (m.id.clone(), m.title.clone(), m.description.clone())
        };
        let on = game.settings.mods.is_on(i);
        if chosen == Some(i) {
            game.native_fill(batch, row, 0xFFA0_A0A0);
            game.native_fill(batch, [row[0] + 1.0, row[1] + 1.0, row[2] - 2.0, row[3] - 2.0], 0xFF00_0000);
        }
        let (name, version) = split_title(&title);
        let x = row[0] + 2.0;
        draw_icon(game, batch, &id, name, [x, y, 32.0, 32.0], on);
        let width = row[2] - 40.0;
        let color = if on { 0xFF_FFFF } else { 0x80_8080 };
        let tag = if on { format!("{name} ({version})") } else { format!("{name} (disabled)") };
        game.native_text(batch, &cut(game, &tag, width), [x + 35.0, y + 1.0], color, false);
        for (k, line) in wrap(game, &description, width - 4.0).into_iter().take(2).enumerate() {
            game.native_text(batch, &line, [x + 39.0, y + 11.0 + k as f32 * 10.0], 0x80_8080, false);
        }
    }
    batch.clip_since(start, l.list[0], l.list[1], l.list[0] + l.list[2], l.list[1] + l.list[3]);
    // The right side: the selected mod.
    if let Some(i) = chosen {
        let (id, title, description, problem, native) = {
            let m = &game.settings.mods.installed[i];
            (m.id.clone(), m.title.clone(), m.description.clone(), m.problem.clone(), m.native.is_some())
        };
        let on = game.settings.mods.is_on(i);
        let (name, version) = split_title(&title);
        let (author, page) = credits(&id);
        let (x, y) = (l.right, DETAILS_Y);
        draw_icon(game, batch, &id, name, [x, y, 32.0, 32.0], on);
        game.native_text(batch, name, [x + 35.0, y + 1.0], 0xFF_FFFF, false);
        game.native_text(batch, &format!("v{version}"), [x + 35.0, y + 12.0], 0x80_8080, false);
        game.native_text(batch, &format!("By {author}"), [x + 35.0, y + 22.0], 0x80_8080, false);
        if native {
            button(game, batch, l.configure, "");
            gear(game, batch, l.configure, horizon_client::hud::rgba(225, 225, 225, 255));
        }
        if !page.is_empty() {
            button(game, batch, l.website, "Website");
        }
        button(game, batch, l.toggle, if on { "Disable" } else { "Enable" });
        let mut ty = l.description[1] + 2.0;
        for line in wrap(game, &description, l.description[2] - 8.0) {
            game.native_text(batch, &line, [x, ty], 0xE0_E0E0, false);
            ty += 10.0;
        }
        if let Some(p) = problem {
            ty += 6.0;
            for line in wrap(game, &p, l.description[2] - 8.0) {
                game.native_text(batch, &line, [x, ty], 0xFF_5555, false);
                ty += 10.0;
            }
        }
        if !page.is_empty() {
            ty += 6.0;
            game.native_text(batch, "Links:", [x, ty], 0xE0_E0E0, false);
            game.native_text(batch, &cut(game, page, l.description[2] - 16.0), [x + 8.0, ty + 10.0], 0x55_55FF, false);
        }
    }
    button(game, batch, l.folder, "Open Mods Folder");
    button(game, batch, l.done, "Done");
}

/// A mod's page in the browser.
fn open_page(url: &str) {
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

fn close() {
    with(|s| s.open = false);
}

pub(super) fn input(game: &mut Game<'_>, event: &WindowEvent) -> bool {
    if !with(|s| s.open) {
        return false;
    }
    let (w, h, scale) = gui(game);
    let l = layout(w, h);
    match event {
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state != ElementState::Pressed {
                return true;
            }
            let name = match event.physical_key {
                PhysicalKey::Code(code) => horizon_client::keybinds::key_name(code).unwrap_or(""),
                _ => "",
            };
            match name {
                "key.keyboard.escape" => close(),
                "key.keyboard.backspace" => with(|s| {
                    s.search.pop();
                    s.scroll = 0.0;
                }),
                _ => {
                    if let Some(text) = event.text.as_ref() {
                        with(|s| {
                            s.search.extend(text.chars().filter(|c| !c.is_control()));
                            s.scroll = 0.0;
                        });
                    }
                }
            }
            true
        }
        WindowEvent::CursorMoved { position, .. } => {
            game.cursor = (position.x as f32 / scale, position.y as f32 / scale);
            true
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let steps = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y,
                MouseScrollDelta::PixelDelta(p) => (p.y / 30.0) as f32,
            };
            let max = (entries(game).len() as f32 * ROW - l.list[3]).max(0.0);
            with(|s| s.scroll = (s.scroll - steps * ROW).clamp(0.0, max));
            true
        }
        WindowEvent::MouseInput { state, button, .. } => {
            if *state != ElementState::Pressed || *button != MouseButton::Left {
                return true;
            }
            let at = game.cursor;
            let list = entries(game);
            let id_of = |game: &Game<'_>, i: usize| game.settings.mods.installed[i].id.clone();
            let selected = with(|s| s.selected.clone());
            let chosen = selected
                .and_then(|id| list.iter().copied().find(|&i| id_of(game, i) == id))
                .or(list.first().copied());
            if inside(l.done, at) {
                game.audio.play_button_click();
                close();
            } else if inside(l.filters, at) {
                game.audio.play_button_click();
                with(|s| s.filters = !s.filters);
            } else if with(|s| s.filters) && inside(l.sort, at) {
                game.audio.play_button_click();
                with(|s| s.descending = !s.descending);
            } else if inside(l.folder, at) {
                game.audio.play_button_click();
                let _ = game.handle_option_events(vec![Event::OpenModFolder]);
            } else if inside(l.list, at) {
                let n = ((at.1 - l.list[1] - 2.0 + with(|s| s.scroll)) / ROW) as usize;
                if let Some(&i) = list.get(n) {
                    let id = id_of(game, i);
                    with(|s| s.selected = Some(id));
                }
            } else if let Some(i) = chosen {
                if inside(l.configure, at) && game.settings.mods.installed[i].native.is_some() {
                    game.audio.play_button_click();
                    close();
                    let from_title = game.title.is_some();
                    game.open_options_at(from_title, Page::NativeMod(i));
                    game.native_settings_refresh(i);
                    with(|s| s.reopen = true);
                } else if inside(l.website, at) {
                    let (_, page) = credits(&game.settings.mods.installed[i].id);
                    if !page.is_empty() {
                        game.audio.play_button_click();
                        open_page(page);
                    }
                } else if inside(l.toggle, at) {
                    game.audio.play_button_click();
                    game.settings.mods.toggle(i);
                    let _ = game.handle_option_events(vec![Event::Changed]);
                }
            }
            true
        }
        _ => false,
    }
}
