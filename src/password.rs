use tui_textarea::TextArea;


pub fn validate_password_strength(textarea: &mut TextArea) -> Option<String> {
    password_strength(textarea.lines()[0].clone())
}

fn password_strength(password: String) -> Option<String> {
     if process_letters(&password, is_numeric) {
        Some(String::from("Password needs one numerical character"))
    } else if process_letters(&password, is_lower) {
        Some(String::from("Password needs one lowercase character"))
    } else if process_letters(&password, is_upper) {
        Some(String::from("Password needs one uppercase character"))
    } else if process_letters(&password, is_special) {
        Some(String::from("Password needs one special character"))
    } else if password.len() < 10 {
        Some(String::from("Password too short"))
    } else {
        None
    }
}

fn process_letters<F>(input: &String, check: F) -> bool where F: Fn(&char) -> bool {
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