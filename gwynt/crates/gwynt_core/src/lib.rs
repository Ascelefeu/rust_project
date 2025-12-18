#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerId {
    One,
    Two,
}

pub type CardId = u32;

#[derive(Clone, Debug)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub power: u8,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub hand: Vec<Card>,
    pub board: Vec<Card>,
    pub passed: bool,
    pub rounds_won: u8,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub current_player: PlayerId,
    pub player1: PlayerState,
    pub player2: PlayerState,
    pub round: u8,
    pub finished: bool, 
}

#[derive(Clone, Debug)]
pub enum Action {
    PlayCard(CardId),
    Pass,
}

impl GameState {
    pub fn new_with_decks(deck1: Vec<Card>, deck2: Vec<Card>) -> GameState {
        GameState {
            current_player: PlayerId::One,
            player1: PlayerState {
                hand: deck1,
                board: Vec::new(),
                passed: false,
                rounds_won: 0,
            },
            player2: PlayerState {
                hand: deck2,
                board: Vec::new(),
                passed: false,
                rounds_won: 0,
            },
            round: 1,
            finished: false,
        }
    }

    pub fn current_player(&self) -> PlayerId {
        self.current_player
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn rounds_won(&self, player: PlayerId) -> u8 {
        match player {
            PlayerId::One => self.player1.rounds_won,
            PlayerId::Two => self.player2.rounds_won,
        }
    }

    pub fn total_power(&self, player: PlayerId) -> u32 {
        let p = match player {
            PlayerId::One => &self.player1,
            PlayerId::Two => &self.player2,
        };
        p.board.iter().map(|c| c.power as u32).sum()
    }

    pub fn winner(&self) -> Option<PlayerId> {
        if !self.finished {
            return None;
        }
        match self.player1.rounds_won.cmp(&self.player2.rounds_won) {
            std::cmp::Ordering::Greater => Some(PlayerId::One),
            std::cmp::Ordering::Less => Some(PlayerId::Two),
            std::cmp::Ordering::Equal => None,
        }
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        if self.finished {
            return Vec::new();
        }

        let player = match self.current_player {
            PlayerId::One => &self.player1,
            PlayerId::Two => &self.player2,
        };

        let mut actions: Vec<Action> = player
            .hand
            .iter()
            .map(|c| Action::PlayCard(c.id))
            .collect();

        if !player.passed {
            actions.push(Action::Pass);
        }

        actions
    }

    pub fn apply_action(&mut self, action: Action) {
        if self.finished {
            return;
        }

        match action {
            Action::PlayCard(card_id) => {
                let player_state = match self.current_player {
                    PlayerId::One => &mut self.player1,
                    PlayerId::Two => &mut self.player2,
                };

                if let Some(pos) = player_state.hand.iter().position(|c| c.id == card_id) {
                    let card = player_state.hand.remove(pos);
                    player_state.board.push(card);
                } else {
                    return;
                }
            }
            Action::Pass => {
                let player_state = match self.current_player {
                    PlayerId::One => &mut self.player1,
                    PlayerId::Two => &mut self.player2,
                };
                player_state.passed = true;
            }
        }

        if self.both_players_done() {
            self.end_round();
        } else {
            self.current_player = match self.current_player {
                PlayerId::One => PlayerId::Two,
                PlayerId::Two => PlayerId::One,
            };
        }
    }

    fn both_players_done(&self) -> bool {
        let p1_done = self.player1.passed || self.player1.hand.is_empty();
        let p2_done = self.player2.passed || self.player2.hand.is_empty();
        p1_done && p2_done
    }

    fn end_round(&mut self) {
        let p1_score = self.total_power(PlayerId::One);
        let p2_score = self.total_power(PlayerId::Two);

        use std::cmp::Ordering;
        match p1_score.cmp(&p2_score) {
            Ordering::Greater => self.player1.rounds_won += 1,
            Ordering::Less => self.player2.rounds_won += 1,
            Ordering::Equal => {
            }
        }

        self.player1.board.clear();
        self.player2.board.clear();
        self.player1.passed = false;
        self.player2.passed = false;

        if self.player1.rounds_won == 2 || self.player2.rounds_won == 2 || self.round == 3 {
            self.finished = true;
        } else {
            self.round += 1;
            self.current_player = PlayerId::One;
        }
    }
}

pub fn example_deck1() -> Vec<Card> {
    (0..10)
        .map(|i| Card {
            id: i,
            name: format!("Soldat A{}", i),
            power: 3,
        })
        .collect()
}

pub fn example_deck2() -> Vec<Card> {
    (100..110)
        .map(|i| Card {
            id: i,
            name: format!("Soldat B{}", i),
            power: 4,
        })
        .collect()
}
