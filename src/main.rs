mod action;
mod agent;
mod color;
mod object;
mod state;

use action::Action;
use agent::Agent;
use color::{print_color, Color};
use object::Object;
use state::State;

use itertools::iproduct;
use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::collections::BTreeMap;

const SEED: u64 = 0;

const WIDTH: usize = 15;
const HEIGHT: usize = 15;
const N_GOALS: u32 = 90;

const N_GENERATIONS: u32 = 200;
const N_TRIALS: u32 = 1;
const N_STEPS: u32 = 150;

const POPULATION_SIZE: usize = 500;
const SELECTION_SIZE: usize = 20;
const MUTATION_PROBABILITY: f32 = 0.005;

type Gen = Lcg128Xsl64;
type World = [[Object; WIDTH]; HEIGHT];
type Location = (usize, usize);

fn get_random_action(rng: &mut Gen) -> Action {
    match rng.gen_range(0..6) {
        0 => Action::MoveUp,
        1 => Action::MoveDown,
        2 => Action::MoveLeft,
        3 => Action::MoveRight,
        4 => Action::MoveRandom,
        _ => Action::PickUp,
    }
}

fn get_random_move(rng: &mut Gen) -> Action {
    match rng.gen_range(0..4) {
        0 => Action::MoveUp,
        1 => Action::MoveDown,
        2 => Action::MoveLeft,
        _ => Action::MoveRight,
    }
}

fn udistance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    let (x1, y1, x2, y2) = (x1 as f32, y1 as f32, x2 as f32, y2 as f32);
    ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
}

fn print_world(world: World, location: Location) {
    let (row, col) = location;
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            let object_string = format!("{} ", world[r][c]);
            let distance = udistance(r, c, row, col);
            if distance == 0.0 {
                print_color(object_string, Color::Red);
            } else if distance < 2.0 {
                print_color(object_string, Color::Blue);
            } else {
                print_color(object_string, Color::Default);
            }
        }
        println!("")
    }
}

fn get_random_location(rng: &mut Gen) -> Location {
    (rng.gen_range(1..=HEIGHT - 2), rng.gen_range(1..=WIDTH - 2))
}

fn create_random_world(rng: &mut Gen) -> World {
    assert!(WIDTH > 3);
    assert!(HEIGHT > 3);

    let mut world = [[Object::Empty; WIDTH]; HEIGHT];

    // Add walls
    for row in 0..HEIGHT {
        world[row][0] = Object::Wall;
        world[row][WIDTH - 1] = Object::Wall;
    }
    for col in 0..WIDTH {
        world[0][col] = Object::Wall;
        world[HEIGHT - 1][col] = Object::Wall;
    }

    // Add goals
    for _ in 0..N_GOALS {
        loop {
            let (row, col) = get_random_location(rng);
            if world[row][col] == Object::Empty {
                world[row][col] = Object::Goal;
                break;
            }
        }
    }

    world
}

fn create_random_agent(rng: &mut Gen, id: i32) -> Agent {
    let mut policy: BTreeMap<State, Action> = BTreeMap::new();

    let objects: [Object; 3] = [Object::Empty, Object::Goal, Object::Wall];
    for (up, down, left, right, center) in
        iproduct!(&objects, &objects, &objects, &objects, &objects)
    {
        // Impossible to get to these states
        let on_wall = *center == Object::Wall;
        let small_vertical = *up == Object::Wall && *down == Object::Wall;
        let small_horizontal = *left == Object::Wall && *right == Object::Wall;
        if on_wall || small_vertical || small_horizontal {
            continue;
        }

        let state = State {
            up: *up,
            down: *down,
            left: *left,
            right: *right,
            center: *center,
        };
        let action = get_random_action(rng);
        policy.insert(state, action);
    }

    Agent {
        id,
        policy,
        score: 0.0,
    }
}

fn get_state(world: World, location: Location) -> State {
    let (row, col) = location;
    assert!(row > 0 && row < HEIGHT - 1);
    assert!(col > 0 && col < WIDTH - 1);

    State {
        up: world[row - 1][col],
        down: world[row + 1][col],
        left: world[row][col - 1],
        right: world[row][col + 1],
        center: world[row][col],
    }
}

