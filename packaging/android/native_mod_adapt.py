#!/usr/bin/env python3
"""Native mods made before Horizon split its client into crates, pointed at where things went.

A mod names what it hooks by crate and module, and is built by Horizon at the commit it pins. Starling
builds every mod at its own commit (native_build.sh), and at that commit `horizon_client` holds only the
game module and a glob of `horizon_client_lib`, which horizon-mixins can't look through. The manifests
are pointed at where things went, and the mods' calls to what changed its shape since are brought up to
date (CALLS, CHANGED). What is left is the compiler's to judge.

    native_mod_adapt.py SOURCE MODS    SOURCE is a checkout, MODS holds <bundle>/horizon-mixin.toml
"""
import json
import pathlib
import re
import sys

ROOTS = {
    "horizon_client_lib": "crates/horizon-client/src/client_lib.rs",
    "horizon_client_render": "crates/horizon-client-render/src/lib.rs",
    "horizon_client_core": "crates/horizon-client-core/src/lib.rs",
    "horizon_client_input": "crates/horizon-client-input/src/lib.rs",
}
FOLDERS = ("crates/horizon-client-render/src/", "crates/horizon-client-core/src/", "crates/horizon-client-input/src/")
OLD_FOLDER = "crates/horizon-client/src/"
OLD_ROOT = OLD_FOLDER + "lib.rs"
# What Horizon renamed since, and takes the same arguments.
RENAMED = {"horizon_client::game::titles::Game<'_>::draw_titles": "horizon_client::game::titles::Game<'_>::draw_title"}
MODULE = re.compile(r"^\s*(?:pub(?:\(crate\))? )?mod (\w+)\s*(?:;|\{)", re.M)
# Calls with the arguments they took (a call, how many, what the new ones are).
CALLS = (
    (".block_loot_at(", 3, "None"),
    (".show_suggestions(", 1, "false"),
    ("ui::slider(", 9, "false"),
    ("super::build_renderer(", 7, "false, false"),
    ("content_root(", 3, "None, &mut |_| {}"),
    # A batch is laid out with the view's GUI scale settings now (the game's, in a mod's helper).
    ("HudBatch::new(", 1, "game.settings.gui_scale_inputs()"),
)
# What became something else. Some of it gives a little up: a LAN guest as the F3 target, `/dh` in the
# chat's completion, a camera's own eye height, the bounds of a dimension lower than the overworld, a
# world's own set of mods and the note about preparing them when a mod calls `content_root` again.
CHANGED = (
    (r"GuestUse::Entity\(Hand::Main\)", "GuestUse::Entity(Hand::Main, _)"),
    (r"Slots::Ender => Vec::new\(\),", "_ => Vec::new(),"),
    (r"\b[\w.]*\.dim\.min_y\(\)", "(-64)"),
    (r"\b[\w.]*\.dim\.max_y\(\)", "319"),
    (r"fn treefall_destroy_progress\(&self", "fn treefall_destroy_progress(&mut self"),
    (r"fn treefall_speed\(game: &crate::game::Game<'_>", "fn treefall_speed(game: &mut crate::game::Game<'_>"),
    (r"chat_animation_layout\(self, height, ticks, focused\)", "chat_animation_layout(self, height, ticks as u64, focused)"),
    (
        r"(\.registry\s*\.state\(state\)\s*\.map\(\|s\| s\.properties\.clone\(\)\)\s*\.unwrap_or_default\(\))",
        r"\1.into()",
    ),
    (r"game\.camera_eye_height\(\)\.unwrap_or\(STANDING_EYE\)", "STANDING_EYE"),
    (r"portal::set_entities\(vertices\)", "portal::set_entities(vertices.0)"),
    (
        r"let Some\(tree\) = remote\.commands\.as_ref\(\)\.and_then\(with_commands\) else \{ return \};"
        r"\s*remote\.commands = Some\(tree\);\s*game\.merge_remote_tree\(\);",
        "let _ = remote;",
    ),
    (
        r"(oversized_in_gui: resolved\.oversized_in_gui,\s*animated,)(?!\s*cit_glint:)",
        r"\1 cit_glint: None, hdr_tier: 0, hdr_light: 0,",
    ),
    (r"extra: &\[\(&Part, \[f32; 4\], u32, u32\)\]", "extra: &[(&Part, [f32; 4], u32, u32, bool)]"),
    (r"crate::game::Game::tooltip_box\(batch, atlas, ", "game.tooltip_box(batch, None, "),
    (r"= self\.guest_hit\(\) \{", "= None::<(u32, _)> {"),
    # The players as the old helper listed them (Horizon's gives handles now).
    (r"\.native_server_players\(\)", ".native_server_players_old()"),
    # The camera type is the mod API's now, and asks `is_first_person`.
    (r"\bcrate::game::perspective::CameraType\b", "::mod_api::CameraType"),
    (r"\.first_person\(\)", ".is_first_person()"),
    (r"game\.native_is_multiplayer\(\)", "game.server_address().is_some()"),
    (r"game\.native_server_address\(\)", "game.server_address().map(|a| a.to_string())"),
    (r"game\.native_world_folder\(\)", "game.native_world_dir().map(|d| d.display().to_string())"),
    (
        r"(lava_flow: from\.lava_flow,)(\s*\}\)\);)",
        r"\1 animated_layers: from.animated_layers.clone(), full_faces: from.full_faces.clone(),"
        r" solid: from.solid.clone(),\2",
    ),
    (
        r"d\.custom_name = Some\(bc::ITEM_NAME\.to_owned\(\)\);",
        "d.custom_name = Some(horizon_world::nbt::Tag::String(bc::ITEM_NAME.into()));",
    ),
)


