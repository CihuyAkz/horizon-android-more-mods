// Original Rust renderer using measurements and text observed from the supplied JARs.
use super::{
    sodium_horizon_exact_model as m, sodium_horizon_model::Editor, sodium_horizon_ui as backend,
    Game, Screen,
};
use horizon_client::{
    hud::{rgba, HudBatch},
    options_file::V,
    settings::Settings,
    text::Text,
};
use std::cell::RefCell;
use winit::keyboard::KeyCode;
struct State {
    values: Vec<i32>,
    original: Vec<i32>,
    scroll: f32,
    page: usize,
    query: String,
    search: bool,
    drag: Option<usize>,
    focus: usize,
    donate: bool,
}
thread_local! {static STATE:RefCell<Option<State>>=const{RefCell::new(None)};}
pub(super) fn clear() {
    STATE.with_borrow_mut(|s| *s = None);
}
fn extra(s: &Settings, id: i32) -> Option<i32> {
    Some(match id {
        171 => (s.gamma * 100.0).round() as i32,
        172 => s.attack_indicator as i32,
        173 => s.autosave_indicator as i32,
        174 => s.cloud_range,
        175 => s.weather_radius,
        176 => s.smooth_lighting as i32,
        177 => s.biome_blend,
        178 => s.entity_shadows as i32,
        179 => s.vignette as i32,
        180 => (s.chunk_fade * 1000.0).round() as i32,
        181 => s.file.int("textureFiltering"),
        182 => s.file.int("maxAnisotropyBit"),
        183 => s.improved_transparency as i32,
        184 => i32::from(s.inactivity_fps_limit == "afk"),
        185 => s.file.bool("advancedItemTooltips") as i32,
        188 => s.extras.hide_block_particles as i32,
        190 => {
            if !s.fullscreen {
                0
            } else if s.file.bool("exclusiveFullscreen") {
                1
            } else {
                2
            }
        }
        _ => return None,
    })
}
fn write_extra(s: &mut Settings, id: i32, v: i32) {
    match id {
        171 => s.gamma = v as f32 / 100.0,
        172 => s.attack_indicator = v as u8,
        173 => s.autosave_indicator = v != 0,
        174 => s.cloud_range = v,
        175 => s.weather_radius = v,
        176 => s.smooth_lighting = v != 0,
        177 => s.biome_blend = v,
        178 => s.entity_shadows = v != 0,
        179 => s.vignette = v != 0,
        180 => s.chunk_fade = v as f32 / 1000.0,
        181 => s.file.set("textureFiltering", V::Int(v)),
        182 => s.file.set("maxAnisotropyBit", V::Int(v)),
        183 => s.improved_transparency = v != 0,
        184 => s.inactivity_fps_limit = if v == 0 { "minimized" } else { "afk" }.into(),
        185 => s.file.set("advancedItemTooltips", V::Bool(v != 0)),
        188 => s.extras.hide_block_particles = v != 0,
        190 => {
            s.fullscreen = v != 0;
            s.file.set("exclusiveFullscreen", V::Bool(v == 1));
        }
        _ => {}
    }
}
fn convert(value: i32, kind: u8, reverse: bool) -> i32 {
    match kind {
        1 => i32::from(value == 0),
        2 => {
            if reverse {
                [0, 3, 1].get(value as usize).copied().unwrap_or(0)
            } else {
                match value {
                    3 => 1,
                    1 => 2,
                    _ => 0,
                }
            }
        }
        4 => {
            if reverse {
                if value == 0 {
                    0
                } else {
                    1
                }
            } else {
                i32::from(value != 0)
            }
        }
        _ => value,
    }
}
fn supported(game: &Game<'_>, i: usize) -> bool {
    let b = m::OPTIONS[i].binding;
    (0..171).contains(&b) || extra(&game.settings, b).is_some()
}
fn snapshot(game: &Game<'_>) -> Vec<i32> {
    let values = backend::read(game);
    m::OPTIONS
        .iter()
        .map(|o| {
            let v = if (0..171).contains(&o.binding) {
                Some(values[o.binding as usize])
            } else {
                extra(&game.settings, o.binding)
            };
            v.map_or(o.value, |v| convert(v, o.transform, false))
        })
        .collect()
}
fn state(game: &Game<'_>, f: impl FnOnce(&mut State)) {
    STATE.with_borrow_mut(|slot| {
        let s = slot.get_or_insert_with(|| {
            let values = snapshot(game);
            State {
                original: values.clone(),
                values,
                scroll: 0.0,
                page: 0,
                query: String::new(),
                search: false,
                drag: None,
                focus: 0,
                donate: true,
            }
        });
        f(s)
    });
}
fn dirty(s: &State) -> bool {
    s.values != s.original
}
fn rows(s: &State) -> Vec<usize> {
    m::OPTIONS
        .iter()
        .enumerate()
        .filter_map(|(i, o)| {
            (s.query.is_empty() || o.name.to_lowercase().contains(&s.query.to_lowercase()))
                .then_some(i)
        })
        .collect()
}
fn pos(l: m::Layout, s: &State, i: usize) -> [f32; 4] {
    if s.query.is_empty() {
        l.row(i, s.scroll)
    } else {
        let p = rows(s).iter().position(|&v| v == i).unwrap_or(0);
        [
            l.list_x() + 3.0,
            l.top + p as f32 * 18.0 - s.scroll,
            204.0,
            18.0,
        ]
    }
}
fn limit(l: m::Layout, s: &State) -> f32 {
    if s.query.is_empty() {
        l.max_scroll()
    } else {
        (rows(s).len() as f32 * 18.0 - (l.bottom - l.top)).max(0.0)
    }
}
fn txt(g: &Game<'_>, b: &mut HudBatch, t: &str, x: f32, y: f32, color: u32) {
    let [_, r, gg, bb] = color.to_be_bytes();
    b.text_styled(
        &g.gui.atlas,
        &Text::legacy(t),
        x,
        y,
        rgba(r, gg, bb, 255),
        true,
    );
}
fn width(g: &Game<'_>, t: &str) -> f32 {
    g.gui.atlas.font.width_text(&Text::legacy(t))
}
fn fit(g: &Game<'_>, s: &str, w: f32) -> String {
    if width(g, s) <= w {
        return s.into();
    }
    let mut s = s.to_owned();
    while !s.is_empty() && width(g, &format!("{s}...")) > w {
        s.pop();
    }
    format!("{s}...")
}
fn label(i: usize, v: i32) -> String {
    let o = m::OPTIONS[i];
    if matches!(o.binding, 0 | 1) { return format!("{v} Chunks"); }
    o.labels
        .iter()
        .find(|(n, _)| *n == v)
        .map_or_else(|| v.to_string(), |(_, s)| (*s).into())
}
fn fill(g: &Game<'_>, b: &mut HudBatch, r: [f32; 4], c: u32) {
    g.native_fill(b, r, c)
}
fn page_color(page: usize, light: bool) -> u32 {
    if page < 3 {
        if light {
            0xFFCAFDED
        } else {
            0xFF94E4D2
        }
    } else if light {
        0xFFD2C0FF
    } else {
        0xFFAB94E4
    }
}
fn button(g: &Game<'_>, b: &mut HudBatch, r: [f32; 4], text: &str, on: bool, mouse: [f32; 2]) {
    fill(
        g,
        b,
        r,
        if on && m::hit(mouse, r) {
            0xE0000000
        } else if on {
            0x90000000
        } else {
            0x40000000
        },
    );
    txt(
        g,
        b,
        text,
        r[0] + (r[2] - width(g, text)) / 2.0,
        r[1] + 6.0,
        if on { 0xFFFFFFFF } else { 0xFFAAAAAA },
    );
}
pub(super) fn draw(game: &Game<'_>, batch: &mut HudBatch) -> bool {
    if !backend::active(game) {
        return false;
    }
    let l = m::Layout::new(batch.width, batch.height);
    let mouse = game.native_cursor();
    state(game, |s| {
        s.scroll = s.scroll.clamp(0.0, limit(l, s));
        // The game renderer supplies the same panorama/world background and menu blur.
        fill(
            game,
            batch,
            [l.x, l.nav_top(), 125.0, l.y + l.height - l.nav_top()],
            0x90000000,
        );
        for (page, &name) in m::PAGES.iter().enumerate() {
            let r = [l.x, l.nav_top() + m::NAV_Y[page] as f32, 125.0, 18.0];
            if s.page == page && s.query.is_empty() {
                fill(game, batch, r, 0x90000000);
                fill(
                    game,
                    batch,
                    [r[0] + 122.0, r[1], 3.0, 18.0],
                    page_color(page, true),
                );
            } else if m::hit(mouse, r) {
                fill(game, batch, r, 0x60000000);
            }
            txt(
                game,
                batch,
                name,
                l.x + 8.0,
                r[1] + 5.0,
                if s.page == page {
                    page_color(page, true)
                } else if page < 3 {
                    0xFF7A9E9D
                } else {
                    0xFF9384AB
                },
            );
        }
        for (p, y, title, ver) in [
            (0, 2.0, "Sodium", "0.9.3-beta.1"),
            (3, 85.0, "Sodium Extra", "0.9.6-beta.1+mc26.3"),
        ] {
            let y = l.nav_top() + y;
            super::sodium_horizon_icons::draw(
                game,
                batch,
                p == 3,
                [l.x + 4.0, y + 3.0],
                20.0,
                Some(page_color(p,true)),
            );
            txt(game, batch, title, l.x + 27.0, y + 4.0, page_color(p, true));
            txt(
                game,
                batch,
                &fit(game, ver, 86.0),
                l.x + 27.0,
                y + 15.0,
                page_color(p, true),
            );
        }
        let search_w = l.width - if s.donate { 133.0 } else { 0.0 };
        fill(game, batch, [l.x, l.y, search_w, 20.0], 0x90000000);
        let text = if s.query.is_empty() {
            "Search...".into()
        } else {
            format!("{}{}", s.query, if s.search { "_" } else { "" })
        };
        txt(
            game,
            batch,
            &fit(game, &text, search_w - 35.0),
            l.x + 5.0,
            l.y + 6.0,
            if s.query.is_empty() {
                0xFFAAAAAA
            } else {
                0xFFE0E0E0
            },
        );
        if !s.query.is_empty() {
            button(
                game,
                batch,
                [l.x + search_w - 20.0, l.y, 20.0, 20.0],
                "×",
                true,
                mouse,
            );
        }
        if s.donate {
            button(
                game,
                batch,
                [l.x + l.width - 128.0, l.y, 100.0, 20.0],
                "Buy us a coffee!",
                true,
                mouse,
            );
            button(
                game,
                batch,
                [l.x + l.width - 25.0, l.y, 20.0, 20.0],
                "x",
                true,
                mouse,
            );
        }
        let start = batch.vertices.len();
        if s.query.is_empty() {
            for h in m::HEADERS {
                let y = l.top + h.y as f32 - s.scroll;
                if y + 18.0 < l.top || y > l.bottom {
                    continue;
                }
                let x = l.list_x() + h.inset as f32;
                fill(game, batch, [x, y, 207.0 - h.inset as f32, 18.0], h.bg);
                let mut tx = x + 6.0;
                if h.kind == 0 {
                    super::sodium_horizon_icons::draw(
                        game,
                        batch,
                        h.page >= 3,
                        [x + 5.0, y + 4.0],
                        10.0,
                        Some(h.color),
                    );
                    tx = x + 18.0;
                }
                txt(game, batch, h.title, tx, y + 5.0, h.color);
            }
        }
        let mut hovered = None;
        for i in rows(s) {
            let o = m::OPTIONS[i];
            let r = pos(l, s, i);
            if r[1] + 18.0 < l.top || r[1] > l.bottom {
                continue;
            }
            let on = supported(game, i) && o.enabled;
            let hover = m::hit(mouse, r) && mouse[1] >= l.top && mouse[1] < l.bottom;
            fill(game, batch, r, if hover { 0xE0000000 } else { 0x40000000 });
            if hover {
                hovered = Some(i)
            }
            let color = if on { 0xFFFFFFFF } else { 0xFFAAAAAA };
            let slider = on && o.kind == 1 && (hover || s.drag == Some(i));
            let value = label(i, s.values[i]);
            let name_width = if slider { r[2] - 114.0 - width(game, &value) } else { 184.0 };
            let mut name = o.name.to_owned();
            if s.values[i] != s.original[i] {
                name.push_str(" *");
            }
            txt(
                game,
                batch,
                &fit(game, &name, name_width.max(0.0)),
                r[0] + 6.0,
                r[1] + 5.0,
                color,
            );
            if !on {
                let w = width(game, &fit(game, &name, 184.0));
                fill(game, batch, [r[0] + 6.0, r[1] + 9.0, w, 1.0], 0xFFAAAAAA);
            }
            if o.enabled {
                if o.kind == 0 {
                    let x = r[0] + r[2] - 16.0;
                    let y = r[1] + 4.0;
                    let c = if s.values[i] != 0 {
                        page_color(o.page, false)
                    } else {
                        color
                    };
                    fill(game, batch, [x, y, 10.0, 1.0], c);
                    fill(game, batch, [x, y + 9.0, 10.0, 1.0], c);
                    fill(game, batch, [x, y, 1.0, 10.0], c);
                    fill(game, batch, [x + 9.0, y, 1.0, 10.0], c);
                    if s.values[i] != 0 {
                        fill(game, batch, [x + 2.0, y + 2.0, 6.0, 6.0], c);
                    }
                } else {
                    let sx = r[0] + r[2] - 96.0;
                    if slider {
                        let t = (s.values[i] - o.min) as f32 / (o.max - o.min).max(1) as f32;
                        fill(game, batch, [sx, r[1] + 8.0, 90.0, 1.0], page_color(o.page, true));
                        fill(game, batch, [(sx + t * 90.0 - 2.0).floor(), r[1] + 4.0, 4.0, 10.0], 0xFFFFFFFF);
                    }
                    txt(
                        game,
                        batch,
                        &value,
                        (if slider { sx - 6.0 } else { r[0] + r[2] - 6.0 }) - width(game, &value),
                        r[1] + 5.0,
                        color,
                    );
                }
            }
        }
        batch.clip_since(start, l.list_x(), l.top, l.list_x() + 219.0, l.bottom);
        if m::hit(mouse, [l.list_x(), l.top, 219.0, l.bottom - l.top]) && limit(l, s) > 0.0 {
            let h = l.bottom - l.top;
            let thumb = (h * h / (limit(l, s) + h)).max(10.0);
            fill(
                game,
                batch,
                [
                    l.list_x() + 212.0,
                    l.top + (h - thumb) * s.scroll / limit(l, s),
                    7.0,
                    thumb,
                ],
                0x90FFFFFF,
            );
        }
        button(game, batch, l.button(0), "Done", true, mouse);
        button(game, batch, l.button(1), "Apply", dirty(s), mouse);
        if dirty(s) {
            button(game, batch, l.button(2), "Undo", true, mouse);
        }
        if let Some(i) = hovered {
            let x = l.list_x() + 222.0;
            let w = (l.x + l.width - x).min(200.0);
            if w >= 100.0 {
                let y = l.top;
                let text = Text::legacy(m::OPTIONS[i].tooltip);
                let lines = game.gui.atlas.font.split(&text, w - 16.0);
                let height = (lines.len() as f32 * 11.0 + 16.0)
                    .min(l.button(2)[1] - y - 5.0)
                    .max(25.0);
                fill(game, batch, [x, y, w, height], 0xEA000000);
                for (n, line) in lines
                    .iter()
                    .take(((height - 16.0) / 11.0) as usize)
                    .enumerate()
                {
                    batch.text_styled(
                        &game.gui.atlas,
                        line,
                        x + 8.0,
                        y + 8.0 + n as f32 * 11.0,
                        rgba(255, 255, 255, 255),
                        true,
                    );
                }
            }
        }
    });
    true
}
fn apply(game: &mut Game<'_>, s: &mut State) {
    if !dirty(s) {
        return;
    }
    backend::EDITOR.with_borrow_mut(|slot| {
        let b = slot.get_or_insert_with(|| Editor::new(backend::read(game)));
        for (i, o) in m::OPTIONS.iter().enumerate() {
            if s.values[i] == s.original[i] || !supported(game, i) {
                continue;
            }
            let v = convert(s.values[i], o.transform, true);
            if (0..171).contains(&o.binding) {
                b.draft[o.binding as usize] = v
            } else {
                write_extra(&mut game.settings, o.binding, v)
            }
        }
        backend::apply(game, b);
    });
    game.settings.save(&Game::options_path());
    game.apply_settings();
    s.original.clone_from(&s.values);
}
fn change(game: &Game<'_>, s: &mut State, i: usize, reverse: bool, fraction: Option<f32>) {
    let o = m::OPTIONS[i];
    if !supported(game, i) || !o.enabled {
        return;
    }
    if o.binding == 23 {
        use horizon_client::graphics::GraphicsApi;
        let choices = GraphicsApi::choices();
        let current = GraphicsApi::ALL.get(s.values[i] as usize).copied().unwrap_or_default();
        let at = choices.iter().position(|&api| api == current).unwrap_or(0);
        let next = if reverse { (at + choices.len() - 1) % choices.len() } else { (at + 1) % choices.len() };
        s.values[i] = GraphicsApi::ALL.iter().position(|&api| api == choices[next]).unwrap_or(0) as i32;
        return;
    }
    let v = if let Some(t) = fraction {
        o.min
            + (t.clamp(0.0, 1.0) * ((o.max - o.min) / o.step.max(1)) as f32).round() as i32
                * o.step.max(1)
    } else if reverse {
        if s.values[i] <= o.min {
            o.max
        } else {
            (s.values[i] - o.step).max(o.min)
        }
    } else if s.values[i] >= o.max {
        o.min
    } else {
        (s.values[i] + o.step).min(o.max)
    };
    s.values[i] = v;
}
pub(super) fn mouse(game: &mut Game<'_>, pressed: bool, right: bool, drag: bool) -> bool {
    if !backend::active(game) {
        return false;
    }
    let (w, h) = game.gui_size();
    let l = m::Layout::new(w, h);
    let p = game.native_cursor();
    STATE.with_borrow_mut(|slot| {
        if slot.is_none() {
            let v = snapshot(game);
            *slot = Some(State {
                original: v.clone(),
                values: v,
                scroll: 0.0,
                page: 0,
                query: String::new(),
                search: false,
                drag: None,
                focus: 0,
                donate: true,
            });
        }
        let s = slot.as_mut().unwrap();
        if !pressed {
            if s.drag.is_some_and(|i| i != usize::MAX) {
                game.audio.play_button_click();
            }
            s.drag = None;
            return;
        }
        if drag {
            if let Some(i) = s.drag {
                if i == usize::MAX {
                    s.scroll = ((p[1] - l.top) / (l.bottom - l.top)).clamp(0.0, 1.0) * limit(l, s);
                } else {
                    let r = pos(l, s, i);
                    change(
                        game,
                        s,
                        i,
                        false,
                        Some((p[0] - (r[0] + r[2] - 96.0)) / 90.0),
                    );
                }
            }
            return;
        }
        for n in 0..3 {
            if m::hit(p, l.button(n)) {
                game.audio.play_button_click();
                match n {
                    0 => {
                        apply(game, s);
                        let _ = game.ui.options.back(&mut game.settings);
                    }
                    1 => apply(game, s),
                    _ => s.values.clone_from(&s.original),
                }
                return;
            }
        }
        if s.donate && m::hit(p, [l.x + l.width - 25.0, l.y, 20.0, 20.0]) {
            game.audio.play_button_click();
            s.donate = false;
            return;
        }
        if s.donate && m::hit(p, [l.x + l.width - 128.0, l.y, 100.0, 20.0]) {
            game.audio.play_button_click();
            game.confirm_trusted_link("https://caffeinemc.net/donate", Screen::Options);
            return;
        }
        let sw = l.width - if s.donate { 133.0 } else { 0.0 };
        if m::hit(p, [l.x, l.y, sw, 20.0]) {
            if !s.query.is_empty() && p[0] > l.x + sw - 20.0 {
                s.query.clear();
                s.scroll = 0.0;
            }
            s.search = true;
            return;
        }
        s.search = false;
        for page in 0..8 {
            if m::hit(p, [l.x, l.nav_top() + m::NAV_Y[page] as f32, 125.0, 18.0]) {
                game.audio.play_button_click();
                s.query.clear();
                s.page = page;
                s.scroll = (m::PAGE_SCROLL[page] as f32).min(l.max_scroll());
                return;
            }
        }
        if m::hit(p, [l.list_x() + 212.0, l.top, 7.0, l.bottom - l.top]) {
            s.drag = Some(usize::MAX);
            s.scroll = ((p[1] - l.top) / (l.bottom - l.top)).clamp(0.0, 1.0) * limit(l, s);
            return;
        }
        for i in rows(s) {
            let r = pos(l, s, i);
            if p[1] >= l.top && p[1] < l.bottom && m::hit(p, r) {
                if !supported(game, i) || !m::OPTIONS[i].enabled { return; }
                s.focus = i;
                if m::OPTIONS[i].kind == 1 {
                    if right || !m::hit(p, [r[0] + r[2] - 96.0, r[1] + 4.0, 90.0, 10.0]) { return; }
                    s.drag = Some(i);
                    change(
                        game,
                        s,
                        i,
                        false,
                        Some((p[0] - (r[0] + r[2] - 96.0)) / 90.0),
                    );
                } else {
                    game.audio.play_button_click();
                    change(game, s, i, right, None)
                }
                return;
            }
        }
    });
    true
}
pub(super) fn scroll(game: &Game<'_>, amount: f32) -> bool {
    if !backend::active(game) {
        return false;
    }
    let (w, h) = game.gui_size();
    let l = m::Layout::new(w, h);
    if !l.list_contains(game.native_cursor()) { return true; }
    state(game, |s| {
        s.scroll = (s.scroll - amount * 18.0).clamp(0.0, limit(l, s))
    });
    true
}
pub(super) fn key(game: &mut Game<'_>, key: KeyCode, typed: Option<&str>) -> bool {
    if !backend::active(game) {
        return false;
    }
    if let Some(t) = typed {
        if type_text(game, t) {
            return true;
        }
    }
    STATE.with_borrow_mut(|slot| {
        let Some(s) = slot.as_mut() else { return };
        if s.search {
            match key {
                KeyCode::Escape | KeyCode::Enter => s.search = false,
                KeyCode::Backspace => {
                    s.query.pop();
                    s.scroll = 0.0;
                }
                KeyCode::KeyA if game.has_control_down() => {
                    s.query.clear();
                    s.scroll = 0.0;
                }
                _ => {}
            }
            return;
        }
        match key {
            KeyCode::Escape => {
                s.values.clone_from(&s.original);
                let _ = game.ui.options.back(&mut game.settings);
            }
            KeyCode::KeyF if game.has_control_down() => s.search = true,
            KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                change(game, s, s.focus, key == KeyCode::ArrowLeft, None)
            }
            KeyCode::ArrowDown | KeyCode::Tab => {
                s.focus = (s.focus + 1) % m::OPTIONS.len();
                s.scroll = (m::OPTIONS[s.focus].y as f32 - 30.0).max(0.0);
            }
            KeyCode::Enter | KeyCode::Space => change(game, s, s.focus, false, None),
            KeyCode::PageDown => {
                s.page = (s.page + 1) % 8;
                s.scroll = m::PAGE_SCROLL[s.page] as f32
            }
            KeyCode::PageUp => {
                s.page = (s.page + 7) % 8;
                s.scroll = m::PAGE_SCROLL[s.page] as f32
            }
            _ => {}
        }
    });
    true
}
pub(super) fn type_text(game: &Game<'_>, t: &str) -> bool {
    if !backend::active(game) {
        return false;
    }
    let mut used = false;
    state(game, |s| {
        if !s.search {
            return;
        }
        if t == "\\b" {
            s.query.pop();
        } else {
            for c in t.chars().filter(|c| !c.is_control()) {
                if s.query.len() < 200 {
                    s.query.push(c)
                }
            }
        }
        s.scroll = 0.0;
        used = true
    });
    used
}
