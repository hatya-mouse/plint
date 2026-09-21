use plint_linter::{Document, LintEntry, LintResult};
use unicode_width::UnicodeWidthStr;

pub(crate) fn print_result(doc: &Document, entries: &[LintEntry]) {
    println!("{}", doc.file_name);

    // Collect the texts to show to calculate the maximum width
    let mut left_texts = Vec::new();
    let mut middle_texts = Vec::new();
    let mut right_texts = Vec::new();

    for entry in entries {
        match &entry.result {
            LintResult::LinterError(error) => {
                left_texts.push("ERROR".to_string());
                middle_texts.push(entry.rule_name.clone());
                right_texts.push(format!("{}", error));
            }
            LintResult::Diagnostic {
                message,
                severity,
                match_data,
            } => {
                if let Some(match_data) = match_data {
                    left_texts.push(format!("{}:{}", match_data.start, match_data.end));
                    middle_texts.push(format!("{}", severity));
                    right_texts.push(message.clone());
                } else {
                    left_texts.push("".to_string());
                    middle_texts.push(format!("{}", severity));
                    right_texts.push(message.clone());
                }
            }
        }
    }

    let left_width = left_texts
        .iter()
        .map(|text| text.width())
        .max()
        .unwrap_or_default();
    let middle_width = middle_texts
        .iter()
        .map(|text| text.width())
        .max()
        .unwrap_or_default();

    // Print the texts using max widths
    for ((left, middle), right) in left_texts.iter().zip(middle_texts).zip(right_texts) {
        println!(
            "  {:<left_width$}  {:<middle_width$}  {}",
            left, middle, right
        );
    }
}
