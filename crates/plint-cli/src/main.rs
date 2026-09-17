use plint_linter::ruleset::Ruleset;

fn main() {
    let condition = plint_linter::Condition::Not(Box::new(plint_linter::Condition::Or(vec![
        plint_linter::Condition::Lt("2".to_string()),
        plint_linter::Condition::Gt("100".to_string()),
    ])));
    let yaml_repr = yaml_serde::to_string(&condition).unwrap();
    println!("{}", yaml_repr);

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

  - id: say-shorter
    message: Saying things shorter is important.
    severity: error
    checker: std.word-count
    condition:
        gt: 3

  - id: too-looooong
    message: The page is too long. Don't let me scroll so much.
    severity: error
    checker: std.line-count
    condition:
        gt: 4
"#;

    let ruleset = Ruleset::from_yaml(ruleset_yaml).unwrap();
    let results = ruleset.lint(&doc);

    println!("{:#?}", results);
}
