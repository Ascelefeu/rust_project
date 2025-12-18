use std::error::Error;
use std::fs::File;
use std::path::Path;

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
    pub is_spy: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub board: Vec<Card>,
    pub passed: bool,
    pub rounds_won: u8,
}

impl PlayerState {
    pub fn draw(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(card) = self.deck.pop() {
                self.hand.push(card);
            } else {
                break;
            }
        }
    }
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
        let mut p1 = PlayerState {
            deck: deck1,
            hand: Vec::new(),
            board: Vec::new(),
            passed: false,
            rounds_won: 0,
        };

        let mut p2 = PlayerState {
            deck: deck2,
            hand: Vec::new(),
            board: Vec::new(),
            passed: false,
            rounds_won: 0,
        };

        // pioche initiale
        p1.draw(10);
        p2.draw(10);

        GameState {
            current_player: PlayerId::One,
            player1: p1,
            player2: p2,
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

        if player.passed {
            return Vec::new();
        }

        let mut actions: Vec<Action> = player
            .hand
            .iter()
            .map(|c| Action::PlayCard(c.id))
            .collect();

        actions.push(Action::Pass);

        actions
    }

    pub fn apply_action(&mut self, action: Action) {
        if self.finished {
            return;
        }

        match action {
            Action::PlayCard(card_id) => {
                let (me, other) = match self.current_player {
                    PlayerId::One => (&mut self.player1, &mut self.player2),
                    PlayerId::Two => (&mut self.player2, &mut self.player1),
                };

                if let Some(pos) = me.hand.iter().position(|c| c.id == card_id) {
                    let card = me.hand.remove(pos);

                    if card.is_spy {
                        other.board.push(card);
                        me.draw(2);
                    } else {
                        me.board.push(card);
                    }
                } else {
                    return;
                }
            }
            Action::Pass => {
                let me = match self.current_player {
                    PlayerId::One => &mut self.player1,
                    PlayerId::Two => &mut self.player2,
                };
                me.passed = true;
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
            Ordering::Equal => { /* égalité : personne ne gagne */ }
        }

        match self.round {
            1 => {
                self.player1.draw(2);
                self.player2.draw(2);
            }
            2 => {
                self.player1.draw(1);
                self.player2.draw(1);
            }
            _ => {}
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

pub fn load_deck_from_csv<P: AsRef<Path>>(
    path: P,
    faction: &str,
    starting_id: CardId,
) -> Result<Vec<Card>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut cards = Vec::new();
    let mut next_id = starting_id;

    for result in rdr.records() {
        let record = result?;
        let rec_faction = record.get(0).unwrap_or("").trim();
        let name = record.get(1).unwrap_or("").trim();
        let power_str = record.get(2).unwrap_or("0").trim();
        let kind = record.get(3).unwrap_or("unit").trim();

        if rec_faction != faction {
            continue;
        }

        let power: u8 = power_str.parse().unwrap_or(0);
        let is_spy = kind.eq_ignore_ascii_case("spy");

        cards.push(Card {
            id: next_id,
            name: name.to_string(),
            power,
            is_spy,
        });

        next_id += 1;
    }

    Ok(cards)
}

pub fn northern_realms_deck() -> Result<Vec<Card>, Box<dyn Error>> {
    load_deck_from_csv("data/decks.csv", "Northern Realms", 0)
}

pub fn nilfgaard_deck() -> Result<Vec<Card>, Box<dyn Error>> {
    load_deck_from_csv("data/decks.csv", "Nilfgaard", 1000)
}
