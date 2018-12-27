pub struct Game {
    rolls: Vec<usize>,
}

const NUMBER_OF_FRAMES_PER_GAME: usize = 10;
const NUMBER_OF_PINS_IN_A_GAME: usize = 10;

impl Game {
    pub fn new() -> Game {
        Game { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: usize) {
        self.rolls.push(pins)
    }

    pub fn score(&self) -> usize {
        let mut total = 0;
        let mut roll_index = 0;
        for _i in 0..NUMBER_OF_FRAMES_PER_GAME {
            total += self.frame_score(roll_index);
            roll_index += if self.is_strike(roll_index) { 1 } else { 2 }
        }
        total
    }

    fn frame_score(&self, index: usize) -> usize {
        if self.is_strike(index) {
            NUMBER_OF_PINS_IN_A_GAME + self.sum_of_consecutive_rolls(index + 1)
        } else {
            if self.is_spare(index) {
                NUMBER_OF_PINS_IN_A_GAME + self.rolls[index + 2]
            } else {
                self.sum_of_consecutive_rolls(index)
            }
        }
    }

    fn is_spare(&self, index: usize) -> bool {
        self.sum_of_consecutive_rolls(index) == NUMBER_OF_PINS_IN_A_GAME
    }

    fn is_strike(&self, index: usize) -> bool {
        self.rolls[index] == NUMBER_OF_PINS_IN_A_GAME
    }

    fn sum_of_consecutive_rolls(&self, index: usize) -> usize {
        self.rolls[index] + self.rolls[index + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_times(game: &mut Game, pins: usize, times: usize) {
        (0..times).for_each(|_| game.roll(pins));
    }

    fn perform_game_test<F: Fn(&mut Game) -> ()>(game_runner: F, expected_score: usize) {
        let mut game = Game::new();
        game_runner(&mut game);
        let score = game.score();
        assert_eq!(expected_score, score);
    }

    fn perform_full_game(rolls: &[usize], expected_score: usize) {
        perform_game_test(
            |g| rolls.iter().for_each(|roll| g.roll(*roll)),
            expected_score,
        );
    }

    fn simple_test(pins: usize, times: usize, expected: usize) {
        perform_game_test(|mut g| roll_times(&mut g, pins, times), expected);
    }

    #[test]
    fn gutter_game_scores_zero() {
        simple_test(0, 20, 0);
    }

    #[test]
    fn ones_only_game_scores_twenty() {
        simple_test(1, 20, 20);
    }

    #[test]
    fn perfect_game_scores_three_hundred() {
        simple_test(10, 12, 300);
    }

    #[test]
    fn spare_should_count_next_throw_as_bonus() {
        perform_game_test(
            |mut g| {
                g.roll(9);
                g.roll(1);
                roll_times(&mut g, 1, 18);
            },
            29,
        );
    }

    #[test]
    fn spare_should_count_next_throw_as_bonus_in_last_frame() {
        perform_game_test(
            |mut g| {
                roll_times(&mut g, 0, 18);
                g.roll(9);
                g.roll(1);
                g.roll(1);
            },
            11,
        );
    }

    #[test]
    fn spare_can_only_occur_in_second_throw_of_frame() {
        perform_game_test(
            |mut g| {
                g.roll(0);
                g.roll(9);
                g.roll(1);
                g.roll(1);
                roll_times(&mut g, 0, 18);
            },
            11,
        );
    }

    #[test]
    fn strike_should_count_next_two_throws_as_bonus() {
        perform_game_test(
            |mut g| {
                g.roll(10);
                g.roll(1);
                g.roll(1);
                roll_times(&mut g, 0, 16);
            },
            14,
        );
    }

    #[test]
    fn strike_should_count_next_two_throws_as_bonus_in_last_frame() {
        perform_game_test(
            |mut g| {
                roll_times(&mut g, 0, 18);
                g.roll(10);
                g.roll(1);
                g.roll(1);
            },
            12,
        );
    }

    #[test]
    fn strike_spare_combination() {
        perform_game_test(
            |mut g| {
                roll_times(&mut g, 0, 10);
                g.roll(10);
                g.roll(1);
                g.roll(1);
                g.roll(9);
                g.roll(1);
                g.roll(1);
                roll_times(&mut g, 0, 3);
            },
            26,
        );
    }

    #[test]
    fn complete_example() {
        perform_full_game(
            &[10, 7, 3, 9, 0, 10, 0, 8, 8, 2, 0, 6, 10, 10, 10, 8, 1],
            167,
        );
    }

    #[test]
    fn another_complete_example() {
        perform_full_game(
            &[1, 2, 4, 6, 8, 1, 4, 6, 4, 6, 10, 10, 5, 3, 6, 3, 7, 1],
            132,
        );
    }

    #[test]
    fn almost_all_strikes() {
        perform_game_test(
            move |mut g| {
                roll_times(&mut g, 10, 10);
                g.roll(7);
                g.roll(1);
            },
            285,
        );
    }
}
