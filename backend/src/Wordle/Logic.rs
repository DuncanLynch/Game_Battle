use crate::Wordle::Types;
pub fn make_guess(guess: &String, word: &String) -> [Wordle::Types::LetterType; 5] {
    let guessChars: Vec<char> = guess.chars().collect();
    let wordChars: Vec<char> = word.chars().collect();
    let mut letters: [Wordle::Types::LetterType; 5] = [];
    let mut shouldCheck: [bool; ]
    for i in (0..4) {
        if guessChars[i] == wordChars[i] {
            letters[i] = Wordle::Types::LetterType::Correct;
        }
    }
}
