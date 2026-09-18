use std::collections::HashMap;

pub(crate) fn args_editor(map: &mut HashMap<String, plint_linter::Value>) {
    inquire::Select::new("", options)
}

pub(crate) fn value_editor(key: String, value: &mut plint_linter::Value) {
    match value {
        plint_linter::Value::String(s) => {
            if let Ok(new_value) = inquire::Editor::new(&key).with_predefined_text(s).prompt() {
                *s = new_value;
            }
        }
        plint_linter::Value::Integer(int) => {
            if let Ok(new_int) = inquire::CustomType::<i64>::new(&key)
                .with_starting_input(&int.to_string())
                .prompt()
            {
                *int = new_int;
            }
        }
        plint_linter::Value::Float(int) => {
            if let Ok(new_int) = inquire::CustomType::<f64>::new(&key)
                .with_starting_input(&int.to_string())
                .prompt()
            {
                *int = new_int;
            }
        }
        plint_linter::Value::Boolean(b) => {
            let new_value = inquire::Select::new(
                &format!("Edit value for key '{}':", key),
                vec!["true", "false"],
            )
            .with_starting_cursor(if *b { 0 } else { 1 })
            .prompt();
            if let Ok(new_value) = new_value {
                *b = new_value == "true";
            }
        }
        _ => {
            println!("Editing this type of value is not supported.");
        }
    }
}

enum MapEditorAction {
    Back,
    EditKey,
    EditValue,
    AddEntry,
    RemoveEntry,
}
