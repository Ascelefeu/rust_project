use std::io::{self, Write};

use gwynt_core::{
    northern_realms_deck,
    nilfgaard_deck,
    Action,
    GameState,
    PlayerId,
};

fn main() {
    let deck1 = northern_realms_deck().expect("Erreur chargement deck Northern Realms");
    let deck2 = nilfgaard_deck().expect("Erreur chargement deck Nilfgaard");

    let mut game = GameState::new_with_decks(deck1, deck2);

    println!("=== Gwynt minimal ===");

    while !game.is_finished() {
        println!("\n--- Manche {} ---", game.round);
        println!(
            "Score manches : P1={} | P2={}",
            game.rounds_won(PlayerId::One),
            game.rounds_won(PlayerId::Two),
        );

        let current = game.current_player();
        println!("Tour de {:?}", current);
        print_player_view(&game, current);

        let actions = game.legal_actions();
        if actions.is_empty() {
            println!("Aucune action possible.");
            break;
        }

        println!("Actions possibles :");
        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::PlayCard(id) => {
                    println!("  {}: Jouer carte id {}", i, id);
                }
                Action::Pass => {
                    println!("  {}: Passer", i);
                }
            }
        }

        let choice = prompt_usize("Choix: ");
        if let Some(action) = actions.get(choice).cloned() {
            game.apply_action(action);
        } else {
            println!("Choix invalide, recommence.");
        }
    }

    println!("\n=== Fin de la partie ===");
    match game.winner() {
        Some(PlayerId::One) => println!("Le joueur 1 gagne !"),
        Some(PlayerId::Two) => println!("Le joueur 2 gagne !"),
        None => println!("Égalité."),
    }
}

fn prompt_usize(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            if let Ok(v) = buf.trim().parse::<usize>() {
                return v;
            }
        }
        println!("Veuillez entrer un nombre.");
    }
}

fn print_player_view(game: &GameState, player: PlayerId) {
    use PlayerId::*;

    let (me, other) = match player {
        One => (&game.player1, &game.player2),
        Two => (&game.player2, &game.player1),
    };

    println!(
        "Votre board (puissance totale = {}):",
        game.total_power(player)
    );
    for card in &me.board {
        println!("  - {} (id {}, power {})", card.name, card.id, card.power);
    }

    println!(
        "Board adverse (puissance totale = {}):",
        game.total_power(if matches!(player, One) { Two } else { One })
    );
    for card in &other.board {
        println!("  - {} (id {}, power {})", card.name, card.id, card.power);
    }

    println!("Votre main :");
    for card in &me.hand {
        println!("  - {} (id {}, power {})", card.name, card.id, card.power);
    }
}
