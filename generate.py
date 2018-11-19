#!/usr/bin/env python

import sys
import os
import yaml
import argparse

SCRIPT_DIR = os.path.dirname(os.path.realpath(__file__))

ANSI_STYLES = {
    "regular": 0,
    "bold": 1,
    "faint": 2,
    "italic": 3,
    "underline": 4,
    "blink": 5,
    "rapid-blink": 6,
    "overline": 53
}


def warning(msg):
    sys.stderr.write(f"Warning: {msg}\n")


def load_filetypes():
    path = os.path.join(SCRIPT_DIR, "filetypes.yml")
    return yaml.load(open(path, "r"))


def get_code(ext):
    if ext[0] == "$":
        return ext[1:]
    else:
        return "*" + ext


def get_mapping(value, path=[]):
    mapping = {}

    if type(value) == list:
        for ext in value:
            code = get_code(ext)
            mapping[code] = path
    elif type(value) == dict:
        for key, child in value.items():
            child_path = path + [key]
            mapping.update(get_mapping(child, path=child_path))
    else:
        raise Exception(f"wrong type in yaml file: {type(value)}")

    return mapping


def load_theme(name):
    theme_file = os.path.join(SCRIPT_DIR, "themes", name + ".yml")
    theme = yaml.load(open(theme_file))

    return theme


def rgb_from_hex(hex_str):
    r = int(hex_str[0:2], 16)
    g = int(hex_str[2:4], 16)
    b = int(hex_str[4:6], 16)
    return r, g, b


def get_color(theme, path):
    assert len(path) > 0

    item = theme
    for key in path:
        if type(item) == dict and \
           ("foreground" in item or "background" in item or "style" in item):
            break

        try:
            item = item[key]
        except KeyError:
            warning("could not resolve path '{}'".format("/".join(path)))
            return "0"

    style = item.get("style", "regular")
    fonttype = ANSI_STYLES[style]
    foreground = theme["colors"].get(item.get("foreground", "default"), "ffffff")  # TODO
    background = theme["colors"].get(item.get("background", None), None)  # TODO

    r, g, b = rgb_from_hex(foreground)
    style = f"{fonttype};38;2;{r};{g};{b}"

    if background:
        r, g, b = rgb_from_hex(background)
        style += f";48;2;{r};{g};{b}"

    return style


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generates LS_COLORS expressions")
    parser.add_argument("--theme", dest="theme", required=True,
                        help="Name of the color theme")

    args = parser.parse_args()

    filetypes = load_filetypes()
    mapping = get_mapping(filetypes)

    theme = load_theme(args.theme)

    ls_colors = []

    def key_length(item): return len(item[0])

    for ext, path in sorted(mapping.items(), key=key_length):
        ls_colors.append("{}={}".format(ext, get_color(theme, path)))

    print(":".join(ls_colors))
