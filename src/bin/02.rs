pub fn part_one(input: &str) -> Option<u32> {
    
    let mut total_score: u32 = 0;
    
    for round in input.lines() {
        let choices: Vec<&str> = round.split_whitespace().collect();
        let opp = choices[0];
        let you = choices[1];
        let mut score: u32 = 0;
        
        match you {
            "X" => {
                score += 1;
                
                match opp {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => {}
                }
            },
            "Y" => {
                score += 2;
                
                match opp {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => {}
                }
            },
            "Z" => {
                score += 3;
                
                match opp {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => {}
                }
            },
            _ => {}
        }
        total_score += score;
    }
    
    Some(total_score)
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

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}