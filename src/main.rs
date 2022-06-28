use crate::data::calculate_main_max_passthrough;
use crate::data::calculate_min_count;
use crate::data::calculate_side_max_passthrough;
use crate::data::fixed_data;
use crate::data::generate_data;
use crate::data::GenerationData;
use crate::data::OptimizationData;
use crate::data::SimulationData;
use clap::Parser;
use optimization::optimize;

pub mod data;
pub mod optimization;
pub mod simulation;
pub mod utils;

/// Evolutionary algorithm to optimize traffic lights on a linear road with intersections
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of iterations to run
    #[clap(short, long, default_value_t = 1000)]
    iterations: usize,

    /// Optimization variant to use
    #[clap(short, long, default_value = "default", possible_values = ["default", "simple"])]
    optimization: String,

    /// Mutation variant to use
    #[clap(short, long, default_value = "prob_bitflip", possible_values = ["none", "bitflip", "prob_bitflip"])]
    mutation: String,

    /// Probability for bitflip in prob_bitflip mutation
    #[clap(short, long, default_value_t = 0.013)]
    probability: f64,

    /// Population size
    #[clap(short = 'P', long, default_value_t = 50)]
    population_size: usize,

    /// Population size
    #[clap(long, default_value_t = 10)]
    parents_size: usize,

    /// Tournament size
    #[clap(short, long, default_value_t = 5)]
    tournament_size: usize,

    /// Number of intersections for the traffic simulation
    #[clap(long, default_value_t = 8)]
    intersections: usize,

    /// Number of timesteps for the traffic simulation
    #[clap(long, default_value_t = 16)]
    timesteps: usize,

    /// Maximum number of cars possible on the main road
    #[clap(long, default_value_t = 20)]
    main_max_count: i32,

    /// Maximum number of cars possible on the side roads
    #[clap(long, default_value_t = 10)]
    side_max_count: i32,

    /// Amount of cars staying on the main road
    #[clap(long, default_value_t = 0.8)]
    main_percentage: f64,

    /// Amount of cars coming to main road from side roads
    #[clap(long, default_value_t = 0.6)]
    side_percentage: f64,

    /// Car traffic data to use for the traffic simulation
    #[clap(long, default_value = "fixed", possible_values = ["fixed", "generate"])]
    data: String,

    /// Use a max passthrough value to limit cars per timestep
    #[clap(short, long)]
    use_max_passthrough: bool,
}

fn main() {
    let args = Args::parse();

    let generation_data = GenerationData {
        main_max_count: args.main_max_count,
        side_max_count: args.side_max_count,
        main_min_count: calculate_min_count(args.main_max_count),
        side_min_count: calculate_min_count(args.side_max_count),
    };

    let traffic_data;
    if args.data == "generate" {
        traffic_data = generate_data(args.intersections, args.timesteps, &generation_data);
    } else {
        traffic_data = fixed_data();
    }

    let simulation_data = SimulationData {
        intersections: args.intersections,
        timesteps: args.timesteps,
        traffic_data,
        use_max_passthrough: args.use_max_passthrough,
        main_max_passthrough: calculate_main_max_passthrough(args.main_max_count),
        side_max_passthrough: calculate_side_max_passthrough(args.side_max_count),
        main_percentage: args.main_percentage,
        side_percentage: args.side_percentage,
    };

    let optimization_data = OptimizationData {
        iterations: args.iterations,
        optimization: args.optimization,
        mutation: args.mutation,
        probability: args.probability,
        population_size: args.population_size,
        parents_size: args.parents_size,
        tournament_size: args.tournament_size,
    };

    optimize(&optimization_data, &simulation_data);
}
