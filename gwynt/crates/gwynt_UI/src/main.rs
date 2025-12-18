use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use gwynt_core::*;
use rand::seq::SliceRandom;

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";

fn main() {
    // Demander les noms
    let name1 = input_string("Nom du joueur 1: ");
    let name2 = input_string("Nom du joueur 2: ");

    // Charger les deux decks (Northern / Nilfgaard)
    let deck_nr = northern_realms_deck().unwrap();
    let deck_nilf = nilfgaard_deck().unwrap();

    // Tirage au sort : qui joue quel deck ?
    let mut choices = [("Northern Realms", deck_nr), ("Nilfgaard", deck_nilf)];
    let mut rng = rand::thread_rng();
    choices.shuffle(&mut rng);

    let (deck1_name, deck1) = &choices[0];
    let (deck2_name, deck2) = &choices[1];

    println!(
        "{}{} jouera le deck {}{}",
        BOLD, name1, deck1_name, RESET
    );
    println!(
        "{}{} jouera le deck {}{}",
        BOLD, name2, deck2_name, RESET
    );
    println!("Appuie sur Entrée pour commencer...");
    let _ = io::stdin().read_line(&mut String::new());

    let mut game = GameState::new_with_decks(deck1.clone(), deck2.clone());

    loop {
        clear();
        render(&game, &name1, &name2, deck1_name, deck2_name);

        if game.finished {
            show_winner(&game, &name1, &name2);
            break;
        }

        let actions = game.legal_actions();
        if actions.is_empty() {
            println!("Aucune action possible (joueur a passé ou n'a plus de cartes).");
            pause();
            // Pour simplifier, on arrête la partie ici.
            break;
        }

        println!("\nActions :");
for (i, a) in actions.iter().enumerate() {
    match a {
        Action::PlayCard(id) => {
            let card = find_card_in_hand(&game, *id).unwrap();
            println!("  [{}] Jouer {}", i, render_card(card));
        }
        Action::Pass => println!("  [{}] Passer", i),
    }
}


        let choice = input_usize("> ");
        if let Some(action) = actions.get(choice).cloned() {
            animate_action(&action);
            game.apply_action(action);
        } else {
            println!("Choix invalide.");
            pause();
        }
    }
}

fn render(
    game: &GameState,
    name1: &str,
    name2: &str,
    deck1_name: &str,
    deck2_name: &str,
) {
    println!("{BOLD}{CYAN}GWYNT — Manche {}/3{RESET}", game.round);
    println!(
        "{}{}{} (deck {}) : {} manches | {}{}{} (deck {}) : {} manches",
        BOLD,
        name1,
        RESET,
        deck1_name,
        game.rounds_won(PlayerId::One),
        BOLD,
        name2,
        RESET,
        deck2_name,
        game.rounds_won(PlayerId::Two),
    );
    println!("Joueur courant : {}", current_player_name(game, name1, name2));
    println!("--------------------------------------------------");

    print_board(game, name1, name2);
    print_hand(game, name1, name2);
}

fn current_player_name(game: &GameState, name1: &str, name2: &str) -> String {
    match game.current_player {
        PlayerId::One => name1.to_string(),
        PlayerId::Two => name2.to_string(),
    }
}

fn print_board(game: &GameState, name1: &str, name2: &str) {
    println!("\n{BOLD}PLATEAU{RESET}");

    show_player(
        name1,
        &game.player1.board,
        game.total_power(PlayerId::One),
    );
    show_player(
        name2,
        &game.player2.board,
        game.total_power(PlayerId::Two),
    );
}

fn show_player(label: &str, board: &[Card], total: u32) {
    print!("{BOLD}{} ({}) :{RESET} ", label, total);
    if board.is_empty() {
        println!("—");
    } else {
        for c in board {
            print!("{} ", render_card(c));
        }
        println!();
    }
}

fn render_card(card: &Card) -> String {
    let color = if card.is_spy { RED } else { GREEN };
    let tags = if card.is_spy { " ESPION" } else { "" };
    format!(
        "{color}[{}:{}{}]{RESET}",
        card.name,
        card.power,
        tags,
    )
}

fn print_hand(game: &GameState, name1: &str, name2: &str) {
    let (player, label) = match game.current_player {
        PlayerId::One => (&game.player1, name1),
        PlayerId::Two => (&game.player2, name2),
    };

    println!("\n{YELLOW}MAIN DE {}{RESET}", label);
    if player.hand.is_empty() {
        println!("  (aucune carte en main)");
        return;
    }
    for (i, c) in player.hand.iter().enumerate() {
        println!("  [{}] {}", i, render_card(c));
    }
}

fn animate_action(action: &Action) {
    match action {
        Action::PlayCard(_) => {
            print!("{GREEN}Carte jouée{RESET}");
        }
        Action::Pass => {
            print!("{YELLOW}Passe{RESET}");
        }
    }
    for _ in 0..3 {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(200));
    }
    println!();
}

fn show_winner(game: &GameState, name1: &str, name2: &str) {
    println!("\n{BOLD}FIN DE PARTIE{RESET}");
    match game.winner() {
        Some(PlayerId::One) => println!("{GREEN}Victoire de {} !{RESET}", name1),
        Some(PlayerId::Two) => println!("{GREEN}Victoire de {} !{RESET}", name2),
        None => println!("{YELLOW}Égalité.{RESET}"),
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn pause() {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
}

fn input_usize(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            if let Ok(v) = s.trim().parse::<usize>() {
                return v;
            }
        }
        println!("Veuillez entrer un nombre.");
    }
}

fn input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn find_card_in_hand<'a>(game: &'a GameState, id: CardId) -> Option<&'a Card> {
    let player = match game.current_player {
        PlayerId::One => &game.player1,
        PlayerId::Two => &game.player2,
    };
    player.hand.iter().find(|c| c.id == id)
}