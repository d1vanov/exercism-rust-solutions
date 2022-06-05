fn all_uppercase(message: &str) -> bool {
    let mut uppercase_found = false;
    for c in message.chars() {
        if c.is_ascii_punctuation() {
            continue;
        }

        if c.is_whitespace() {
            continue;
        }

        if c.is_ascii_digit() {
            continue;
        }

        if c.is_lowercase() {
            return false;
        }

        uppercase_found = true;
    }

    uppercase_found
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.len() > 0 {
        if all_uppercase(message) {
            if message.ends_with('?') {
                return "Calm down, I know what I'm doing!";
            }
            else {
                return "Whoa, chill out!";
            }
        }

        if message.ends_with('?') {
            return "Sure.";
        }
    }
    else {
        return "Fine. Be that way!";
    }

    return "Whatever.";
}
