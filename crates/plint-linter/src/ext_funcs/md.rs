use plint_lang::{Interpreter, Value};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

pub(super) fn add_funcs(interpreter: &mut Interpreter) {
    interpreter.add_func("md.headings", Box::new(md_headings));
    interpreter.add_func("md.paragraphs", Box::new(md_paragraphs));
}

fn md_headings(args: &[Value]) -> Result<Value, String> {
    let (string, desired_level): (&str, Option<i64>) = match args {
        [Value::String(text)] => (text, None),
        [Value::String(text), Value::Integer(level)] => (text, Some(*level)),
        [Value::String(text), Value::Match(m), Value::Integer(level)] => match text.get(*m) {
            Some(substring) => (substring, Some(*level)),
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for md.headings function".to_string()),
    };

    let mut matches = Vec::new();
    let iter = Parser::new(string).into_offset_iter();

    let mut heading_start = None;
    for (event, range) in iter {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                heading_start = Some(range.start);
            }
            Event::End(TagEnd::Heading(level)) => {
                if let Some(desired_level) = desired_level {
                    let is_desired_level = match level {
                        HeadingLevel::H1 => desired_level == 1,
                        HeadingLevel::H2 => desired_level == 2,
                        HeadingLevel::H3 => desired_level == 3,
                        HeadingLevel::H4 => desired_level == 4,
                        HeadingLevel::H5 => desired_level == 5,
                        HeadingLevel::H6 => desired_level == 6,
                    };

                    if !is_desired_level {
                        heading_start = None;
                        continue;
                    }
                }

                let Some(heading_start) = heading_start.take() else {
                    continue;
                };

                matches.push(Value::Match((heading_start..range.end).into()));
            }
            _ => (),
        }
    }

    Ok(Value::List(matches))
}

fn md_paragraphs(args: &[Value]) -> Result<Value, String> {
    let string: &str = match args {
        [Value::String(text)] => text,
        [Value::String(text), Value::Match(m)] => match text.get(*m) {
            Some(substring) => substring,
            None => return Ok(Value::Null),
        },
        _ => return Err("Invalid arguments for md.paragraphs function".to_string()),
    };

    let mut matches = Vec::new();
    let iter = Parser::new(string).into_offset_iter();

    let mut para_start = None;
    for (event, range) in iter {
        match event {
            Event::Start(Tag::Paragraph) => {
                para_start = Some(range.start);
            }
            Event::End(TagEnd::Paragraph) => {
                if let Some(para_start) = para_start.take() {
                    matches.push(Value::Match((para_start..range.end).into()));
                }
            }
            _ => (),
        }
    }

    Ok(Value::List(matches))
}
