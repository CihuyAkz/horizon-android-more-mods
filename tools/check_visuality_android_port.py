#!/usr/bin/env python3
"""Static checks for the Visuality Android adapter (no Rust toolchain required)."""
from pathlib import Path
import re
import tomllib
import json

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "mods/official/visuality-horizon/mixins/visuality_horizon"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def check_rust_paths(source: Path) -> None:
    text = source.read_text(encoding="utf-8")
    refs = re.findall(r'^\s*#\[path\s*=\s*"([^"]+)"\]', text, re.MULTILINE)
    refs += re.findall(r'include!\("([^"]+)"\)', text)
    for rel in refs:
        target = (source.parent / rel).resolve()
        require(target.is_file(), f"missing Rust path from {source.relative_to(ROOT)}: {rel}")


def main() -> None:
    mixin = tomllib.loads((MOD / "horizon-mixin.toml").read_text(encoding="utf-8"))
    native = json.loads((ROOT / "mods/official/visuality-horizon/horizon-native.json").read_text(encoding="utf-8"))
    require(mixin["mod"]["id"] == native["id"] == "visuality_horizon", "Visuality ID mismatch")
    require(mixin["mod"]["version"] == native["version"] == "1.0.2", "Visuality version mismatch")
    require(mixin["compatibility"]["horizon_revision"] == native["horizon"], "Visuality Horizon revision mismatch")

    expected_injections = {
        "tick", "frame", "list", "settings", "settings_click", "world_drawn",
        "render_needs", "hurt", "spawned", "removed", "left", "dimension",
        "ambient", "slime",
    }
    injections = {item["id"]: item for item in mixin.get("injection", [])}
    require(set(injections) == expected_injections,
            f"Visuality injection list changed: {sorted(set(injections) ^ expected_injections)}")
    for item in injections.values():
        require((MOD / item["helper"]).is_file(), f"missing upstream helper: {item['helper']}")

    sources = [
        ROOT / "crates/horizon-client/src/game.rs",
        ROOT / "crates/horizon-client/src/game/android_bundled.rs",
        ROOT / "crates/horizon-client/src/game/remote.rs",
        ROOT / "crates/horizon-client/src/animate_tick.rs",
        ROOT / "crates/horizon-client/src/mobs/cube.rs",
        ROOT / "crates/horizon-client-render/src/gfx/native_render.rs",
    ]
    for source in sources:
        check_rust_paths(source)

    game = sources[0].read_text(encoding="utf-8")
    require("mod visuality_horizon;" in game, "Visuality game glue module is not declared")
    require((MOD / "game_glue.rs").is_file(), "Visuality game glue source is missing")

    bundled = sources[1].read_text(encoding="utf-8")
    for call in [
        "visuality_listed(", "visuality_page(", "visuality_click(",
        "visuality_tick(", "visuality_frame(",
    ]:
        require(call in bundled, f"Visuality Android bundle call is missing: {call}")

    remote = sources[2].read_text(encoding="utf-8")
    for call in ["visuality_hurt(&packet)", "visuality_entity_changed(id)", "visuality_session()"]:
        require(call in remote, f"Visuality remote hook call is missing: {call}")
    require(remote.count("visuality_session();") >= 2, "Visuality session reset must cover leave and dimension change")

    ambient = sources[3].read_text(encoding="utf-8")
    require("visuality_ambient_hook::visuality_ambient(ctx, state, pos)" in ambient,
            "Visuality ambient block hook is not called")
    cube = sources[4].read_text(encoding="utf-8")
    require("visuality_slime_hook::visuality_slime(m)" in cube,
            "Visuality slime landing hook is not called")

    render = sources[5].read_text(encoding="utf-8")
    require("visuality_horizon::world_drawn(world)" in render,
            "Visuality world renderer hook is not called")
    require("visuality_horizon::render_needs(needs)" in render,
            "Visuality depth requirement hook is not called")

    root_toml = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    engine_path = "mods/official/visuality-horizon/mixins/visuality_horizon/engine"
    require(engine_path in root_toml["workspace"]["members"], "Visuality engine is not a workspace member")
    render_manifest = tomllib.loads((ROOT / "crates/horizon-client-render/Cargo.toml").read_text(encoding="utf-8"))
    android_deps = render_manifest["target"]["cfg(target_os = \"android\")"]["dependencies"]
    require("visuality-horizon-engine" in android_deps, "Android renderer is missing the Visuality engine dependency")
    require((ROOT / engine_path / "src/atlas_data.rs").is_file(), "Visuality compiled atlas source is missing")

    lock = (ROOT / "Cargo.lock").read_text(encoding="utf-8")
    require('name = "visuality-horizon-engine"\nversion = "1.0.2"' in lock,
            "Visuality engine package is absent from Cargo.lock")
    render_lock = re.search(r'name = "horizon-client-render"\nversion = "0\.1\.0"\ndependencies = \[(.*?)\n\]', lock, re.S)
    require(render_lock is not None and '"visuality-horizon-engine"' in render_lock.group(1),
            "Visuality engine is absent from the renderer lock dependencies")

    license_source = (ROOT / engine_path / "src/lib.rs").read_text(encoding="utf-8")
    require("MIT License, Copyright (c) 2026 BenjiSyS" in license_source,
            "Visuality MIT attribution is missing from its engine source")

    print("OK: Visuality manifest, 14 Android hook adapters, renderer dependency, atlas and license resolve")


if __name__ == "__main__":
    main()
