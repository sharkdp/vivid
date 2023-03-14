# unreleased

## Changes


## New filetypes


## New themes



# v0.9.0

## Changes

- Make the output order deterministic, see #63 and #87 (@tavianator)
- Added more filetypes, see #95 (@Babkock)
- Make it possible to build vivid with LTO, see #90 (@ixti)

## Themes

- Added alabaster_dark, see #91 (@p00f)
- Added catppuccin-*, see #94 (@etenzy)
- Added modus-operandi, see #96 (@apraga)
- Updates in the lava theme, see #88 (@fredericrous)
- Updates in the dracula theme, see #85 (@yavko)



# v0.8.0

## Changes

- Added new core filetypes (reset_to_normal, multi_hard_link, door, setuid, setgid), see #78 (@ixti)
- Added instructions on how to preview themes in a user-defined directory, see #75 (@WoLpH)

## New filetypes

- Opus and WavPack audio extensions, see #71 (@desbma)
- `.webm`, see #72 (@desbma)
- `.pyd`, `.pyo`, see #73 (@bl-ue)

## New themes

- Nord, see #66 (@Utagai)
- Lava, see #66 (@fredericrous)
- Iceberg, see #70 (@p00f)
- Gruvbox, see #79 (@ixti)
- Dracula, see #80 (@yavko)


# v0.7.0

- List available themes via `vivid themes`, see #48 (@gillespiecd)
- Added "One" theme, see #51 (@mortezadadgar)
- Fix panic if stdout is closed, see #56

# v0.6.0

- The default themes and config files are now embedded into the `vivid` binary.
  See #43 (@LordFlashmeow)
- Added `XDG_CONFIG_HOME` as a possible source for theme files, see #40 (@LordFlashmeow)
- Better error handling, see #43 (@LordFlashmeow)
- Added macOS category to unimportant files, see #33 (@gseidler)
- Added more media file types, see #38 (@gseidler)
- Add new core filetypes for `sticky`/`other_writable`/`sticky_other_writable`, see #34 (@mattbalvin)

# v0.5.0

- Added `jellybeans` theme, see #26 (@chandlerc)
- Added `solarized-dark` and `solarized-light` theme, see #30 (@menelaos)
- Theme arguments can be a path to a YAML theme file, see #23
- Added LLVM file types, see #27 (@chandlerc)
- Added support for block and character devices
- Added `preview` subcommand, see #13

# v0.4.0

- Better theme- and filetype-db handling, closes #12
- Added unit tests
- Updated filetypes database
- Better error message if filetype database could not be found, closes #9
- Allow RRGGBB colors do be used directly, closes #18

# v0.3.0

- Rename project to "vivid" (was: dircolors-hd)
- Updates to themes and filetypes database

# v0.2.0

- Added `snazzy` theme
- Updated filetype database
- Better error handling

# v0.1.0

Initial version
