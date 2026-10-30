# The Linting Language

```
for word in words(doc) (
    if contains(doc, word, "TODO") (
        word
    )
)
```

This program loops over all words in the document, checks if the word contains "TODO", and returns the range of the word.

You can also write complex rules using variables:

```
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
