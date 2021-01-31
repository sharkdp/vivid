# unreleased

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
