# Horizonmatica Android storage

Horizonmatica's Android schematic directory is:

`/storage/emulated/0/HorizonAndroid/Litematica/`

The app creates this directory and the file `put your schematic over here.txt` at startup. Put `.litematic`,
`.schem`, `.schematic`, and structure `.nbt` files in this folder.

This is direct filesystem access. It does **not** use the Storage Access Framework (SAF).

## Android permission

The Android manifest declares `MANAGE_EXTERNAL_STORAGE`. On Android 11 (API 30) and newer,
the user must still grant **All files access** manually:

1. Open Android Settings.
2. Open Apps → Horizon Android → Special app access → All files access.
3. Enable access for Horizon Android.
4. Restart Horizon Android so it can create the folder and placeholder file.

Without that special access, Android's scoped-storage rules prevent an app targeting SDK 35
from writing arbitrary folders under shared storage. The older `WRITE_EXTERNAL_STORAGE`
permission does not grant that access on Android 11+.

Google Play restricts use of `MANAGE_EXTERNAL_STORAGE`; this configuration is intended for
your own/sideloaded build and may require a policy review for Play distribution.

## Applying the source changes

1. Extract this ZIP at the repository root. It adds the three original mod source trees under
   `mods/official/` and the Horizonmatica placeholder template.
2. Apply `litematica-storage.patch` from the ZIP root:

   `git apply litematica-storage.patch`

The three `.hnmod` packages are preserved as source imports. This Android repository uses a
fixed static native-mod bundle; adding an HNMOD source directory alone does not compile or
activate the mod. The mods still require Android-native porting and a successful APK build.
