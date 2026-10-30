<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/hatya-mouse/plint/main/assets/plint_logo_white.png">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/hatya-mouse/plint/main/assets/plint_logo_black.png">
        <img src="https://raw.githubusercontent.com/hatya-mouse/plint/main/assets/plint_logo_black.png" width="320px" alt="Description">
    </picture>
</div>

---

`plint` is a linter for text files.
It finds errors and warnings defined in the rulesets. Rulesets are written in a custom language designed for linting.

[![asciicast](./assets/plint.gif)](https://asciinema.org/a/1270537)
[Watch on asciinema](https://asciinema.org/a/1270537)

## Ruleset

A ruleset is defined in YAML file like below. It has some metadata and the actual list of rules.

```yaml
name: ruleset
authors: hatya-mouse
description: An example ruleset
version: 1
creation_time: 2026-10-31T00:56:38.015284Z
rules: []
```

A rule has `name`, `message`, `severity` and `code` fields.
`message` is a message that is shown when the code returns matches.
`severity` can only be either `info`, `advisory`, `warning` and `error`.

An actual rule definition may look like below:

```yaml
- name: todo
  message: The file contains TODO.
  severity: error
  code: |
    for word in words(doc) (
        if contains(doc, word, "TODO") (
            word
        )
    )
```

The condition of the rule is defined using a custom language.

Detailed explanation of the language is available [here](./language.md).

## Quickstart

If you **have `cargo` installed**, run the following command to install the latest version of `plint`:

```bash
cargo install plint
```

If you **use Linux or macOS** but **don't have `cargo` installed**, run the install script using the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/hatya-mouse/plint/main/install.sh | sh
```

If you **use neither Linux nor maxOS** and **don't have `cargo` installed**,
download the compiled binary from [Releases](https://github.com/hatya-mouse/plint/releases/).

## Features

- Find errors & warnings in your text files
- Support for Markdown files
- Define rules in a custom linting language

## Credits

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
