fn main() {
    let mut biggest_bags: Vec<i32> = vec![0, 0, 0];

    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        println!("In Elf Bag: ");
        let mut total_in_bag: i32 = 0;
        for line in group.lines() {
            total_in_bag += line.parse::<i32>().unwrap();
        }
        biggest_bags.sort();
        println!("{}", total_in_bag);
        if total_in_bag > biggest_bags[0] {
            biggest_bags[0] = total_in_bag;
        }
    }

    println!("Biggest Bag: {:?}", biggest_bags.iter().sum::<i32>());
}
