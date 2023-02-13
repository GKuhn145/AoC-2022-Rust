use std::fs;

#[derive(Debug)]
struct Stack {
    content: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Self { content: vec![] }
    }
}

fn stack_parser(stack_str: Vec<&str>) -> Vec<Stack> {
    let num_stacks = ((stack_str[0].len() + 1) / 4) as usize;
    let mut result: Vec<Stack> = vec![];
    for i in 0..num_stacks {
        let mut stack = Stack::new();
        for line in stack_str.iter() {
            let character = line.chars().nth(4 * i + 1).unwrap();
            if character != ' ' {
                stack.content.push(character);
            }
        }
        stack.content.reverse();
        result.push(stack);
    }
    result
}

fn move_parser(moves_str: Vec<&str>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for move_str in moves_str.iter() {
        let moves_vec: Vec<i32> = move_str
            .split(" ")
            .into_iter()
            .filter(|&x| x.chars().any(|c| c.is_digit(10)))
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        result.push(moves_vec);
    }
    result
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let stack_and_moves = input.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", stack_and_moves);
    let stack_str = stack_and_moves[0].lines().collect::<Vec<&str>>();
    let moves_str = stack_and_moves[1].lines().collect::<Vec<&str>>();

    let mut stacks = stack_parser(stack_str);
    println!("Stacks Before: {:?}", stacks);

    let moves = move_parser(moves_str);

    for instruction in moves {
        println!("\ninstruction : {:?}", instruction);
        let move_from = &mut stacks[(instruction[1] - 1) as usize];
        let mut vec_of_chars_to_add = vec![];
        for _ in 0..instruction[0] {
            let char_to_move = move_from.content.pop().unwrap();
            vec_of_chars_to_add.push(char_to_move);
        }
        let move_to = &mut stacks[(instruction[2] - 1) as usize];
        vec_of_chars_to_add.reverse();
        for c in vec_of_chars_to_add {
            move_to.content.push(c);
        }
    }

    let mut response = "".to_string();
    for stack in stacks {
        response.push(*stack.content.last().unwrap())
    }

    println!("Stacks after: {:?}", response);
}
