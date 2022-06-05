fn bottle_part(n: i32) -> String {
    if n == 0 {
        return "No more bottles".to_string();
    } else if n == 1 {
        return "1 bottle".to_string();
    } else if n < 0 {
        return "".to_string();
    }

    return format!("{} bottles", n);
}

fn second_line_first_part(n: i32) -> String {
    if n < 0 {
        "Go to the store and buy some more".to_string()
    } else if n < 1 {
        "Take it down and pass it around".to_string()
    } else {
        "Take one down and pass it around".to_string()
    }
}

pub fn verse(n: u32) -> String {
    let current_bottle_part = bottle_part(n as i32);
    let next_bottle_first_part = second_line_first_part(n as i32 - 1);

    let mut next_num_for_next_bottle_second_part = n as i32 - 1;
    if n == 0 {
        next_num_for_next_bottle_second_part = 99;
    }

    let next_bottle_second_part = bottle_part(next_num_for_next_bottle_second_part);
    return format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        current_bottle_part,
        current_bottle_part.to_lowercase(),
        next_bottle_first_part,
        next_bottle_second_part.to_lowercase())
}

pub fn sing(start: u32, end: u32) -> String {
    if start > 99 || start < end {
        return "".to_string();
    }

    let mut result = String::new();
    for i in 0..(start - end + 1) {
        result += &verse(start - i);
        if i != start - end {
            result += "\n";
        }
    }

    return result;
}
