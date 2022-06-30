use bit_vec::BitVec;
use rand::Rng;

use crate::data::OptimizationData;

pub fn get_best_and_worst_candidate(
    population: &Vec<Vec<BitVec>>,
    values: &Vec<i32>,
) -> (Vec<BitVec>, i32, i32) {
    let mut highest_index: usize = 0;
    let mut lowest_index: usize = 0;

    for i in 0..values.len() {
        if values[i] > values[highest_index] {
            highest_index = i;
        }
        if values[i] < values[lowest_index] {
            lowest_index = i;
        }
    }

    (
        population[highest_index].clone(),
        values[highest_index].clone(),
        values[lowest_index].clone(),
    )
}

pub fn get_mean_value(values: &Vec<i32>) -> i32 {
    values.iter().sum::<i32>() / values.len() as i32
}

pub fn calculate_skaled_fitness(population_values: &Vec<i32>, best: &i32, worst: &i32) -> Vec<f64> {
    let fitness: Vec<f64> = population_values
        .iter()
        .map(|&value| (value - worst) as f64 / (best - worst) as f64)
        .collect();
    fitness
}

pub fn calculate_rank_based_fitness(population_values: &Vec<i32>) -> Vec<f64> {
    let mut ordered_values: Vec<(usize, i32)> = Vec::with_capacity(population_values.len());

    for (index, value) in population_values.iter().enumerate() {
        ordered_values.push((index, value.clone()));
    }
    ordered_values.sort_by(|(_, a_value), (_, b_value)| b_value.cmp(a_value));

    let mut fitness: Vec<f64> = vec![0.0; population_values.len()];
    for (index, (original_index, _)) in ordered_values.iter().enumerate() {
        fitness[*original_index] = (2.0 / population_values.len() as f64)
            * (1.0 - ((index + 1) as f64 - 1.0) / (population_values.len() as f64 - 1.0));
    }
    fitness
}

pub fn distinct_random(min: usize, max: usize, count: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut random_values: Vec<usize> = Vec::with_capacity(count);
    while random_values.len() != count {
        let random_index = rng.gen_range(min..max);
        if !random_values.contains(&random_index) {
            random_values.push(random_index);
        }
    }
    random_values
}

pub fn tournament(
    population_values: &Vec<i32>,
    optimization_data: &OptimizationData,
) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut winners: Vec<usize> = Vec::with_capacity(optimization_data.parents_size);

    for _ in 0..optimization_data.parents_size {
        let mut selected: Vec<usize> = Vec::with_capacity(optimization_data.tournament_size);
        while selected.len() != optimization_data.tournament_size {
            let random_index = rng.gen_range(0..population_values.len());
            if !selected.contains(&random_index) && !winners.contains(&random_index) {
                selected.push(random_index);
            }
        }

        let mut winner: usize = selected[0];
        for index in selected.iter() {
            if population_values[*index] > population_values[winner] {
                winner = *index;
            }
        }
        winners.push(winner);
    }
    winners
}
