# vivid

[![Build Status](https://travis-ci.org/sharkdp/vivid.svg?branch=master)](https://travis-ci.org/sharkdp/vivid)

A manager for `LS_COLORS` expressions. In contrast to `dircolors`, it uses a YAML-based
configuration file for the [filetype-database](config/filetypes.yml) and the [color
themes](themes/molokai.yml). Colors can be specified in `#RRGGBB` format instead of cryptic ANSI
codes.

**Usage:**
``` bash
export LS_COLORS="$(vivid generate filetypes.yml --theme themes/molokai.yml)"
```

![Preview of "ls -R"](https://i.imgur.com/oekLIya.png)

## True color

By default, `vivid` runs in true color mode (24-bit). If you don't use a [terminal
that supports 24-bit colors](https://gist.github.com/XVilka/8346728), use the `--color-mode 8-bit`
option when running `vivid`.

## Installation

### On Debian-based systems

``` bash
wget "https://github.com/sharkdp/vivid/releases/download/v0.3.0/vivid_0.3.0_amd64.deb"
sudo dpkg -i vivid_0.3.0_amd64.deb
```

### On other distrubutions

Check out the [release page](https://github.com/sharkdp/vivid/releases) for binary builds.

### Via cargo

If you have Rust 1.30 or higher, you can install `vivid` from source via `cargo`:
```
cargo install vivid
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Similar and related projects

- https://github.com/karlding/dirchromatic
- https://github.com/trapd00r/LS_COLORS
- https://geoff.greer.fm/lscolors/
- `LS_COLORS` themes:
   - https://github.com/seebi/dircolors-solarized
   - https://github.com/ivoarch/dircolors-zenburn
   - https://github.com/arcticicestudio/nord-dircolors
