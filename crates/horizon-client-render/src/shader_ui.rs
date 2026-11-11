//! Horizon's shader settings screen, on the settings shell (`settings_shell`): the shader packs in
//! `shaderpacks/` and the options of the picked one. Its pages are the pack list, then the pack's
//! own screens (a `shaders/` pack's `screen.NAME`s, a Horizon pack's `[[pages]]`), read from the
//! pack each time the rows are built; the shell draws and drives them, searches every screen,
//! collapses the screens into one row when the page list has no room for them, and has Undo. A
//! `shaders/` pack's options apply when Apply is pressed or the screen closes, as the pack loads
//! again with them; a Horizon pack's apply at once.
use crate::shader_pack_list::Kind;
use crate::{
    lang,
    options_ui::Event,
    settings::Settings,
    settings_shell::{self as shell, Group, Output, Row, Sheet, Value},
    text::Text,
};
use horizon_shader_api::{OptionKind, PageItem};
use std::collections::BTreeMap;

/// A page of the screen: the pack list, or screen `.0` of the picked pack (`screens`, 0 its
/// main one).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tab {
    Packs,
    Screen(u16),
}

/// A row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Item {
    /// `s.shader_packs[.0]`: picks it, or turns it off when it is the pick.
    Pack(u16),
    /// Opens `shaderpacks/`.
    Folder,
    /// Option `.0` of the picked pack: of a `shaders/` pack, the `.0`th of its options by name;
    /// of a Horizon pack, its `[[options]]` entry.
    Option(u16),
    /// Opens screen `.0`.
    Link(u16),
    /// The next of a `shaders/` pack's profiles.
    Profile,
    /// Every option of the pack back to its default.
    Reset,
    /// Horizon's own Shadow Cache (`Graphics::shadow_cache`), for every pack: not the pack's.
    ShadowCache,
    /// The next of the resolutions a pack can run at: packs cost by the pixel.
    Resolution,
    /// The next of the frame limits, so a pack can hold its speed on a phone that heats up.
    FpsLimit,
    /// Which profile of a `shaders/` pack runs: Auto, the pack's own defaults or one it lists.
    PackProfile,
}

const TITLE: &str = "Shaders";
const PACKS: &str = "Shader packs";
const GENERAL: &str = "General";
/// The groups: the packs, Horizon's own setting beside them, a pack's options.
const INSTALLED: &str = "Installed";
const BUILT_IN: &str = "Built-in settings";
const PACK_OPTIONS: &str = "Pack settings";
/// The line over the picked pack's screens in the page list.
const PACK_HEADING: &str = "%s settings";
/// What the strip says of a pack's own screens: they are the pack's, not Horizon's.
const PACK_SETTINGS: &str = "Settings made by the pack's author: they change only this pack.";
const NO_PACKS: &str = "No shader packs yet: open the pack folder and put some in it.";
const PICK_A_PACK: &str = "Pick a shader pack. Picking the one in use turns shaders off.";
const PENDING: &str = "Changed options apply with Apply, or when you leave.";
/// Pack descriptions run longer than Horizon's own: the strip has room for a line more.
const NOTE_LINES: usize = 3;
/// Screens this many links from the main one, or fewer, are in the page list.
const LISTED_DEPTH: u8 = 1;

/// One of the picked pack's screens.
struct PackScreen {
    title: String,
    /// What the strip says while nothing on it is pointed at.
    about: String,
    items: Vec<Item>,
    /// How many links away from the main screen: the page list holds the first two levels.
    depth: u8,
}

/// The picked pack: its index in `s.shader_packs` and kind, and for a `shaders/` pack whether its
/// options (`s.shader_options`) are read.
fn picked(s: &Settings) -> Option<(usize, Kind)> {
    let i = s
        .shader_packs
        .iter()
        .position(|p| !s.horizon.shader_pack.is_empty() && p.key == s.horizon.shader_pack)?;
    Some((i, s.shader_packs[i].kind))
}

/// The picked `shaders/` pack's options, when they are the picked pack's.
fn classic(s: &Settings) -> Option<&crate::shader_options::ShaderOptions> {
    s.shader_options
        .as_ref()
        .filter(|o| o.pack == s.horizon.shader_pack)
}

/// The picked Horizon pack, and its index in `s.shader_packs`.
fn native(s: &Settings) -> Option<(usize, &crate::shader_pack_list::Entry)> {
    picked(s)
        .filter(|p| p.1 == Kind::Horizon)
        .map(|(i, _)| (i, &s.shader_packs[i]))
}

/// The `.0`th option name of a `shaders/` pack.
fn option_name(o: &crate::shader_options::ShaderOptions, i: u16) -> Option<&str> {
    o.options.keys().nth(usize::from(i)).map(String::as_str)
}

fn plain(t: Text) -> String {
    t.strip_legacy().plain()
}

/// The picked pack's screens, its main one first, then in the order their links are met.
fn screens(s: &Settings) -> Vec<PackScreen> {
    match picked(s) {
        Some((_, Kind::Classic)) => classic(s).map_or_else(Vec::new, classic_screens),
        Some((i, Kind::Horizon)) => native_screens(&s.shader_packs[i]),
        None => Vec::new(),
    }
}

fn classic_screens(o: &crate::shader_options::ShaderOptions) -> Vec<PackScreen> {
    use horizon_shaders::ScreenItem;
    let keys: Vec<&String> = o.options.keys().collect();
    let mut names = vec![(String::new(), 0u8)];
    let mut out = Vec::new();
    let mut at = 0;
    while let Some((name, depth)) = names.get(at).cloned() {
        at += 1;
        let mut items = Vec::new();
        for item in o.screens.screen(&name).map_or(&[][..], |(items, _)| items) {
            match item {
                ScreenItem::Option(n) => {
                    if let Some(k) = keys.iter().position(|k| *k == n) {
                        items.push(Item::Option(k as u16));
                    }
                }
                ScreenItem::Screen(n) if o.screens.screen(n).is_some() => {
                    let k = match names.iter().position(|(m, _)| m == n) {
                        Some(k) => k,
                        None => {
                            names.push((n.clone(), depth.saturating_add(1)));
                            names.len() - 1
                        }
                    };
                    items.push(Item::Link(k as u16));
                }
                ScreenItem::Profile if !o.screens.profiles.is_empty() => items.push(Item::Profile),
                _ => {}
            }
        }
        let comment = |key: &str| o.comment(key).map(plain);
        let (title, about) = if name.is_empty() {
            items.push(Item::Reset);
            (lang::get(GENERAL), lang::get(PACK_SETTINGS))
        } else {
            let title = plain(o.screen_label(&name));
            let title = title.strip_suffix("...").unwrap_or(&title).to_owned();
            let about = comment(&format!("screen.{name}")).unwrap_or_default();
            (title, about)
        };
        out.push(PackScreen {
            title,
            about,
            items,
            depth,
        });
    }
    out
}

fn native_screens(pack: &crate::shader_pack_list::Entry) -> Vec<PackScreen> {
    let count = pack.options.len().min(pack.values.len());
    let reset = |mut items: Vec<Item>, first: bool| {
        if first {
            items.push(Item::Reset);
        }
        items
    };
    if pack.pages.is_empty() {
        if count == 0 {
            return Vec::new();
        }
        let items = (0..count).map(|o| Item::Option(o as u16)).collect();
        return vec![PackScreen {
            title: lang::get(GENERAL),
            about: lang::get(PACK_SETTINGS),
            items: reset(items, true),
            depth: 0,
        }];
    }
    // A page's depth: links from the first page are one away, the rest two.
    let linked: Vec<usize> = pack.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            PageItem::Page(p) => Some(*p),
            _ => None,
        })
        .collect();
    pack.pages
        .iter()
        .enumerate()
        .map(|(i, page)| {
            let items = page
                .items
                .iter()
                .filter_map(|item| match *item {
                    PageItem::Option(o) if o < count => Some(Item::Option(o as u16)),
                    PageItem::Page(p) if p < pack.pages.len() && p != i => {
                        Some(Item::Link(p as u16))
                    }
                    _ => None,
                })
                .collect();
            PackScreen {
                title: if i == 0 {
                    lang::get(GENERAL)
                } else {
                    page.name.clone()
                },
                about: page.description.clone(),
                items: reset(items, i == 0),
                depth: match i {
                    0 => 0,
                    _ if linked.contains(&i) => 1,
                    _ => 2,
                },
            }
        })
        .collect()
}

<<<<<<< HEAD
/// The resolution a pack runs at: Auto holds a heavy pack below what the player chose.
fn resolution_value(s: &Settings) -> String {
    let scale = s.horizon.resolution_scale;
    let limit = (crate::graphics::auto_shader_scale() * 100.0).round() as u32;
    let limited = s.horizon.shader_choice().is_some_and(|(pack, profile)| {
        profile == crate::graphics::AUTO_PROFILE && !crate::graphics::light_shader_pack(&pack)
    }) && scale > limit;
    if limited {
        format!("{limit}% (Auto)")
    } else {
        format!("{scale}%")
    }
}

/// The profile of the picked pack that runs, as the row shows it.
fn pack_profile_value(s: &Settings) -> String {
    let profile = s.horizon.shader_choice().map(|c| c.1).unwrap_or_default();
    match profile.as_str() {
        "" => lang::get("Pack Default"),
        crate::graphics::AUTO_PROFILE => lang::get("Auto"),
        profile => profile.to_owned(),
    }
}

/// What the options screen must do after an interaction here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    ButtonClick,
    Changed,
    SliderDrag,
    /// Another pack was picked, or the pick turned off (`s.horizon.shader_pack` holds it now).
    Picked,
    OpenFolder,
    /// Save the `shaders/` pack's changed options and load it again with them.
    Apply,
    /// The options of the Horizon pack `s.shader_packs[.0]` changed: save them.
    NativeChanged(usize),
    /// Load the picked pack again: what it runs with changed.
    Reload,
    /// The next of the picked pack's profiles, which the options screen keeps.
    NextPackProfile,
    Done,
||||||| c98a51a664
/// What the options screen must do after an interaction here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    ButtonClick,
    Changed,
    SliderDrag,
    /// Another pack was picked, or the pick turned off (`s.horizon.shader_pack` holds it now).
    Picked,
    OpenFolder,
    /// Save the `shaders/` pack's changed options and load it again with them.
    Apply,
    /// The options of the Horizon pack `s.shader_packs[.0]` changed: save them.
    NativeChanged(usize),
    Done,
=======
/// A screen's name in the page list and over its rows.
fn screen_name(i: u16, screens: &[PackScreen]) -> String {
    let screen = screens.get(usize::from(i));
    screen.map_or_else(String::new, |s| s.title.clone())
>>>>>>> arismoko/main
}

