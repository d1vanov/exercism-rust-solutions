// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health >= 1 {
            return None;
        }

        let mut mana = None;
        if self.level >= 10 {
            mana = Some(100u32);
        }

        return Some(Player{health: 100, mana, level: self.level});
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;
                }
                else {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                }
            },
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                }
                else {
                    self.health -= mana_cost;
                }
                return 0;
            }
        }
    }
}