def with_arguments(text, call, count, new):
    """`text` with `new` after the arguments of each `call` that has `count` of them."""
    at = 0
    while (start := text.find(call, at)) >= 0:
        first = start + len(call)
        depth, commas, i = 1, 0, first
        while depth and i < len(text):
            depth += (text[i] in "([{") - (text[i] in ")]}")
            commas += text[i] == "," and depth == 1
            i += 1
        inner = text[first : i - 1].rstrip()
        trailing = inner.endswith(",")
        if commas + (not trailing) == count:
            kept = inner[:-1] if trailing else inner
            text = f"{text[:first]}{kept}, {new}{',' if trailing else ''}{text[first + len(inner) :]}"
        at = first
    return text


# Calls of a helper whose new shape takes other arguments, by how many they pass: the old shape
# Starling keeps as `<name>_old` (game/starling/native_compat.rs).
OLD_SHAPES = ((".native_text_width(", 1, ".native_text_width_old("),)


def count_arguments(text, first):
    """The number of arguments of the call whose `(` ends at `first`, and where its `)` is."""
    depth, commas, i = 1, 0, first
    while depth and i < len(text):
        depth += (text[i] in "([{") - (text[i] in ")]}")
        commas += text[i] == "," and depth == 1
        i += 1
    inner = text[first : i - 1].strip()
    return (commas + (not inner.endswith(","))) if inner else 0, i


def old_shapes(text):
    for call, count, old in OLD_SHAPES:
        at = 0
        while (start := text.find(call, at)) >= 0:
            if count_arguments(text, start + len(call))[0] == count:
                text = text[:start] + old + text[start + len(call) :]
            at = start + 1
    return text


def old_api(text):
    """The client's old `mod_api` module, which Horizon made the `mod_api` crate: Starling keeps it
    with its old names as `starling_mod_api` (a module named `mod_api` would be ambiguous with the
    crate). NBT moved from horizon-world to horizon-core."""
    text = re.sub(r"\buse (horizon_client|crate)::mod_api;", r"use \1::starling_mod_api as mod_api;", text)
    text = re.sub(r"\bhorizon_world::nbt\b", "horizon_core::nbt", text)
    # The mods crate is horizon-mod-package now, with the same modules.
    text = re.sub(r"\bhorizon_mods(?=::)", "horizon_mod_package", text)
    text = text.replace("crates/horizon-mods/", "crates/horizon-mod-package/")
    return re.sub(r"\b(horizon_client|crate)::mod_api\b", r"\1::starling_mod_api", text)


