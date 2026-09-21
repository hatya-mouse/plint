# `plint`

`plint` is a linter for text files. You can define your own rules using the custom language designed for linting.

## Features

- Lint text files in your terminal
- Write "rulesets" with human-readable YAML files
- Express complex rules using the linting language

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
