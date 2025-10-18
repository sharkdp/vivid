# vivid

[![CICD](https://github.com/sharkdp/vivid/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/vivid/actions/workflows/CICD.yml)
![Crates.io](https://img.shields.io/crates/v/vivid)

*vivid* is a generator for the **`LS_COLORS`** environment variable that controls the colorized output of
[`ls`](https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html#ls-invocation), [`tree`](http://mama.indstate.edu/users/ice/tree/),
[`fd`](https://github.com/sharkdp/fd), [`bfs`](https://github.com/tavianator/bfs), [`dust`](https://github.com/bootandy/dust) and many other tools.

It uses a YAML configuration format for the [filetype-database](config/filetypes.yml)
and the [color themes](themes/). In contrast to
[`dircolors`](https://www.gnu.org/software/coreutils/manual/html_node/dircolors-invocation.html#dircolors-invocation),
the database and the themes are organized in different files. This allows users to
choose and customize color themes independent from the collection of file extensions.
Instead of using cryptic ANSI escape codes, colors can be specified in the `RRGGBB`
format and will be translated to either truecolor (24-bit) ANSI codes or 8-bit codes
for older terminal emulators.

## Preview

| `snazzy` | `molokai` | `ayu` |
| --- | --- | --- |
| ![snazzy theme](https://i.imgur.com/ECdQqxb.png) | ![molokai theme](https://i.imgur.com/5OiAczQ.png) | ![ayu theme](https://i.imgur.com/LC4Cx8E.png) |

| `lava` |
| --- |
| ![lava](https://user-images.githubusercontent.com/702227/124368181-77caa700-dc56-11eb-8286-95283e9a2b04.png) |


## Usage

Choose a [color theme](themes/) (for example: `molokai`). Then, add this to your shells RC file
(`~/.bashrc`, `~/.zshrc`, …):

``` bash
export LS_COLORS="$(vivid generate molokai)"
```

or for Fish:

```fish
set -gx LS_COLORS "$(vivid generate molokai)"
```

### Theme preview

To try all available themes with your current directory:

``` bash
for theme in $(vivid themes); do
    echo "Theme: $theme"
    LS_COLORS=$(vivid generate $theme)
    ls
    echo
done
```

### Terminals without true color support

By default, `vivid` runs in true color mode (24-bit). If you don't have a [terminal
that supports 24-bit colors](https://gist.github.com/XVilka/8346728), use the `--color-mode 8-bit`
option when running `vivid`. This will generate interpolated 8-bit colors:
``` bash
export LS_COLORS="$(vivid -m 8-bit generate molokai)"
```

### Customization

Custom [`filetypes.yml` databases](config/filetypes.yml) can be placed in `/usr/share/vivid`, `$HOME/.config/vivid`, or `$XDG_CONFIG_HOME/vivid` on POSIX systems,
or in `%APPDATA%\vivid` on Windows systems.

Custom color themes go into a `themes` subfolder, respectively.  You can also specify an explicit path to your custom theme: `vivid generate path/to/my_theme.yml`.
As a starting point, you can use one of the [bundled themes](themes/).


## Installation

### On Debian-based systems

Download one of the Debian packages from the [release page](https://github.com/sharkdp/vivid/releases)
and install it via `dpkg -i`:

``` bash
wget "https://github.com/sharkdp/vivid/releases/download/v0.10.1/vivid_0.10.1_amd64.deb"
sudo dpkg -i vivid_0.10.1_amd64.deb
```

### On Arch Linux

You can install `vivid` from [the official package repository](https://www.archlinux.org/packages/community/x86_64/vivid/):

``` bash
pacman -S vivid
```

### On Gentoo Linux

You can install `vivid` from [GURU Overlay](https://wiki.gentoo.org/wiki/Project:GURU/Information_for_End_Users)

``` bash
emerge vivid
```

### On FreeBSD

You can install `vivid` from [the FreeBSD Ports Collection](https://www.freshports.org/sysutils/vivid/):

``` bash
pkg install vivid
```

### On macOS

You can install `vivid` from [Homebrew](https://github.com/Homebrew/homebrew-core/blob/HEAD/Formula/vivid.rb):

``` bash
brew install vivid
```

Note that the BSD version of `ls` does not use `LS_COLORS`, but you can use the GNU version of `ls` instead:
```bash
brew install coreutils
alias ls="gls --color"
```

### On other distributions

Check out the [release page](https://github.com/sharkdp/vivid/releases) for binary builds.

### From source

If you have Rust 1.54 or higher, you can install `vivid` from source via `cargo`:
``` bash
cargo install vivid
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Useful resources

File types:
- https://en.wikipedia.org/wiki/List_of_file_formats
- https://fileinfo.com/

ANSI colors:
- https://jonasjacek.github.io/colors/

Similar and related projects:

- https://github.com/karlding/dirchromatic
- https://github.com/trapd00r/LS_COLORS
- https://geoff.greer.fm/lscolors/
- `LS_COLORS` themes:
   - https://github.com/seebi/dircolors-solarized
   - https://github.com/ivoarch/dircolors-zenburn
   - https://github.com/arcticicestudio/nord-dircolors
   - https://github.com/peterhellberg/dircolors-jellybeans
   - https://github.com/KKPMW/dircolors-moonshine
