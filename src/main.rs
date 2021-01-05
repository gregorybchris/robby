use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::collections::HashMap;

mod action;
mod object;
mod robot;
mod state;

use action::Action;
use object::Object;
use robot::Robot;
use state::State;

const WIDTH: usize = 15;
const HEIGHT: usize = 15;
const N_CANS: u32 = 70;

const N_GENERATIONS: u32 = 200;
const N_TRIALS: u32 = 1;
const N_STEPS: u32 = 100;

const POPULATION_SIZE: usize = 400;
const SELECTION_SIZE: usize = 30;
const MUTATION_PROBABILITY: f32 = 0.01;

const ALL_OBJECTS: [Object; 3] = [Object::Empty, Object::Can, Object::Wall];

type Gen = Lcg128Xsl64;
type Room = [[Object; WIDTH]; HEIGHT];
type Location = (usize, usize);

fn get_random_action(rng: &mut Gen, move_only: bool) -> Action {
    if move_only {
        match rng.gen_range(0..4) {
            0 => Action::MoveUp,
            1 => Action::MoveDown,
            2 => Action::MoveLeft,
            _ => Action::MoveRight,
        }
    } else {
        match rng.gen_range(0..6) {
            0 => Action::MoveUp,
            1 => Action::MoveDown,
            2 => Action::MoveLeft,
            3 => Action::MoveRight,
            4 => Action::MoveRandom,
            _ => Action::PickUp,
        }
    }
}

fn is_deterministic_action(action: Action) -> bool {
    return action == Action::MoveUp
        || action == Action::MoveDown
        || action == Action::MoveLeft
        || action == Action::MoveRight
        || action == Action::PickUp;
}

fn print_room(room: Room, location: Location) {
    let (row, col) = location;

    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            let cell = room[r][c];
            if r == row && c == col {
                print!("\x1B[31m{}\x1B[0m ", cell);
            } else if (r as i32 - row as i32).abs() < 2 && (c as i32 - col as i32).abs() < 2 {
                print!("\x1B[34m{}\x1B[0m ", cell);
            } else {
                print!("{} ", cell);
            }
        }
        println!("")
    }
}

fn get_random_location(rng: &mut Gen) -> Location {
    let row = rng.gen_range(1..=HEIGHT - 2);
    let col = rng.gen_range(1..=WIDTH - 2);
    (row, col)
}

fn create_random_room(rng: &mut Gen) -> Room {
    let mut room = [[Object::Empty; WIDTH]; HEIGHT];

    // Add walls
    for row in 0..HEIGHT {
        room[row][0] = Object::Wall;
        room[row][WIDTH - 1] = Object::Wall;
    }
    for col in 0..WIDTH {
        room[0][col] = Object::Wall;
        room[HEIGHT - 1][col] = Object::Wall;
    }

    // Add cans
    for _ in 0..N_CANS {
        loop {
            let (row, col) = get_random_location(rng);
            if room[row][col] == Object::Empty {
                room[row][col] = Object::Can;
                break;
            }
        }
    }

    room
}

fn create_random_robot(rng: &mut Gen, id: i32) -> Robot {
    let mut policy: HashMap<State, Action> = HashMap::new();

    // TODO: Fix this to use something like itertools cartesian product
    for up in &ALL_OBJECTS {
        for down in &ALL_OBJECTS {
            for left in &ALL_OBJECTS {
                for right in &ALL_OBJECTS {
                    for center in &ALL_OBJECTS {
                        let state = State {
                            up: *up,
                            down: *down,
                            left: *left,
                            right: *right,
                            center: *center,
                        };
                        if *center == Object::Wall {
                            continue;
                        }
                        if *up == Object::Wall && *down == Object::Wall {
                            continue;
                        }
                        if *left == Object::Wall && *right == Object::Wall {
                            continue;
                        }
                        let action = get_random_action(rng, false);
                        policy.insert(state, action);
                    }
                }
            }
        }
    }

    Robot {
        id,
        policy,
        score: 0.0,
    }
}

fn get_state(room: Room, location: Location) -> State {
    let (row, col) = location;
    assert!(row > 0 && row < HEIGHT - 1);
    assert!(col > 0 && col < WIDTH - 1);

    State {
        up: room[row - 1][col],
        down: room[row + 1][col],
        left: room[row][col - 1],
        right: room[row][col + 1],
        center: room[row][col],
    }
}

