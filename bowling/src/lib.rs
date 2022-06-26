#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum FrameResult {
    Open(u16, u16),
    Spare(u16),
    Strike,
    FinalFrameWithBonus(u16, u16, u16),
}

#[derive(PartialEq)]
enum NextThrow {
    First,
    Second,
    Bonus,
}

pub struct BowlingGame {
    bonus_throw: bool,
    first_throw_pins: u16,
    second_throw_pins: u16,
    remaining_pins: u16,
    next_throw: NextThrow,
    frame_results: Vec<FrameResult>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            bonus_throw: false,
            first_throw_pins: 0,
            second_throw_pins: 0,
            remaining_pins: 10,
            next_throw: NextThrow::First,
            frame_results: Vec::with_capacity(10),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.remaining_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame_results.len() == 10 {
            return Err(Error::GameComplete);
        }

        let is_final_frame = self.frame_results.len() == 9;

        match self.next_throw {
            NextThrow::First => {
                if pins == 10 && is_final_frame {
                    self.bonus_throw = true;
                    self.first_throw_pins = 10;
                    self.remaining_pins = 10;
                    self.next_throw = NextThrow::Second;
                } else if pins == 10 {
                    self.frame_results.push(FrameResult::Strike);
                    self.first_throw_pins = 0;
                    self.remaining_pins = 10;
                    self.next_throw = NextThrow::First;
                } else {
                    self.first_throw_pins = pins;
                    self.remaining_pins -= pins;
                    self.next_throw = NextThrow::Second;
                }
            }
            NextThrow::Second => {
                let total = self.first_throw_pins + pins;
                if total == 10 && is_final_frame {
                    self.bonus_throw = true;
                }

                if self.bonus_throw {
                    self.second_throw_pins = pins;
                    if pins == self.remaining_pins {
                        self.remaining_pins = 10;
                    } else {
                        self.remaining_pins -= pins;
                    }
                    self.next_throw = NextThrow::Bonus;
                } else if total == 10 {
                    self.frame_results
                        .push(FrameResult::Spare(self.first_throw_pins));
                    self.first_throw_pins = 0;
                    self.next_throw = NextThrow::First;
                    self.remaining_pins = 10;
                } else {
                    self.frame_results
                        .push(FrameResult::Open(self.first_throw_pins, pins));
                    self.first_throw_pins = 0;
                    self.next_throw = NextThrow::First;
                    self.remaining_pins = 10;
                }
            }
            NextThrow::Bonus => {
                if pins > self.remaining_pins {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.frame_results.push(FrameResult::FinalFrameWithBonus(
                    self.first_throw_pins,
                    self.second_throw_pins,
                    pins,
                ));
            }
        }
        Ok(())
    }

    fn get_next_roll(&self, current_position: usize, frame_results: &[FrameResult]) -> u16 {
        let next_position = current_position + 1;
        match frame_results[next_position] {
            FrameResult::Open(pins, _) => pins,
            FrameResult::Spare(pins) => pins,
            FrameResult::Strike => 10,
            FrameResult::FinalFrameWithBonus(pins, _, _) => pins,
        }
    }

    fn get_next_two_rolls(&self, current_position: usize, frame_results: &[FrameResult]) -> u16 {
        let next_position = current_position + 1;
        match frame_results[next_position] {
            FrameResult::Open(first, second) => first + second,
            FrameResult::Spare(_) => 10,
            FrameResult::Strike => 10 + self.get_next_roll(next_position, frame_results),
            FrameResult::FinalFrameWithBonus(first, second, _) => first + second,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_results.len() < 10 {
            return None;
        }

        let mut total = 0;
        for (index, frame_result) in self.frame_results.iter().enumerate() {
            let frame_score = match frame_result {
                FrameResult::Open(first, second) => first + second,
                FrameResult::Spare(_) => 10 + self.get_next_roll(index, &self.frame_results),
                FrameResult::Strike => 10 + self.get_next_two_rolls(index, &self.frame_results),
                FrameResult::FinalFrameWithBonus(first, second, bonus) => first + second + bonus,
            };
            total += frame_score
        }
        Some(total)
    }
}
