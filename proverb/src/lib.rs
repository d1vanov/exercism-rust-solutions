pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut proverb = String::new();
    for (i, _) in list.iter().enumerate() {
        if i == list.len() - 1 {
            break;
        }
        proverb += format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str();
    }

    proverb += "And all for the want of a ";
    proverb += list[0];
    proverb += ".";
    proverb
}
