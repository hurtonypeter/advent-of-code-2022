#[derive(Debug)]
#[derive(PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Target {
    Loose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Game {
    opponent: Sign,
    player: Sign,
    target: Target,
}

fn parse_game(data: &str) -> Game {
    let draws: Vec<&str> = data.split_whitespace().collect();

    let opponent = match draws[0] {
        "A" => Sign::Rock,
        "B" => Sign::Paper,
        "C" => Sign::Scissor,
        _ => panic!()
    };

    let player = match draws[1] {
        "X" => Sign::Rock,
        "Y" => Sign::Paper,
        "Z" => Sign::Scissor,
        _ => panic!()
    };

    let target = match draws[1] {
        "X" => Target::Loose,
        "Y" => Target::Draw,
        "Z" => Target::Win,
        _ => panic!()
    };

    Game { opponent, player, target }
}

fn parse_input() -> Vec<Game> {
    let content = std::fs::read_to_string("input/day02.txt").unwrap();

    let games = content
        .lines()
        .map(|line| parse_game(line))
        .collect();

    games
}

fn get_game_points(opponent: &Sign, player: &Sign) -> i32 {
    return if
    (opponent == &Sign::Rock && player == &Sign::Paper) ||
        (opponent == &Sign::Paper && player == &Sign::Scissor) ||
        (opponent == &Sign::Scissor && player == &Sign::Rock) { 6 } else if
    (opponent == &Sign::Rock && player == &Sign::Rock) ||
        (opponent == &Sign::Paper && player == &Sign::Paper) ||
        (opponent == &Sign::Scissor && player == &Sign::Scissor) { 3 } else { 0 };
}

fn get_pick_points(pick: &Sign) -> i32 {
    return match pick {
        Sign::Rock => 1,
        Sign::Paper => 2,
        Sign::Scissor => 3
    };
}

pub fn part_one() {
    let before = std::time::Instant::now();

    let games = parse_input();

    let mut sum: i32 = 0;
    for game in games {
        let point_for_game = get_game_points(&game.opponent, &game.player);

        let point_for_pick = get_pick_points(&game.player);

        sum += point_for_game + point_for_pick;
    }

    println!("[{:.2?}]\t Day 2 part 1 result: {}", before.elapsed(), sum);
}

pub fn part_two() {
    let before = std::time::Instant::now();

    let games = parse_input();

    let mut sum: i32 = 0;
    for game in games {
        let pick: Sign = if game.opponent == Sign::Rock && game.target == Target::Win {
            Sign::Paper
        } else if game.opponent == Sign::Rock && game.target == Target::Draw {
            Sign::Rock
        } else if game.opponent == Sign::Rock && game.target == Target::Loose {
            Sign::Scissor
        } else if game.opponent == Sign::Paper && game.target == Target::Win {
            Sign::Scissor
        } else if game.opponent == Sign::Paper && game.target == Target::Draw {
            Sign::Paper
        } else if game.opponent == Sign::Paper && game.target == Target::Loose {
            Sign::Rock
        } else if game.opponent == Sign::Scissor && game.target == Target::Win {
            Sign::Rock
        } else if game.opponent == Sign::Scissor && game.target == Target::Draw {
            Sign::Scissor
        } else if game.opponent == Sign::Scissor && game.target == Target::Loose {
            Sign::Paper
        } else {
            panic!();
        };

        let point_for_game = get_game_points(&game.opponent, &pick);

        let point_for_pick = get_pick_points(&pick);

        sum += point_for_game + point_for_pick;
    }

    println!("[{:.2?}]\t Day 2 part 2 result: {}", before.elapsed(), sum);
}