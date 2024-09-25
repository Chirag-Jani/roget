use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

// check whether the guess is valid
pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(
                DICTIONARY
                    .lines()
                    .map(|line| line.split_once(' ').expect("word + space + freq").0),
            ),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();

        // WORDLE only allows 6 guesses.
        // We allow more to avoid chopping off the score distribution for stats purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history);

            if guess == answer {
                return Some(i);
            }

            assert!(self.dictionary.contains(&*guess));

            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}
impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];

        // mark green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct
            }
        }

        // mark yellow
        let mut used = [false; 5];

        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                // already marked green
                continue;
            }

            if let Some(_) = answer.chars().enumerate().find_map(|(idx, chr)| {
                if chr == g && !used[idx] {
                    used[idx] = true;
                    return Some(idx);
                }
                None
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history: ident| $impl: block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {
    mod game {

        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let word = Wordle::new();
            let guesser = guesser!(|_history| { "right".to_string() });

            assert_eq!(word.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let word = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });

            assert_eq!(word.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let word = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                return "wrong".to_string();
            });

            assert_eq!(word.play("right", guesser), Some(3));
        }

        #[test]
        fn oppsie() {
            let word = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });

            assert_eq!(word.play("right", guesser), None);
        }
    }
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {
                Correctness::Correct
            };
            (M) => {
                Correctness::Misplaced
            };
            (W) => {
                Correctness::Wrong
            };
            ($($c: tt)+) => {
                [
                    $(mask!($c)),+
                ]
            }
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C])
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "lmnop"), mask![W W W W W])
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "cdbea"), mask![M M M M M])
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W])
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W])
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W])
        }

        #[test]
        fn random_1() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W])
        }

        #[test]
        fn random_2() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C])
        }
    }
}
