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
| 6 | 0xb1ca2097b481 for files, 0x01a420b2fd41 for directories |
| 28 | Unknown |
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

* Engine/ - seemingly nothing interesting
* HatinTimeGame/ - vast majority of contents
  * Config/ - All settings applying to multiple save files
    * HatinTimeEngine.ini - Unreal Engine 3 configuration
    * HatinTimeGame.ini - Game configuration, nothing that looks interesting
    * HatinTimeInput.ini - List of buttons
    * HatinTimeLightmass.ini - Lighting configuration
    * HatinTimeMods.ini - Empty on Switch, contains mod options on PC
    * HatinTimeOnline.ini - Online party name, unused on Switch
    * HatinTimeSystemSettings.ini - Graphics settings
    * HatinTimeUI.ini - Unknown
    * Switch/ - Contains overrides for HatinTime inis
    * Other folders contain settings for Wii U(!), Xbox One, and PS4. These are missing on PC.
  * Cooked/ - Game data
    * Startup.upk - Most game data
    * HatinTimeGame.u, HatinTimeGame_Content.u - Unknown
    * Other files aren't very interesting
  * Localization/ - Dialogue
    * Folders for every supported language (INT is English)
      * Intuitively-named inis containing dialogue
  * SaveData/ - Save data
    * saveX - zero-indexed save slots, unknown format
