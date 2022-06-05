pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut brackets: Vec<char> = Vec::new();

    for c in string.chars() {
        if c == '(' || c == '{' || c == '[' {
            brackets.push(c);
            continue;
        }

        if c != ')' && c != '}' && c != ']' {
            continue;
        }

        if let Some(top) = brackets.last() {
            if c == ')' && *top != '(' {
                return false;
            }

            if c == '}' && *top != '{' {
                return false;
            }

            if c == ']' && *top != '[' {
                return false;
            }

            brackets.remove(brackets.len() - 1);
        }
        else {
            return false;
        }
    }

    brackets.is_empty()
}
