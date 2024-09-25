use roget::Wordle;

const GAMES: &str = include_str!("../answers.txt");
fn main() {
    for answer in GAMES.split_whitespace() {
        let word = Wordle::new();
        let guesser = roget::algorithms::Naive::new();
        word.play(&answer, guesser);
    }
}
