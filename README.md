# `plint`

`plint` is a linter for text files. You can define your own rules using the custom language designed for linting.

## Features

- Lint text files in your terminal
- Write "rulesets" with human-readable YAML files
- Express complex rules using the linting language

## Quickstart

You can lint the text files using the following command:

```bash
plint lint [FILE]...
plint lint [FILE]... --rulesets [RULESET]...
```

Without `--rulesets` argument, all the installed rulesets will be used for linting.

## Linting Language

I designed my original linting language to express complex rules as short as possible.

```swift
for sentence in sentences(doc) (
    length = char_count(doc, sentence)
    words = word_count(doc, sentence)
    has_hedging = or(
        contains(doc, sentence, "I think"),
        contains(doc, sentence, "I believe"),
        contains(doc, sentence, "perhaps"),
        contains(doc, sentence, "probably")
    )

    if and(
        gt(length, 100),
        gt(words, 15),
        has_hedging
    ) (
        sentence
    )
)
```

In this language, all statements are expressions that return values. Variables can have value of any type.

## Rulesets

You can write rulesets for `plint` easily.

To create a new ruleset, run the following command:

```bash
plint ruleset create [NAME]
```

Then, run the following command to open the YAML ruleset file in your default editor:

```bash
plint ruleset edit [NAME]
```

You can remove the both groups and the rulesets using the following command:

```bash
plint remove [TARGET]...
```

`[TARGET]` can be both a group and a ruleset.

## Group

`plint` supports grouping multiple rulesets into a single group to organize your rulesets and use them easily in `lint` command.

You can create groups using the following command:

```bash
plint group create [NAME]
```

To add or remove rulesets to/from the group, run the following command:

```bash
plint group add-set [NAME] --rulesets [RULESET]...
plint group remove-set [NAME] --rulesets [RULESET]...
```
