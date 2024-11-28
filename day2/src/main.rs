use std::collections::HashMap;
use std::fs;

#[derive(PartialEq)]
enum AgentAction {
    Rock,
    Paper,
    Scissors,
}

enum GameOutcome {
    Win,
    Lose,
    Draw,
}

fn main() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let strategies_ord: Vec<&str> = puzzle_input.lines().collect();

    let points = play_strategies(strategies_ord);
    println!("Points: {points}");
}

fn play_strategies(strats: Vec<&str>) -> u32 {
    let mut total = 0;

    for strat in strats {
        let (opponent_action, goal) = parse_strategy(strat);

        match goal {
            GameOutcome::Win => match opponent_action {
                AgentAction::Rock => {
                    total += 2;
                }
                AgentAction::Paper => {
                    total += 3;
                }
                AgentAction::Scissors => {
                    total += 1;
                }
            },
            GameOutcome::Draw => match opponent_action {
                AgentAction::Rock => {
                    total += 1;
                }
                AgentAction::Paper => {
                    total += 2;
                }
                AgentAction::Scissors => {
                    total += 3;
                }
            },
            GameOutcome::Lose => match opponent_action {
                AgentAction::Rock => {
                    total += 3;
                }
                AgentAction::Paper => {
                    total += 1;
                }
                AgentAction::Scissors => {
                    total += 2;
                }
            },
        }

        match goal {
            GameOutcome::Win => {
                total += 6;
            },
            GameOutcome::Draw => {
                total += 3;
            },
            GameOutcome::Lose => {},
        }
    }

    total
}

fn parse_strategy(strat: &str) -> (AgentAction, GameOutcome) {
    let mut strat_iter = strat.split(' ');

    let first = strat_iter.next().unwrap();
    let second = strat_iter.next().unwrap();

    let first_char = first.parse().unwrap();
    let second_char = second.parse().unwrap();

    let opponent_action;
    let goal;

    if let Some(a) = parse_action(first_char) {
        opponent_action = a;
    } else {
        panic!("Failed to initialize opponent_action in parse_strategy function.");
    }

    if let Some(g) = parse_goal(second_char) {
        goal = g;
    } else {
        panic!("Failed to initialize goal in parse_strategy function.");
    }

    (opponent_action, goal)
}

fn play(opp: AgentAction, response: AgentAction) -> GameOutcome {
    if opp == response {
        return GameOutcome::Draw;
    }

    if opp == AgentAction::Rock {
        if response == AgentAction::Paper {
            return GameOutcome::Win;
        }
    } else if opp == AgentAction::Paper {
        if response == AgentAction::Scissors {
            return GameOutcome::Win;
        }
    } else if opp == AgentAction::Scissors {
        if response == AgentAction::Rock {
            return GameOutcome::Win;
        }
    }

    GameOutcome::Lose
}

fn parse_action(input: char) -> Option<AgentAction> {
    match input {
        'A' => Some(AgentAction::Rock),
        'B' => Some(AgentAction::Paper),
        'C' => Some(AgentAction::Scissors),
        _ => None,
    }
}

fn parse_goal(input: char) -> Option<GameOutcome> {
    match input {
        'X' => Some(GameOutcome::Lose),
        'Y' => Some(GameOutcome::Draw),
        'Z' => Some(GameOutcome::Win),
        _ => None,
    }
}
