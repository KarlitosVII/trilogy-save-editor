## 2.2.1
- ME1LE: Fixed `bonus talents` abilities that did not appear in the HUD

## 2.2.0
- Long-standing requests have finally been implemented (Yay !)
- ME1LE: Added the ability to change `player class` and `specialization`
- ME1LE: Added `bonus talents` selector

## 2.1.4
- Always opens the dialogs in the last opened save directory
- ME2: Fixed ship upgrade plots
- ME2: Fixed number of missions after crew abducted
- ME2: Added number of missions since last main story mission (useful to avoid being forced to do Collector ship, IFF activation, etc.)
- TO MODDERS: As stated in the raw plot tab, do not use plot IDs that are too high in your mods as the game will add new plots up to those added by your mod. For example, ME3 has `~40 000` booleans and if you add the plot `1 000 000` the game will add `960 000` plots in every saves that use your mod. The plot table is now `25x bigger` than before by just adding one plot!

## 2.1.3
- Reverted color values to float so you can use values higher than 1.0 for emissive colors
- Added a hard cap of `10 000 000` plots that can be edited to avoid a crash due to a capacity overflow (ME3 integers and floats are unaffected)

## 2.1.2
- Fixed `Open / Save` dialog not showing up for some people (for real this time)

## 2.1.1
- Fixed `Open / Save` dialog not showing up for some people

## 2.1.0
- Support for Gibbed's head morphs (.me2headmorph, .me3headmorph)

## 2.0.0
- Rebuilt the entire UI from scratch (from Dear Imgui / Vulkan / DX to Wasm / Web View)
- That means no more crashes related to wgpu / Vulkan / DX
- You can now add and edit all your plots in the `Raw plot` tab (not just the labelled ones)
- Xbox 360 support for ME2OT and ME3 (You can convert your save by saving it with the other file extension)
- Added a filter in the ME1LE inventory to make it easier to find items
- Better readability of ME1OT raw data (removal of all `m_` prefixes and removal of `duplicate` buttons, duplication is now auto)
- Added an auto update, it check once a day on the TSE repository (github) and displays a button to install the new version
- Better font for high DPI

## 1.13.1
- ME2: Added all armors plots
- Various small fixes and additions
- Merged ME1 plot db into ME3's

## 1.13.0
- PS4 saves support
- Updated ME1 raw plot database
- Fix ME1 plots through ME1 raw database
- Thanks again to Bioware for the quick update of their plot databases

## 1.12.1
- ME2LE: Added Legendary missing data (ME1 Import Rewards)
- ME1LE: Added remaining data (Journal, Codex, Player, Squad, Maps, World, Mako, etc.)
- Added `Raw Plot` tab to all games with databases (minus ME1(LE) booleans and floats)
- Added and fixed a lot of plots through the databases
- BIG thanks to Bioware for giving us ME1LE save format, plot databases and for responding so quickly to my issues. Love you <3

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
