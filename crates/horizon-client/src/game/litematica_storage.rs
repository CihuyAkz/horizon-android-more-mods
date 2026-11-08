use std::{
    cell::{Cell, RefCell},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crate::game::native_hooks::NativeMod;

const ID: &str = "horizonmatica";
const VERSION: &str = "0.2.0";
const PLACEHOLDER: &str = "put your schematic over here.txt";

thread_local! {
    static LAST_POLL: Cell<Option<Instant>> = const { Cell::new(None) };
    static LAST_RESULT: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(super) fn listed(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: ID.into(),
        title: format!("Horizonmatica {VERSION}"),
        description: "Schematic folder for .litematic, .schem, .schematic and .nbt files".into(),
        keys: Vec::new(),
    });
}

pub(super) fn settings(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != ID {
        return;
    }
    let ready = match directory_path() {
        Ok(directory) => match probe_directory(&directory) {
            Ok(true) => {
                buttons.push("Schematics folder: READY".into());
                true
            }
            Ok(false) => {
                buttons.push("Schematics folder: NOT FOUND".into());
                false
            }
            Err(error) => {
                buttons.push(format!("Schematics folder access failed: {error}"));
                false
            }
        },
        Err(error) => {
            buttons.push(format!("Schematics folder path unavailable: {error}"));
            false
        }
    };
    buttons.push(if ready {
        "Re-verify schematics folder".into()
    } else {
        "Create / repair schematics folder".into()
    });
    if let Some(result) = LAST_RESULT.with(|last| last.borrow().clone()) {
        buttons.push(result);
    }
}

pub(super) fn click(mod_id: &str, button: usize) {
    if mod_id != ID || button != 1 {
        return;
    }
    match directory_path().and_then(|directory| ensure_directory_at(&directory)) {
        Ok(()) => LAST_RESULT.with(|last| {
            *last.borrow_mut() = Some("Schematics folder verified".into())
        }),
        Err(error) => {
            LAST_RESULT.with(|last| {
                *last.borrow_mut() = Some(format!("Folder check failed: {error}"))
            });
            log::warn!("Horizonmatica storage: {error}");
        }
    }
}

pub(super) fn poll() {
    let due = LAST_POLL.with(|last| {
        let now = Instant::now();
        if last
            .get()
            .is_some_and(|previous| now.duration_since(previous) < Duration::from_secs(2))
        {
            return false;
        }
        last.set(Some(now));
        true
    });
    if !due {
        return;
    }
    match directory_path().and_then(|directory| ensure_directory_at(&directory)) {
        Ok(()) => LAST_RESULT.with(|last| {
            *last.borrow_mut() = Some("Schematics folder verified".into())
        }),
        Err(error) => {
            LAST_RESULT.with(|last| {
                *last.borrow_mut() = Some(format!("Folder check failed: {error}"))
            });
            log::warn!("Horizonmatica storage: {error}");
        }
    }
}

fn directory_path() -> Result<PathBuf, String> {
    let config = crate::game::Game::native_config_dir()?;
    let game_directory = config
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", config.display()))?;
    Ok(game_directory.join("schematics"))
}

fn probe_directory(directory: &Path) -> Result<bool, String> {
    match std::fs::metadata(directory) {
        Ok(metadata) if !metadata.is_dir() => {
            return Err(format!("{} is not a directory", directory.display()));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(format!("{}: {error}", directory.display())),
    }
    let mut entries = std::fs::read_dir(directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    if let Some(entry) = entries.next() {
        entry.map_err(|error| format!("{}: {error}", directory.display()))?;
    }
    let probe = directory.join(format!(".horizonmatica-write-test-{}", std::process::id()));
    std::fs::write(&probe, b"ok").map_err(|error| format!("{}: {error}", probe.display()))?;
    std::fs::remove_file(&probe).map_err(|error| format!("{}: {error}", probe.display()))?;
    Ok(true)
}

fn ensure_directory_at(directory: &Path) -> Result<(), String> {
    std::fs::create_dir_all(directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    if !directory.is_dir() {
        return Err(format!("{} exists but is not a directory", directory.display()));
    }
    let marker = directory.join(PLACEHOLDER);
    if !marker.is_file() {
        std::fs::write(
            &marker,
            "Put your .litematic, .schem, .schematic, or .nbt files in this folder.\n",
        )
        .map_err(|error| format!("{}: {error}", marker.display()))?;
    }
    probe_directory(directory)?;
    Ok(())
}
