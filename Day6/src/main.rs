use i
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    // // let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    // let mut buff: Vec<char> = vec![];
    // let mut chars_no_repeat = 0;
    // let window: i32 = 14;
    // for (idx, c) in input.chars().enumerate() {
    //     if buff.len() > (window - 1) as usize {
    //         buff.remove(0);
    //     }

    //     if buff.contains(&c) {
    //         let index_of_repeated = buff.iter().enumerate().find(|x| x.1 == &c).unwrap().0;
    //         chars_no_repeat -= index_of_repeated as i32;
    //         buff.drain(0..=index_of_repeated);
    //     } else {
    //         chars_no_repeat += 1;
    //     }

    //     buff.push(c);

    //     if chars_no_repeat == window {
    //         println!("Found index: {}", idx + 1);
    //         break;
    //     }
    // }
    fn find_how_long_til_n_unique_chars(input: &str, n: usize) -> Option<usize> {
        input
            .as_bytes()
            .windows(n)
            .position(|chars| chars.iter().all_unique())
            .map(|idx| idx + n)
    }
    let result = find_how_long_til_n_unique_chars(&input, 14).unwrap();
    println!("Index: {}", result)
}