def code(text):
    for call, count, new in CALLS:
        text = with_arguments(text, call, count, new)
    for old, new in CHANGED:
        text = re.sub(old, new, text)
    return old_api(old_shapes(text))


def owners(source):
    """Each top-level module's crate, the first root that declares it."""
    found = {}
    for crate, root in ROOTS.items():
        for module in MODULE.findall((source / root).read_text(encoding="utf-8")):
            found.setdefault(module, crate)
    return found


def adapt(text, source, modules):
    def target(match):
        path = match[2]
        for old, new in RENAMED.items():
            path = path.replace(old, new)
        path = re.sub(
            r"\bhorizon_client::(\w+)",
            lambda m: f"{modules[m[1]]}::{m[1]}" if m[1] in modules else m[0],
            path,
        )
        return f'{match[1]}{path}"'

    def file(match):
        path = match[2]
        if path == OLD_ROOT:
            path = ROOTS["horizon_client_lib"]
        elif path.startswith(OLD_FOLDER) and not (source / path).exists():
            moved = (folder + path[len(OLD_FOLDER) :] for folder in FOLDERS)
            path = next((p for p in moved if (source / p).exists()), path)
        return f'{match[1]}{path}"'

    # A mob's extra parts gained a flag; a binding spells the type as its helper does (CHANGED).
    text = re.sub(r"(\[f32;\s*4\],\s*u32,\s*u32)\)\]", r"\1, bool)]", text)
    # Treefall's helper takes the game by `&mut` now (CHANGED), and so does the target.
    text = re.sub(
        r'(helper = "speed\.rs"\s*bindings = \[\s*\{ name = "game", ty = ")&crate::game::Game',
        r"\1&mut crate::game::Game",
        text,
    )
    text = re.sub(r'^(\s*target\s*=\s*")([^"]*)"', target, text, flags=re.M)
    text = re.sub(r'^(\s*file\s*=\s*")([^"]*)"', file, text, flags=re.M)
    return old_api(text)


# Where a module's items are found, from the source as rustfmt leaves it: impl blocks and items start
# at the line's start, methods four spaces in. Generated module lists must be written out first
# (horizon-materialize, native_build.sh).
CRATES = {"horizon_client": "crates/horizon-client/src/lib.rs", **ROOTS}
ATTRIBUTE = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
DECLARED = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?mod (\w+);")
IMPL = re.compile(r"impl(?:<.*?>)?\s+(?:(.+?)\s+for\s+)?(?:[\w:]+::)?(\w+)")
QUALIFIERS = r"(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+|extern\s+\"C\"\s+)*"
FUNCTION = re.compile(QUALIFIERS + r"fn\s+(\w+)")
METHOD = re.compile(r"    " + QUALIFIERS + r"fn\s+(\w+)")
TYPE = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|type|trait|union)\s+(\w+)")
REEXPORT = re.compile(r"pub(?:\([^)]*\))?\s+use\s+([^;]+);")


class Module:
    def __init__(self):
        self.methods, self.functions, self.types = set(), set(), set()
        # A method's receiver (`&self`, `&mut self`) and parameter names, by (type, name).
        self.parameters = {}
        self.file = None


def parameters(signature):
    """A `fn` signature's receiver and parameter names."""
    found = re.search(r"\bfn\s+\w+\s*(?:<[^(]*>)?\s*\(", signature)
    if not found:
        return "", []
    start = found.end() - 1
    depth, end = 0, len(signature)
    for i in range(start, len(signature)):
        depth += (signature[i] == "(") - (signature[i] == ")")
        if depth == 0:
            end = i
            break
    inner = signature[start + 1 : end]
    receiver = "&mut self" if re.match(r"\s*&\s*mut\s+self\b", inner) else "&self" if re.match(r"\s*&\s*self\b", inner) else ""
    return receiver, re.findall(r"(?:^|[,(])\s*(?:mut\s+)?(\w+)\s*:(?!:)", inner)


