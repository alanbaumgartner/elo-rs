mod elo {
    use std::collections::HashMap;
    
    pub struct League {
        pub players: HashMap<u128, Player>,
        // pub games: HashMap<u128, Game>,
        pub start_elo: u16,
        pub n_val: f32,
        pub k_val: f32,
        pub k_reduction: fn(u16, f32) -> f32,
    }

    impl Default for League {
        fn default() -> Self {
            fn k_reduction(x: u16, y: f32) -> f32 {
                match x {
                    0...30 => y,
                    _ => y / 2 as f32,
                }
            }
            Self {
                start_elo: 1000,
                n_val: 400 as f32,
                k_val: 40 as f32,
                k_reduction: k_reduction,
                players: HashMap::new(),
            } //, games: HashMap::new()}
        }
    }

    impl League {
        pub fn new() -> League {
            League {
                ..Default::default()
            }
        }

        pub fn add_player(&mut self, new_player: Player) {
            self.players.insert(new_player.id, new_player);
        }

        pub fn get_player_rating(&self, id: u128) -> u16 {
            self.players.get(&id).unwrap().rating
        }

        pub fn update_players(&mut self, id_one: u128, id_two: u128, winner: bool) {
            let rating_one = match self.players.get(&id_one) {
                Some(player) => player.rating,
                None => 0,
            };

            let rating_two = match self.players.get(&id_two) {
                Some(player) => player.rating,
                None => 0,
            };

            if rating_one < 100 || rating_two < 100 {
                println!("One or more players not found!");
            } else {
                self.players.get_mut(&id_one).unwrap().update_rating(
                    rating_two,
                    self.n_val,
                    self.k_val,
                    self.k_reduction,
                    winner,
                );
                self.players.get_mut(&id_two).unwrap().update_rating(
                    rating_one,
                    self.n_val,
                    self.k_val,
                    self.k_reduction,
                    !winner,
                );
            }
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

        pub fn update_rating(
            &mut self,
            other: u16,
            n_val: f32,
            k_val: f32,
            k_reduction: fn(u16, f32) -> f32,
            won: bool,
        ) {
            let ra = self.rating as f32;
            let rb = other as f32;
            let x = ra - rb;
            let exponent = -(x / n_val);

            let expected = 1 as f32 / (1 as f32 + (10 as f32).powf(exponent));

            let k = k_reduction(self.total_games(), k_val);

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

    impl Game {}
}

use elo::League;

fn main() {
    fn k_reduction(x: u16, y: f32) -> f32 {
        match x {
            0...30 => y,
            _ => y / 2 as f32,
        }
    }

    let mut league = League {
        k_reduction: k_reduction,
        ..Default::default()
    };
    let a = elo::Player {
        id: 1,
        rating: 1000,
        wins: 0,
        losses: 0,
    };
    let b = elo::Player {
        id: 2,
        rating: 1000,
        wins: 0,
        losses: 0,
    };
    league.add_player(a);
    league.add_player(b);

    league.update_players(1, 2, true);

    let c = league.get_player_rating(1);
    let d = league.get_player_rating(2);
    println!("{} {}", c, d);
}
