use inquire::{CustomUserError, validator::Validation};

pub(crate) fn name_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.chars().count() > 256 {
        Ok(Validation::Invalid(
            "Name must be 256 characters or less".into(),
        ))
    } else if input.is_empty() {
        Ok(Validation::Invalid("Name must not be empty".into()))
    } else if input
        .chars()
        .any(|char| !char.is_ascii_alphanumeric() && char != '-')
    {
        Ok(Validation::Invalid(
            "Name must only contain alphabets, numbers and hyphens".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
}

pub(crate) fn version_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.trim().is_empty() {
        return Ok(Validation::Valid);
    }

    match input.trim().parse::<u64>() {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid("Input must be a valid integer".into())),
    }
}
