// extern crate advent2015lib;

use std::str::Split;

use advent2015lib::parse_file_to_string;

pub struct AdBox (
    i32,
    i32,
    i32,
);

impl AdBox {
    fn from_vec(trip: Vec<i32>) -> Self {
        AdBox(trip[0], trip[1], trip[2])
    }

    fn calc_paper_area(self: &Self) -> i32 {
        let sidesvec = vec![
            self.0*self.1,
            self.0*self.2,
            self.1*self.2
        ];
        sidesvec.iter().fold(*sidesvec.iter().min().expect("Bruh"), |acc, x| acc+2*x)
    
    }

    fn calc_ribbon_length(self: &Self) -> i32 {
        let mut lengthsvec = vec![
            self.0,
            self.1,
            self.2
        ];
        lengthsvec.sort();
        2*lengthsvec[0] + 2*lengthsvec[1] + lengthsvec.iter().fold(1, |acc, x| acc*x)
    }
}

fn split_to_boxes(bruh: Split<&str>) -> AdBox {
    let bruh: Vec<i32> = bruh.map(|x| x.parse::<i32>().expect("Oops"))
        .collect();
    AdBox::from_vec(bruh)
}


fn main() {
    let inputstr = parse_file_to_string();
    // let inputvec = parse_string_day2(&inputstr);
    let thing: Vec<AdBox> = inputstr.lines()
        .map(|mystr| 
            mystr.split("x"))
        .map(split_to_boxes)
        .collect();

    // println!("{inputstr}");

    let answer1 = thing.iter()
        .map(|x| x.calc_paper_area())
        .fold(0, |acc, x| acc+x);

    println!("Square feet of paper needed: {answer1}");

    let answer2 = thing.iter()
        .map(|x| x.calc_ribbon_length())
        .fold(0, |acc, x| acc+x);

        println!("Feet of ribbon needed: {answer2}");
    
}

