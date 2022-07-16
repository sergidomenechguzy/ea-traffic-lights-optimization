use crate::data::{
    calculate_max_passthrough, calculate_min_count, fixed_data, generate_data, ConfigurationData,
    GenerationData, OptimizationData, SimulationData,
};
use chrono::{DateTime, Local};
use clap::Parser;
use optimization::optimize;
use plotters::prelude::{
    BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, LineSeries,
};
use plotters::style::{BLUE, WHITE};
use std::time::Instant;
use utils::get_highest_and_lowest;

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
    #[clap(short, long)]
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

    /// Mutation variant to use
    #[clap(short, long, default_value = "two_point", possible_values = ["one_point", "two_point"])]
    recombination: String,

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
    #[clap(long, default_value = "ratio", possible_values = ["ratio", "difference", "driving_cars", "waiting_cars"])]
    fitness_value: String,

    /// Number of intersections for the traffic simulation
    #[clap(long, default_value_t = 8)]
    intersections: usize,

    /// Number of timesteps for the traffic simulation
    #[clap(long, default_value_t = 16)]
    timesteps: usize,

    /// Disable the increasing passthrough to keep max passthrough always the same
    #[clap(long)]
    disable_increasing_passthrough: bool,

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
        intersections: args.intersections,
        timesteps: args.timesteps,
        main_max_count: args.main_max_count,
        side_max_count: args.side_max_count,
        main_min_count: calculate_min_count(args.main_max_count),
        side_min_count: calculate_min_count(args.side_max_count),
    };

    let traffic_data;
    if configuration_data.data == "generate" {
        traffic_data = generate_data(&generation_data);
        println!("{:?}", traffic_data);
    } else {
        traffic_data = fixed_data();
    }

    let optimization_data = OptimizationData {
        iterations: args.iterations,
        optimization: args.optimization,
        mutation: args.mutation,
        recombination: args.recombination,
        probability_bitflip: args.probability_bitflip,
        probability_recombination: args.probability_recombination,
        population_size: args.population_size,
        parents_size: args.parents_size,
        tournament_size: args.tournament_size,
        fitness_value: args.fitness_value,
    };

    let simulation_data = SimulationData {
        traffic_data,
        disable_increasing_passthrough: args.disable_increasing_passthrough,
        disable_max_passthrough: args.disable_max_passthrough,
        max_passthrough: calculate_max_passthrough(args.main_max_count),
        main_percentage: args.main_percentage,
        side_percentage: args.side_percentage,
    };

    let mut plot_data: Vec<f64> = Vec::new();
    if configuration_data.plot {
        plot_data = Vec::with_capacity(optimization_data.iterations);
    }

    if configuration_data.benchmark {
        let mut accumulated_results = 0.0;
        let mut accumulated_durations = 0.0;
        for _ in 0..configuration_data.benchmark_iterations {
            let start = Instant::now();
            accumulated_results += optimize(
                &configuration_data,
                &optimization_data,
                &simulation_data,
                &generation_data,
                &mut plot_data,
            );
            accumulated_durations += start.elapsed().as_secs_f64();
        }
        println!(
            "Mean of best individual over {} iterations: {:.4}",
            configuration_data.benchmark_iterations,
            accumulated_results / configuration_data.benchmark_iterations as f64
        );
        println!(
            "Mean of optimization duration over {} iterations: {:.4}s",
            configuration_data.benchmark_iterations,
            accumulated_durations / configuration_data.benchmark_iterations as f64
        );
    } else {
        optimize(
            &configuration_data,
            &optimization_data,
            &simulation_data,
            &generation_data,
            &mut plot_data,
        );
    }

    if configuration_data.plot {
        let now: DateTime<Local> = Local::now();
        let mut plot_path = String::from("plots/");
        plot_path.push_str(&optimization_data.optimization);
        plot_path.push_str("--");
        if optimization_data.optimization == "genetic" {
            plot_path.push_str(&optimization_data.recombination);
            plot_path.push_str("--");
            plot_path.push_str(&optimization_data.population_size.to_string());
            plot_path.push_str("--");
        }
        plot_path.push_str(&optimization_data.iterations.to_string());
        plot_path.push_str("--");
        plot_path.push_str(&now.format("%F-%H-%M-%S").to_string());
        plot_path.push_str(".png");

        let plot_draw_area = BitMapBackend::new(&plot_path, (800, 600)).into_drawing_area();
        plot_draw_area.fill(&WHITE).unwrap();

        let (hightest_index, lowest_index) = get_highest_and_lowest(&plot_data);
        let plot_min = (plot_data[lowest_index] * 10.0).floor() / 10.0;
        let plot_max = (plot_data[hightest_index] * 10.0).ceil() / 10.0;

        let mut ctx = ChartBuilder::on(&plot_draw_area)
            .margin(30)
            .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .set_label_area_size(LabelAreaPosition::Left, 20)
            .build_cartesian_2d(0..optimization_data.iterations, plot_min..plot_max)
            .unwrap();

        ctx.configure_mesh()
            .light_line_style(&WHITE)
            .draw()
            .unwrap();

        ctx.draw_series(LineSeries::new(
            (0..optimization_data.iterations).map(|x| (x, plot_data[x])),
            &BLUE,
        ))
        .unwrap();
    }
}