fn update_world(
    rng: &mut Gen,
    world: &mut World,
    location: Location,
    action: Action,
) -> (Location, i32) {
    let mut score: i32 = 0;
    let (mut row, mut col) = location;
    match action {
        Action::MoveUp => {
            if world[row - 1][col] != Object::Wall {
                row -= 1;
            }
        }
        Action::MoveDown => {
            if world[row + 1][col] != Object::Wall {
                row += 1;
            }
        }
        Action::MoveLeft => {
            if world[row][col - 1] != Object::Wall {
                col -= 1;
            }
        }
        Action::MoveRight => {
            if world[row][col + 1] != Object::Wall {
                col += 1;
            }
        }
        Action::MoveRandom => {
            let random_move = get_random_move(rng);
            return update_world(rng, world, location, random_move);
        }
        Action::PickUp => {
            if world[row][col] == Object::Goal {
                world[row][col] = Object::Empty;
                score += 1;
            }
        }
    };
    ((row, col), score)
}

fn evaluate_agent(rng: &mut Gen, agent: &Agent, debug: bool) -> f32 {
    if debug {
        println!("Evaluating agent {}", agent.id);
    }
    let mut world = create_random_world(rng);
    let mut location = get_random_location(rng);
    let mut total_score = 0;
    for _ in 0..N_TRIALS {
        let mut trial_score = 0;
        for _ in 0..N_STEPS {
            let state = get_state(world, location);
            match agent.policy.get(&state) {
                Some(action) => {
                    let (new_location, step_score) =
                        update_world(rng, &mut world, location, *action);
                    if debug {
                        println!("Perform {}", *action);
                        println!(
                            "Location ({}, {}) -> ({}, {})",
                            location.0, location.1, new_location.0, new_location.1
                        );
                        print_world(world, new_location);
                    }
                    let updated = new_location != location || step_score != 0;
                    if *action != Action::MoveRandom && !updated {
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

fn crossover_agents(rng: &mut Gen, parent_a: &Agent, parent_b: &Agent, id: i32) -> Agent {
    let mut policy: BTreeMap<State, Action> = BTreeMap::new();
    let parent_fraction: f32 = rng.gen();
    for (state, action_a) in &parent_a.policy {
        // Take state-action pair from random parent
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

        // Random mutation
        let mutation_rand: f32 = rng.gen();
        if mutation_rand < MUTATION_PROBABILITY {
            policy.insert(*state, get_random_action(rng));
        }
    }

    Agent {
        id,
        policy,
        score: 0.0,
    }
}

fn main() {
    let mut rng: Gen = Pcg64::seed_from_u64(SEED);

    let mut id_count = 0;

    println!("Creating a population of size: {}", POPULATION_SIZE);
    let mut population: Vec<Agent> = Vec::new();
    for _ in 0..POPULATION_SIZE {
        let agent = create_random_agent(&mut rng, id_count);
        id_count += 1;
        population.push(agent);
    }

    let mut best_agent_id = 0;
    for generation_number in 0..N_GENERATIONS {
        print!("Generation {}", generation_number);

        // Evaluate all agents
        for mut agent in &mut population {
            agent.score = evaluate_agent(&mut rng, agent, false);
        }

        // Drop all agents except some of the best
        population.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        population.truncate(SELECTION_SIZE);

        // Get the best score obtained
        let best_agent_of_generation = &population[0];
        best_agent_id = best_agent_of_generation.id;
        println!(" => best score {}", best_agent_of_generation.score);

        // Fill population with crossover children of existing agents
        while population.len() < POPULATION_SIZE as usize {
            let parent_a = &population[rng.gen_range(0..SELECTION_SIZE)];
            let parent_b = &population[rng.gen_range(0..SELECTION_SIZE)];
            let child = crossover_agents(&mut rng, parent_a, parent_b, id_count);
            id_count += 1;
            population.push(child);
        }
    }

    // Perform a final evaluation on the best agent to mitigate bias
    for agent in &population {
        if agent.id == best_agent_id {
            for _ in 0..1 {
                let best_score = evaluate_agent(&mut rng, &agent, false);
                println!("Best agent final score: {}", best_score);
            }
        }
    }
}
