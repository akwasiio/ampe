enum PlayerType {
    Same,
    Different,
}

enum Leg {
    Right,
    Left,
    None,
}

struct Player {
    player_type: PlayerType,
    leg_played: Leg,
    score: u16,
}

impl Player {
    pub fn new(player_type: PlayerType) -> Self {
        Self {
            player_type,
            leg_played: Leg::None,
            score: 0,
        }
    }
}

struct Game;

impl Game {
    pub fn play(first_player: &mut Player, second_player: &mut Player) {
        match first_player.player_type {
            PlayerType::Same => {
                match (&first_player.leg_played, &second_player.leg_played) {
                    (Leg::Left, Leg::Left) => {
                        first_player.score += 10
                    }
                    (Leg::Right, Leg::Right) => {
                        first_player.score += 10;
                    }
                    (_, _) => {
                        second_player.score += 10
                    }
                }
            }
            PlayerType::Different => {
                match (&first_player.leg_played, &second_player.leg_played) {
                    (Leg::Right, Leg::Left) => {
                        first_player.score += 10
                    }
                    (Leg::Left, Leg::Right) => {
                        first_player.score += 10;
                    }
                    (_, _) => {
                        second_player.score += 10
                    }
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{Game, Leg, Player, PlayerType};

    #[test]
    fn test_same_leg() {
        let mut player_1 = Player::new(PlayerType::Same);
        let mut player_2 = Player::new(PlayerType::Different);

        player_1.leg_played = Leg::Left;
        player_2.leg_played = Leg::Left;

        Game::play(&mut player_1, &mut player_2);
        assert_eq!(player_1.score, 10);
        assert_eq!(player_2.score, 0)

    }

    #[test]
    fn test_different_leg() {
        let mut player_1 = Player::new(PlayerType::Same);
        let mut player_2 = Player::new(PlayerType::Different);

        player_1.leg_played = Leg::Left;
        player_2.leg_played = Leg::Right;

        Game::play(&mut player_1, &mut player_2);
        assert_eq!(player_1.score, 0);
        assert_eq!(player_2.score, 10)
    }
}

fn main() {}