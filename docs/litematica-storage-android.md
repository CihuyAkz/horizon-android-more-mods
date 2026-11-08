# Horizonmatica shared storage on Android

The Android client creates this exact folder after storage access is granted:

`/storage/emulated/0/HorizonAndroid/Litematica/`

It also creates the marker file `put your schematic over here.txt` there.
Place `.litematic`, `.schem`, `.schematic`, and structure `.nbt` files in this directory.

## Granting access

Open **Options > Mods > Horizonmatica** and tap **Grant All files access**. Android opens
its app-specific **Special app access** page (or the global special-access list if the
app-specific page is unavailable). This is not listed under the ordinary runtime-permission
page. Enable the access for Horizon Android and return to the game. The native frame hook
checks the permission and creates the directory automatically after access is granted.

Android 11 (API 30) and newer use `MANAGE_EXTERNAL_STORAGE`. Android 9 and older use the
legacy write-storage permission. Android 10 (API 29) cannot use this direct path with the
app's target SDK 35; SAF is intentionally not used, so direct shared-folder access is not
available on that version.

This is a sideload/development configuration. Google Play restricts `MANAGE_EXTERNAL_STORAGE`
and may reject apps that do not qualify under its all-files-access policy.

## What this source change fixes

- Adds the missing `MANAGE_EXTERNAL_STORAGE` manifest declaration.
- Adds a settings button that opens Android's special app-access screen with the correct
  `package:<applicationId>` URI and a global-settings fallback.
- Checks `Environment.isExternalStorageManager()` instead of assuming the manifest grant.
- Creates the exact folder and marker file after permission is granted, including when the
  user returns from system settings.
- Registers Horizonmatica 0.2.0 in Options > Mods so the storage setup is discoverable.

The source tree for ChatHeads, Horizonmatica, and Item New is included intact. Their complete
Android feature runtimes are not all wired by this change; the Horizonmatica list entry is
currently a storage/setup entry, not a claim that schematic rendering and printer placement
are already active in the Android game.
