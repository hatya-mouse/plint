# The Linting Language

## Basics

All statement in the language can return values, including `for` and `if-else`.

`if-else` statement does not require that the types of the return values are the same. You can also return value in `if` statement without `else`.

This is possible because they return `null` by default.

This mean that these two programs are the same:

*Program 1*
```rust
if gt(value, 5) (
    "Greater than 5"
)
```

*Program 2*
```rust
if gt(value, 5) (
    "Greater than 5"
) else (
    null
)
```

`for` statement loops over a given array.

Unlike `if` that returns `null` if a return value is not specified, `for` statement collects non-`null` values and return them as an array.
If the statement inside `for` only returns `null`, outer `for` returns an empty array.

```rust
for heading in md.headings(doc) (
    if gt(word_count(doc, heading), 15) (
        heading
    )
)
```

## Returning Errors

A program used for linting is supposed to return an array of `Match`es.

`Match` type only holds the range in the document. For example, in the program below, a function called `sentences` return an array of `Match`es.

Note that `doc` variable contains a string of the entire file to be linted.

```rust
for sentence in sentences(doc) (
    if gt(word_count(doc, sentence), 40) (
        sentence
    )
)
```

You may need to pass the `doc` variable to some functions, as `Match` does not contain the string itself, but the range.

## Sample Programs

```rust
for word in words(doc) (
    if contains(doc, word, "TODO") (
        word
    )
)
```

This program loops over all words in the document, checks if the word contains "TODO", and returns the range of the word.

You can also write complex rules using variables:

```rust
last_heading_level = 0
for heading in md.headings(doc) (
    heading_level = len(regex(doc, heading, "#"))
    heading_diff = sub(heading_level, last_heading_level)

    last_heading_level = heading_level

    if gt(heading_diff, 1) (
        heading
    )
)
```

This program keeps tracking the heading level of the previous heading, and returns the range of the heading if the level is skipped.
