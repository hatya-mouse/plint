use plint_linter::ruleset::Ruleset;

fn main() {
    let doc = plint_linter::Document::new(
        "hoge.txt".to_string(),
        r#"
            hello
            helloooo

            hello
            hello hello
        "#
        .to_string(),
    );

    let ruleset_yaml = r#"
name: basic-ruleset
description: This is a basic ruleset.
version: 1

rules:
  - id: no-hello
    message: '"hello" is not allowed.'
    severity: error
    checker: std.regex
    args:
        pattern: hello

  - id: invalid-hello
    message: '"helloooo" is not allowed.'
    severity: error
    checker: std.regex
    args:
        pattern: helloooo
"#;

    let ruleset = Ruleset::from_yaml(ruleset_yaml).unwrap();
    let results = ruleset.lint(&doc);

    println!("{:#?}", results);
}
