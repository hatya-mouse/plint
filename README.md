# plint

`plint` is a linter for text files.
It finds errors and warnings defined in the rulesets. Rulesets are written in a custom language designed for linting.

![Demo GIF Animation](https://raw.githubusercontent.com/hatya-mouse/plint/main/assets/demo.gif)

# Quickstart

**If you have `cargo` installed**, run the following command to install the latest version of `plint`:

```bash
cargo install plint
```

**If you use Linux or macOS but don't have `cargo` installed**, run the install script using the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/hatya-mouse/plint/main/install.sh | sh
```

**If you use neither Linux nor maxOS and don't have `cargo` installed**,
download the compiled binary from [Releases](https://github.com/hatya-mouse/plint/releases/).

# Features

- Find errors & warnings in your text files
- Support for Markdown files
- Define rules in a custom linting language

# Credits

Here are some of the crates I used for plint:

| Name | Description |
| - | - |
| clap | Rust crate to parse subcommands and arguments easily and show helps |
| inquire | Asks if the user is sure when removing rulesets & groups |
| nom | Used to parse custom linting language |
| owo-colors | Colorizes the terminal output by adding ANSI color instructions |
| regex | Used for getting the matches of the given regex pattern |
| serde | Used together with yaml_serde to serialize and deserialize the rulesets and groups |
| unicode-segmentation | Counts characters (considering multi-bytes characters), words, and sentences |
| yaml_serde | Used to load and save the rulesets and groups from/to the disk in YAML format |
