pub fn part_one(input: &str) -> Option<u32> {
    let mut most_calories: u32 = 0;
    
    for elf in input.replace("\r", "").split("\n\n") {
        let rations: Vec<u32> = elf.lines().map(|x| x.parse().unwrap()).collect();
        let total_calories = rations.iter().sum();
        if total_calories > most_calories {
            most_calories = total_calories;
        }
    }
    Some(most_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vector: Vec<u32> = Vec::new();
    
    for elf in input.replace("\r", "").split("\n\n") {
        let rations: Vec<u32> = elf.lines().map(|x| x.parse().unwrap()).collect();
        let total_calories: u32 = rations.iter().sum();
        
        match vector.binary_search(&total_calories) {
            Ok(_pos) => {}
            Err(pos) => vector.insert(pos, total_calories),
        }
    }
    Some(vector[0] + vector[1] + vector[2])
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
