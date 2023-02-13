fn part1(input: &str) -> u32 {
    let mut common_letters: Vec<u32> = vec![];
    for line in input.split("\n") {
        let (sec1, sec2) = line.split_at(line.len() / 2);
        for char in sec1.chars() {
            if sec2.contains(char) {
                if char.is_uppercase() {
                    common_letters.push(char as u32 - 38);
                } else {
                    common_letters.push(char as u32 - 96);
                }
                break;
            }
        }
    }
    common_letters.iter().sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut count = 0;
    let mut group: Vec<&str> = vec![];
    let mut groups: Vec<Vec<&str>> = vec![];
    for line in input.split("\n") {
        if count > 2 {
            groups.push(group);
            group = vec![];
            count = 0;
        }
        group.push(line);
        count += 1;
    }
    let mut common_letters: Vec<u32> = vec![];
    for group in groups {
        let mut common_chars_in_first_pair: Vec<_> = vec![];
        for char in group[0].chars() {
            if group[1].contains(char) {
                common_chars_in_first_pair.push(char);
            }
        }

        for char in common_chars_in_first_pair {
            if group[2].contains(char) {
                println!("Group: {:?}, common_char: {}", group, char);
                if char.is_uppercase() {
                    common_letters.push(char as u32 - 38);
                } else {
                    common_letters.push(char as u32 - 96);
                }
                break;
            }
        }
    }
    common_letters.iter().sum::<u32>()
}

fn main() {
    let input = include_str!("input.txt");
    let output1 = part1(input);
    let output2 = part2(input);
    println!("part 1 solution: {}", output1);
    println!("part 2 solution: {}", output2);
}
