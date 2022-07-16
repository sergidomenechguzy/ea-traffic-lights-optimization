use bit_vec::BitVec;
use rand::random;
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct ConfigurationData {
    pub silent: bool,
    pub print_final_simulation: bool,
    pub benchmark: bool,
    pub benchmark_iterations: i32,
    pub plot: bool,
    pub data: String,
}

#[derive(Debug)]
pub struct GenerationData {
    pub intersections: usize,
    pub timesteps: usize,
    pub main_max_count: i32,
    pub side_max_count: i32,
    pub main_min_count: i32,
    pub side_min_count: i32,
}

#[derive(Debug)]
pub struct SimulationData {
    pub traffic_data: Vec<Vec<TrafficState>>,
    pub disable_increasing_passthrough: bool,
    pub disable_max_passthrough: bool,
    pub max_passthrough: i32,
    pub main_percentage: f64,
    pub side_percentage: f64,
}

#[derive(Debug)]
pub struct OptimizationData {
    pub iterations: usize,
    pub optimization: String,
    pub mutation: String,
    pub recombination: String,
    pub probability_bitflip: f64,
    pub probability_recombination: f64,
    pub population_size: usize,
    pub parents_size: usize,
    pub tournament_size: usize,
    pub fitness_value: String,
}

#[derive(Clone)]
pub struct TrafficState {
    pub main_from_prev: i32,
    pub main_from_next: i32,
    pub side: i32,
}

impl fmt::Debug for TrafficState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n{{main_from_prev: {:02}, main_from_next: {:02}, side: {:02}}}",
            self.main_from_prev, self.main_from_next, self.side
        )
    }
}

fn random_traffic_value(generation_data: &GenerationData, main: bool) -> i32 {
    let mut rng = rand::thread_rng();
    if main {
        return rng.gen_range(generation_data.main_min_count..generation_data.main_max_count);
    }
    rng.gen_range(generation_data.side_min_count..generation_data.side_max_count)
}

fn build_traffic_state_initial(generation_data: &GenerationData) -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(generation_data, true),
        main_from_next: random_traffic_value(generation_data, true),
        side: random_traffic_value(generation_data, false),
    }
}

fn build_traffic_state_base(generation_data: &GenerationData) -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: 0,
        side: random_traffic_value(generation_data, false),
    }
}

fn build_traffic_state_first_intersection(generation_data: &GenerationData) -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(generation_data, true),
        main_from_next: 0,
        side: random_traffic_value(generation_data, false),
    }
}

fn build_traffic_state_last_intersection(generation_data: &GenerationData) -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: random_traffic_value(generation_data, true),
        side: random_traffic_value(generation_data, false),
    }
}

pub fn build_empty_traffic_state() -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: 0,
        side: 0,
    }
}

pub fn calculate_min_count(max: i32) -> i32 {
    ((max as f64) / 3.0).round() as i32
}

pub fn calculate_max_passthrough(max: i32) -> i32 {
    ((max as f64) * 0.8).round() as i32
}

pub fn calculate_increased_max_passthrough(max: i32) -> i32 {
    ((max as f64) * 1.4).round() as i32
}

pub fn generate_data(generation_data: &GenerationData) -> Vec<Vec<TrafficState>> {
    let mut data: Vec<Vec<TrafficState>> = Vec::with_capacity(generation_data.intersections);
    for index in 0..generation_data.intersections {
        let mut traffic_data: Vec<TrafficState> = Vec::with_capacity(generation_data.timesteps);

        let random_initial_traffic = build_traffic_state_initial(generation_data);
        traffic_data.push(random_initial_traffic);

        for _ in 1..generation_data.timesteps {
            if index == 0 {
                traffic_data.push(build_traffic_state_first_intersection(generation_data));
            } else if index == generation_data.intersections - 1 {
                traffic_data.push(build_traffic_state_last_intersection(generation_data));
            } else {
                traffic_data.push(build_traffic_state_base(generation_data));
            }
        }
        let new_intersection = traffic_data;
        data.push(new_intersection);
    }
    data
}

pub fn generate_candidate(intersections: usize, timesteps: usize) -> Vec<BitVec> {
    let mut candidate: Vec<BitVec> = Vec::with_capacity(intersections);
    for _ in 0..intersections {
        let mut data = BitVec::with_capacity(timesteps);
        for _ in 0..timesteps {
            data.push(random())
        }
        candidate.push(data);
    }
    candidate
}

pub fn generate_population(
    population_size: usize,
    intersections: usize,
    timesteps: usize,
) -> Vec<Vec<BitVec>> {
    let mut population: Vec<Vec<BitVec>> = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        let candidate = generate_candidate(intersections, timesteps);
        population.push(candidate);
    }
    population
}

// 8 by 8 fixed data
// pub fn fixed_data() -> Vec<Vec<TrafficState>> {
//     let traffic_data: Vec<Vec<TrafficState>> = vec![
//         vec![
//             TrafficState {
//                 main_from_prev: 14,
//                 main_from_next: 13,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 17,
//                 main_from_next: 0,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 12,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 13,
//                 main_from_next: 0,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 16,
//                 main_from_next: 0,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 16,
//                 main_from_next: 0,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 19,
//                 main_from_next: 0,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 16,
//                 main_from_next: 0,
//                 side: 7,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 12,
//                 main_from_next: 19,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 7,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 19,
//                 main_from_next: 9,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 9,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 8,
//                 main_from_next: 11,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 15,
//                 main_from_next: 10,
//                 side: 9,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 6,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 16,
//                 main_from_next: 16,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 8,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 19,
//                 main_from_next: 13,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 9,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 0,
//                 side: 5,
//             },
//         ],
//         vec![
//             TrafficState {
//                 main_from_prev: 14,
//                 main_from_next: 9,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 17,
//                 side: 6,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 18,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 18,
//                 side: 3,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 15,
//                 side: 5,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 12,
//                 side: 7,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 12,
//                 side: 4,
//             },
//             TrafficState {
//                 main_from_prev: 0,
//                 main_from_next: 13,
//                 side: 8,
//             },
//         ],
//     ];

//     traffic_data
// }

// 8 by 16 fixed data
pub fn fixed_data() -> Vec<Vec<TrafficState>> {
    let traffic_data: Vec<Vec<TrafficState>> = vec![
        vec![
            TrafficState {
                main_from_prev: 07,
                main_from_next: 17,
                side: 08,
            },
            TrafficState {
                main_from_prev: 18,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 12,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 08,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 09,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 18,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 08,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 13,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 17,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 09,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 15,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 13,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 16,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 10,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 17,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 09,
                main_from_next: 00,
                side: 08,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 18,
                main_from_next: 08,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 18,
                main_from_next: 13,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 15,
                main_from_next: 09,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 17,
                main_from_next: 11,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 10,
                main_from_next: 10,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 07,
                main_from_next: 14,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 08,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 05,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 00,
                side: 09,
            },
        ],
        vec![
            TrafficState {
                main_from_prev: 07,
                main_from_next: 16,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 16,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 08,
                side: 09,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 08,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 09,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 13,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 15,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 16,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 15,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 13,
                side: 03,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 11,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 07,
                side: 04,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 17,
                side: 06,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 13,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 10,
                side: 07,
            },
            TrafficState {
                main_from_prev: 00,
                main_from_next: 11,
                side: 09,
            },
        ],
    ];

    traffic_data
}
