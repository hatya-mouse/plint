use owo_colors::OwoColorize;
use plint_linter::{Document, LintEntry, LintResult, ruleset::Severity};
use unicode_width::UnicodeWidthStr;

pub(crate) fn print_result(doc: &Document, entries: &[LintEntry], verbose: bool) {
    println!("{}", doc.file_name);

    // Collect the texts to show to calculate the maximum width
    let mut left_texts = Vec::new();
    let mut middle_texts = Vec::new();
    let mut right_texts = Vec::new();
    let mut highest_severity = None;

    for entry in entries {
        match &entry.result {
            LintResult::LinterError(error) => {
                left_texts.push("ERROR".to_string());
                middle_texts.push(to_single_line(&entry.rule_name));
                right_texts.push(format!("{}", error));
            }
            LintResult::Diagnostic {
                message,
                severity,
                match_data,
            } => {
                highest_severity = Some(
                    highest_severity.map_or(*severity, |highest: Severity| highest.max(*severity)),
                );
                if let Some(match_data) = match_data {
                    left_texts.push(format!("{}:{}", match_data.start, match_data.end));
                    middle_texts.push(format_severity_colored(severity));
                    right_texts.push(to_single_line(message));
                } else {
                    left_texts.push("".to_string());
                    middle_texts.push(format_severity_colored(severity));
                    right_texts.push(to_single_line(message));
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

    // Print the highest severity when verbose is specified
    if verbose && let Some(highest_severity) = highest_severity {
        println!(
            "  Highest severity: {}\n",
            format_severity_colored(&highest_severity)
        );
    }

    // Print the texts using max widths
    for ((left, middle), right) in left_texts.iter().zip(middle_texts).zip(right_texts) {
        println!(
            "  {:<left_width$}  {:<middle_width$}  {}",
            left, middle, right
        );
    }
}

fn format_severity_colored(severity: &Severity) -> String {
    match severity {
        Severity::Error => {
            format!("{}", "error".bold().red())
        }
        Severity::Warning => {
            format!("{}", "warning".bold().yellow())
        }
        Severity::Advisory => {
            format!("{}", "advisory".bold().purple())
        }
        Severity::Info => {
            format!("{}", "info".bold())
        }
    }
}

fn to_single_line(message: &str) -> String {
    message.replace("\n", " ")
}
