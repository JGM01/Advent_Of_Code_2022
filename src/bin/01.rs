pub fn part_one(input: &str) -> Option<u32> {
    let mut most_calories: u32 = 0;
    let clean_input = input.replace("\r", "");
    let rations_lists = clean_input.split("\n\n");
    
    for elf in rations_lists {
        let rations: Vec<u32> = elf.lines().map(|x| x.parse().unwrap()).collect();
        let total_calories = rations.iter().sum();
        if total_calories > most_calories {
            most_calories = total_calories;
        }
    }
    
    Some(most_calories)
    
}

pub fn part_two(input: &str) -> Option<u32> {    
    let mut top_three: Vec<u32> = vec![0; 3];
    
    let clean_input = input.replace("\r", "");
    let rations_lists = clean_input.split("\n\n");
    
    for elf in rations_lists {
        let rations: Vec<u32> = elf.lines().map(|x| x.parse().unwrap()).collect();
        let total_calories = rations.iter().sum();
        
        top_three = change_rankings(top_three, total_calories)
    }
    
    let most_calories: u32 = top_three.iter().sum();
    
    Some(most_calories)
}


fn change_rankings(mut current_ranking: Vec<u32>, new_entry: u32) -> Vec<u32> {
    if new_entry > current_ranking[0] {
        current_ranking[2] = current_ranking[1];
        current_ranking[1] = current_ranking[0];
        current_ranking[0] = new_entry;
    } else if new_entry > current_ranking[1] {
        current_ranking[2] = current_ranking[1];
        current_ranking[1] = new_entry;
    } else if new_entry > current_ranking[2] {
        current_ranking[2] = new_entry;
    }
    current_ranking
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