def module_items(text):
    """A file's methods (type, name) with their parameters, free functions and type names, and its
    `mod x;` lines with their `#[path]` when they have one."""
    found, declared, attributes, inside, signature = Module(), [], [], None, None
    for line in text.splitlines():
        if signature is not None:
            signature[1].append(line)
            if "{" in line or line.rstrip().endswith(";"):
                found.parameters[signature[0]] = parameters(" ".join(signature[1]))
                signature = None
            continue
        if inside is not None:
            if line.startswith("}"):
                inside = None
            elif inside and (m := METHOD.match(line)):
                key = (inside, m[1])
                found.methods.add(key)
                if "{" in line or line.rstrip().endswith(";"):
                    found.parameters[key] = parameters(line)
                else:
                    signature = (key, [line])
            continue
        if line.startswith("#["):
            attributes.append(line)
            continue
        if line.startswith("impl") and line.rstrip().endswith("{") and (m := IMPL.match(line)):
            # A trait's methods are named `<Type as Trait>::name`.
            inside = trait_marker(m[2], m[1]) if m[1] else m[2]
        elif m := DECLARED.match(line):
            path = next((p[1] for a in attributes if (p := ATTRIBUTE.search(a))), None)
            declared.append((m[1], path))
        elif m := FUNCTION.match(line):
            found.functions.add(m[1])
        elif m := TYPE.match(line):
            found.types.add(m[1])
        elif m := REEXPORT.match(line):
            for name in re.findall(r"(?:\bas\s+)?\b([A-Z]\w*)\b(?=\s*(?:,|\}|$))", m[1]):
                found.types.add(name)
        attributes = []
    return found, declared


def module_tree(source):
    """Every module of the crates in CRATES, by its path (`horizon_client::game::native_hooks`)."""
    tree = {}

    def walk(file, path, owns_folder):
        if path in tree or not file.is_file():
            return
        found, declared = module_items(file.read_text(encoding="utf-8", errors="replace"))
        found.file = file.relative_to(source).as_posix()
        tree[path] = found
        folder = file.parent if owns_folder else file.parent / file.stem
        for name, attribute in declared:
            if attribute:
                child = file.parent / attribute
                walk(child, f"{path}::{name}", child.name == "mod.rs")
            elif (folder / f"{name}.rs").is_file():
                walk(folder / f"{name}.rs", f"{path}::{name}", False)
            else:
                walk(folder / name / "mod.rs", f"{path}::{name}", True)

    for crate, root in CRATES.items():
        walk(source / root, crate, True)
    return tree


def trait_marker(kind, trait):
    """How a trait's methods are indexed: `<Game as ApplicationHandler>` (the trait by its last name)."""
    return f"<{kind} as {re.sub(r'<.*', '', trait.split('::')[-1]).strip()}>"


TRAIT_PART = re.compile(r"<\s*(\w+)(?:<[^>]*>)?\s+as\s+([\w:]+)(?:<[^>]*>)?\s*>")


def split_target(target):
    """`crate::module::Type<'_>::name` as (module, type with its generics, type, name),
    `crate::module::<Type<'_> as Trait>::name` as (module, that part, its trait marker, name), or
    `crate::module::name` as (module, None, None, name); None for a trait's `<T as Trait>::name` at
    the crate's root."""
    if target.startswith("<"):
        return None
    parts = re.split(r"::(?![^<]*>)", target)
    if len(parts) >= 3 and parts[-2].startswith("<"):
        m = TRAIT_PART.fullmatch(parts[-2])
        if not m:
            return None
        return "::".join(parts[:-2]), parts[-2], trait_marker(m[1], m[2]), parts[-1]
    if len(parts) >= 3 and parts[-2][:1].isupper():
        return "::".join(parts[:-2]), parts[-2], re.sub(r"<.*", "", parts[-2]), parts[-1]
    return "::".join(parts[:-1]), None, None, parts[-1]


