use crate::data::calculate_main_max_passthrough;
use crate::data::calculate_min_count;
use crate::data::calculate_side_max_passthrough;
use crate::data::fixed_data;
use crate::data::generate_data;
use crate::data::ConfigurationData;
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
    /// Hide output on iterations with improvements
    #[clap(short, long)]
    silent: bool,

    /// Print the simulation data for the final best candidate
    #[clap(long)]
    print_final_simulation: bool,

    /// Run optimization a set amount of times (default 20) and show mean of results
    #[clap(short, long)]
    benchmark: bool,

    /// Number of times to run optimization in benchmark
    #[clap(long, default_value_t = 20)]
    benchmark_iterations: i32,

    /// Draw plot of best values of each iteration
    #[clap(long)]
    plot: bool,

    /// Car traffic data to use for the traffic simulation
    #[clap(short, long, default_value = "fixed", possible_values = ["fixed", "generate"])]
    data: String,

    /// Maximum number of cars possible on the main road
    #[clap(long, default_value_t = 20)]
    main_max_count: i32,

    /// Maximum number of cars possible on the side roads
    #[clap(long, default_value_t = 10)]
    side_max_count: i32,

    /// Number of iterations to run
    #[clap(short, long, default_value_t = 1000)]
    iterations: usize,

    /// Optimization variant to use
    #[clap(short, long, default_value = "genetic", possible_values = ["genetic", "hillclimb"])]
    optimization: String,

    /// Mutation variant to use
    #[clap(short, long, default_value = "prob_bitflip", possible_values = ["none", "bitflip", "prob_bitflip"])]
    mutation: String,

    /// Probability for bitflip in prob_bitflip mutation
    #[clap(long, default_value_t = 0.0078125)]
    probability_bitflip: f64,

    /// Probability for bitflip in prob_bitflip mutation
    #[clap(long, default_value_t = 0.75)]
    probability_recombination: f64,

    /// Population size
    #[clap(long, default_value_t = 50)]
    population_size: usize,

    /// Parent population size
    #[clap(long, default_value_t = 10)]
    parents_size: usize,

    /// Tournament size
    #[clap(long, default_value_t = 5)]
    tournament_size: usize,

    /// Fitness value to use during optimization
    #[clap(long, default_value = "difference", possible_values = ["difference", "driving_cars", "waiting_cars"])]
    fitness_value: String,

    /// Number of intersections for the traffic simulation
    #[clap(long, default_value_t = 8)]
    intersections: usize,

    /// Number of timesteps for the traffic simulation
    #[clap(long, default_value_t = 16)]
    timesteps: usize,

    /// Disable the max passthrough value to not limit cars per timestep
    #[clap(long)]
    disable_max_passthrough: bool,

    /// Amount of cars staying on the main road
    #[clap(long, default_value_t = 0.8)]
    main_percentage: f64,

    /// Amount of cars coming to main road from side roads
    #[clap(long, default_value_t = 0.6)]
    side_percentage: f64,
}

fn main() {
    let args = Args::parse();

    let configuration_data = ConfigurationData {
        silent: args.silent,
        print_final_simulation: args.print_final_simulation,
        benchmark: args.benchmark,
        benchmark_iterations: args.benchmark_iterations,
        plot: args.plot,
        data: args.data,
    };

    let generation_data = GenerationData {
        main_max_count: args.main_max_count,
        side_max_count: args.side_max_count,
        main_min_count: calculate_min_count(args.main_max_count),
        side_min_count: calculate_min_count(args.side_max_count),
    };

    let traffic_data;
    if configuration_data.data == "generate" {
        traffic_data = generate_data(args.intersections, args.timesteps, &generation_data);
        println!("{:?}", traffic_data);
    } else {
        traffic_data = fixed_data();
    }

    let optimization_data = OptimizationData {
        iterations: args.iterations,
        optimization: args.optimization,
        mutation: args.mutation,
        probability_bitflip: args.probability_bitflip,
        probability_recombination: args.probability_recombination,
        population_size: args.population_size,
        parents_size: args.parents_size,
        tournament_size: args.tournament_size,
        fitness_value: args.fitness_value,
    };

    let simulation_data = SimulationData {
        intersections: args.intersections,
        timesteps: args.timesteps,
        traffic_data,
        disable_max_passthrough: args.disable_max_passthrough,
        main_max_passthrough: calculate_main_max_passthrough(args.main_max_count),
        side_max_passthrough: calculate_side_max_passthrough(args.side_max_count),
        main_percentage: args.main_percentage,
        side_percentage: args.side_percentage,
    };

    if args.benchmark {
        let mut accumulated_results = 0;
        for _ in 0..args.benchmark_iterations {
            accumulated_results +=
                optimize(&configuration_data, &optimization_data, &simulation_data);
        }
        println!(
            "Mean of best individual over {} iterations: {}",
            args.benchmark_iterations,
            accumulated_results / args.benchmark_iterations
        );
    } else {
        optimize(&configuration_data, &optimization_data, &simulation_data);
    }
}
