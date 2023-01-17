
use advent2015lib::parse_file_to_string;
use std::collections::HashMap;
use std::fmt;


#[derive(Debug)]
pub struct Santa {
    x: i32,
    y: i32,
    map: HashMap<(i32, i32), i32>
}

impl fmt::Display for Santa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Santa:\n\tAt ({}, {})\n\t{:?}", self.x, self.y, self.map)
    }
}

impl Santa {
    pub fn new() -> Self {
        Santa {
            x: 0,
            y: 0,
            map: HashMap::<(i32, i32), i32>::from([ ((0,0), 1) ])
        }
    }

    pub fn visit(self: &mut Self, x: i32, y: i32) {
        self.map.entry((x,y)).and_modify(|v| *v += 1).or_insert(1);
    }

    pub fn unique_houses(&self) -> usize {
        self.map.len()
    }
}

fn main() {
    let inputstr = parse_file_to_string();

    // Part 1
    let mut my_pt1 = Santa::new();
    for arrow in inputstr.chars() {
        match arrow {
            '>' => my_pt1.x += 1,
            '<' => my_pt1.x -= 1,
            '^' => my_pt1.y += 1,
            'v' => my_pt1.y -= 1,
            _ => panic!("Eh?")
        };
        my_pt1.visit(my_pt1.x, my_pt1.y);
    }

    // println!("{}", my_pt1);
    let answer1 = my_pt1.unique_houses();
    println!("Answer to part 1: {answer1}");


    let mut my_santa = Santa::new();
    let mut my_robo = Santa::new();

    let mut san_vec = vec![&mut my_santa, &mut my_robo];
    let num_santas = san_vec.len();
    let mut idx = 0;
    let mut chosen: &mut Santa;
    for arrow in inputstr.chars() {
        chosen = san_vec[idx % num_santas];
        match arrow {
            '>' => chosen.x += 1,
            '<' => chosen.x -= 1,
            '^' => chosen.y += 1,
            'v' => chosen.y -= 1,
            _ => panic!("Eh?")
        };
        chosen.visit(chosen.x, chosen.y);
        idx += 1;
    }

    let mut newmap = HashMap::new();
    for (key, value) in my_santa.map.into_iter() {
        newmap.insert(key, value);
    }
    for (key, value) in my_robo.map.into_iter() {
        newmap.insert(key, value);
    }

    let placeholder = Santa {
        x:0,
        y:0,
        map:newmap
    };
    println!("Answer to part 2: {}", placeholder.unique_houses());

}
