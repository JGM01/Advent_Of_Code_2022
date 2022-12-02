pub fn part_one(input: &str) -> Option<u32> {
    
    let answers: u32 = input.lines().map(|x| {
                let bytes = x.as_bytes();
                let you = bytes[2] as char;
                let opp = bytes[0] as char;
                
                match you {
                    'X' => {
                        match opp {
                            'A' => 4,
                            'B' => 1,
                            'C' => 7,
                            _ => 0
                        }
                    },
                    'Y' => {
                        match opp {
                            'A' => 8,
                            'B' => 5,
                            'C' => 2,
                            _ => 0
                        }
                    },
                    'Z' => {
                        match opp {
                            'A' => 3,
                            'B' => 9,
                            'C' => 6,
                            _ => 0
                        }
                    },
                    _ => 0
                }
            })
            .sum();
    Some(answers)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    
    for round in input.lines() {
        let choices: Vec<&str> = round.split_whitespace().collect();
        let opp = choices[0];
        let outcome = choices[1];
        let mut score: u32 = 0;
        
        match opp {
            "A" => {
                match outcome {
                    "X" => score += 3,
                    "Y" => score += 4,
                    "Z" => score += 8,
                    _ => {}
                }
            },
            "B" => {
                match outcome {
                    "X" => score += 1,
                    "Y" => score += 5,
                    "Z" => score += 9,
                    _ => {}
                }
            },
            "C" => {
               match outcome {
                    "X" => score += 2,
                    "Y" => score += 6,
                    "Z" => score += 7,
                    _ => {}
                }
            },
            _ => {}
        }
        total_score += score;
    }
    
    Some(total_score)
}

enum Shape {
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2,
}

enum RoundState {
    LOSS = 0,
    DRAW = 1,
    WIN = 2,
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}