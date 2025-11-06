
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LetterType {
    Missing,
    Present,
    Correct,
}

#[derivce(Clone, Debug)]
pub enum GameState {
    Waiting,
    Playing,
    Over,
    Restarting,
}

#[derive(Clone, Debug)]
pub struct Guess {
    pub guess: String,
    pub results: [LetterType; 5],
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub guesses: Vec<Option<Guess>>,
    pub token: String,
    pub finished: bool,
}

impl Player {
    pub fn new(name: String, token: String) -> Self {
        Self {
            name,
            guesses: vec![None; 6],
            token,
            finished: false,
        }
    }
}

pub struct GameSession {
    pub players: Vec<Player>,
    pub word: String,
    pub session_password: String,
    pub state: GameState,
}

impl GameSession {
    pub fn make_guess(&self, guess: &str, token: &str) -> [LetterType; 5] {
        let guess_chars: Vec<char> = guess.chars().collect();
        let word_chars: Vec<char> = self.word.chars().collect();
        let mut letters: [LetterType; 5]  = [LetterType::Missing; 5];
        let mut should_check: [bool; 5] = [true; 5];

        for i in 0..5 {
            if guess_chars[i] == word_chars[i] {
                letters[i] = LetterType::Correct;
                should_check[i] = false;
            }
        }
        for i in 0..5 {
            if letters[i] == LetterType::Missing && word_chars.contains(&guess_chars[i]) {
                letters[i] = LetterType::Present;
            }
        }

        letters
    }

    pub fn check_finished(&self) -> bool {
        for player in self.players {
            if !player.finished {
                return false
            }
        }
        true
    }
}
