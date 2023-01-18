use advent2015lib::parse_file_to_string;
use fancy_regex::Regex;
use lazy_static::lazy_static;


fn three_vowels(linestr: &str) -> bool {
    let vowelvec = vec!['a', 'e', 'i', 'o', 'u'];
    let mut sum = 0;
    for v in vowelvec {
        sum += linestr.matches(v).count();
    }
    sum >= 3
}

fn double_letter(linestr: &str) -> bool {
    lazy_static!{
        static ref DOUBLE: Regex = Regex::new("(.)\\1{1,}").unwrap();
    }
    DOUBLE.is_match(linestr).unwrap()
}

fn not_naughty(linestr: &str) -> bool {
    lazy_static!{
        static ref NAUGHTY: Regex = Regex::new("ab|cd|pq|xy").unwrap();
    }
    !NAUGHTY.is_match(linestr).unwrap()
}

fn is_nice_pt1(linestr: &str) -> bool {
    // >= 3 vowels
    // >= 1 double letter (dd)
    // does not contain any of (ab, cd, pq, xy)
    three_vowels(linestr) && double_letter(linestr) && not_naughty(linestr)
}

fn is_nice_pt2(linestr: &str) -> bool {
    // a pair of letters appearing twice AND not overlapping
    // a letter sandwich
    lazy_static!{
        static ref PAIRNOLAP: Regex = Regex::new("(..).*\\1").unwrap();
        static ref SANDWICH: Regex = Regex::new("(.).\\1").unwrap();
    }
    PAIRNOLAP.is_match(linestr).unwrap() && SANDWICH.is_match(linestr).unwrap()
}


fn main() {
    let inputstr = parse_file_to_string();
    let answer1: usize = inputstr.lines()
        .filter(|x| is_nice_pt1(x))
        .count();
    dbg!(answer1);

    let answer2: usize = inputstr.lines()
    .filter(|x| is_nice_pt2(x))
    .count();
    dbg!(answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let teststr = "bruhhrub".to_owned();
        assert!(double_letter(&teststr));
    }
}