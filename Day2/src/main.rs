const LOST: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

enum Input {
    Rock,
    Paper,
    Scissors,
}

trait Value {
    fn win_val(&self) -> i32;
    fn draw_val(&self) -> i32;
    fn lost_val(&self) -> i32;
}

impl Value for Input {
    fn win_val(&self) -> i32 {
        match self {
            Input::Rock => 2,
            Input::Paper => 3,
            Input::Scissors => 1,
        }
    }
    fn draw_val(&self) -> i32 {
        match self {
            Input::Rock => 1,
            Input::Paper => 2,
            Input::Scissors => 3,
        }
    }
    fn lost_val(&self) -> i32 {
        match self {
            Input::Rock => 3,
            Input::Paper => 1,
            Input::Scissors => 2,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lost,
}

impl Outcome {
    fn play(self, input: &Input) -> i32 {
        match self {
            Outcome::Win => input.win_val() + WIN,
            Outcome::Draw => input.draw_val() + DRAW,
            Outcome::Lost => input.lost_val() + LOST,
        }
    }
}

impl Input {
    #[allow(dead_code)]
    fn compare(self, other: &Input) -> i32 {
        let score: i32 = match self {
            Input::Paper => {
                let points = 2;
                match other {
                    Input::Paper => points + DRAW,
                    Input::Rock => points + WIN,
                    Input::Scissors => points + LOST,
                }
            }
            Input::Rock => {
                let points = 1;
                match other {
                    Input::Paper => points + LOST,
                    Input::Rock => points + DRAW,
                    Input::Scissors => points + WIN,
                }
            }
            Input::Scissors => {
                let points = 3;
                match other {
                    Input::Paper => points + WIN,
                    Input::Rock => points + LOST,
                    Input::Scissors => points + DRAW,
                }
            }
        };
        score
    }
}

fn main() {
    let mut total_score: i32 = 0;
    for line in include_str!("input.txt").replace("\r\n", "\n").split("\n") {
        let splited_line: Vec<_> = line.split(" ").collect();
        let player1: Option<Input> = match splited_line[0] {
            "A" => Some(Input::Rock),
            "B" => Some(Input::Paper),
            "C" => Some(Input::Scissors),
            _ => None,
        };

        let outcome = match splited_line[1] {
            "X" => Some(Outcome::Lost),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        };
        let curr_score = outcome.unwrap().play(&player1.unwrap());

        println!("{:?} = {}", splited_line, curr_score);
        total_score += curr_score;
    }
    println!("{}", total_score);
}
