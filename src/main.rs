use std::fs;

const BAG: [(&'static str, i32); 3] = [("blue", 14), ("green", 13), ("red", 12)];

fn main() {
    let content = fs::read_to_string("./input.txt").expect("File should exit.");
    let result: i32 = content
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(get_valid_game_id)
        .sum();

    println!("Sum of all game ID is: {result}");

    let power: i32 = content
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(get_power_values)
        .sum();
    
    println!("Sum of the power is: {power}");
}

fn get_valid_game_id(value: &&str) -> i32 {
    let game_info = value.split(':').collect::<Vec<&str>>();
    let games = game_info[1]
        .split(';')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim().split(", ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let game_id: i32 = game_info[0]
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .last()
        .unwrap()
        .parse()
        .expect("ID should be a number");

    if games.iter().all(check_game) {
        game_id
    } else {
        0
    }
}

fn check_game(hints: &Vec<&str>) -> bool {
    let mut result = true;

    for hint in hints {
        if let [amount, color] = hint.split(' ').collect::<Vec<&str>>()[0..] {
            let amount: i32 = amount
                .trim()
                .parse()
                .expect("Amount should be a valid number");

            result = result
                && BAG
                    .iter()
                    .any(|cube_in_bag| cube_in_bag.0 == color && cube_in_bag.1 >= amount);
        } else {
            panic!("Should be a valid cube information");
        }
    }

    result
}

fn get_power_values(value: &&str) -> i32 {
    let game_info = value.split(':').collect::<Vec<&str>>();
    let games = game_info[1]
        .split(';')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim().split(", ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>(); 

    let mut minimum_game_value: [(&str, i32); 3] = [
        ("blue", std::i32::MIN),
        ("green", std::i32::MIN),
        ("red", std::i32::MIN)
    ];
    let mut index = 0;


    for game in games {
        for round in game {
            if let [amount, color] = round.split(' ').collect::<Vec<&str>>()[0..] {
                let amount: i32 = amount
                    .trim()
                    .parse()
                    .expect("Amount should be a valid number");

                while index < minimum_game_value.len() {
                    if minimum_game_value[index].0 == color && minimum_game_value[index].1 < amount {
                        minimum_game_value[index].1 = amount;
                    }

                    index += 1;
                }

                index = 0;
            } else {
                panic!("Should be a valid cube information");
            }
        }
    }

    minimum_game_value.iter().map(|x| x.1).reduce(|acc, current| acc * current).expect("Should have valid values")
}
