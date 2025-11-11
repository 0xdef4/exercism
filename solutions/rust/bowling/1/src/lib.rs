#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls_vec: [[i16; 2]; 10],
    fill_balls: Vec<i16>,
    index_of_rolls_vec: usize,
    index_of_rolls: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls_vec: [
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
                [-1, -1],
            ],
            fill_balls: vec![],
            index_of_rolls_vec: 0,
            index_of_rolls: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // todo!("Record that {pins} pins have been scored");
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.index_of_rolls_vec == 10
            && self.fill_balls.len() == 0
            && self.rolls_vec[9][0] != 10
            && self.rolls_vec[9].iter().sum::<i16>() != 10
        {
            return Err(Error::GameComplete);
        }

        if self.index_of_rolls_vec > 9 {
            if self.rolls_vec[9].iter().sum::<i16>() == 10 && self.fill_balls.len() == 1 {
                return Err(Error::GameComplete);
            }
            if self.rolls_vec[9][0] == 10 && self.fill_balls.len() == 2 {
                return Err(Error::GameComplete);
            }

            if self.rolls_vec[9][0] == 10
                && self.fill_balls.len() == 1
                && self.fill_balls[0] != 10
                && pins == 10
            {
                return Err(Error::NotEnoughPinsLeft);
            }

            if self.rolls_vec[9][0] == 10
                && self.fill_balls.len() == 1
                && self.fill_balls[0] != 10
                && self.fill_balls[0] + pins as i16 > 10
            {
                return Err(Error::NotEnoughPinsLeft);
            }

            self.fill_balls.push(pins as i16);
        } else {
            // 저장
            if let Some(elem) = self.rolls_vec[self.index_of_rolls_vec].get_mut(self.index_of_rolls)
            {
                *elem = pins as i16;
            }

            if self.rolls_vec[self.index_of_rolls_vec].iter().sum::<i16>() > 10
                && self.rolls_vec[self.index_of_rolls_vec]
                    .iter()
                    .all(|e| *e != -1)
            {
                return Err(Error::NotEnoughPinsLeft);
            }

            // 내려가는 경우들
            if self.rolls_vec[self.index_of_rolls_vec][0] == 10
                || self.rolls_vec[self.index_of_rolls_vec].iter().sum::<i16>() == 10
                || self.rolls_vec[self.index_of_rolls_vec]
                    .iter()
                    .all(|e| *e != -1)
            {
                self.index_of_rolls_vec += 1;
                self.index_of_rolls = 0;
            } else {
                self.index_of_rolls = 1;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // todo!("Return the score if the game is complete, or None if not.");
        if self.index_of_rolls_vec == 0 && self.index_of_rolls == 0 {
            return None;
        }

        if self.index_of_rolls_vec != 10 {
            return None;
        }

        if self.rolls_vec[9].iter().sum::<i16>() == 10 && self.fill_balls.is_empty() {
            return None;
        }

        if self.rolls_vec[9][0] == 10 && self.fill_balls.len() != 2 {
            return None;
        }

        let mut total = Vec::new();

        for (i, rolls) in self.rolls_vec.iter().enumerate() {
            let mut sum_of_frame = 0;
            if rolls[0] == 10 {
                let sum_of_strike_case = self.rolls_vec[i + 1..]
                    .iter()
                    .flat_map(|x| x.iter())
                    .chain(self.fill_balls.iter())
                    .filter(|e| **e != -1)
                    .take(2)
                    .sum::<i16>();

                sum_of_frame = sum_of_frame + 10 + dbg!(sum_of_strike_case);
                total.push(sum_of_frame);
            } else if rolls.iter().sum::<i16>() == 10 {
                let sum_of_spare_case = self.rolls_vec[i + 1..]
                    .iter()
                    .flat_map(|x| x.iter())
                    .chain(self.fill_balls.iter())
                    .filter(|e| **e != -1)
                    .take(1)
                    .sum::<i16>();
                sum_of_frame = sum_of_frame + 10 + sum_of_spare_case;
                total.push(sum_of_frame);
            } else {
                let sum_of_open_case = rolls.iter().sum::<i16>();
                sum_of_frame = sum_of_frame + sum_of_open_case;
                total.push(sum_of_frame);
            }
        }

        Some(total.iter().sum::<i16>() as u16)
    }
}
