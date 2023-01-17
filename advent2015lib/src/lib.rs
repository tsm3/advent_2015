use std::fs;

pub fn parse_file_to_string() -> String {
    // let file_path = "resource/input".to_owned();
    let input = fs::read_to_string("resource/input")
        .expect("Can't parse??");
    // println!("{:?}", input);
    input
}

#[cfg(test)]
mod tests {
    use super::*;

}
