pub fn raindrops(n: u32) -> String {
    let pling = if n % 3 == 0 { "Pling" } else { "" };
    let plang = if n % 5 == 0 { "Plang" } else { "" };
    let plong = if n % 7 == 0 { "Plong" } else { "" };

    if pling.len() != 0 || plang.len() != 0 || plong.len() != 0 {
        return format!("{}{}{}", pling, plang, plong);
    }

    return format!("{}", n);
}
