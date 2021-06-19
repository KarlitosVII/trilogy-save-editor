## Unreleased
- ME2LE: Added Legendary missing data (ME1 Import Rewards)
- ME1LE: Added remaining data (Journal, Codex, Player, Squad, Maps, World, Mako, etc.)

## 1.11.0
- Added a command line tool to compare the plots of 2 saves, additional information with the --help argument
- Added a `Reload` button for those who often need to reload there saves (modders, etc.)

## 1.10.0
- ME1LE: Added `Inventory` tab
- ME1LE: Fixed a rare `unexpected end of file...` error

## 1.9.0
- ME1LE: Added player and squad talent reset
- Fallback to DirectX 11 if backend panic on startup
- Added command line options (with the ability to choose the backend), additional help with the --help argument
- Can now open a save by dropping the file into the window

## 1.8.3
- Support for Cyrillic and Japanese characters
- Fixed lag when moving the window (caused by frame limit in previous version)
- Reduced CPU usage in some cases (particularly noticeable when opening `Head Morph > Lod0`)

## 1.8.2
- Fixed `add` button not working for War Assets and some other lists
- 60 fps limit for those who have a display above 60Hz. (May reduce CPU / GPU usage on some configs)
- Various small fixes

## 1.8.1
- Fixed `invalid value: '6', expected variant index 0 <= i < 6`
- Open save dialog in the same directory of the save

## 1.8.0
- ME1: Renamed salvage to omnigel
- ME1LE: Added player talent points and some stats
- ME1LE: Added squad (with talents, talent points, inventory and stats)
- ME1LE: Added map name, location and rotation
- ME1LE: Added difficulty, character creation date and player controller
- When creating backup, copy the old save instead of renaming it. This should fix the game loading old save.

## 1.7.2
- Swapped ME1LE medigel and grenades
- Fixed some ME3 bonus power names
- Unhide some raw data that can be used for modding purpose (Debug name, placeables, doors, etc.)

## 1.7.1
- Changed dialog library, I hope it will fix `Open` dialog not opening

## 1.7.0
- Added ME1LE resources (credits, grenades, medigel, salvage) and face code
- Added ME1LE raw player inventory
- Changed backend (again) with a more robust one, it will choose a supported backend on your system (Vulkan, DX11/12, etc.)

## 1.6.1
- Changed backend from OpenGL to Vulkan, I hope it will fix GPU and OpenGL related bugs

## 1.6.0
- Added ME1LE Level and Current XP
- Added ME1LE Raw talents
- Converted raw texts to title case for better readability

## 1.5.0
- Added ME1LE `General` tab with basic information such as Name, Gender, Origin, Notoriety and Morality
- Added ME1LE `Head Morph` tab with Import / Export and raw data

## 1.3.2
- New UNC plots in ME1 (thanks to Yggge)
- Added clarification for editing ME1 plot in ME2 save

## 1.3.1
- Fix ME1LE `unexpected end of file...` error for some people

## 1.3.0
- Initial Mass Effect 1 Legendary support (only plot)

## 1.2.0
- ME2/3 Legendary support

## 1.1.2
- Changing ME2/3 origin / notoriety will update ME1's
- Changing ME3 gender will change Loco / Lola plot corresponding to new gender

## 1.1.1
- High CPU usage fix

## 1.1.0
- HiDPI fix
- Possibility to modify previously read-only ME1 raw strings
- Minor fixes

## 1.0.0
- Initial release
