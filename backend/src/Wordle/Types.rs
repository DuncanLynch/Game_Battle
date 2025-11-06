pub enum LetterType {
    Missing,
    Present,
    Correct,
}

pub struct Guess {
    guess: String,
    results: [LetterType; 5],
} 

pub struct Player {
    name: String,
    guesses: [Guess; 6],
    token: String,
}

pub struct Game {
    players: Vec<Player>,
    word: String,
    session_password: String,
}
