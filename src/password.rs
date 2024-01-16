use passwords::{analyzer, scorer, PasswordGenerator};
use tui_textarea::TextArea;

pub fn generate_strong_password(length: usize) -> String {
    // uses thread_rng which is marked as cryptographically secure, see:
    // https://rust-random.github.io/book/guide-rngs.html
    let generator = PasswordGenerator::new()
        .length(length)
        .numbers(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .strict(true);

    generator
        .generate_one()
        .expect("Failed to generate Password")
}

pub fn validate_password_strength(textarea: &mut TextArea) -> (Option<String>, u32) {
    // Returns an error if password not strong enough otherwise nothing
    // Returns an integer with an external password score
    let input = textarea.lines()[0].clone();
    let score = scorer::score(&analyzer::analyze(&input)).floor() as u32;

    if let Some(mut strength) = password_strength(&input) {
        return (Some(strength), score);
    }
    (None, score)
}

fn password_strength(password: &String) -> Option<String> {
    // checks password if requirements are fulfilled
    if process_letters(password, is_numeric) {
        Some(String::from("Password needs one numerical character"))
    } else if process_letters(password, is_lower) {
        Some(String::from("Password needs one lowercase character"))
    } else if process_letters(password, is_upper) {
        Some(String::from("Password needs one uppercase character"))
    } else if process_letters(password, is_special) {
        Some(String::from("Password needs one special character"))
    } else if password.len() < 10 {
        Some(String::from("Password needs 10 or more characters"))
    } else if password.len() > 255 {
        Some(String::from("Maybe choose a shorter password..."))
    } else {
        None
    }
}

fn process_letters<F>(input: &str, check: F) -> bool
where
    F: Fn(&char) -> bool,
{
    // processes a strings characters with a provided function
    for char in input.chars() {
        if check(&char) {
            return false;
        }
    }
    true
}

fn is_numeric(input: &char) -> bool {
    input.is_numeric()
}

fn is_upper(input: &char) -> bool {
    input.is_uppercase()
}

fn is_lower(input: &char) -> bool {
    input.is_lowercase()
}

fn is_special(input: &char) -> bool {
    !input.is_alphanumeric()
}