fn update_room(
    rng: &mut Gen,
    room: &mut Room,
    location: Location,
    action: Action,
) -> (Location, i32) {
    let mut score: i32 = 0;
    let (mut row, mut col) = location;
    match action {
        Action::MoveUp => {
            if room[row - 1][col] != Object::Wall {
                row -= 1;
            }
        }
        Action::MoveDown => {
            if room[row + 1][col] != Object::Wall {
                row += 1;
            }
        }
        Action::MoveLeft => {
            if room[row][col - 1] != Object::Wall {
                col -= 1;
            }
        }
        Action::MoveRight => {
            if room[row][col + 1] != Object::Wall {
                col += 1;
            }
        }
        Action::MoveRandom => {
            let random_move = get_random_action(rng, true);
            return update_room(rng, room, location, random_move);
        }
        Action::PickUp => {
            if room[row][col] == Object::Can {
                room[row][col] = Object::Empty;
                score += 1;
            }
        }
    };
    ((row, col), score)
}

fn evaluate_robot(rng: &mut Gen, robot: &Robot, debug: bool) -> f32 {
    let mut room = create_random_room(rng);
    let mut location = get_random_location(rng);
    let mut total_score = 0;
    for _ in 0..N_TRIALS {
        let mut trial_score = 0;
        for _ in 0..N_STEPS {
            let state = get_state(room, location);
            match robot.policy.get(&state) {
                Some(action) => {
                    let (new_location, step_score) = update_room(rng, &mut room, location, *action);
                    if debug {
                        println!("\nPerform {}", *action);
                        println!(
                            "Location ({}, {}) -> ({}, {})",
                            location.0, location.1, new_location.0, new_location.1
                        );
                        print_room(room, new_location);
                    }
                    let updated = new_location != location || step_score != 0;
                    if is_deterministic_action(*action) && !updated {
                        break;
                    }
                    location = new_location;
                    trial_score += step_score;
                }
                None => assert!(false, "Unknown policy"),
            };
        }
        total_score += trial_score;
    }
    (total_score as f32) * 1.0 / (N_TRIALS as f32)
}

fn crossover_robots(rng: &mut Gen, parent_a: &Robot, parent_b: &Robot, id: i32) -> Robot {
    let mut policy: HashMap<State, Action> = HashMap::new();
    let parent_fraction: f32 = rng.gen();
    for (state, action_a) in &parent_a.policy {
        let parent_rand: f32 = rng.gen();
        if parent_rand < parent_fraction {
            policy.insert(*state, *action_a);
        } else {
            match parent_b.policy.get(state) {
                Some(action_b) => {
                    policy.insert(*state, *action_b);
                }
                None => assert!(false, "Unknown policy"),
            };
        }
        let mutation_rand: f32 = rng.gen();
        if mutation_rand < MUTATION_PROBABILITY {
            let random_action = get_random_action(rng, false);
            policy.insert(*state, random_action);
        }
    }

    Robot {
        id,
        policy,
        score: 0.0,
    }
}

fn main() {
    let seed = 1;
    let mut rng: Gen = Pcg64::seed_from_u64(seed);

    let mut id_count = 0;

    println!("Creating a population of size: {}", POPULATION_SIZE);
    let mut population: Vec<Robot> = Vec::new();
    for _ in 0..POPULATION_SIZE {
        let robot = create_random_robot(&mut rng, id_count);
        id_count += 1;
        population.push(robot);
    }

    let mut best_robot_id = 0;
    for generation_number in 0..N_GENERATIONS {
        print!("Generation {}", generation_number);

        // Evaluate all robots
        for mut robot in &mut population {
            robot.score = evaluate_robot(&mut rng, robot, false);
        }

        // Drop all robots except some of the best
        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        population.truncate(SELECTION_SIZE);

        // Get the best score obtained
        let best_robot_of_generation = &population[0];
        best_robot_id = best_robot_of_generation.id;
        println!(" => best score {}", best_robot_of_generation.score);

        // Fill population with crossover children of existing robots
        while population.len() < POPULATION_SIZE as usize {
            let parent_a = &population[rng.gen_range(0..SELECTION_SIZE)];
            let parent_b = &population[rng.gen_range(0..SELECTION_SIZE)];
            let child = crossover_robots(&mut rng, parent_a, parent_b, id_count);
            id_count += 1;
            population.push(child);
        }
    }

    // Evaluate best robot again
    let debug = false;
    for robot in &population {
        if robot.id == best_robot_id {
            println!("Previous best score: {}", robot.score);
            for _ in 0..1 {
                let best_score = evaluate_robot(&mut rng, &robot, debug);
                println!("Best test score: {}", best_score);
            }
        }
    }
}
