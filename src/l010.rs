use std::cmp::min;

#[derive(Clone, Debug, PartialEq)]
enum Symbol {
    Specific(char),
    Any,
    Repeat,
}

impl Symbol {
    fn is_suitable(&self, c: char) -> bool {
        match self {
            Symbol::Specific(self_c) => *self_c == c,
            _ => true,
        }
    }
}

fn as_char(c_str: &str) -> char {
    c_str.chars().next().unwrap()
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        let any = as_char(".");
        let repeat = as_char("*");

        match value {
            c if c == any => Self::Any,
            c if c == repeat => Self::Repeat,
            c => Self::Specific(c),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Repeat {
    Number(usize),
    AtLeast(usize),
    Any,
}

impl Repeat {
    fn allow_zero(&self) -> bool {
        vec![Repeat::Any, Repeat::AtLeast(0), Repeat::Number(0)].contains(self)
    }

    fn decrement(&mut self) {
        match self {
            Repeat::Number(ref mut n) => *n -= 1,
            Repeat::AtLeast(ref mut al) if *al > 0 => *al -= 1,
            _ => {}
        }
    }

    fn is_decremental(&self) -> bool {
        match self {
            Repeat::Number(n) => *n > 0,
            Repeat::AtLeast(al) => *al > 0,
            _ => true
        }
    }
}

#[derive(Clone, Debug)]
struct Pattern {
    symbol: Symbol,
    repeat: Repeat,
}

impl Pattern {
    fn new(c: char) -> Self {
        Pattern { symbol: c.into(), repeat: Repeat::Number(1) }
    }
}

fn get_patterns(text: String) -> Vec<Pattern> {
    let mut result: Vec<Pattern> = vec![];

    for c in text.chars() {
        match result.last_mut() {
            None => result.push(Pattern::new(c)),
            Some(pattern) => {
                let c_symbol: Symbol = c.into();
                match c_symbol {
                    Symbol::Repeat => match pattern.repeat {
                        Repeat::Number(n) => pattern.repeat = match n <= 1 {
                            false => Repeat::AtLeast(n - 1),
                            true => Repeat::Any
                        },
                        Repeat::AtLeast(al) => pattern.repeat = match al <= 1 {
                            false => Repeat::AtLeast(al - 1),
                            true => Repeat::Any,
                        },
                        Repeat::Any => {}
                    },
                    c_symbol if c_symbol == pattern.symbol => pattern.repeat = match pattern.repeat {
                        Repeat::Number(n) => Repeat::Number(n + 1),
                        Repeat::AtLeast(al) => Repeat::AtLeast(al + 1),
                        Repeat::Any => Repeat::AtLeast(1),
                    },
                    _ => result.push(Pattern::new(c))
                }
            }
        };
    }

    result
}

fn is_match(chars: Vec<char>, mut patterns: Vec<Pattern>) -> bool {
    let ps_copy = patterns.clone();
    let less_patterns = ps_copy[min(1, ps_copy.len())..].to_vec();
    match chars.first() {
        None => match patterns.first_mut() {
            None => true,
            Some(p) => match p.repeat.allow_zero() {
                true => is_match(vec![], less_patterns),
                false => false,
            }
        },
        Some(&c) => match patterns.first_mut() {
            None => false,
            Some(p) => {
                let less_chars = chars[1..].to_vec();
                let lc_copy = less_chars.clone();
                let lp_copy = less_patterns.clone();
                let p_copy = ps_copy.clone();
                let c_copy = chars.clone();
                let pi_copy = p.clone();
                let check_all_less_options = move || match pi_copy.symbol.is_suitable(c) {
                    true => is_match(lc_copy.clone(), lp_copy.clone())
                        || is_match(lc_copy.clone(), p_copy)
                        || is_match(c_copy.clone(), lp_copy.clone()),
                    false => is_match(c_copy, lp_copy),
                };
                match p.repeat {
                    Repeat::Any => check_all_less_options(),
                    Repeat::AtLeast(_) => match p.repeat.is_decremental() {
                        true => match p.symbol.is_suitable(c) {
                            true => {
                                p.repeat.decrement();
                                is_match(less_chars, patterns)
                            }
                            false => false,
                        },
                        false => check_all_less_options(),
                    },
                    Repeat::Number(_) => match p.repeat.is_decremental() {
                        true => match p.symbol.is_suitable(c) {
                            true => {
                                p.repeat.decrement();
                                is_match(less_chars, patterns)
                            },
                            false => false,
                        },
                        false => is_match(chars, less_patterns),
                    }
                }
            }
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let patterns = get_patterns(p);

        is_match(chars, patterns)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::Solution;

    #[rstest]
    #[case("aa", "a", false)]
    #[case("aa", "a*", true)]
    #[case("ab", ".*", true)]
    #[case("aab", "c*a*b", true)]
    #[case("aaa", "a.a", true)]
    #[case("aaa", "a*a", true)]
    #[case("aaa", "ab*a*c*a", true)]
    #[case("a", "ab*a", false)]
    #[case("a", "..*", true)]
    #[case("abbbcd", "ab*bbbcd", true)]
    #[case("bbbaccbbbaababbac", ".b*b*.*...*.*c*.", true)]
    #[case("bbcacbabbcbaaccabc", "b*a*a*.c*bb*b*.*.*", true)]
    fn test(#[case] s: String, #[case] p: String, #[case] expected: bool) {
        let result = Solution::is_match(s, p);
        assert_eq!(result, expected);
    }
}
