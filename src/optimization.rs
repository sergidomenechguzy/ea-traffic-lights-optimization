use crate::data::generate_candidate;
use crate::data::generate_population;
use crate::data::OptimizationData;
use crate::data::SimulationData;
use crate::simulation::simulate;
use crate::simulation::simulate_population;
use crate::utils::distinct_random;
use crate::utils::get_best_and_worst_candidate;
use crate::utils::get_median_value;
use crate::utils::tournament;
use bit_vec::BitVec;
use rand::Rng;

fn bitflip(input: &Vec<BitVec>) -> Vec<BitVec> {
    let mut modified = input.clone();
    let mut rng = rand::thread_rng();
    let index1 = rng.gen_range(0..modified.len());
    let index2 = rng.gen_range(0..modified[index1].len());
    match modified[index1].get(index2) {
        Some(prev) => modified[index1].set(index2, !prev),
        None => {}
    }
    modified
}

fn probability_bitflip(input: &Vec<BitVec>, probability: f64) -> Vec<BitVec> {
    let mut modified = input.clone();
    let mut rng = rand::thread_rng();
    // let mut bits_modified = 0;

    for index1 in 0..modified.len() {
        for index2 in 0..modified[index1].len() {
            if rng.gen::<f64>() < probability {
                match modified[index1].get(index2) {
                    Some(prev) => {
                        modified[index1].set(index2, !prev);
                        // bits_modified += 1;
                    }
                    None => {}
                }
            }
        }
    }
    // println!("{}", bits_modified);
    modified
}

fn mutation(candidate: &Vec<BitVec>, optimization_data: &OptimizationData) -> Vec<BitVec> {
    let mutated_candidate;
    if optimization_data.mutation == "prob_bitflip" {
        mutated_candidate = probability_bitflip(candidate, optimization_data.probability);
    } else if optimization_data.mutation == "bitflip" {
        mutated_candidate = bitflip(candidate);
    } else {
        mutated_candidate = candidate.clone();
    }
    mutated_candidate
}

fn one_point_crossover(
    input1: &Vec<BitVec>,
    input2: &Vec<BitVec>,
    simulation_data: &SimulationData,
    // results: &mut Vec<Vec<BitVec>>,
) -> (Vec<BitVec>, Vec<BitVec>) {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(1..simulation_data.timesteps);

    let mut crossover1: Vec<BitVec> = Vec::with_capacity(simulation_data.intersections);
    let mut crossover2: Vec<BitVec> = Vec::with_capacity(simulation_data.intersections);

    for index in 0..simulation_data.intersections {
        let mut bitvec1 = input1[index].clone();
        let mut bitvec2 = input2[index].clone();

        let mut splitoff1 = bitvec1.split_off(random_index);
        let mut splitoff2 = bitvec2.split_off(random_index);

        bitvec1.append(&mut splitoff2);
        bitvec2.append(&mut splitoff1);

        crossover1.push(bitvec1);
        crossover2.push(bitvec2);
    }

    // results.push(crossover1);
    // results.push(crossover2);
    (crossover1, crossover2)
}

fn recombination(
    candidate1: &Vec<BitVec>,
    candidate2: &Vec<BitVec>,
    simulation_data: &SimulationData,
) -> (Vec<BitVec>, Vec<BitVec>) {
    // let mut results: Vec<Vec<BitVec>> = Vec::with_capacity(2);
    // one_point_crossover(candidate2, candidate1, simulation_data, &mut results);
    // results

    one_point_crossover(candidate1, candidate2, simulation_data)
}

fn selection(
    population: &Vec<Vec<BitVec>>,
    population_values: &Vec<i32>,
    optimization_data: &OptimizationData,
    simulation_data: &SimulationData,
) -> Vec<Vec<BitVec>> {
    let selected = tournament(population_values, optimization_data);
    let mut next_population: Vec<Vec<BitVec>> =
        Vec::with_capacity(optimization_data.population_size);
    for _ in 0..optimization_data.population_size / 2 {
        let randoms = distinct_random(0, selected.len(), 2);
        let (recomb1, recomb2) = recombination(
            &population[selected[randoms[0]]],
            &population[selected[randoms[1]]],
            simulation_data,
        );

        next_population.push(mutation(&recomb1, optimization_data));
        next_population.push(mutation(&recomb2, optimization_data));
    }
    next_population
}

fn optimize_simple(optimization_data: &OptimizationData, simulation_data: &SimulationData) {
    let mut candidate =
        generate_candidate(simulation_data.intersections, simulation_data.timesteps);
    let mut candidate_value = simulate(&simulation_data, &candidate);
    println!("0:\t{:?}\t{}", candidate, candidate_value);

    for it in 0..optimization_data.iterations {
        let mutated_candidate = mutation(&candidate, optimization_data);

        let mutated_candidate_value = simulate(&simulation_data, &mutated_candidate);
        if candidate_value < mutated_candidate_value {
            candidate = mutated_candidate;
            candidate_value = mutated_candidate_value;

            println!("{}:\t{:?}\t{}", it + 1, candidate, candidate_value);
        }
    }
}

fn optimize_default(optimization_data: &OptimizationData, simulation_data: &SimulationData) {
    let mut population = generate_population(
        optimization_data.population_size,
        simulation_data.intersections,
        simulation_data.timesteps,
    );
    let mut population_values = simulate_population(simulation_data, &population);
    let (mut best, mut best_value, _) =
        get_best_and_worst_candidate(&population, &population_values);
    println!(
        "0:\t{:?}\t{}\t{}",
        best,
        best_value,
        get_median_value(&population_values)
    );

    for it in 0..optimization_data.iterations {
        let next_population = selection(
            &population,
            &population_values,
            optimization_data,
            simulation_data,
        );

        let next_population_values = simulate_population(simulation_data, &next_population);

        population = next_population;
        population_values = next_population_values;

        let (next_best, next_best_value, _) =
            get_best_and_worst_candidate(&population, &population_values);
        if next_best_value > best_value {
            best = next_best;
            best_value = next_best_value;

            println!(
                "{}:\t{:?}\t{}\t{}",
                it + 1,
                best,
                best_value,
                get_median_value(&population_values)
            );
        }
    }
}

pub fn optimize(optimization_data: &OptimizationData, simulation_data: &SimulationData) {
    if optimization_data.optimization == "default" {
        optimize_default(optimization_data, simulation_data);
    } else if optimization_data.optimization == "simple" {
        optimize_simple(optimization_data, simulation_data);
    }
}
