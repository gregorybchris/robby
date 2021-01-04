use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng,
};
use rand_pcg::{Lcg128Xsl64, Pcg64};
use std::collections::HashMap;
use std::fmt;

type Gen = Lcg128Xsl64;

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

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
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

const WIDTH: usize = 12;
const HEIGHT: usize = 10;
const N_CANS: u32 = 20;

// const N_GENERATIONS: u32 = 100;
// const N_ITERATIONS: u32 = 3;
// const N_STEPS: u32 = 500;

// const POPULATION_SIZE: u32 = 300;
// const SELECTION_SIZE: u32 = 20;
// const MUTATION_PROBABILITY: f32 = 0.001;

const ALL_OBJECTS: [Object; 3] = [Object::Empty, Object::Can, Object::Wall];

fn get_random_location(rng: &mut Gen) -> (usize, usize) {
    let row = rng.gen_range(1..=HEIGHT - 2);
    let col = rng.gen_range(1..=WIDTH - 2);
    (row, col)
}

fn create_random_room(rng: &mut Gen) -> [[Object; WIDTH]; HEIGHT] {
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
struct Stimulus {
    up: Object,
    down: Object,
    left: Object,
    right: Object,
    center: Object,
}

struct Robot {
    id: i32,
    generation: i32,
    actions: HashMap<Stimulus, Action>,
}

fn create_random_robot(rng: &mut Gen, id: i32, generation: i32) -> Robot {
    let random_value: f32 = rng.gen();
    println!("{}", random_value);

    let mut actions = HashMap::new();
    // TODO: Fix this to use something like itertools cartesian product
    for up in ALL_OBJECTS.iter() {
        for down in ALL_OBJECTS.iter() {
            for left in ALL_OBJECTS.iter() {
                for right in ALL_OBJECTS.iter() {
                    for center in ALL_OBJECTS.iter() {
                        let stimulus = Stimulus {
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
                        let action: Action = rand::random();
                        actions.insert(stimulus, action);
                    }
                }
            }
        }
    }

    let robot = Robot {
        id,
        generation,
        actions,
    };
    robot
}

fn print_stimulus(stimulus: Stimulus) {
    println!("[ {} ]", stimulus.up);
    println!("[{}{}{}]", stimulus.left, stimulus.center, stimulus.right);
    println!("[ {} ]", stimulus.down);
}

fn main() {
    let mut rng: Gen = Pcg64::seed_from_u64(0);

    let room = create_random_room(&mut rng);

    for row in &room {
        for cell in row {
            print!("{} ", cell);
        }
        println!("");
    }
    let generation = 0;
    let id = 0;
    let robot = create_random_robot(&mut rng, id, generation);

    println!("ID: {}", robot.id);
    println!("Generation: {}", robot.generation);
    println!("Number of actions: {}", robot.actions.len());

    for (stimulus, action) in robot.actions {
        print_stimulus(stimulus);
        println!("-> {}\n", action);
    }

    // let stimulus = Stimulus {
    //     up: Object::Can,
    //     down: Object::Can,
    //     left: Object::Can,
    //     right: Object::Can,
    //     center: Object::Can,
    // };
    // match robot.actions.get(&stimulus) {
    //     Some(v) => println!("{}", v),
    //     None => println!("No value"),
    // };
}