/// A row as its name and its value; `None` makes it a button across the row.
fn parts(item: Item, s: &Settings, screens: &[PackScreen]) -> (String, Option<String>) {
    match item {
        Item::Pack(i) => {
            let pack = s.shader_packs.get(usize::from(i));
            let name = pack.map_or_else(String::new, |p| p.title.clone());
            (name, Some(String::new()))
        }
        Item::Folder => (lang::get("pack.openFolder"), None),
        Item::Link(i) => (format!("{}...", screen_name(i, screens)), None),
        Item::Reset => (lang::get("Reset all options"), None),
        Item::ShadowCache => (lang::get("Shadow Cache"), Some(String::new())),
        Item::Profile => match classic(s) {
            Some(o) => shell::name_and_value(&plain(o.profile_label())),
            None => (String::new(), None),
        },
        Item::Option(i) => {
            if let Some((_, pack)) = native(s) {
                let i = usize::from(i);
                let (Some(o), Some(v)) = (pack.options.get(i), pack.values.get(i)) else {
                    return (String::new(), None);
                };
                let value = crate::shader_pack_list::show_value(&o.kind, *v);
                return (o.name.clone(), Some(value));
            }
            match classic(s).and_then(|o| Some((o, option_name(o, i)?))) {
                Some((o, name)) => (plain(o.option_name(name)), Some(plain(o.value_text(name)))),
                None => (String::new(), None),
            }
        }
    }
}

/// A tick box's state, for a pack (picked) or an on/off option.
fn switch(item: Item, s: &Settings) -> Option<bool> {
    match item {
        Item::ShadowCache => Some(s.horizon.shadow_cache),
        Item::Pack(i) => Some(
            s.shader_packs
                .get(usize::from(i))
                .is_some_and(|p| p.key == s.horizon.shader_pack),
        ),
        Item::Option(i) => match picked(s)? {
            (p, Kind::Horizon) => {
                let pack = &s.shader_packs[p];
                let i = usize::from(i);
                match pack.options.get(i)?.kind {
                    OptionKind::Bool { .. } => Some(*pack.values.get(i)? != 0.0),
                    _ => None,
                }
            }
            (_, Kind::Classic) => {
                let o = classic(s)?;
                let name = option_name(o, i)?;
                o.options[name]
                    .switch
                    .then(|| o.value(name) == Some("true"))
            }
        },
        _ => None,
    }
}

/// A slider's position, for an option the pack shows as one.
fn slider(item: Item, s: &Settings) -> Option<f32> {
    let Item::Option(i) = item else { return None };
    match picked(s)? {
        (p, Kind::Horizon) => s.shader_packs[p].slider_position(usize::from(i)),
        (_, Kind::Classic) => {
            let o = classic(s)?;
            o.slider_position(option_name(o, i)?)
        }
    }
}

/// A slider's stops, which Left and Right step between: a `shaders/` option's values, a Horizon
/// float's steps from its minimum (the last one its maximum) or an int's whole numbers.
fn slider_steps(item: Item, s: &Settings) -> Option<u32> {
    let Item::Option(i) = item else { return None };
    let stops = match picked(s)? {
        (p, Kind::Horizon) => match s.shader_packs[p].options.get(usize::from(i))?.kind {
            OptionKind::Float { min, max, step, .. } => {
                crate::shader_pack_list::float_steps(min, max, step) as f32
            }
            OptionKind::Int { min, max, .. } => (max - min) as f32,
            _ => return None,
        },
        (_, Kind::Classic) => {
            let o = classic(s)?;
            o.options[option_name(o, i)?].values.len().saturating_sub(1) as f32
        }
    };
    (stops >= 1.0).then_some(stops as u32)
}

/// What a row does, or says of itself, for the strip and the search.
fn description(item: Item, s: &Settings, screens: &[PackScreen]) -> String {
    match item {
        Item::Pack(i) => {
            let Some(pack) = s.shader_packs.get(usize::from(i)) else {
                return String::new();
            };
            let kind = lang::get(match pack.kind {
                Kind::Horizon => "A Horizon shader pack.",
                Kind::Classic => "An OptiFine or Iris shader pack.",
            });
            let what = lang::get(if pack.key == s.horizon.shader_pack {
                "In use: press to turn shaders off."
            } else {
                "Press to use it."
            });
            format!("{kind} {what} ({})", pack.file)
        }
        Item::Folder => lang::get("Put shader pack .zip files or folders in it."),
        Item::Reset => lang::get("Puts every option of this pack back to its default."),
        Item::ShadowCache => {
            lang::get("Reuses the pack shadow map when sun, view and world match. Off by default.")
        }
        Item::Profile => classic(s)
            .and_then(|o| o.comment("profile"))
            .map(plain)
            .unwrap_or_else(|| lang::get("Sets many options at once, as the pack groups them.")),
        Item::Link(i) => screens
            .get(usize::from(i))
            .map(|s| s.about.clone())
            .unwrap_or_default(),
        Item::Option(i) => match native(s) {
            Some((_, pack)) => pack
                .options
                .get(usize::from(i))
                .map(|o| o.description.clone())
                .unwrap_or_default(),
            None => classic(s)
                .and_then(|o| o.comment(&format!("option.{}", option_name(o, i)?)))
                .map(plain)
                .unwrap_or_default(),
        },
    }
}

/// A row as the shell draws it: a tick box for a pack and an on/off option, a slider for an
/// option the pack shows as one, a value between `<` `>` for the other options and the profile,
/// a button across the row for the folder, a link and Reset.
fn row(item: Item, s: &Settings, screens: &[PackScreen]) -> Row<Item> {
    let (name, text) = parts(item, s, screens);
    let value = match (item, switch(item, s), slider(item, s), text) {
        (_, Some(on), ..) => Value::Switch(on),
        (_, None, Some(position), Some(text)) => Value::Slider {
            position,
            steps: slider_steps(item, s),
            text,
        },
        (Item::Option(_) | Item::Profile, None, None, Some(text)) => Value::Choice(text),
        _ => Value::Press,
    };
    Row::new(item, name, value).about(description(item, s, screens))
}

/// The pack list, and Horizon's own setting beside it.
fn pack_sheet(s: &Settings) -> Sheet<Tab, Item> {
    let packs = (0..s.shader_packs.len()).map(|i| Item::Pack(i as u16));
    let installed = packs.chain([Item::Folder]);
    let about = lang::get(if s.shader_packs.is_empty() {
        NO_PACKS
    } else {
        PICK_A_PACK
    });
    let groups = vec![
        Group {
            name: INSTALLED,
            rows: installed.map(|item| row(item, s, &[])).collect(),
        },
        Group {
            name: BUILT_IN,
            rows: vec![row(Item::ShadowCache, s, &[])],
        },
    ];
    Sheet::new(Tab::Packs, lang::get(PACKS), about, groups)
}

/// The picked pack's name, over its screens in the page list.
fn pack_name(s: &Settings) -> &str {
    match native(s) {
        Some((_, pack)) => pack.title.as_str(),
        None => (s.horizon.shader_pack)
            .strip_suffix(".zip")
            .unwrap_or(&s.horizon.shader_pack),
    }
}

/// The shader screen. Its rows read the settings; what it keeps is for Undo and a drag.
#[derive(Default)]
pub(crate) struct ShaderScreen {
    /// The Horizon pack whose slider a drag moved: saved once, when it lets go.
    drag_changed: Option<usize>,
    /// The values this screen saved to a `shaders/` pack's file (picking another pack, Apply),
    /// by pack: Undo puts back the ones the screen opened with over them.
    written: BTreeMap<String, BTreeMap<String, String>>,
}

impl ShaderScreen {
    /// The picked `shaders/` pack's options are saved: what Undo has to put back over.
    fn saving(&mut self, s: &Settings) {
        if let Some(o) = s.shader_options.as_ref() {
            self.written.insert(o.pack.clone(), o.chosen.clone());
        }
    }
}

impl shell::Screen for ShaderScreen {
    type Page = Tab;
    type Row = Item;
    fn title(&self) -> &'static str {
        TITLE
    }
    fn note_lines(&self) -> usize {
        NOTE_LINES
    }
    /// The rows read the packs, the pick, its options and the Shadow Cache.
    fn stamp(&self, s: &Settings) -> u64 {
        let packs: Vec<_> = (s.shader_packs.iter())
            .map(|p| (&p.key, &p.title, &p.values, p.options.len(), p.pages.len()))
            .collect();
        let options = (s.shader_options.as_ref()).map(|o| (&o.pack, &o.chosen, o.changed));
        let h = &s.horizon;
        shell::stamp_of(&(&h.shader_pack, h.shadow_cache, packs, options))
    }
    /// The pack list, then the picked pack's screens under its name: those the main screen links
    /// to in the page list, the ones farther in opened by their links.
    fn sheets(&self, s: &Settings) -> Vec<Sheet<Tab, Item>> {
        let screens = screens(s);
        let pending = self.pending(s);
        let mut sheets = vec![pack_sheet(s)];
        for (i, screen) in screens.iter().enumerate() {
            let about = if pending {
                lang::get(PENDING)
            } else {
                screen.about.clone()
            };
            let rows = (screen.items.iter())
                .map(|&item| row(item, s, &screens))
                .collect();
            let groups = vec![Group {
                name: PACK_OPTIONS,
                rows,
            }];
            let mut sheet = Sheet::new(Tab::Screen(i as u16), screen.title.clone(), about, groups);
            sheet.listed = screen.depth <= LISTED_DEPTH;
            sheet.heading = (i == 0).then(|| lang::tr(PACK_HEADING, &[&pack_name(s)]));
            sheets.push(sheet);
        }
        sheets
    }
    fn link(&self, item: Item) -> Option<Tab> {
        match item {
<<<<<<< HEAD
            Item::Pack(i) => {
                let pack = s.shader_packs.get(usize::from(i));
                let name = pack.map_or_else(String::new, |p| p.title.clone());
                (name, Some(String::new()))
            }
            Item::Folder => (lang::get("pack.openFolder"), None),
            Item::Link(i) => (
                format!("{}...", Self::tab_name(Tab::Screen(i), screens)),
                None,
            ),
            Item::Reset => (lang::get("Reset all options"), None),
            Item::ShadowCache => (lang::get("Shadow Cache"), Some(String::new())),
            Item::Resolution => (lang::get("Resolution"), Some(resolution_value(s))),
            Item::FpsLimit => {
                let fps = match s.max_fps {
                    fps if fps >= 260 => lang::get("Off"),
                    fps => fps.to_string(),
                };
                (lang::get("FPS Limit"), Some(fps))
            }
            Item::PackProfile => (lang::get("Profile"), Some(pack_profile_value(s))),
            Item::Profile => match classic(s) {
                Some(o) => shell::name_and_value(&plain(o.profile_label())),
                None => (String::new(), None),
            },
            Item::Option(i) => {
                if let Some(pack) = native {
                    let i = usize::from(i);
                    let (Some(o), Some(v)) = (pack.options.get(i), pack.values.get(i)) else {
                        return (String::new(), None);
                    };
                    let value = crate::shader_pack_list::show_value(&o.kind, *v);
                    return (o.name.clone(), Some(value));
                }
                match classic(s).and_then(|o| Some((o, option_name(o, i)?))) {
                    Some((o, name)) => {
                        (plain(o.option_name(name)), Some(plain(o.value_text(name))))
                    }
                    None => (String::new(), None),
                }
            }
        }
    }
    /// A tick box's state, for a pack (picked) or an on/off option.
    fn switch(item: Item, s: &Settings) -> Option<bool> {
        match item {
            Item::ShadowCache => Some(s.horizon.shadow_cache),
            Item::Pack(i) => Some(
                s.shader_packs
                    .get(usize::from(i))
                    .is_some_and(|p| p.key == s.horizon.shader_pack),
            ),
            Item::Option(i) => match picked(s)? {
                (p, Kind::Horizon) => {
                    let pack = &s.shader_packs[p];
                    let i = usize::from(i);
                    match pack.options.get(i)?.kind {
                        OptionKind::Bool { .. } => Some(*pack.values.get(i)? != 0.0),
                        _ => None,
                    }
                }
                (_, Kind::Classic) => {
                    let o = classic(s)?;
                    let name = option_name(o, i)?;
                    o.options[name]
                        .switch
                        .then(|| o.value(name) == Some("true"))
                }
            },
||||||| c98a51a664
            Item::Pack(i) => {
                let pack = s.shader_packs.get(usize::from(i));
                let name = pack.map_or_else(String::new, |p| p.title.clone());
                (name, Some(String::new()))
            }
            Item::Folder => (lang::get("pack.openFolder"), None),
            Item::Link(i) => (
                format!("{}...", Self::tab_name(Tab::Screen(i), screens)),
                None,
            ),
            Item::Reset => (lang::get("Reset all options"), None),
            Item::ShadowCache => (lang::get("Shadow Cache"), Some(String::new())),
            Item::Profile => match classic(s) {
                Some(o) => shell::name_and_value(&plain(o.profile_label())),
                None => (String::new(), None),
            },
            Item::Option(i) => {
                if let Some(pack) = native {
                    let i = usize::from(i);
                    let (Some(o), Some(v)) = (pack.options.get(i), pack.values.get(i)) else {
                        return (String::new(), None);
                    };
                    let value = crate::shader_pack_list::show_value(&o.kind, *v);
                    return (o.name.clone(), Some(value));
                }
                match classic(s).and_then(|o| Some((o, option_name(o, i)?))) {
                    Some((o, name)) => {
                        (plain(o.option_name(name)), Some(plain(o.value_text(name))))
                    }
                    None => (String::new(), None),
                }
            }
        }
    }
    /// A tick box's state, for a pack (picked) or an on/off option.
    fn switch(item: Item, s: &Settings) -> Option<bool> {
        match item {
            Item::ShadowCache => Some(s.horizon.shadow_cache),
            Item::Pack(i) => Some(
                s.shader_packs
                    .get(usize::from(i))
                    .is_some_and(|p| p.key == s.horizon.shader_pack),
            ),
            Item::Option(i) => match picked(s)? {
                (p, Kind::Horizon) => {
                    let pack = &s.shader_packs[p];
                    let i = usize::from(i);
                    match pack.options.get(i)?.kind {
                        OptionKind::Bool { .. } => Some(*pack.values.get(i)? != 0.0),
                        _ => None,
                    }
                }
                (_, Kind::Classic) => {
                    let o = classic(s)?;
                    let name = option_name(o, i)?;
                    o.options[name]
                        .switch
                        .then(|| o.value(name) == Some("true"))
                }
            },
=======
            Item::Link(i) => Some(Tab::Screen(i)),
>>>>>>> arismoko/main
            _ => None,
        }
    }
