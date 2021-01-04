use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::collections::HashMap;
use std::fmt;

const WIDTH: usize = 12;
const HEIGHT: usize = 10;
const N_CANS: u32 = 25;

const N_GENERATIONS: u32 = 50;
const N_TRIALS: u32 = 1;
const N_STEPS: u32 = 300;

const POPULATION_SIZE: usize = 100;
const SELECTION_SIZE: usize = 20;
const MUTATION_PROBABILITY: f32 = 0.001;

type Gen = Lcg128Xsl64;
type Room = [[Object; WIDTH]; HEIGHT];
type Location = (usize, usize);
type ID = i32;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum Object {
    Empty,
    Can,
    Wall,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '_',
            Self::Can => 'O',
            Self::Wall => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveRandom,
    PickUp,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::MoveUp => 'U',
            Self::MoveDown => 'D',
            Self::MoveLeft => 'L',
            Self::MoveRight => 'R',
            Self::MoveRandom => '?',
            Self::PickUp => 'P',
        };
        write!(f, "{}", c)
    }
}

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

const ALL_OBJECTS: [Object; 3] = [Object::Empty, Object::Can, Object::Wall];

// fn print_state(state: State) {
//     println!("[ {} ]", state.up);
//     println!("[{}{}{}]", state.left, state.center, state.right);
//     println!("[ {} ]", state.down);
// }

// fn print_room(room: Room) {
//     for row in &room {
//         for cell in row {
//             print!("{} ", cell);
//         }
//         println!("");
//     }
// }

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

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
struct State {
    up: Object,
    down: Object,
    left: Object,
    right: Object,
    center: Object,
}

struct Robot {
    id: ID,
    policy: HashMap<State, Action>,
    score: f32,
}

fn create_random_robot(rng: &mut Gen, id: ID) -> Robot {
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
    let mut score = 0;
    let (mut row, mut col) = location;
    // println!("Performing action {}", action);
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

fn evaluate_robot(rng: &mut Gen, robot: &Robot) -> f32 {
    let mut room = create_random_room(rng);
    // print_room(room);
    let mut location = get_random_location(rng);
    let mut total_score = 0;
    for _ in 0..N_TRIALS {
        let mut trial_score = 0;
        for _ in 0..N_STEPS {
            let state = get_state(room, location);
            // println!("Robot at {}, {}", location.0, location.1);
            // print_state(state);
            match robot.policy.get(&state) {
                Some(action) => {
                    let (new_location, step_score) = update_room(rng, &mut room, location, *action);
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

fn crossover_robots(rng: &mut Gen, parent_a: &Robot, parent_b: &Robot, id: ID) -> Robot {
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

    for generation_number in 0..N_GENERATIONS {
        print!("Generation: {}", generation_number);
        for mut robot in &mut population {
            robot.score = evaluate_robot(&mut rng, robot);
        }

        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        population.truncate(SELECTION_SIZE);

        let best_robot = &population[0];
        println!(
            " => best score {} (robot {})",
            best_robot.score, best_robot.id
        );

        while population.len() < POPULATION_SIZE as usize {
            let parent_a = &population[rng.gen_range(0..SELECTION_SIZE)];
            let parent_b = &population[rng.gen_range(0..SELECTION_SIZE)];
            let child = crossover_robots(&mut rng, parent_a, parent_b, id_count);
            id_count += 1;
            population.push(child);
        }
    }
}
