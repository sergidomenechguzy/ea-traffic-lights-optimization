# Traffic Lights Optimization

CLI tool using one of two variants of evolutionary algorithms to optimize traffic lights on a single street.

Includes:
- data generation or static data
- traffic simulation
- hillclimber with probability bitflip mutation
- genetic algorithm using probability bitflip mutation, 1-point-crossover or 2-point-crossover and tournament selection

## Local Setup

### Prerequisites

You need Rust and Cargo installed to run this cli tool. The easiest way to get both is to simply install rustup [https://rustup.rs/](https://rustup.rs/)

### Installation

Clone the repository and use cargo to install all dependencies, build the tool and run it:
```
cargo run
```

## Configuration

This tool includes lots of configuration options you can set via cli arguments when running `cargo run`.

For example to remove the iterational improvement output run:
```
cargo run -- -s
// or
cargo run -- --silent
```

To list all possible configuration options run:
```
cargo run -- --help
```

### All configuration options
```
-b, --benchmark
        Run optimization a set amount of times (default 20) and show mean of results

    --benchmark-iterations <BENCHMARK_ITERATIONS>
        Number of times to run optimization in benchmark [default: 20]

-d, --data <DATA>
        Car traffic data to use for the traffic simulation [default: fixed] [possible values:
        fixed, generate]

    --disable-increasing-passthrough
        Disable the increasing passthrough to keep max passthrough always the same

    --disable-max-passthrough
        Disable the max passthrough value to not limit cars per timestep

    --fitness-value <FITNESS_VALUE>
        Fitness value to use during optimization [default: ratio] [possible values: ratio,
        difference, driving_cars, waiting_cars]

-h, --help
        Print help information

-i, --iterations <ITERATIONS>
        Number of iterations to run [default: 1000]

    --intersections <INTERSECTIONS>
        Number of intersections for the traffic simulation [default: 8]

-m, --mutation <MUTATION>
        Mutation variant to use [default: prob_bitflip] [possible values: none, bitflip,
        prob_bitflip]

    --main-max-count <MAIN_MAX_COUNT>
        Maximum number of cars possible on the main road [default: 20]

    --main-percentage <MAIN_PERCENTAGE>
        Amount of cars staying on the main road [default: 0.8]

-o, --optimization <OPTIMIZATION>
        Optimization variant to use [default: genetic] [possible values: genetic, hillclimb]

-p, --plot
        Draw plot of best values of each iteration

    --parents-size <PARENTS_SIZE>
        Parent population size [default: 10]

    --population-size <POPULATION_SIZE>
        Population size [default: 50]

    --print-final-simulation
        Print the simulation data for the final best candidate

    --probability-bitflip <PROBABILITY_BITFLIP>
        Probability for bitflip in prob_bitflip mutation [default: 0.0078125]

    --probability-recombination <PROBABILITY_RECOMBINATION>
        Probability for bitflip in prob_bitflip mutation [default: 0.75]

-r, --recombination <RECOMBINATION>
        Mutation variant to use [default: two_point] [possible values: one_point, two_point]

-s, --silent
        Hide output on iterations with improvements

    --side-max-count <SIDE_MAX_COUNT>
        Maximum number of cars possible on the side roads [default: 10]

    --side-percentage <SIDE_PERCENTAGE>
        Amount of cars coming to main road from side roads [default: 0.6]

    --timesteps <TIMESTEPS>
        Number of timesteps for the traffic simulation [default: 16]

    --tournament-size <TOURNAMENT_SIZE>
        Tournament size [default: 5]

-V, --version
        Print version information
```
