use hex;

use md5;

fn main() {
    // let mut inputstr = "abcdef".to_owned();
    let mut inputstr = "iwrupvqb".to_owned();
    let mut mutstr = String::new();
    for i in 0.. {  
        mutstr = inputstr.clone();
        mutstr.push_str(&i.to_string());

        let num = md5::compute(mutstr);
        let bruh: [u8; 16] = num.try_into().unwrap();
        // Change below line to get 5 or 6 0s
        if hex::encode(bruh).starts_with("000000") {
            dbg!(num);
            dbg!(i);
            return
        }

    }
}
