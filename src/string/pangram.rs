use std::collections::HashMap;
use std::io::{self, Write};
use std::iter::repeat;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PangramKind {
    /// Uses each letter in the aphabet exactely once
    Perfect,
    /// Uses each letter in the aphabet,
    Good,
    /// Doest not contain every letter in the alphabet
    Bad,
}

#[rustfmt::skip]
/// The input should be an ascii string for now
pub fn is_pangram(input_str: &str) -> PangramKind {
    if !input_str.is_ascii() {
        return PangramKind::Bad;
    }

    let abc = "abcdefghijklmnopqrstuvwxyz";
    let mut map: HashMap<char, u32> = abc
        .chars()
        .zip(repeat(0)
        .take(abc.len()))
        .collect();


    for ch in input_str.to_lowercase().chars() {
        if ch.is_whitespace() || ch.is_ascii_punctuation(){ continue; }

        if !ch.is_ascii_alphabetic() {
            return PangramKind::Bad;
        }

        if let Some(times) = map.get_mut(&ch) {
            *times += 1;
        } else {
            return PangramKind::Bad
        }
    }

    let mut perfect = true;


    for (char, times) in map {
        if times == 0 {
            return PangramKind::Bad;
        }
        if times > 1 {
            perfect = false;
        }
    }

    if perfect {
        PangramKind::Perfect
    } else {
        PangramKind::Good
    }
}

/// Having fun with macros
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! pangram_test {
        ($fname: ident, $result: expr, $($input: expr),+ $(,)?) => {
            #[test]
            fn $fname() {
                $(
                    assert_eq!($result,
                               is_pangram($input),
                               "Failed on input: {}",
                               $input
                    );
                )+
            }
        };
    }

    pangram_test!(
        perfect,
        PangramKind::Perfect,
        "abcdefghijklmnopqrstuvwxyz",
        "Mr Jock, TV quiz PhD, bags few lynx,",
        "Cwm fjord-bank glyhps vext quiz"
    );

    pangram_test!(
        good,
        PangramKind::Good,
        "AaBbcdefghijklmnopqrstuvwxyz",
        "Waltz, bad nymph, for quick jigs vex vec jigs"
    );

    pangram_test!(
        bad,
        PangramKind::Bad,
        "_bcdefghijklmnopqrstuvwxyz",
        "abc",
        "",
        " "
    );
}
