# dircolors-hd

**WORK IN PROGRESS**

A generator for `LS_COLORS` expressions similar to `dircolors`. It uses a YAML-based for the
filetype-database as well as for the color themes.

Usage:
``` bash
export LS_COLORS="$(python generate.py --theme molokai)"
```

![Preview of "ls -R"](https://i.imgur.com/oekLIya.png)

## Current limitations

Only works on terminals with truecolor (24-bit) support. Adding 8-bit support would
certainly be possible by using an approximation scheme such as the one we use in
[bat](https://github.com/sharkdp/bat) (https://crates.io/crates/ansi_colours).

For a list of truecolor terminals, see [this article](https://gist.github.com/XVilka/8346728).

### Similar and related projects

- https://github.com/karlding/dirchromatic
- https://github.com/trapd00r/LS_COLORS
- https://geoff.greer.fm/lscolors/
- Themes:
   - https://github.com/seebi/dircolors-solarized
   - https://github.com/ivoarch/dircolors-zenburn
   - https://github.com/arcticicestudio/nord-dircolors
