![vivid](doc/vivid.png)
<br><br>

*vivid* is a generator for the `LS_COLORS` environment variable that controls the colorized output of
[`ls`](https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html#ls-invocation), [`tree`](http://mama.indstate.edu/users/ice/tree/),
[`fd`](https://github.com/sharkdp/fd), etc.

It uses a YAML-based configuration format for the [filetype-database](config/filetypes.yml)
and the [color themes](themes/). In contrast to
[`dircolors`](https://www.gnu.org/software/coreutils/manual/html_node/dircolors-invocation.html#dircolors-invocation),
the database and the themes are organized in different files. This allows users to
choose and customize color themes independent from the collection of file extensions.
Instead of using (cryptic) ANSI escape codes, colors can be specified in the `RRGGBB`
format and will be translated to either truecolor (24-bit) ANSI codes or 8-bit codes
for older terminal emulators.

#### Preview

| `snazzy` | `molokai` | `ayu` |
| --- | --- | --- |
| ![snazzy theme](https://i.imgur.com/ECdQqxb.png) | ![molokai theme](https://i.imgur.com/5OiAczQ.png) | ![ayu theme](https://i.imgur.com/LC4Cx8E.png) |


#### Usage

Choose a [color theme](themes/) (for example: `molokai`). Then, add this to your shells RC file
(`~/.bashrc`, `~/.zshrc`, …):

``` bash
export LS_COLORS="$(vivid generate molokai)"
```

#### True color

By default, `vivid` runs in true color mode (24-bit). If you don't use a [terminal
that supports 24-bit colors](https://gist.github.com/XVilka/8346728), use the `--color-mode 8-bit`
option when running `vivid` (`vivid -m 8-bit generate molokai`). This will use interpolated 8-bit
colors.

## Installation

[![Build Status](https://travis-ci.org/sharkdp/vivid.svg?branch=master)](https://travis-ci.org/sharkdp/vivid)

### On Debian-based systems

Download one of the Debian packages from the [release page](https://github.com/sharkdp/vivid/releases)
and install it via `dpkg -i`:

``` bash
wget "https://github.com/sharkdp/vivid/releases/download/v0.5.0/vivid_0.5.0_amd64.deb"
sudo dpkg -i vivid_0.5.0_amd64.deb
```

### On Arch Linux

You can install `vivid` from [the official package repository](https://www.archlinux.org/packages/community/x86_64/vivid/):

``` bash
pacman -S vivid
```

### On other distributions

Check out the [release page](https://github.com/sharkdp/vivid/releases) for binary builds.

Make sure that you install the contents of the `share/vivid` folder to
`/usr/share/vivid` or `$HOME/.config/vivid` on POSIX systems and `%APPDATA%\vivid` on Windows systems.

### Via cargo

If you have Rust 1.31 or higher, you can install `vivid` from source via `cargo`:
```
cargo install vivid
```

Make sure to install the assets (database: `$srcdir/config/filetypes.yml` and themes: `$srcdir/themes`) to one of the locations listed in the ["On other distributions" section](#on-other-distributions).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Useful sources

File types:
- https://en.wikipedia.org/wiki/List_of_file_formats
- https://fileinfo.com/

ANSI colors:
- https://jonasjacek.github.io/colors/

### Similar and related projects

- https://github.com/karlding/dirchromatic
- https://github.com/trapd00r/LS_COLORS
- https://geoff.greer.fm/lscolors/
- `LS_COLORS` themes:
   - https://github.com/seebi/dircolors-solarized
   - https://github.com/ivoarch/dircolors-zenburn
   - https://github.com/arcticicestudio/nord-dircolors
   - https://github.com/peterhellberg/dircolors-jellybeans
   - https://github.com/KKPMW/dircolors-moonshine
