// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base = 221f64 * (speed as f64);
    if speed >= 1 && speed <= 4 {
        return base;
    }
    else if speed >= 5 && speed <= 8 {
        return base * 0.9;
    }
    else if speed >= 9 && speed <= 10 {
        return base * 0.77;
    }
    return 0.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60f64) as u32
}
