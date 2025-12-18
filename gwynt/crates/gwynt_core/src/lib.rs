use std::error::Error;
use std::fs::File;
use std::path::Path;

pub type CardId = u32;

#[derive(Copy, Clone, Debug)]
pub enum Row {
    Melee,
    Ranged,
    Siege,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardKind {
    Unit,
    Spy,
    // plus tard: Weather, Buff, Hero...
}

#[derive(Clone, Debug)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub power: u8,
    pub kind: CardKind,
    pub row: Row,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerId {
    One,
    Two,
}

#[derive(Clone, Debug)]
pub struct Board {
    pub melee: Vec<Card>,
    pub ranged: Vec<Card>,
    pub siege: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            melee: Vec::new(),
            ranged: Vec::new(),
            siege: Vec::new(),
        }
    }

    pub fn total_power(&self) -> u32 {
        self.melee
            .iter()
            .chain(self.ranged.iter())
            .chain(self.siege.iter())
            .map(|c| c.power as u32)
            .sum()
    }

    pub fn push_card(&mut self, card: Card) {
        match card.row {
            Row::Melee => self.melee.push(card),
            Row::Ranged => self.ranged.push(card),
            Row::Siege => self.siege.push(card),
        }
    }

    pub fn clear(&mut self) {
        self.melee.clear();
        self.ranged.clear();
        self.siege.clear();
    }
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub board: Board,
    pub passed: bool,
    pub rounds_won: u8,
}

impl PlayerState {
    pub fn total_power(&self) -> u32 {
        self.board.total_power()
    }

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
    pub player1: PlayerState,
    pub player2: PlayerState,
    pub current_player: PlayerId,
    pub round: u8,
    pub finished: bool,
}

#[derive(Clone, Debug)]
pub enum Action {
    PlayCard(CardId),
    Pass,
}

impl GameState {
    pub fn new_with_decks(deck1: Vec<Card>, deck2: Vec<Card>) -> Self {
        let mut p1 = PlayerState {
            deck: deck1,
            hand: Vec::new(),
            board: Board::new(),
            passed: false,
            rounds_won: 0,
        };
        let mut p2 = PlayerState {
            deck: deck2,
            hand: Vec::new(),
            board: Board::new(),
            passed: false,
            rounds_won: 0,
        };

        p1.draw(7);
        p2.draw(7);

        Self {
            player1: p1,
            player2: p2,
            current_player: PlayerId::One,
            round: 1,
            finished: false,
        }
    }

    pub fn total_power(&self, id: PlayerId) -> u32 {
        match id {
            PlayerId::One => self.player1.total_power(),
            PlayerId::Two => self.player2.total_power(),
        }
    }

    pub fn rounds_won(&self, id: PlayerId) -> u8 {
        match id {
            PlayerId::One => self.player1.rounds_won,
            PlayerId::Two => self.player2.rounds_won,
        }
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

        let mut actions: Vec<Action> =
            player.hand.iter().map(|c| Action::PlayCard(c.id)).collect();

        actions.push(Action::Pass);
        actions
    }

    pub fn apply_action(&mut self, action: Action) {
        if self.finished {
            return;
        }

        match action {
            Action::PlayCard(id) => {
                let (me, other) = match self.current_player {
                    PlayerId::One => (&mut self.player1, &mut self.player2),
                    PlayerId::Two => (&mut self.player2, &mut self.player1),
                };

                if let Some(pos) = me.hand.iter().position(|c| c.id == id) {
                    let card = me.hand.remove(pos);
                    match card.kind {
                        CardKind::Spy => {
                            // espion : va sur le board adverse, te fait piocher 2 cartes
                            other.board.push_card(card);
                            me.draw(2);
                        }
                        CardKind::Unit => {
                            me.board.push_card(card);
                        }
                    }
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
        (self.player1.passed || self.player1.hand.is_empty())
            && (self.player2.passed || self.player2.hand.is_empty())
    }

    fn end_round(&mut self) {
        let p1 = self.total_power(PlayerId::One);
        let p2 = self.total_power(PlayerId::Two);

        if p1 > p2 {
            self.player1.rounds_won += 1;
        } else if p2 > p1 {
            self.player2.rounds_won += 1;
        }

        if self.round == 1 {
            self.player1.draw(2);
            self.player2.draw(2);
        } else if self.round == 2 {
            self.player1.draw(1);
            self.player2.draw(1);
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

/// Charge un deck depuis un CSV faction,name,power,kind,row
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
        let kind_str = record.get(3).unwrap_or("unit").trim();
        let row_str = record.get(4).unwrap_or("melee").trim();

        if rec_faction != faction {
            continue;
        }

        let power: u8 = power_str.parse().unwrap_or(0);
        let kind = match kind_str.to_ascii_lowercase().as_str() {
            "spy" => CardKind::Spy,
            _ => CardKind::Unit,
        };
        let row = match row_str.to_ascii_lowercase().as_str() {
            "ranged" => Row::Ranged,
            "siege" => Row::Siege,
            _ => Row::Melee,
        };

        cards.push(Card {
            id: next_id,
            name: name.to_string(),
            power,
            kind,
            row,
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