def relocate(target, tree):
    """`target` where the game has its function now, when it moved to another module and is the
    only one of its type and name (or the only free function of its name); otherwise as it was."""
    split = split_target(target)
    if split is None:
        return target
    module, spelled, kind, name = split
    if kind:
        here = lambda m: (kind, name) in tree[m].methods
    else:
        here = lambda m: name in tree[m].functions
    if module in tree and here(module):
        return target
    found = [m for m in tree if here(m)]
    if not found:
        return target
    crate = module.split("::")[0]

    def closeness(m):
        common = 0
        for a, b in zip(m.split("::"), module.split("::")):
            if a != b:
                break
            common += 1
        return (m.split("::")[0] == crate, common)

    found.sort(key=closeness, reverse=True)
    if len(found) > 1 and closeness(found[0]) == closeness(found[1]):
        return target
    return f"{found[0]}::{spelled}::{name}" if kind else f"{found[0]}::{name}"


def relocate_file(path, tree, source):
    """A `[[module]]`'s destination file when it moved: the file the module its old path names is in
    now (Horizon moved the game's files into area folders, keeping their module paths)."""
    if (source / path).exists() or not path.endswith(".rs"):
        return path
    for crate, root in CRATES.items():
        folder = root.rsplit("/", 1)[0] + "/"
        if not path.startswith(folder):
            continue
        rest = path[len(folder) : -len(".rs")].removesuffix("/mod")
        module = tree.get(f"{crate}::{rest.replace('/', '::')}")
        if module and module.file:
            return module.file
    return path


# The hook catalog Starling 0.8.0 had (its native-hooks.json): a named hook it listed and the game no
# longer does (`hook = "frame"`) is resolved to where its function is now (the old hooks are in
# game/starling/native_compat.rs).
OLD_CATALOG = pathlib.Path(__file__).with_name("native-hooks-0.8.json")


def complete_catalog(source, tree):
    """The old catalog's hooks added to the one horizon-materialize wrote, where they are not in it;
    the catalog's targets by (name, version)."""
    catalog_file = source / "native-hooks.json"
    if not catalog_file.is_file() or not OLD_CATALOG.is_file():
        return {}
    catalog = json.loads(catalog_file.read_text(encoding="utf-8"))
    known = {(hook["name"], hook["version"]) for hook in catalog["hooks"]}
    added = 0
    for hook in json.loads(OLD_CATALOG.read_text(encoding="utf-8"))["hooks"]:
        if (hook["name"], hook["version"]) in known:
            continue
        target = relocate(hook["target"], tree)
        split = split_target(target)
        module, _, kind, name = split if split else (None, None, None, None)
        if module not in tree or (
            (kind, name) not in tree[module].methods if kind else name not in tree[module].functions
        ):
            continue
        catalog["hooks"].append({"name": hook["name"], "version": hook["version"], "target": target})
        added += 1
    catalog_file.write_text(json.dumps(catalog, indent=2), encoding="utf-8")
    if added:
        print(f"native_mod_adapt: {added} hooks of Starling 0.8.0's catalog added to native-hooks.json")
    return {(hook["name"], hook["version"]): hook["target"] for hook in catalog["hooks"]}


# Parameters Horizon renamed in its hooks, which a binding's value names: `batch` is `draw` in all its
# drawing hooks.
RENAMED_PARAMETERS = {"batch": "draw"}


