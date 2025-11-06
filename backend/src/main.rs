mod wordle;


fn main() {
    let mut creator: Player = wordle::Player::new("HypaDeficit", uuid::UUId::new_v4());
    let game: wordle::GameSession = wordle::GameSession::new(creator, "password");
    let guess: [wordle::LetterType; 5] = wordle::GameSession::make_guess(&self, "zxwzz", creator.token);
    println!("{guess}");
}
