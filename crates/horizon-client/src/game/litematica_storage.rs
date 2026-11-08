//! Android direct shared-storage bootstrap for Horizonmatica.
//! No Storage Access Framework is used. Android 11+ uses MANAGE_EXTERNAL_STORAGE;
//! Android 9 and older use the legacy WRITE_EXTERNAL_STORAGE runtime permission.

use std::{cell::Cell, path::Path, time::{Duration, Instant}};
use jni::objects::{JObject, JValue};
use crate::game::native_hooks::NativeMod;

const ID: &str = "horizonmatica";
const VERSION: &str = "0.2.0";
const DIRECTORY: &str = "/storage/emulated/0/HorizonAndroid/Litematica";
const PLACEHOLDER: &str = "put your schematic over here.txt";

thread_local! {
    static LAST_POLL: Cell<Option<Instant>> = const { Cell::new(None) };
}

pub(super) fn listed(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: ID.into(),
        title: format!("Horizonmatica {VERSION}"),
        description: format!("Litematica schematic storage setup at {DIRECTORY}. Android rendering and placement runtime are not yet enabled."),
        keys: Vec::new(),
    });
}

pub(super) fn settings(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != ID {
        return;
    }
    let sdk = sdk_int().unwrap_or(0);
    let granted = has_storage_access();
    let status = if sdk == 29 {
        "Unsupported on Android 10 with this app's scoped-storage target".to_string()
    } else if granted {
        "All files access: GRANTED".to_string()
    } else if sdk >= 30 {
        "Grant All files access (opens Android Special app access)".to_string()
    } else {
        "Grant shared-storage permission".to_string()
    };
    buttons.push(status);
    buttons.push(if granted {
        "Create / verify Litematica folder".into()
    } else {
        "Open storage permission settings".into()
    });
}

pub(super) fn click(mod_id: &str, button: usize) {
    if mod_id != ID || button > 1 {
        return;
    }
    if has_storage_access() {
        if let Err(error) = ensure_directory() {
            log::warn!("Horizonmatica storage: {error}");
        }
    } else if !request_storage_access() {
        log::warn!("Horizonmatica could not open Android storage permission settings");
    }
}

/// Called from the Android native frame hook. When the user returns from Settings
/// after granting access, the folder and marker file are created automatically.
pub(super) fn poll() {
    let due = LAST_POLL.with(|last| {
        let now = Instant::now();
        if last.get().is_some_and(|previous| now.duration_since(previous) < Duration::from_secs(2)) {
            return false;
        }
        last.set(Some(now));
        true
    });
    if !due || !has_storage_access() {
        return;
    }
    if let Err(error) = ensure_directory() {
        log::warn!("Horizonmatica storage: {error}");
    }
}

fn sdk_int() -> Option<i32> {
    crate::platform::with_context(|env, _| {
        env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()
    })
}

fn has_storage_access() -> bool {
    crate::platform::with_context(|env, activity| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        if sdk >= 30 {
            return env.call_static_method(
                "android/os/Environment",
                "isExternalStorageManager",
                "()Z",
                &[],
            )?.z();
        }
        if sdk <= 28 {
            let permission = JObject::from(env.new_string("android.permission.WRITE_EXTERNAL_STORAGE")?);
            return Ok(env.call_method(
                activity,
                "checkSelfPermission",
                "(Ljava/lang/String;)I",
                &[JValue::Object(&permission)],
            )?.i()? == 0);
        }
        // Android 10 (API 29) cannot grant the direct shared path to an app
        // targeting API 35 without the legacy storage model; SAF is deliberately not used.
        Ok(false)
    }).unwrap_or(false)
}

fn request_storage_access() -> bool {
    crate::platform::with_context(|env, activity| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        let package = env.call_method(activity, "getPackageName", "()Ljava/lang/String;", &[])?.l()?;
        let package: String = env.get_string((&package).into())?.into();
        let uri_text = JObject::from(env.new_string(format!("package:{package}"))?);
        let uri = env.call_static_method(
            "android/net/Uri",
            "parse",
            "(Ljava/lang/String;)Landroid/net/Uri;",
            &[JValue::Object(&uri_text)],
        )?.l()?;

        if sdk >= 30 {
            let action = JObject::from(env.new_string("android.settings.MANAGE_APP_ALL_FILES_ACCESS_PERMISSION")?);
            let intent = env.new_object(
                "android/content/Intent",
                "(Ljava/lang/String;)V",
                &[JValue::Object(&action)],
            )?;
            env.call_method(
                &intent,
                "setData",
                "(Landroid/net/Uri;)Landroid/content/Intent;",
                &[JValue::Object(&uri)],
            )?;
            if env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&intent)],
            ).is_ok() {
                return Ok(true);
            }
            // Some Android builds omit the app-specific screen; fall back to the
            // global Special app access page so the user can select Horizon Android.
            let _ = env.exception_clear();
            let action = JObject::from(env.new_string("android.settings.MANAGE_ALL_FILES_ACCESS_PERMISSION")?);
            let intent = env.new_object(
                "android/content/Intent",
                "(Ljava/lang/String;)V",
                &[JValue::Object(&action)],
            )?;
            env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&intent)],
            )?;
            return Ok(true);
        }

        if sdk <= 28 {
            let action = JObject::from(env.new_string("android.settings.APPLICATION_DETAILS_SETTINGS")?);
            let intent = env.new_object(
                "android/content/Intent",
                "(Ljava/lang/String;)V",
                &[JValue::Object(&action)],
            )?;
            env.call_method(
                &intent,
                "setData",
                "(Landroid/net/Uri;)Landroid/content/Intent;",
                &[JValue::Object(&uri)],
            )?;
            env.call_method(
                activity,
                "startActivity",
                "(Landroid/content/Intent;)V",
                &[JValue::Object(&intent)],
            )?;
            return Ok(true);
        }
        Ok(false)
    }).unwrap_or(false)
}

fn ensure_directory() -> Result<(), String> {
    if !has_storage_access() {
        return Err("Storage access is not granted. Open Horizonmatica in Options > Mods and tap the storage permission button.".into());
    }
    let directory = Path::new(DIRECTORY);
    std::fs::create_dir_all(directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let marker = directory.join(PLACEHOLDER);
    if !marker.exists() {
        std::fs::write(
            &marker,
            "Put your .litematic, .schem, .schematic, or .nbt files in this folder.\n",
        ).map_err(|error| format!("{}: {error}", marker.display()))?;
    }
    Ok(())
}
