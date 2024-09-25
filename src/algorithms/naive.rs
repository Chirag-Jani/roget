use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line.split_once(" ").expect("Expected: Line + Space + Freq");
                let count: usize = count.parse().expect("Every Count is a number");
                (word, count)
            })),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, _history: &[Guess]) -> String {
        let mut best: Option<Candidate> = None;
        for (&word, &count) in &self.remaining {
            let goodness = 6.9;
            if let Some(c) = best {
                // is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        count,
                        goodness,
                    });
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                });
            }
        }
        todo!();
    }
}
