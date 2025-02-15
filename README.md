[![check](https://github.com/steven-omaha/pacdef/actions/workflows/check.yml/badge.svg)](https://github.com/steven-omaha/pacdef/actions/workflows/check.yml)

# pacdef

multi-backend declarative package manager for Linux


## Installation

### Arch Linux
`pacdef` is available in the AUR [as stable release](https://aur.archlinux.org/packages/pacdef) or [development version](https://aur.archlinux.org/packages/pacdef-git)
The AUR package will also provide completions for `zsh`.

### other
Install it from [crates.io](https://crates.io/crates/pacdef) using this command.
```bash
$ cargo install [-F <backend>[,...]] pacdef
```

See below ("[supported backends](#supported-backends)") for the feature flags you will need for your distribution.

To get zsh completion to work you must copy the `_completion.zsh` file to the right folder manually and rename it to `_pacdef`.

## Use-case

`pacdef` allows the user to have consistent packages among multiple Linux machines and different backends by managing packages in group files.
The idea is that (1) any package in the group files ("managed packages") will be installed explicitly, and (2) explicitly installed packages *not* found in any of the group files ("unmanaged packages") will be removed.
The group files are maintained outside of `pacdef` by any VCS, like git. 

If you work with multiple Linux machines and have asked yourself "*Why do I have the program that I use every day on my other machine not installed here?*", then `pacdef` is the tool for you.


### Of groups, sections, and packages

`pacdef` manages multiple package groups (group files) that, e.g., may be tied to a specific use-case.
Each group has one or more section(s) which correspond to a specific backend, like your system's package manager (`pacman`, `apt`, ...), or your programming languages package manger (`cargo`, `pip`, ...).
Each section contains one or more packages that can be installed respective package manager.

This image illustrates the relationship.
```
       1   n       1   n         1   n      
pacdef ----> group ----> section ----> package 
```



### Example

Let's assume you have the following group files.

`base`:

```ini
[arch]
paru
zsh

[rust]
pacdef
topgrade
```

`development`:

```ini
[arch]
rustup
rust-analyzer

[rust]
cargo-tree
flamegraph
```

Pacdef will make sure you have the following packages installed for each package manager:

- Arch (`pacman`, AUR helpers): paru, zsh, rustup, rust-analyzer
- Rust (`cargo`): pacdef, topgrade, cargo-tree, flamegraph

Note that the name of the section corresponds to the ecosystem it relates to, rather than the package manager it uses.


## Supported backends

At the moment, supported backends are the following.
Pull requests for additional backends are welcome!

| Application | Package Manager | Section     | feature flag | Notes                                                                                                     |
|-------------|-----------------|-------------|--------------|-----------------------------------------------------------------------------------------------------------|
| Arch Linux  | `pacman`        | `[arch]`    | `arch`       | includes pacman-wrapping AUR helpers (configurable)                                                       |
| Debian      | `apt`           | `[debian]`  | `debian`     | minimum supported apt-version unknown ([upstream issue](https://gitlab.com/volian/rust-apt/-/issues/20))  |
| Flatpak     | `flatpak`       | `[flatpak]` | built-in     | can manage either system-wide or per-user installation (configurable)                                     |
| Python      | `pip`           | `[python]`  | built-in     |                                                                                                           |
| Rust        | `cargo`         | `[rust]`    | built-in     |                                                                                                           |

Backends that have a `feature flag` require setting the respective flag for the build process.
The appropriate system libraries and their header files must be present on the machine and be detectable by `pkg-config`.
For backends that state "built-in", they are always supported during compile time.
Any backend can be disabled during runtime (see below, "[Configuration](#configuration)").

For example, to build `pacdef` with support for Debian Linux, you can run one of the two commands.
* (recommended) `cargo install -F debian pacdef`, this downloads and builds it from [https://crates.io](https://crates.io)
* in a clone of this repository, `cargo install --path . -F debian`

### Example

This tree shows my pacdef repository (not the `pacdef` config dir).
```
.
├── generic
│   ├── audio
│   ├── base
│   ├── desktop
│   ├── private
│   ├── rust
│   ├── wayland
│   ├── wireless
│   ├── work
│   └── xorg
├── hosts
│   ├── hostname_a
│   ├── hostname_b
│   └── hostname_c
└── pacdef.yaml
```

- The `base` group holds all packages I need unconditionally, and includes things like zfs,
  [paru](https://github.com/Morganamilo/paru) and [neovim](https://github.com/neovim/neovim).
- In `xorg` and `wayland` I have stored the respective graphic servers and DEs.
- `wireless` contains tools like `iwd` and `bluez-utils` for machines with wireless interfaces.
- Under `hosts` I have one file for each machine I use. The filenames match the corresponding hostname. The packages
  are specific to one machine only, like device drivers, or any programs I use exclusively on that machine.

Usage on different machines: 

- home server: `base private hostname_a`
- private PC: `audio base desktop private rust wayland hostname_b`
- work PC: `base desktop rust work xorg hostname_c`


## Commands

| Subcommand                        | Description                                                           |
|-----------------------------------|-----------------------------------------------------------------------|
| `group import [<path>...]`        | create a symlink to the specified group file(s) in your groups folder | 
| `group list`                      | list names of all groups                                              |  
| `group new [-e] [<group>...]`     | create new groups, use `-e` to edit them immediately after creation   | 
| `group remove [<group>...]`       | remove a previously imported group                                    |
| `group show [<group>...]`         | show contents of a group                                              |  
| `package clean [--noconfirm]`     | remove all unmanaged packages                                         |
| `package review`                  | for each unmanaged package interactively decide what to do            |
| `package search <regex>`          | search for managed packages that match the search string              |
| `package sync [--noconfirm]`      | install managed packages                                              |
| `package unmanaged`               | show all unmanaged packages                                           |
| `version`                         | show version information, supported backends                          |

### Aliases

Most subcommands have aliases. 
For example, instead of `pacdef package sync` you can write `pacdef p sy`, and `pacdef group show` would become `pacdef g s`.

Use `--help` or the zsh completion to find the right aliases.


## Configuration

On first execution, it will create an empty config file under `$XDG_CONFIG_HOME/pacdef/pacdef.yaml`.
The following key-value pairs can be set.
The listed values are the defaults.

```yaml
aur_helper: paru  # AUR helper to use on Arch Linux (paru, yay, ...)
aur_rm_args: []  # additional args to pass to AUR helper when removing packages (optional)
disabled_backends: []  # backends that pacdef should not manage, e.g. ["python"], this can reduce runtime if the package manager is notoriously slow (like pip)

warn_not_symlinks: true  # warn if a group file is not a symlink
flatpak_systemwide: true  # whether flatpak packages should be installed system-wide or per user
```


## Group file syntax

Group files loosely follow the syntax for `ini`-files.

1. Sections begin by their name in brackets.
2. One package per line. 
3. Anything after a `#` is ignored.
4. Empty lines are ignored.
5. If a package exists in multiple repositories, the repo can be specified as prefix followed by a forward slash.
   The package manager must understand this notation.

Example:
```ini
[arch]
alacritty
firefox  # this comment is ignored
libreoffice-fresh
mycustomrepo/zsh-theme-powerlevel10k

[rust]
cargo-update
topgrade
```

## Misc.

### Naming

`pacdef` combines the words "package" and "define".


### minimum supported rust version (MSRV)

MSRV is 1.65.0. Development is conducted against the latest stable version.
