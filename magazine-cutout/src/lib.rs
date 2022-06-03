// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    for a in magazine {
        magazine_map.entry(a).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut note_map: HashMap<&str, u32> = HashMap::new();
    for a in note {
        note_map.entry(a).and_modify(|v| *v += 1).or_insert(1);
    }

    for (note_key, note_value) in note_map.iter() {
        match magazine_map.entry(note_key) {
            Entry::Occupied(a) => {
                if a.get() < note_value {
                    return false;
                }
            },
            Entry::Vacant(_) => {
                return false;
            }
        }
    }

    return true;
}
