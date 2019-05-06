mod elo {
    pub struct Player {
        pub rating: u16,
        pub wins: u16,
        pub losses: u16,
    }

    impl Player {
        pub fn total_games(&self) -> u16 {
            self.wins + self.losses
        }

        pub fn total_wins(&self) -> u16 {
            self.wins
        }

        pub fn total_losses(&self) -> u16 {
            self.losses
        }

        pub fn rating(&self) -> u16 {
            self.rating
        }

        pub fn update_rating(&mut self, other: u16) {
            let ra = self.rating as f64;
            let rb =  other as f64;
            let probability = (10 as f64).powf(1.0 * (ra - rb) / 400 as f64);
            // self.rating = (1.0 * (1.0 / (1 as f64 + 1.0 * ten.powf(1.0 * (ra - rb) / 400 as f64) as f64))) as u16;
            self.rating = 0;
        }
    }

    struct Game {
        team_one: Vec<Player>,
        team_two: Vec<Player>,
        started: bool,
        finished: bool,
    }

    impl Game {

    }
}

fn main() {
    let mut a = elo::Player{rating: 1200, wins : 5, losses: 5};
    a.update_rating(1200);
    let b = a.rating();
    println!("{}", b);
}