def rebind(text, tree, hooks):
    """Each injection's binding values naming a parameter Horizon renamed in its target (by `target`,
    or by `hook` through the catalog) name the new one."""
    blocks = re.split(r"(?=^\[\[injection\]\])", text, flags=re.M)
    for i, block in enumerate(blocks):
        target = re.search(r'^\s*target\s*=\s*"([^"]*)"', block, re.M)
        hook = re.search(r'^\s*hook\s*=\s*"([^"]*)"', block, re.M)
        version = re.search(r"^\s*hook_version\s*=\s*(\d+)", block, re.M)
        if target:
            target = target[1]
        elif hook and version:
            target = hooks.get((hook[1], int(version[1])))
        split = target and split_target(target)
        if not split or split[0] not in tree:
            continue
        module, spelled, kind, name = split
        names = tree[module].parameters.get((kind, name), ("", []))[1]
        for old, new in RENAMED_PARAMETERS.items():
            if old in names or new not in names:
                continue
            blocks[i] = block = re.sub(
                r'(value\s*=\s*")([^"]*)(")',
                lambda m, o=old, n=new: m[1] + re.sub(rf"\b{o}\b", n, m[2]) + m[3],
                block,
            )
        # Bindings to parameters the function no longer has, when Starling keeps the old one
        # beside it as `<name>_old` (game/starling/native_compat.rs): the old one.
        used = set()
        for value in re.findall(r'value\s*=\s*"([^"]*)"', block):
            used.update(re.findall(r"\b([a-z_]\w*)\b", value))
        if kind and target and (used - set(names) - {"self", "mut"}):
            old_target = relocate(f"{module}::{spelled}::{name}_old", tree)
            old_split = split_target(old_target)
            if old_split and old_split[0] in tree and (kind, f"{name}_old") in tree[old_split[0]].methods:
                if re.search(r'^\s*target\s*=', block, re.M):
                    block = re.sub(r'^(\s*target\s*=\s*")[^"]*"', rf'\g<1>{old_target}"', block, count=1, flags=re.M)
                else:
                    block = re.sub(r'^(\s*hook\s*=\s*"[^"]*"\s*\n)', rf'\1target = "{old_target}"\n', block, count=1, flags=re.M)
                    block = re.sub(r'^\s*hook(_version)?\s*=.*\n', "", block, flags=re.M)
                blocks[i] = block
                print(f"native_mod_adapt: {name} with its old parameters is {old_target}")
    return "".join(blocks)


INCLUDE = re.compile(r'^include!\("([\w/]+\.rs)"\);\n', re.M)


def inline_includes(source):
    """A game file's `include!("shared.rs")` of a file beside it, replaced by that file: the same
    items in the same module, which horizon-mixins can then reach (it does not look into an
    `include!`). Horizon moved most of the old native_hooks.rs into one of those (helpers mods
    hook); the included files have no inner attributes, and their `#[path]`s are from the same
    folder."""
    # Only the hooks' folder: Horizon's generator wants the `Game` methods of another folder's
    # shared.rs (lan_actions) where they are.
    for path in list((source / "crates/horizon-client/src/game/native_hooks").glob("*.rs")):
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8")
        if "include!(\"" not in text:
            continue

        written = []

        def inlined(match, folder=path.parent):
            written.append(folder / match[1])
            included = written[-1].read_text(encoding="utf-8")
            return included if included.endswith("\n") else included + "\n"

        new = INCLUDE.sub(inlined, text)
        if new != text:
            path.write_text(new, encoding="utf-8")
            # Horizon's generator reads the hook files one by one: the same items twice are refused.
            for file in written:
                file.unlink()


def main():
    source, mods = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    modules = owners(source)
    inline_includes(source)
    tree = module_tree(source)
    hooks = complete_catalog(source, tree)
    for manifest in mods.glob("*/horizon-mixin.toml"):
        old = manifest.read_text(encoding="utf-8")
        new = adapt(old, source, modules)

        def moved(match, name=manifest.parent.name):
            target = relocate(match[2], tree)
            if target != match[2]:
                print(f"native_mod_adapt: {name}: {match[2]} is now {target}")
            return f'{match[1]}{target}"'

        new = re.sub(r'^(\s*target\s*=\s*")([^"]*)"', moved, new, flags=re.M)
        new = re.sub(
            r'^(\s*file\s*=\s*")([^"]*)"',
            lambda m: f'{m[1]}{relocate_file(m[2], tree, source)}"',
            new,
            flags=re.M,
        )
        new = rebind(new, tree, hooks)
        if new != old:
            manifest.write_text(new, encoding="utf-8")
            print(f"native_mod_adapt: {manifest.parent.name} pointed at where things went")
    for source_file in mods.glob("*/**/*.rs"):
        old = source_file.read_text(encoding="utf-8", errors="surrogateescape")
        new = code(old)
        if new != old:
            source_file.write_text(new, encoding="utf-8", errors="surrogateescape")
            print(f"native_mod_adapt: {source_file.parent.name}/{source_file.name} brought up to date")


if __name__ == "__main__":
    main()
