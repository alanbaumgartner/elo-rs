mod elo {
    use std::collections::HashMap;

    pub struct League {
        pub players: HashMap<u128, Player>,
        pub games: HashMap<u128, Game>,
        pub start_elo: u16,
        pub n_val: f32,
        pub k_val: f32,
        pub k_reduction: Fn(u16, f32) -> f32,
    }

    impl Default for League {
        fn default() -> Self {

            fn k_reduction (x: u16, y: f32) -> f32 {
                match x {
                    0 ... 30 => y,
                    _ => y / 2 as f32,
                }
            };

            Self{start_elo: 1000, n_val: 400 as f32, k_val: 40 as f32, k_reduction: &k_reduction, players: HashMap::new(), games: HashMap::new()}
        }
    }

    impl League {
        pub fn new() -> League {
            League{start_elo: 1000, n_val: 400 as f32, k_val: 40 as f32, k_reduction: 30, players: HashMap::new(), games: HashMap::new()}
        }
        pub fn start_elo(mut self, val: u16) -> League {
            self.start_elo = val;
            self
        }
        pub fn n_val(mut self, val: f32) -> League {
            self.n_val = val;
            self
        }
    }
    


    pub struct Player {
        pub id: u128,
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

        pub fn update_rating(&mut self, other: u16, n_val: f32, k_val: f32, won: bool) {
            let ra = self.rating as f32;
            let rb =  other as f32;
            let x = ra - rb;
            let exponent = -(x / n_val);

            let expected = 1 as f32 / (1 as f32 + (10 as f32).powf(exponent));

            let k = match self.total_games() {
                0 ... 30 => k_val,
                _ => k_val / 2 as f32,
            } as f32;

            if won {
                self.rating = (ra + (k * (1 as f32 - expected))) as u16;
            } else {
                self.rating = (ra + (k * (0 as f32 - expected))) as u16;
            }
        }
    }

    pub struct Game {
        player_one: Player,
        player_two: Player,
        // team_one: Vec<Player>,
        // team_two: Vec<Player>,
        // started: bool,
        // finished: bool,
    }

    impl Game {

    }
}

use elo::League;

fn main() {
    let mut league = League{..Default::default()};
    let mut a = elo::Player{id: 1, rating: 1000, wins : 0, losses: 0};
    a.update_rating(1000, 400 as f32, 40 as f32, true);
    let mut b = elo::Player{id: 2, rating: 1300, wins : 0, losses: 0};
    // league.players.insert(1, a);
    // league.players.insert(1, b);
    // league.NewGame(1, 2);
    // a.update_rating(3000,true);
    let c = a.rating();
    println!("{}", c);
}
