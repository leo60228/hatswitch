# Hat Switch modding notes

## Save format
### GameState
| Length | Description |
| --- | --- |
| 8 | Unknown |
| ? | Entries |

### Entry
| Length | Description |
| --- | --- |
| 8 | 0xb1ca2097b481 for files, 0x01a420b2fd41 for directories |
| 8 | FILETIME creation time |
| 8 | FILETIME access time |
| 8 | FILETIME modify time |
| 2 | u16 little-endian: length of path not including null terminator |
| ? | Null-terminated path |
| | Remainder for files only |
| 8 | Unknown, if interpreted as a little-endian u64, it's almost the length, but with a varying offset |
| 4 | Magic 0xe9b7123a |
| ? | Contents |

All paths are prefixed with `/w/DataChanges/`, and the remainder consists of a path in the romfs to seemingly replace.
Due to the lack of working repacker, this is not able to be confirmed.

## Filesystem
This seemingly consists of the romfs overlayed with the `/w/DataChanges` directory in the save.
Assuming this is the case, it's mostly identical to the PC version.

* eon.txt - seems to be compilation logs from a user called "Meku" (hmmmmm)
* Engine/ - seemingly nothing interesting
* HatinTimeGame/ - vast majority of contents
  * Config/ - All settings applying to multiple save files
    * HatinTimeEngine.ini - Unreal Engine 3 configuration
    * HatinTimeGame.ini - Game configuration, nothing that looks interesting
    * HatinTimeInput.ini - List of buttons
    * HatinTimeLightmass.ini - Lighting configuration
    * HatinTimeMods.ini - Empty on Switch, contains mod options on PC. In the initial update, this contains settings for the playable Beter Griffin mod. No, I don't know why.
    * HatinTimeOnline.ini - Online party name, unused on Switch. Defaults to `meku` (I'm starting to sense a pattern here...).
    * HatinTimeSystemSettings.ini - Graphics settings
    * HatinTimeUI.ini - Unknown
    * DefaultEngine.ini - List of packages to load from CookedPC/ (could this be used to load scripts?)
    * Switch/ - Contains overrides for HatinTime inis
    * Other folders contain settings for Wii U(!), Xbox One, and PS4. These are missing on PC.
  * CookedPC/ - Game data
    * Startup.upk - Some game data (mostly scripts)
    * HatinTimeGame.u, HatinTimeGameContent.u - Majority of non-texture assets and scripts
    * Maps/ - Maps. All maps on PC and Switch can be replaced with others.
      * TimeRift_Water/ - Blue time rifts
      * TimeRift_Cave/ - Purple time rifts
      * castle/ - The End
      * chapter3/ - Battle of the Birds (yes, I know this is Chapter 2)
      * DLC_CatMetro/ - Nyakuza Metro (this does exist on the Switch version, and is functional)
      * harbor/ - Mafia Town
      * hub/ - Spaceship
      * literallycantsink/ - Arctic Cruise
      * sandandsails/ - Alpine Skyline (this was originally a desert level, hence the filenames)
      * subconforest/ - Subcon Forest
      * system/ - Title screen and credits
    * Other files aren't very interesting
  * Localization/ - Dialogue
    * Folders for every supported language (INT is English)
      * Intuitively-named inis containing dialogue
  * SaveData/ - Save data
    * saveX - Zero-indexed save slots, unknown format. Saves can put put in the romfs, and they will be copied to the save file upon loading. The format is compatible with PC, despite differing filenames.
  
Timestamps are specified using the [FILETIME windows API struct](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime)
