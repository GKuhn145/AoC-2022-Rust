use std::fs;

fn part1(input: &str) -> i32 {
    let range_pairs: Vec<&str> = input.lines().collect();
    let mut count = 0;
    for range_pair in range_pairs {
        let splited_range: Vec<&str> = range_pair.split(",").collect();

        let vec1: Vec<i32> = splited_range[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let vec2: Vec<i32> = splited_range[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let range1 = vec1[0]..=vec1[1];
        let range2 = vec2[0]..=vec2[1];

        if (range1.contains(&vec2[0]) && range1.contains(&vec2[1]))
            || (range2.contains(&vec1[0]) && range2.contains(&vec1[1]))
        {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> i32 {
    let range_pairs: Vec<&str> = input.lines().collect();
    let mut count = 0;
    for range_pair in range_pairs {
        let splited_range: Vec<&str> = range_pair.split(",").collect();

        let vec1: Vec<i32> = splited_range[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let vec2: Vec<i32> = splited_range[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let range1 = vec1[0]..=vec1[1];
        let range2 = vec2[0]..=vec2[1];

        if (range1.contains(&vec2[0]) || range1.contains(&vec2[1]))
            || (range2.contains(&vec1[0]) || range2.contains(&vec1[1]))
        {
            count += 1;
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let result1 = part1(&input);
    println!("Result part 1: {}", result1);

    let result2 = part2(&input);
    println!("Result part 2: {}", result2);
}
