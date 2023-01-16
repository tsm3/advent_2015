
use std::env;
use std::fs;


enum MyParseError {
    NotAParen
}

fn parse_input() -> String {
    let file_path = "resource/input".to_owned();
    let input = fs::read_to_string(file_path)
        .expect("Can't parse??");
    println!("{:?}", input);
    input
}

fn count_paren(sum: i32, paren: i32) -> i32 {
    // println!("{} , {}", sum, paren);
    sum + paren
}

fn map_paren(paren: char) -> Result<i32, MyParseError> {
    match paren {
        '(' => Ok(1),
        ')' => Ok(-1),
        _   => Err(MyParseError::NotAParen)
    }
}

fn main() {
    let input = parse_input();
    let mut acc: i32 = 0;
    let num: Vec<i32> = input.chars()
        .map(map_paren)
        .filter_map(|x| x.ok())
        .collect();
        // .map(|x|)
        // .fold(0, count_paren);
    let mut first_enter_basement = 0;
    for (idx, number) in num.iter().enumerate() {
        acc += number;
        if acc < 0 && first_enter_basement == 0 {
            first_enter_basement = idx +1;
        }
    }
    println!("{acc:?}");
    println!("{first_enter_basement:?}");
}