<<<<<<< HEAD
    fn slider(item: Item, s: &Settings) -> Option<f32> {
        let Item::Option(i) = item else { return None };
        match picked(s)? {
            (p, Kind::Horizon) => s.shader_packs[p].slider_position(usize::from(i)),
            (_, Kind::Classic) => {
                let o = classic(s)?;
                o.slider_position(option_name(o, i)?)
            }
        }
    }
    /// What a row does, or says of itself, for the strip.
    fn description(item: Item, s: &Settings, screens: &[Screen]) -> String {
        let native = picked(s)
            .filter(|p| p.1 == Kind::Horizon)
            .map(|p| &s.shader_packs[p.0]);
        match item {
            Item::Pack(i) => {
                let Some(pack) = s.shader_packs.get(usize::from(i)) else {
                    return String::new();
                };
                let kind = lang::get(match pack.kind {
                    Kind::Horizon => "A Horizon shader pack.",
                    Kind::Classic => "An OptiFine or Iris shader pack.",
                });
                let what = lang::get(if pack.key == s.horizon.shader_pack {
                    "In use: press to turn shaders off."
                } else {
                    "Press to use it."
                });
                format!("{kind} {what} ({})", pack.file)
            }
            Item::Folder => format!(
                "{} {}",
                lang::get("Put shader pack .zip files or folders in it."),
                crate::options_ui::shader_folder_note()
            ),
            Item::Reset => lang::get("Puts every option of this pack back to its default."),
            Item::ShadowCache => lang::get(
                "Reuses the pack shadow map when sun, view and world match. Off by default.",
            ),
            Item::Resolution => {
                lang::get("Packs cost by the pixel: fewer run faster, and the world looks softer.")
            }
            Item::FpsLimit => lang::get(
                "A pack heats a phone up: a limit it can hold stays smoother than one it can't.",
            ),
            Item::PackProfile => lang::get(
                "Auto picks the pack's light settings for this device. Pack Default runs it as it comes.",
            ),
            Item::Profile => classic(s)
                .and_then(|o| o.comment("profile"))
                .map(plain)
                .unwrap_or_else(|| {
                    lang::get("Sets many options at once, as the pack groups them.")
                }),
            Item::Link(i) => screens
                .get(usize::from(i))
                .map(|s| s.about.clone())
                .unwrap_or_default(),
            Item::Option(i) => match native {
                Some(pack) => pack
                    .options
                    .get(usize::from(i))
                    .map(|o| o.description.clone())
                    .unwrap_or_default(),
                None => classic(s)
                    .and_then(|o| o.comment(&format!("option.{}", option_name(o, i)?)))
                    .map(plain)
                    .unwrap_or_default(),
            },
        }
    }
    /// The rows of a tab.
    fn items(tab: Tab, s: &Settings, screens: &[Screen]) -> Vec<Item> {
        match tab {
            Tab::Packs => {
                let mut rows: Vec<Item> = (0..s.shader_packs.len())
                    .map(|i| Item::Pack(i as u16))
                    .chain([Item::Folder, Item::ShadowCache])
                    .collect();
                if s.horizon.shader_choice().is_some() {
                    rows.extend([Item::Resolution, Item::FpsLimit]);
                    if picked(s).is_some_and(|p| p.1 == Kind::Classic) {
                        rows.push(Item::PackProfile);
                    }
                }
                rows
            }
            Tab::Screen(i) => screens
                .get(usize::from(i))
                .map(|s| s.items.clone())
                .unwrap_or_default(),
        }
    }
    /// The list's lines and its content height: the open page or, while searching, every page's
    /// rows whose name or description has the search in it.
    fn lines(&self, s: &Settings, screens: &[Screen]) -> (Vec<Line>, f32) {
        let query = self.query();
        let mut lines = shell::Lines::new();
        let tabs =
            std::iter::once(Tab::Packs).chain((0..screens.len()).map(|i| Tab::Screen(i as u16)));
        for tab in tabs {
            if query.is_empty() && tab != self.tab {
                continue;
            }
            let rows: Vec<Item> = Self::items(tab, s, screens)
                .into_iter()
                .filter(|&item| {
                    query.is_empty()
                        || self.focus == Some(Target::Item(item))
                        || self.dragging == Some(item)
                        || (!matches!(item, Item::Link(_) | Item::Reset)
                            && (Self::parts(item, s, screens)
                                .0
                                .to_lowercase()
                                .contains(&query)
                                || Self::description(item, s, screens)
                                    .to_lowercase()
                                    .contains(&query)))
                })
                .collect();
            if rows.is_empty() {
                continue;
            }
            // The plate's heading names the open page; results go under their page's name.
            // Horizon's own settings stand apart from the packs, and the packs' from Horizon's.
            let (built_in, rows): (Vec<Item>, Vec<Item>) = rows.into_iter().partition(|&i| {
                matches!(
                    i,
                    Item::ShadowCache | Item::Resolution | Item::FpsLimit | Item::PackProfile
                )
            });
            // On a phone what a pack costs goes above however many packs there are.
            let first = cfg!(target_os = "android");
            if first && !built_in.is_empty() {
                lines.group(tab, "Built-in settings", built_in.clone());
            }
            match (query.is_empty(), tab) {
                (true, Tab::Packs) => lines.group(tab, "Installed", rows),
                (true, Tab::Screen(_)) => lines.group(tab, "Pack settings", rows),
                (false, _) if !rows.is_empty() => lines.titled(tab, rows),
                (false, _) => {}
            }
            if !first && !built_in.is_empty() {
                lines.group(tab, "Built-in settings", built_in);
            }
        }
        lines.finish()
    }
    fn layout(&mut self, s: &Settings, w: f32, h: f32) -> (Columns, Vec<Screen>, Vec<Line>, f32) {
        let cols = Columns::of(w, h);
        let screens = screens(s);
        if let Tab::Screen(i) = self.tab
            && usize::from(i) >= screens.len()
        {
            self.tab = Tab::Packs;
            self.path.clear();
        }
        let (lines, content) = self.lines(s, &screens);
        self.scroll.clamp(cols.top, cols.bottom, content);
        (cols, screens, lines, content)
    }
    /// Open `tab` from the page list: the plate shows it or, while searching, scrolls to it.
    fn pick(&mut self, tab: Tab, lines: &[Line], cols: &Columns, content: f32) {
        self.tab = tab;
        self.path.clear();
        self.scroll.amount = 0.0;
        if self.query().is_empty() {
            return;
        }
        if let Some(line) = lines.iter().find(|l| l.page == tab) {
            self.scroll.amount = line.y;
            self.scroll.clamp(cols.top, cols.bottom, content);
            self.picked = Some(tab);
        }
    }
    fn has(&self, tab: Tab, lines: &[Line]) -> bool {
        self.query().is_empty() || lines.iter().any(|l| l.page == tab)
    }
    /// The page the plate shows: the open one or, while searching, the one picked, else the
    /// topmost line's.
    fn current(&self, lines: &[Line]) -> Option<Tab> {
        if self.query().is_empty() {
            return Some(self.tab);
        }
        self.picked.or_else(|| {
            lines
                .iter()
                .find(|l| l.y + l.h() > self.scroll.amount)
                .map(|l| l.page)
        })
    }
    /// The page list's tab that holds `tab`: a screen deeper than the list shows lights the one
    /// its links were followed from.
    fn lit(&self, tab: Option<Tab>, side: &[Tab]) -> Option<Tab> {
        let tab = tab?;
        if side.contains(&tab) {
            return Some(tab);
        }
        self.path
            .iter()
            .rev()
            .map(|&i| Tab::Screen(i))
            .find(|t| side.contains(t))
    }
    fn row_y(&self, cols: &Columns, line: &Line) -> f32 {
        cols.top + line.y - self.scroll.amount
    }

    pub fn draw(
        &mut self,
        s: &Settings,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        cursor: (f32, f32),
        in_world: bool,
    ) {
        let (w, h) = (batch.width, batch.height);
        let (cols, screens, lines, content) = self.layout(s, w, h);
        shell::backdrop(batch, atlas, in_world);
        let current = self.current(&lines);
        let side = Self::side_tabs(&screens, &cols);
        shell::mark(batch, atlas, &cols, &lang::get("Shaders"));
        let lit = self.lit(current, &side);
        for (i, &tab) in side.iter().enumerate() {
            let name = Self::tab_name(tab, &screens);
            let name = style::fit(atlas, &name, style::CONTROL_W - 16.0);
            let flags = (
                self.has(tab, &lines),
                lit == Some(tab),
                self.focus == Some(Target::Tab(tab)),
            );
            shell::page_tab(batch, atlas, Self::tab_rect(&cols, i), &name, flags, cursor);
        }
        // The picked pack's own screens go under its name, apart from Horizon's pack list.
        if side.len() > 1 {
            let [x, y, w, _] = cols.page(1);
            let pack = match picked(s) {
                Some((i, Kind::Horizon)) => s.shader_packs[i].title.as_str(),
                _ => s
                    .horizon
                    .shader_pack
                    .strip_suffix(".zip")
                    .unwrap_or(&s.horizon.shader_pack),
            };
            let caption = lang::tr("%s settings", &[&pack]);
            let rect = [x + 2.0, y + 6.0, w - 4.0, 8.0];
            crate::widgets::record(&caption, rect, false);
            let caption = Text::literal(style::fit(atlas, &caption, w - 4.0));
            batch.text_styled(atlas, &caption, rect[0], rect[1], style::MUTED, true);
        }
        let focused = self.focus == Some(Target::Search);
        shell::search_field(&mut self.search, batch, atlas, &cols, cursor, focused);

        let query = self.search.value.trim();
        let heading = if query.is_empty() {
            Self::tab_name(current.unwrap_or(self.tab), &screens)
        } else {
            lang::get("Results for \"%s\"").replace("%s", query)
        };
        shell::plate(batch, atlas, &cols, &heading);

        let over_list = cursor.1 >= cols.top && cursor.1 < cols.bottom;
        let mut described = None;
        shell::list(
            batch,
            atlas,
            &cols,
            &lines,
            self.scroll.amount,
            |tab| Self::tab_name(tab, &screens),
            |batch, item: Item, y| {
                let [rx, ry, rw, rh] = cols.row(y);
                let hover = over_list && ui::hovered(cursor, rx, ry, rw, rh);
                let focused = self.focus == Some(Target::Item(item));
                if hover || self.dragging == Some(item) || (focused && described.is_none()) {
                    described = Some(item);
                }
                let (name, value) = Self::parts(item, s, &screens);
                let label = match &value {
                    Some(v) if !v.is_empty() => format!("{name}: {v}"),
                    _ => name.clone(),
                };
                let view = shell::RowView {
                    label: &label,
                    name: &name,
                    value: value.as_deref(),
                    switch: Self::switch(item, s),
                    slider: Self::slider(item, s),
                    live: true,
                    hover,
                    focused,
                    held: self.dragging == Some(item),
                    key: None,
                };
                shell::row(batch, atlas, &cols, y, &view);
            },
        );
        shell::scroller(batch, atlas, &cols, content, self.scroll.amount);

        // The strip: the pointed-at row, else what the page holds.
        let [x, y, ..] = cols.dock;
        style::panel(batch, atlas, cols.dock);
        let (name, text) = match described {
            Some(item) => (
                Self::parts(item, s, &screens).0,
                Self::description(item, s, &screens),
            ),
            None => match current {
                Some(Tab::Packs) | None => (
                    lang::get("Shader packs"),
                    lang::get(if s.shader_packs.is_empty() {
                        "No shader packs yet: open the pack folder and put some in it."
                    } else {
                        "Pick a shader pack. Picking the one in use turns shaders off."
                    }),
                ),
                Some(tab) => {
                    let about = match tab {
                        Tab::Screen(i) => screens[usize::from(i)].about.clone(),
                        Tab::Packs => String::new(),
                    };
                    let about = if Self::pending(s) {
                        lang::get("Changed options apply with Apply, or when you leave.")
                    } else {
                        about
                    };
                    (Self::tab_name(tab, &screens), about)
                }
            },
        };
        batch.text_styled(
            atlas,
            &Text::literal(name),
            x + 12.0,
            y + 8.0,
            style::WHITE,
            true,
        );
        shell::dock_note(batch, atlas, &cols, text, style::GREY);

        let mut foot = vec![(cols.button(0), "gui.done", Target::Done, Declared::Done)];
        if Self::pending(s) {
            foot.push((cols.button(1), "Apply", Target::Apply, Declared::Apply));
        }
        for (rect, label, target, declared) in foot {
            let label = Text::literal(lang::get(label));
            let focused = self.focus == Some(target);
            crate::widgets::declare_action(declared, rect, true);
            shell::foot_button(batch, atlas, rect, &label, cursor, (focused, true));
        }
    }

    /// A press (or a drag with `drag`) at `(x, y)`.
    pub fn click(
        &mut self,
        s: &mut Settings,
        (x, y): (f32, f32),
        (w, h): (f32, f32),
        drag: bool,
        right: bool,
        shift: bool,
    ) -> Vec<Action> {
        let (cols, screens, lines, content) = self.layout(s, w, h);
        let at = |[rx, ry, rw, rh]: [f32; 4]| ui::hovered((x, y), rx, ry, rw, rh);
        if drag {
            if self.scroll.held() {
                self.picked = None;
                self.scroll.drag(y, cols.top, cols.bottom, content);
                return Vec::new();
            }
            if let Some(item) = self.dragging {
                let actions = Self::set_slider(s, item, cols.slider_at(x));
                return self.hold(actions);
            }
            return Vec::new();
        }
        if cols.scrolls()
            && self
                .scroll
                .press((x, y), cols.bar_x(), cols.top, cols.bottom, content)
        {
            return Vec::new();
        }
        if at(cols.search()) {
            if !self.search.value.is_empty() && at(cols.clear()) {
                self.search.set_value("");
                self.searched();
                return vec![Action::ButtonClick];
            }
            self.focus = Some(Target::Search);
            self.search.reset_blink();
            self.search.move_cursor_to_end(false);
            return Vec::new();
        }
        if self.focus == Some(Target::Search) {
            self.focus = None;
        }
        if at(cols.button(0)) {
            self.focus = Some(Target::Done);
            return vec![Action::ButtonClick, Action::Done];
        }
        if Self::pending(s) && at(cols.button(1)) {
            self.focus = None;
            return vec![Action::ButtonClick, Action::Apply];
        }
        for (i, tab) in Self::side_tabs(&screens, &cols).into_iter().enumerate() {
            if at(Self::tab_rect(&cols, i)) && self.has(tab, &lines) {
                self.focus = Some(Target::Tab(tab));
                self.pick(tab, &lines, &cols, content);
                return vec![Action::ButtonClick];
            }
        }
        if y < cols.top || y >= cols.bottom {
            return Vec::new();
        }
        let Some(item) = lines.iter().find_map(|line| {
            let item = line.setting?;
            at(cols.row(self.row_y(&cols, line))).then_some(item)
        }) else {
            return Vec::new();
        };
        self.focus = Some(Target::Item(item));
        self.activate(s, item, right || shift, Some((x, &cols)))
    }
    /// Option `item`'s slider at `at` (0 to 1).
    fn set_slider(s: &mut Settings, item: Item, at: f32) -> Vec<Action> {
        let Item::Option(i) = item else {
            return Vec::new();
        };
        match picked(s) {
            Some((p, Kind::Horizon)) => {
                if s.shader_packs[p].set_slider_position(usize::from(i), at) {
                    return vec![Action::SliderDrag, Action::NativeChanged(p)];
                }
            }
            Some((_, Kind::Classic)) => {
                let pack = s.horizon.shader_pack.clone();
                if let Some(o) = s.shader_options.as_mut().filter(|o| o.pack == pack)
                    && let Some(name) = option_name(o, i).map(str::to_owned)
                {
                    o.set_position(&name, at);
                    return vec![Action::SliderDrag];
                }
            }
            None => {}
        }
        Vec::new()
    }
    /// Option `item`'s slider one of its own steps on (back with `!forward`), stopping at its ends.
    fn step_slider(s: &mut Settings, item: Item, forward: bool) -> Vec<Action> {
        let Item::Option(i) = item else {
            return Vec::new();
        };
        match picked(s) {
            Some((p, Kind::Horizon)) => {
                if s.shader_packs[p].nudge_option(usize::from(i), !forward) {
                    return vec![Action::SliderDrag, Action::NativeChanged(p)];
                }
                Vec::new()
            }
            Some((_, Kind::Classic)) => {
                let Some(o) = classic(s) else {
                    return Vec::new();
                };
                let Some(name) = option_name(o, i) else {
                    return Vec::new();
                };
                let n = o.options[name].values.len().saturating_sub(1).max(1) as f32;
                let Some(pos) = o.slider_position(name) else {
                    return Vec::new();
                };
                let next = ((pos * n).round() + if forward { 1.0 } else { -1.0 }) / n;
                if !(0.0..=1.0).contains(&next) {
                    return Vec::new();
                }
                Self::set_slider(s, item, next)
            }
            None => Vec::new(),
        }
    }
    /// Press `item`: `at` is where on its row and its slider's track (`None` from the keyboard).
    fn activate(
        &mut self,
        s: &mut Settings,
        item: Item,
        back: bool,
        at: Option<(f32, &Columns)>,
    ) -> Vec<Action> {
        if Self::slider(item, s).is_some() {
            self.can_change_value = true;
            return match at {
                Some((x, cols)) if x >= cols.track().0 => {
                    self.dragging = Some(item);
                    let actions = Self::set_slider(s, item, cols.slider_at(x));
                    self.hold(actions)
                }
                _ => Vec::new(),
            };
        }
        self.can_change_value = false;
||||||| c98a51a664
    fn slider(item: Item, s: &Settings) -> Option<f32> {
        let Item::Option(i) = item else { return None };
        match picked(s)? {
            (p, Kind::Horizon) => s.shader_packs[p].slider_position(usize::from(i)),
            (_, Kind::Classic) => {
                let o = classic(s)?;
                o.slider_position(option_name(o, i)?)
            }
        }
    }
    /// What a row does, or says of itself, for the strip.
    fn description(item: Item, s: &Settings, screens: &[Screen]) -> String {
        let native = picked(s)
            .filter(|p| p.1 == Kind::Horizon)
            .map(|p| &s.shader_packs[p.0]);
        match item {
            Item::Pack(i) => {
                let Some(pack) = s.shader_packs.get(usize::from(i)) else {
                    return String::new();
                };
                let kind = lang::get(match pack.kind {
                    Kind::Horizon => "A Horizon shader pack.",
                    Kind::Classic => "An OptiFine or Iris shader pack.",
                });
                let what = lang::get(if pack.key == s.horizon.shader_pack {
                    "In use: press to turn shaders off."
                } else {
                    "Press to use it."
                });
                format!("{kind} {what} ({})", pack.file)
            }
            Item::Folder => lang::get("Put shader pack .zip files or folders in it."),
            Item::Reset => lang::get("Puts every option of this pack back to its default."),
            Item::ShadowCache => lang::get(
                "Reuses the pack shadow map when sun, view and world match. Off by default.",
            ),
            Item::Profile => classic(s)
                .and_then(|o| o.comment("profile"))
                .map(plain)
                .unwrap_or_else(|| {
                    lang::get("Sets many options at once, as the pack groups them.")
                }),
            Item::Link(i) => screens
                .get(usize::from(i))
                .map(|s| s.about.clone())
                .unwrap_or_default(),
            Item::Option(i) => match native {
                Some(pack) => pack
                    .options
                    .get(usize::from(i))
                    .map(|o| o.description.clone())
                    .unwrap_or_default(),
                None => classic(s)
                    .and_then(|o| o.comment(&format!("option.{}", option_name(o, i)?)))
                    .map(plain)
                    .unwrap_or_default(),
            },
        }
    }
    /// The rows of a tab.
    fn items(tab: Tab, s: &Settings, screens: &[Screen]) -> Vec<Item> {
        match tab {
            Tab::Packs => (0..s.shader_packs.len())
                .map(|i| Item::Pack(i as u16))
                .chain([Item::Folder, Item::ShadowCache])
                .collect(),
            Tab::Screen(i) => screens
                .get(usize::from(i))
                .map(|s| s.items.clone())
                .unwrap_or_default(),
        }
    }
    /// The list's lines and its content height: the open page or, while searching, every page's
    /// rows whose name or description has the search in it.
    fn lines(&self, s: &Settings, screens: &[Screen]) -> (Vec<Line>, f32) {
        let query = self.query();
        let mut lines = shell::Lines::new();
        let tabs =
            std::iter::once(Tab::Packs).chain((0..screens.len()).map(|i| Tab::Screen(i as u16)));
        for tab in tabs {
            if query.is_empty() && tab != self.tab {
                continue;
            }
            let rows: Vec<Item> = Self::items(tab, s, screens)
                .into_iter()
                .filter(|&item| {
                    query.is_empty()
                        || self.focus == Some(Target::Item(item))
                        || self.dragging == Some(item)
                        || (!matches!(item, Item::Link(_) | Item::Reset)
                            && (Self::parts(item, s, screens)
                                .0
                                .to_lowercase()
                                .contains(&query)
                                || Self::description(item, s, screens)
                                    .to_lowercase()
                                    .contains(&query)))
                })
                .collect();
            if rows.is_empty() {
                continue;
            }
            // The plate's heading names the open page; results go under their page's name.
            // Horizon's own settings stand apart from the packs, and the packs' from Horizon's.
            let (built_in, rows): (Vec<Item>, Vec<Item>) =
                rows.into_iter().partition(|&i| i == Item::ShadowCache);
            match (query.is_empty(), tab) {
                (true, Tab::Packs) => lines.group(tab, "Installed", rows),
                (true, Tab::Screen(_)) => lines.group(tab, "Pack settings", rows),
                (false, _) if !rows.is_empty() => lines.titled(tab, rows),
                (false, _) => {}
            }
            if !built_in.is_empty() {
                lines.group(tab, "Built-in settings", built_in);
            }
        }
        lines.finish()
    }
    fn layout(&mut self, s: &Settings, w: f32, h: f32) -> (Columns, Vec<Screen>, Vec<Line>, f32) {
        let cols = Columns::of(w, h);
        let screens = screens(s);
        if let Tab::Screen(i) = self.tab
            && usize::from(i) >= screens.len()
        {
            self.tab = Tab::Packs;
            self.path.clear();
        }
        let (lines, content) = self.lines(s, &screens);
        self.scroll.clamp(cols.top, cols.bottom, content);
        (cols, screens, lines, content)
    }
    /// Open `tab` from the page list: the plate shows it or, while searching, scrolls to it.
    fn pick(&mut self, tab: Tab, lines: &[Line], cols: &Columns, content: f32) {
        self.tab = tab;
        self.path.clear();
        self.scroll.amount = 0.0;
        if self.query().is_empty() {
            return;
        }
        if let Some(line) = lines.iter().find(|l| l.page == tab) {
            self.scroll.amount = line.y;
            self.scroll.clamp(cols.top, cols.bottom, content);
            self.picked = Some(tab);
        }
    }
    fn has(&self, tab: Tab, lines: &[Line]) -> bool {
        self.query().is_empty() || lines.iter().any(|l| l.page == tab)
    }
    /// The page the plate shows: the open one or, while searching, the one picked, else the
    /// topmost line's.
    fn current(&self, lines: &[Line]) -> Option<Tab> {
        if self.query().is_empty() {
            return Some(self.tab);
        }
        self.picked.or_else(|| {
            lines
                .iter()
                .find(|l| l.y + l.h() > self.scroll.amount)
                .map(|l| l.page)
        })
    }
    /// The page list's tab that holds `tab`: a screen deeper than the list shows lights the one
    /// its links were followed from.
    fn lit(&self, tab: Option<Tab>, side: &[Tab]) -> Option<Tab> {
        let tab = tab?;
        if side.contains(&tab) {
            return Some(tab);
        }
        self.path
            .iter()
            .rev()
            .map(|&i| Tab::Screen(i))
            .find(|t| side.contains(t))
    }
    fn row_y(&self, cols: &Columns, line: &Line) -> f32 {
        cols.top + line.y - self.scroll.amount
    }

    pub fn draw(
        &mut self,
        s: &Settings,
        batch: &mut HudBatch,
        atlas: &HudAtlas,
        cursor: (f32, f32),
        in_world: bool,
    ) {
        let (w, h) = (batch.width, batch.height);
        let (cols, screens, lines, content) = self.layout(s, w, h);
        shell::backdrop(batch, atlas, in_world);
        let current = self.current(&lines);
        let side = Self::side_tabs(&screens, &cols);
        shell::mark(batch, atlas, &cols, &lang::get("Shaders"));
        let lit = self.lit(current, &side);
        for (i, &tab) in side.iter().enumerate() {
            let name = Self::tab_name(tab, &screens);
            let name = style::fit(atlas, &name, style::CONTROL_W - 16.0);
            let flags = (
                self.has(tab, &lines),
                lit == Some(tab),
                self.focus == Some(Target::Tab(tab)),
            );
            shell::page_tab(batch, atlas, Self::tab_rect(&cols, i), &name, flags, cursor);
        }
        // The picked pack's own screens go under its name, apart from Horizon's pack list.
        if side.len() > 1 {
            let [x, y, w, _] = cols.page(1);
            let pack = match picked(s) {
                Some((i, Kind::Horizon)) => s.shader_packs[i].title.as_str(),
                _ => s
                    .horizon
                    .shader_pack
                    .strip_suffix(".zip")
                    .unwrap_or(&s.horizon.shader_pack),
            };
            let caption = lang::tr("%s settings", &[&pack]);
            let rect = [x + 2.0, y + 6.0, w - 4.0, 8.0];
            crate::widgets::record(&caption, rect, false);
            let caption = Text::literal(style::fit(atlas, &caption, w - 4.0));
            batch.text_styled(atlas, &caption, rect[0], rect[1], style::MUTED, true);
        }
        let focused = self.focus == Some(Target::Search);
        shell::search_field(&mut self.search, batch, atlas, &cols, cursor, focused);

        let query = self.search.value.trim();
        let heading = if query.is_empty() {
            Self::tab_name(current.unwrap_or(self.tab), &screens)
        } else {
            lang::get("Results for \"%s\"").replace("%s", query)
        };
        shell::plate(batch, atlas, &cols, &heading);

        let over_list = cursor.1 >= cols.top && cursor.1 < cols.bottom;
        let mut described = None;
        shell::list(
            batch,
            atlas,
            &cols,
            &lines,
            self.scroll.amount,
            |tab| Self::tab_name(tab, &screens),
            |batch, item: Item, y| {
                let [rx, ry, rw, rh] = cols.row(y);
                let hover = over_list && ui::hovered(cursor, rx, ry, rw, rh);
                let focused = self.focus == Some(Target::Item(item));
                if hover || self.dragging == Some(item) || (focused && described.is_none()) {
                    described = Some(item);
                }
                let (name, value) = Self::parts(item, s, &screens);
                let label = match &value {
                    Some(v) if !v.is_empty() => format!("{name}: {v}"),
                    _ => name.clone(),
                };
                let view = shell::RowView {
                    label: &label,
                    name: &name,
                    value: value.as_deref(),
                    switch: Self::switch(item, s),
                    slider: Self::slider(item, s),
                    live: true,
                    hover,
                    focused,
                    held: self.dragging == Some(item),
                    key: None,
                };
                shell::row(batch, atlas, &cols, y, &view);
            },
        );
        shell::scroller(batch, atlas, &cols, content, self.scroll.amount);

        // The strip: the pointed-at row, else what the page holds.
        let [x, y, ..] = cols.dock;
        style::panel(batch, atlas, cols.dock);
        let (name, text) = match described {
            Some(item) => (
                Self::parts(item, s, &screens).0,
                Self::description(item, s, &screens),
            ),
            None => match current {
                Some(Tab::Packs) | None => (
                    lang::get("Shader packs"),
                    lang::get(if s.shader_packs.is_empty() {
                        "No shader packs yet: open the pack folder and put some in it."
                    } else {
                        "Pick a shader pack. Picking the one in use turns shaders off."
                    }),
                ),
                Some(tab) => {
                    let about = match tab {
                        Tab::Screen(i) => screens[usize::from(i)].about.clone(),
                        Tab::Packs => String::new(),
                    };
                    let about = if Self::pending(s) {
                        lang::get("Changed options apply with Apply, or when you leave.")
                    } else {
                        about
                    };
                    (Self::tab_name(tab, &screens), about)
                }
            },
        };
        batch.text_styled(
            atlas,
            &Text::literal(name),
            x + 12.0,
            y + 8.0,
            style::WHITE,
            true,
        );
        shell::dock_note(batch, atlas, &cols, text, style::GREY);

        let mut foot = vec![(cols.button(0), "gui.done", Target::Done, Declared::Done)];
        if Self::pending(s) {
            foot.push((cols.button(1), "Apply", Target::Apply, Declared::Apply));
        }
        for (rect, label, target, declared) in foot {
            let label = Text::literal(lang::get(label));
            let focused = self.focus == Some(target);
            crate::widgets::declare_action(declared, rect, true);
            shell::foot_button(batch, atlas, rect, &label, cursor, (focused, true));
        }
    }

    /// A press (or a drag with `drag`) at `(x, y)`.
    pub fn click(
        &mut self,
        s: &mut Settings,
        (x, y): (f32, f32),
        (w, h): (f32, f32),
        drag: bool,
        right: bool,
        shift: bool,
    ) -> Vec<Action> {
        let (cols, screens, lines, content) = self.layout(s, w, h);
        let at = |[rx, ry, rw, rh]: [f32; 4]| ui::hovered((x, y), rx, ry, rw, rh);
        if drag {
            if self.scroll.held() {
                self.picked = None;
                self.scroll.drag(y, cols.top, cols.bottom, content);
                return Vec::new();
            }
            if let Some(item) = self.dragging {
                let actions = Self::set_slider(s, item, cols.slider_at(x));
                return self.hold(actions);
            }
            return Vec::new();
        }
        if cols.scrolls()
            && self
                .scroll
                .press((x, y), cols.bar_x(), cols.top, cols.bottom, content)
        {
            return Vec::new();
        }
        if at(cols.search()) {
            if !self.search.value.is_empty() && at(cols.clear()) {
                self.search.set_value("");
                self.searched();
                return vec![Action::ButtonClick];
            }
            self.focus = Some(Target::Search);
            self.search.reset_blink();
            self.search.move_cursor_to_end(false);
            return Vec::new();
        }
        if self.focus == Some(Target::Search) {
            self.focus = None;
        }
        if at(cols.button(0)) {
            self.focus = Some(Target::Done);
            return vec![Action::ButtonClick, Action::Done];
        }
        if Self::pending(s) && at(cols.button(1)) {
            self.focus = None;
            return vec![Action::ButtonClick, Action::Apply];
        }
        for (i, tab) in Self::side_tabs(&screens, &cols).into_iter().enumerate() {
            if at(Self::tab_rect(&cols, i)) && self.has(tab, &lines) {
                self.focus = Some(Target::Tab(tab));
                self.pick(tab, &lines, &cols, content);
                return vec![Action::ButtonClick];
            }
        }
        if y < cols.top || y >= cols.bottom {
            return Vec::new();
        }
        let Some(item) = lines.iter().find_map(|line| {
            let item = line.setting?;
            at(cols.row(self.row_y(&cols, line))).then_some(item)
        }) else {
            return Vec::new();
        };
        self.focus = Some(Target::Item(item));
        self.activate(s, item, right || shift, Some((x, &cols)))
    }
    /// Option `item`'s slider at `at` (0 to 1).
    fn set_slider(s: &mut Settings, item: Item, at: f32) -> Vec<Action> {
        let Item::Option(i) = item else {
            return Vec::new();
        };
        match picked(s) {
            Some((p, Kind::Horizon)) => {
                if s.shader_packs[p].set_slider_position(usize::from(i), at) {
                    return vec![Action::SliderDrag, Action::NativeChanged(p)];
                }
            }
            Some((_, Kind::Classic)) => {
                let pack = s.horizon.shader_pack.clone();
                if let Some(o) = s.shader_options.as_mut().filter(|o| o.pack == pack)
                    && let Some(name) = option_name(o, i).map(str::to_owned)
                {
                    o.set_position(&name, at);
                    return vec![Action::SliderDrag];
                }
            }
            None => {}
        }
        Vec::new()
    }
    /// Option `item`'s slider one of its own steps on (back with `!forward`), stopping at its ends.
    fn step_slider(s: &mut Settings, item: Item, forward: bool) -> Vec<Action> {
        let Item::Option(i) = item else {
            return Vec::new();
        };
        match picked(s) {
            Some((p, Kind::Horizon)) => {
                if s.shader_packs[p].nudge_option(usize::from(i), !forward) {
                    return vec![Action::SliderDrag, Action::NativeChanged(p)];
                }
                Vec::new()
            }
            Some((_, Kind::Classic)) => {
                let Some(o) = classic(s) else {
                    return Vec::new();
                };
                let Some(name) = option_name(o, i) else {
                    return Vec::new();
                };
                let n = o.options[name].values.len().saturating_sub(1).max(1) as f32;
                let Some(pos) = o.slider_position(name) else {
                    return Vec::new();
                };
                let next = ((pos * n).round() + if forward { 1.0 } else { -1.0 }) / n;
                if !(0.0..=1.0).contains(&next) {
                    return Vec::new();
                }
                Self::set_slider(s, item, next)
            }
            None => Vec::new(),
        }
    }
    /// Press `item`: `at` is where on its row and its slider's track (`None` from the keyboard).
    fn activate(
        &mut self,
        s: &mut Settings,
        item: Item,
        back: bool,
        at: Option<(f32, &Columns)>,
    ) -> Vec<Action> {
        if Self::slider(item, s).is_some() {
            self.can_change_value = true;
            return match at {
                Some((x, cols)) if x >= cols.track().0 => {
                    self.dragging = Some(item);
                    let actions = Self::set_slider(s, item, cols.slider_at(x));
                    self.hold(actions)
                }
                _ => Vec::new(),
            };
        }
        self.can_change_value = false;
=======
    /// A pack is picked (or turned off) and saved, the last pack's changed options first; an
    /// option steps (back with Shift); the folder opens.
    fn press(&mut self, s: &mut Settings, item: Item, back: bool) -> Vec<Output> {
        let changed = vec![Output::Event(Event::Changed)];
>>>>>>> arismoko/main
        let pack = s.horizon.shader_pack.clone();
        let native = native(s).map(|p| p.0);
        match item {
            Item::Pack(i) => {
                let Some(p) = s.shader_packs.get(usize::from(i)) else {
                    return Vec::new();
                };
                s.horizon.shader_pack = if s.horizon.shader_pack == p.key {
                    String::new()
                } else {
                    p.key.clone()
                };
                let mut outputs = Vec::new();
                // The last pack's changed options are kept before the next one's are read.
                if s.shader_options.as_ref().is_some_and(|o| o.changed) {
                    self.saving(s);
                    outputs.push(Output::Event(Event::ShaderOptions));
                }
                outputs.push(Output::Event(Event::ShaderPack));
                outputs.push(Output::Event(Event::OpenShaderOptions));
                outputs
            }
            Item::Folder => vec![Output::Event(Event::OpenShaderPackFolder)],
            Item::ShadowCache => {
                s.horizon.shadow_cache = !s.horizon.shadow_cache;
                changed
            }
<<<<<<< HEAD
            Item::Resolution => {
                const STEPS: [u32; 6] = [100, 75, 50, 40, 30, 25];
                let now = s.horizon.resolution_scale;
                let next = STEPS.iter().position(|&v| v == now).map_or(2, |i| i + 1);
                s.horizon.resolution_scale = STEPS[next % STEPS.len()];
                vec![Action::ButtonClick, Action::Reload, Action::Changed]
            }
            Item::FpsLimit => {
                const STEPS: [u32; 4] = [30, 40, 60, 260];
                let next = STEPS
                    .iter()
                    .position(|&v| v == s.max_fps)
                    .map_or(0, |i| i + 1);
                s.max_fps = STEPS[next % STEPS.len()];
                changed
            }
            Item::PackProfile => vec![Action::ButtonClick, Action::NextPackProfile],
            Item::Link(i) => {
                if let Tab::Screen(from) = self.tab {
                    self.path.push(from);
                }
                self.tab = Tab::Screen(i);
                self.scroll.amount = 0.0;
                self.search.set_value("");
                self.picked = None;
                self.focus = None;
                vec![Action::ButtonClick]
            }
            Item::Profile => match options {
||||||| c98a51a664
            Item::Link(i) => {
                if let Tab::Screen(from) = self.tab {
                    self.path.push(from);
                }
                self.tab = Tab::Screen(i);
                self.scroll.amount = 0.0;
                self.search.set_value("");
                self.picked = None;
                self.focus = None;
                vec![Action::ButtonClick]
            }
            Item::Profile => match options {
=======
            // The shell follows a link (`link`).
            Item::Link(_) => Vec::new(),
            Item::Profile => match s.shader_options.as_mut().filter(|o| o.pack == pack) {
>>>>>>> arismoko/main
                Some(o) => {
                    o.next_profile();
                    changed
                }
                None => Vec::new(),
            },
            Item::Reset => match native {
                Some(p) => {
                    s.shader_packs[p].reset_options();
                    vec![Output::Event(Event::NativePackOptions(p))]
                }
                None => match s.shader_options.as_mut().filter(|o| o.pack == pack) {
                    Some(o) => {
                        o.reset();
                        changed
                    }
                    None => Vec::new(),
                },
            },
            Item::Option(i) => match native {
                Some(p) if s.shader_packs[p].step_option(usize::from(i), back, false) => {
                    vec![Output::Event(Event::NativePackOptions(p))]
                }
                Some(_) => Vec::new(),
                None => {
                    let options = s.shader_options.as_mut().filter(|o| o.pack == pack);
                    match options.and_then(|o| Some((option_name(o, i)?.to_owned(), o))) {
                        Some((name, o)) => {
                            o.cycle(&name, back);
                            changed
                        }
                        None => Vec::new(),
                    }
                }
            },
        }
    }
    /// A slider moves an option to the value nearest it.
    fn slide(&mut self, s: &mut Settings, item: Item, position: f32) {
        let Item::Option(i) = item else {
            return;
        };
        if let Some((p, _)) = native(s) {
            if s.shader_packs[p].set_slider_position(usize::from(i), position) {
                self.drag_changed = Some(p);
            }
            return;
        }
        let pack = s.horizon.shader_pack.clone();
        if let Some(o) = s.shader_options.as_mut().filter(|o| o.pack == pack)
            && let Some(name) = option_name(o, i).map(str::to_owned)
        {
            o.set_position(&name, position);
        }
    }
    /// A Horizon pack's options are saved, and the pack rebuilt, once a drag.
    fn slider_released(&mut self) -> Vec<Output> {
        let changed = self.drag_changed.take();
        changed.map_or_else(Vec::new, |p| {
            vec![Output::Event(Event::NativePackOptions(p))]
        })
    }
    /// With the "Scroll wheel changes settings" extra, the wheel steps an option between `<` `>`
    /// and the profile (up goes back); the other rows leave it to scroll the list.
    fn wheel_step(&mut self, s: &mut Settings, item: Item, back: bool) -> Option<Vec<Output>> {
        let steps = match item {
            Item::Profile => true,
            Item::Option(_) => switch(item, s).is_none() && slider(item, s).is_none(),
            _ => false,
        };
        if !steps {
            return None;
        }
        let mut outputs = vec![Output::Event(Event::ButtonClick)];
        outputs.extend(self.press(s, item, back));
        Some(outputs)
    }
    fn undoes(&self) -> bool {
        true
    }
    /// The pick, the Shadow Cache, the Horizon packs' options (saved again) and the options of the
    /// `shaders/` pack picked when the screen opened (saved when it closes, or Apply), as the
    /// screen opened with them. Another `shaders/` pack keeps what was saved for it.
    fn undo(&mut self, s: &mut Settings, before: &Settings) -> Vec<Output> {
        let mut outputs = Vec::new();
        s.horizon.shadow_cache = before.horizon.shadow_cache;
        for (i, pack) in s.shader_packs.iter_mut().enumerate() {
            let was = before.shader_packs.iter().find(|p| p.key == pack.key);
            if let Some(was) = was.filter(|was| was.values != pack.values) {
                pack.values = was.values.clone();
                outputs.push(Output::Event(Event::NativePackOptions(i)));
            }
        }
        if s.horizon.shader_pack != before.horizon.shader_pack {
            // The pick goes back with its options as they were; what this screen saved over
            // them is saved again when it closes. The other pack's changes are dropped.
            s.horizon.shader_pack = before.horizon.shader_pack.clone();
            s.shader_options = before.shader_options.clone();
            if let Some(o) = s.shader_options.as_mut()
                && let Some(written) = self.written.get(&o.pack)
            {
                o.saved_as(written.clone());
            }
            outputs.push(Output::Event(Event::ShaderPack));
            if s.shader_options.is_none() {
                outputs.push(Output::Event(Event::OpenShaderOptions));
            }
        } else if let Some(o) = s.shader_options.as_mut()
            && let Some(was) = before.shader_options.as_ref().filter(|w| w.pack == o.pack)
        {
            o.restore(was.chosen.clone());
        }
        outputs
    }
    /// Undo shows exactly while the pick, the Shadow Cache or a pack's options differ from what
    /// the screen opened with: an option stepped back to its value isn't a change.
    fn differs(&self, s: &Settings, before: &Settings) -> Option<bool> {
        let native = s.shader_packs.iter().any(|pack| {
            let was = before.shader_packs.iter().find(|p| p.key == pack.key);
            was.is_some_and(|was| was.values != pack.values)
        });
        let options = match (classic(s), before.shader_options.as_ref()) {
            (Some(o), Some(was)) => o.pack == was.pack && o.chosen != was.chosen,
            _ => false,
        };
        let h = (&s.horizon, &before.horizon);
        let pick = h.0.shader_pack != h.1.shader_pack;
        Some(pick || h.0.shadow_cache != h.1.shadow_cache || native || options)
    }
    /// A `shaders/` pack's options wait for Apply until the pack loads again with them.
    fn pending(&self, s: &Settings) -> bool {
        classic(s).is_some_and(|o| o.changed)
    }
    /// The options are saved, and the pack loads again with them.
    fn apply(&mut self, s: &mut Settings) -> Vec<Output> {
        self.saving(s);
        vec![
            Output::Event(Event::ShaderOptions),
            Output::Event(Event::OpenShaderOptions),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hud::{HudAtlas, HudBatch};
    use crate::settings_shell::{Hosted, Part, Screen as _, Shell, Target};
    use crate::shader_options::ShaderOptions;
    use crate::shader_pack_list::Entry;
    use horizon_shaders::{Lang, Macros, PackOption, Properties, Screens};
    use winit::keyboard::KeyCode as K;

    const SIZE: (f32, f32) = (854.0, 480.0);
    const CLICK: Output = Output::Event(Event::ButtonClick);
    const CHANGED: Output = Output::Event(Event::Changed);

    fn atlas() -> HudAtlas {
        HudAtlas::build(&horizon_assets::resource_source::ResourceSource::directory(
            "/nonexistent/horizon-test-assets".into(),
        ))
        .unwrap()
    }

    fn entry(file: &str, kind: Kind) -> Entry {
        Entry {
            file: file.into(),
            key: file.into(),
            title: file.into(),
            kind,
            options: Vec::new(),
            values: Vec::new(),
            pages: Vec::new(),
        }
    }

    fn option(name: &str, values: &[&str], switch: bool) -> (String, PackOption) {
        let o = PackOption {
            name: name.into(),
            default: values[0].into(),
            values: values.iter().map(|v| (*v).into()).collect(),
            switch,
            tested: true,
        };
        (name.to_owned(), o)
    }

    /// `A.zip` picked, with `options` laid out by `properties`.
    fn classic_pack(options: BTreeMap<String, PackOption>, properties: &str) -> Settings {
        let mut s = Settings {
            shader_packs: vec![entry("A.zip", Kind::Classic), entry("B.zip", Kind::Classic)],
            ..Settings::default()
        };
        s.horizon.shader_pack = "A.zip".into();
        let screens = Screens::parse(
            &Properties::parse(properties, &Macros::standard()),
            &options,
        );
        s.shader_options = Some(ShaderOptions::new(
            "A.zip".into(),
            options,
            screens,
            Lang::default(),
            BTreeMap::new(),
        ));
        s
    }

    /// Two packs, the first picked: a `shaders/` pack with a main screen, a Lighting screen and
    /// a Shadows screen inside it. Options by name: BLOOM 0, QUALITY 1, SHADOW_RES 2, SUN 3.
    fn settings() -> Settings {
        let options = BTreeMap::from([
            option("BLOOM", &["false", "true"], true),
            option("QUALITY", &["1", "2", "3"], false),
            option("SHADOW_RES", &["1024", "2048", "4096"], false),
            option("SUN", &["1", "2"], false),
        ]);
        classic_pack(
            options,
            "screen = <profile> [LIGHTING] BLOOM QUALITY\nscreen.LIGHTING = SUN [SHADOWS]\n\
             screen.SHADOWS = SHADOW_RES\nsliders = SHADOW_RES\n\
             profile.LOW = SHADOW_RES=1024\nprofile.HIGH = SHADOW_RES=4096\n",
        )
    }

    /// The picked pack with `n` screens, each linked from its main one: more than the page list
    /// of a short window holds.
    fn many_screens(n: usize) -> Settings {
        let names: Vec<String> = (0..n).map(|i| format!("S{i}")).collect();
        let options = names
            .iter()
            .map(|name| option(&format!("O_{name}"), &["false", "true"], true))
            .collect();
        let links: Vec<String> = names.iter().map(|n| format!("[{n}]")).collect();
        let mut text = format!("screen = {}\n", links.join(" "));
        for name in &names {
            text += &format!("screen.{name} = O_{name}\n");
        }
        classic_pack(options, &text)
    }

    fn shell_at(tab: Tab) -> Shell<ShaderScreen> {
        let mut shell = Shell::<ShaderScreen>::default();
        shell.show(tab);
        shell
    }

    fn press(shell: &mut Shell<ShaderScreen>, s: &mut Settings, item: Item) -> Vec<Output> {
        let at = shell.point_at(s, SIZE, item);
        shell.click(s, at, SIZE, false, false, false)
    }

    #[test]
    fn a_packs_screens_are_pages_its_main_one_first_under_the_packs_name() {
        let s = settings();
        let sheets = ShaderScreen::default().sheets(&s);
        let pages: Vec<(Tab, &str, bool)> = (sheets.iter())
            .map(|p| (p.id, p.name.as_str(), p.listed))
            .collect();
        // The page list holds the main screen and the ones it links to; Shadows opens from
        // Lighting.
        assert_eq!(
            pages,
            [
                (Tab::Packs, "Shader packs", true),
                (Tab::Screen(0), "General", true),
                (Tab::Screen(1), "LIGHTING", true),
                (Tab::Screen(2), "SHADOWS", false),
            ]
        );
        assert_eq!(sheets[1].heading.as_deref(), Some("A settings"));
        assert!(
            sheets
                .iter()
                .filter(|p| p.id != Tab::Screen(0))
                .all(|p| p.heading.is_none())
        );
        let rows: Vec<Item> = sheets[1].groups[0].rows.iter().map(|r| r.id).collect();
        assert_eq!(
            rows,
            [
                Item::Profile,
                Item::Link(1),
                Item::Option(0),
                Item::Option(1),
                Item::Reset
            ]
        );
        // A tick box, a value between arrows, a slider in its pack's steps, and links as buttons.
        let value = |sheet: usize, item| {
            let rows = &sheets[sheet].groups[0].rows;
            rows.iter().find(|r| r.id == item).unwrap().value.clone()
        };
        assert_eq!(value(1, Item::Option(0)), Value::Switch(false));
        assert_eq!(value(1, Item::Option(1)), Value::Choice("1".into()));
        assert_eq!(value(1, Item::Link(1)), Value::Press);
        assert!(matches!(
            value(3, Item::Option(2)),
            Value::Slider { steps: Some(2), .. }
        ));
        // Another pack's options are not this one's: only the pack list.
        let mut other = s.clone();
        other.horizon.shader_pack = "B.zip".into();
        assert_eq!(ShaderScreen::default().sheets(&other).len(), 1);
    }

    #[test]
    fn a_short_page_list_collapses_a_packs_screens_and_leaves_none_out() {
        let s = many_screens(12);
        let mut shell = shell_at(Tab::Screen(4));
        // 427x480 GUI px: tall enough for the pack list, the pack's line and its 13 screens.
        let mut batch = HudBatch::scratch([854, 960]);
        shell.draw(&s, &mut batch, &atlas(), (0.0, 0.0), false, None);
        assert_eq!(shell.page_places().len(), 1 + 13);
        let mut batch = HudBatch::scratch([854, 480]);
        shell.draw(&s, &mut batch, &atlas(), (0.0, 0.0), false, None);
        let parts: Vec<(Tab, Part)> = shell.page_places().iter().map(|p| (p.0, p.2)).collect();
        assert_eq!(
            parts,
            [
                (Tab::Packs, Part::Tab),
                (Tab::Screen(3), Part::Prev),
                (Tab::Screen(4), Part::Shown),
                (Tab::Screen(5), Part::Next)
            ]
        );
    }

    #[test]
    fn picking_a_pack_and_picking_it_again_turns_shaders_off() {
        for (kind, key) in [(Kind::Classic, "B.zip"), (Kind::Horizon, "horizon:warm")] {
            let mut s = settings();
            s.shader_packs[1].kind = kind;
            s.shader_packs[1].key = key.into();
            let mut shell = shell_at(Tab::Packs);
            let picks = vec![
                CLICK,
                Output::Event(Event::ShaderPack),
                Output::Event(Event::OpenShaderOptions),
            ];
            assert_eq!(press(&mut shell, &mut s, Item::Pack(1)), picks);
            assert_eq!(s.horizon.shader_pack, key);
            assert_eq!(picked(&s), Some((1, kind)));
            assert_eq!(switch(Item::Pack(1), &s), Some(true));
            press(&mut shell, &mut s, Item::Pack(1));
            assert_eq!(s.horizon.shader_pack, "");
            assert_eq!(switch(Item::Pack(0), &s), Some(false));
        }
    }

    #[test]
    fn options_wait_for_apply_and_links_go_in_and_escape_back_out() {
        let mut s = settings();
        let mut shell = shell_at(Tab::Screen(0));
        let is_pending = |shell: &mut Shell<ShaderScreen>, s: &Settings| {
            shell.foot_at(s, SIZE).iter().any(|f| f.0 == Target::Apply)
        };
        assert!(!is_pending(&mut shell, &s));
        assert_eq!(press(&mut shell, &mut s, Item::Option(0)), [CLICK, CHANGED]);
        assert_eq!(switch(Item::Option(0), &s), Some(true));
        assert!(is_pending(&mut shell, &s));
        // Stepped back, nothing waits.
        press(&mut shell, &mut s, Item::Option(0));
        assert!(!is_pending(&mut shell, &s));
        press(&mut shell, &mut s, Item::Option(0));
        // Apply saves them and reads the pack again.
        let apply = shell
            .foot_at(&s, SIZE)
            .into_iter()
            .find(|f| f.0 == Target::Apply);
        let [x, y, w, h] = apply.unwrap().1;
        let applied = shell.click(
            &mut s,
            (x + w / 2.0, y + h / 2.0),
            SIZE,
            false,
            false,
            false,
        );
        let reload = [Event::ShaderOptions, Event::OpenShaderOptions].map(Output::Event);
        assert_eq!(applied, [&[CLICK][..], &reload].concat());
        // Into Lighting, then Shadows, which isn't in the page list: Escape backs out of each.
        press(&mut shell, &mut s, Item::Link(1));
        press(&mut shell, &mut s, Item::Link(2));
        assert_eq!(shell.open_page(), Some(Tab::Screen(2)));
        let mut batch = HudBatch::scratch([1708, 960]);
        shell.draw(&s, &mut batch, &atlas(), (0.0, 0.0), false, None);
        assert_eq!(shell.lit_page(), Some(Tab::Screen(1)));
        assert!(shell.key(&mut s, K::Escape, None, false, SIZE).is_empty());
        assert_eq!(shell.open_page(), Some(Tab::Screen(1)));
        shell.key(&mut s, K::Escape, None, false, SIZE);
        let done = shell.key(&mut s, K::Escape, None, false, SIZE);
        assert_eq!(
            done,
            [Output::Done],
            "the main screen's Escape closes the screen"
        );
        // Reset puts the option back.
        press(&mut shell, &mut s, Item::Reset);
        assert_eq!(switch(Item::Option(0), &s), Some(false));
    }

    #[test]
    fn left_and_right_step_an_option_and_a_shift_click_steps_it_back() {
        let mut s = settings();
        let mut shell = shell_at(Tab::Screen(0));
        let value = |s: &Settings| classic(s).unwrap().value("QUALITY").map(str::to_owned);
        shell.set_focus(Some(Target::Row(Item::Option(1))));
        shell.key(&mut s, K::ArrowRight, None, false, SIZE);
        assert_eq!(value(&s).as_deref(), Some("2"));
        shell.key(&mut s, K::ArrowLeft, None, false, SIZE);
        shell.key(&mut s, K::ArrowLeft, None, false, SIZE);
        assert_eq!(value(&s).as_deref(), Some("3"));
        let at = shell.point_at(&s, SIZE, Item::Option(1));
        shell.click(&mut s, at, SIZE, false, true, false);
        assert_eq!(
            value(&s).as_deref(),
            Some("3"),
            "the right button presses nothing"
        );
        shell.click(&mut s, at, SIZE, false, false, true);
        assert_eq!(value(&s).as_deref(), Some("2"));
        assert_eq!(shell.focused_row(), Some(Item::Option(1)));
    }

    #[test]
    fn the_search_finds_an_option_on_a_screen_inside_another() {
        let s = settings();
        let mut shell = shell_at(Tab::Packs);
        shell.set_search("shadow_res");
        assert_eq!(shell.listed(&s, SIZE), [(Tab::Screen(2), Item::Option(2))]);
        assert_eq!(slider(Item::Option(2), &s), Some(0.0));
    }

    #[test]
    fn tab_reaches_every_pack_the_folder_and_the_built_in_setting() {
        let mut s = settings();
        let mut shell = shell_at(Tab::Packs);
        let mut seen = Vec::new();
        for _ in 0..12 {
            shell.key(&mut s, K::Tab, None, false, SIZE);
            if let Some(item) = shell.focused_row()
                && !seen.contains(&item)
            {
                seen.push(item);
            }
        }
<<<<<<< HEAD
        assert_eq!(
            seen,
            [
                Item::Pack(0),
                Item::Pack(1),
                Item::Folder,
                Item::ShadowCache,
                Item::Resolution,
                Item::FpsLimit,
                Item::PackProfile
            ]
        );
||||||| c98a51a664
        assert_eq!(
            seen,
            [
                Item::Pack(0),
                Item::Pack(1),
                Item::Folder,
                Item::ShadowCache
            ]
        );
=======
        let all = [
            Item::Pack(0),
            Item::Pack(1),
            Item::Folder,
            Item::ShadowCache,
        ];
        assert_eq!(seen, all);
>>>>>>> arismoko/main
    }

    /// A Horizon pack picked, with a float option from 0 to 1 in steps of 0.1 at 0.5.
    fn horizon_pack() -> Settings {
        let mut pack = entry("H", Kind::Horizon);
        pack.key = "horizon:slider_test".into();
        pack.options = vec![horizon_shader_api::PackOption {
            id: "strength".into(),
            name: "Strength".into(),
            description: String::new(),
            kind: OptionKind::Float {
                default: 0.5,
                min: 0.0,
                max: 1.0,
                step: 0.1,
            },
        }];
        pack.values = vec![0.5];
        let mut s = Settings {
            shader_packs: vec![pack],
            ..Settings::default()
        };
        s.horizon.shader_pack = "horizon:slider_test".into();
        s
    }

    #[test]
    fn a_horizon_packs_slider_drag_is_saved_once_when_released_and_a_key_step_at_once() {
        let mut s = horizon_pack();
        let mut shell = shell_at(Tab::Screen(0));
        let (_, y) = shell.point_at(&s, SIZE, Item::Option(0));
        let (track, width) = crate::settings_shell::Columns::of(SIZE.0, SIZE.1).track();
        let start = shell.click(&mut s, (track + 2.0, y), SIZE, false, false, false);
        assert_eq!(start, [Output::Event(Event::SliderDrag)]);
        shell.click(&mut s, (track + width, y), SIZE, true, false, false);
        assert_eq!(s.shader_packs[0].values[0], 1.0);
        let saved = vec![Output::Event(Event::NativePackOptions(0))];
        assert_eq!(shell.release(), Some(saved.clone()));
        assert_eq!(shell.release(), None);
        // Left steps it one of its steps, saved as it goes.
        let stepped = shell.key(&mut s, K::ArrowLeft, None, false, SIZE);
        assert_eq!(
            stepped,
            [&[Output::Event(Event::SliderDrag)][..], &saved].concat()
        );
        assert!((s.shader_packs[0].values[0] - 0.9).abs() < 1e-6);
    }

    #[test]
    fn undo_puts_back_the_pick_the_options_and_a_horizon_packs_values() {
        let mut s = settings();
        s.shader_packs.push(horizon_pack().shader_packs.remove(0));
        let mut shell = shell_at(Tab::Screen(0));
        let undo_shows = |shell: &mut Shell<ShaderScreen>, s: &Settings| {
            shell.foot_at(s, SIZE).iter().any(|f| f.0 == Target::Undo)
        };
        press(&mut shell, &mut s, Item::Option(0));
        assert!(undo_shows(&mut shell, &s));
        // Stepped back to how it opened, there is nothing to undo.
        press(&mut shell, &mut s, Item::Option(0));
        assert!(!undo_shows(&mut shell, &s));
        // A changed option, then the Horizon pack picked and its option moved.
        press(&mut shell, &mut s, Item::Option(0));
        shell.show(Tab::Packs);
        let picked = press(&mut shell, &mut s, Item::Pack(2));
        assert!(
            picked.contains(&Output::Event(Event::ShaderOptions)),
            "{picked:?}"
        );
        // The game reads the next pack's options: a Horizon pack has none of these.
        s.shader_options = None;
        // Its slider, a step on from the keyboard.
        shell.show(Tab::Screen(0));
        shell.set_focus(Some(Target::Row(Item::Option(0))));
        shell.key(&mut s, K::Enter, None, false, SIZE);
        shell.key(&mut s, K::ArrowRight, None, false, SIZE);
        assert_ne!(s.shader_packs[2].values[0], 0.5);
        let undo = shell
            .foot_at(&s, SIZE)
            .into_iter()
            .find(|f| f.0 == Target::Undo);
        let [x, y, w, h] = undo.unwrap().1;
        let outputs = shell.click(
            &mut s,
            (x + w / 2.0, y + h / 2.0),
            SIZE,
            false,
            false,
            false,
        );
        // The pick and the Horizon pack's value are back and saved; the `shaders/` pack's options
        // are back as it opened, to be saved over what picking away saved.
        assert_eq!(s.horizon.shader_pack, "A.zip");
        assert_eq!(s.shader_packs[2].values[0], 0.5);
        for event in [Event::NativePackOptions(2), Event::ShaderPack] {
            assert!(
                outputs.contains(&Output::Event(event.clone())),
                "{outputs:?}"
            );
        }
        assert_eq!(switch(Item::Option(0), &s), Some(false));
        assert!(classic(&s).unwrap().changed);
        assert!(!undo_shows(&mut shell, &s));
    }

    #[test]
    fn what_a_pack_costs_is_set_only_with_a_pack_on() {
        let mut s = settings();
        s.horizon.resolution_scale = 50;
        s.max_fps = 60;
        let mut ui = ShaderUi::default();
        let packs = |s: &Settings| ShaderUi::items(Tab::Packs, s, &screens(s));
        assert!(packs(&s).contains(&Item::PackProfile));

        // A `shaders/` pack starts again at the new resolution; a frame limit needs no start.
        assert_eq!(
            ui.activate(&mut s, Item::Resolution, false, None),
            [Action::ButtonClick, Action::Reload, Action::Changed]
        );
        assert_eq!(s.horizon.resolution_scale, 40);
        assert_eq!(
            ui.activate(&mut s, Item::FpsLimit, false, None),
            [Action::ButtonClick, Action::Changed]
        );
        assert_eq!(s.max_fps, 260);
        assert_eq!(
            ui.activate(&mut s, Item::FpsLimit, false, None),
            [Action::ButtonClick, Action::Changed]
        );
        assert_eq!(s.max_fps, 30);

        // A Horizon pack has no profiles, and with no pack there is nothing to set.
        s.shader_packs[0].kind = Kind::Horizon;
        assert!(!packs(&s).contains(&Item::PackProfile));
        assert!(packs(&s).contains(&Item::Resolution));
        s.horizon.shader_pack.clear();
        assert!(!packs(&s).contains(&Item::Resolution));
    }

    #[test]
    fn every_text_is_in_the_language_table() {
        let mut s = settings();
        let (_, mut missed) = lang::scoped(lang::Language::load(&[], "de_de"), || {
            let screen = ShaderScreen::default();
            screen.sheets(&s);
            s.shader_packs[0].kind = Kind::Horizon;
<<<<<<< HEAD
            ShaderUi::description(Item::Pack(0), &s, &screens);
            for text in [
                "Shaders",
                "Shader packs",
                "Installed",
                "General",
                "Pack settings",
                "Built-in settings",
                "%s settings",
                PACK_SETTINGS,
                "Shadow Cache",
                "Reuses the pack shadow map when sun, view and world match. Off by default.",
                "Apply",
                "Search options",
                "Results for \"%s\"",
                "No shader packs yet: open the pack folder and put some in it.",
                "Pick a shader pack. Picking the one in use turns shaders off.",
                "Changed options apply with Apply, or when you leave.",
                "Resolution",
                "FPS Limit",
                "Profile",
                "Pack Default",
                "Auto",
                "Off",
                "Packs cost by the pixel: fewer run faster, and the world looks softer.",
                "A pack heats a phone up: a limit it can hold stays smoother than one it can't.",
                "Auto picks the pack's light settings for this device. Pack Default runs it as it comes.",
            ] {
||||||| c98a51a664
            ShaderUi::description(Item::Pack(0), &s, &screens);
            for text in [
                "Shaders",
                "Shader packs",
                "Installed",
                "General",
                "Pack settings",
                "Built-in settings",
                "%s settings",
                PACK_SETTINGS,
                "Shadow Cache",
                "Reuses the pack shadow map when sun, view and world match. Off by default.",
                "Apply",
                "Search options",
                "Results for \"%s\"",
                "No shader packs yet: open the pack folder and put some in it.",
                "Pick a shader pack. Picking the one in use turns shaders off.",
                "Changed options apply with Apply, or when you leave.",
            ] {
=======
            screen.sheets(&s);
            s.shader_packs.clear();
            screen.sheets(&s);
            for text in [TITLE, PENDING, "Apply", "Undo", "Search options"] {
>>>>>>> arismoko/main
                lang::get(text);
            }
        });
        // The folder's name and how a value is joined to its name are vanilla's.
        missed.retain(|key| !VANILLA_KEYS.contains(&key.as_str()));
        assert_eq!(missed, Vec::<String>::new());
    }
    const VANILLA_KEYS: [&str; 3] = ["pack.openFolder", "options.generic_value", "options.off"];
}
