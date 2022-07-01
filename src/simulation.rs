use crate::data::build_empty_traffic_state;
use crate::data::OptimizationData;
use crate::data::SimulationData;
use crate::data::TrafficState;
use bit_vec::BitVec;
use std::cmp::min;

fn extract_step(traffic_data: &Vec<Vec<TrafficState>>, t: usize) -> Vec<TrafficState> {
    let mut step_data: Vec<TrafficState> = Vec::new();
    for intersection in traffic_data.iter() {
        match intersection.get(t) {
            Some(traffic_data) => step_data.push(traffic_data.clone()),
            None => step_data.push(build_empty_traffic_state()),
        }
    }
    step_data
}

fn calc_next(val: i32, fac: f64) -> i32 {
    ((val as f64) * fac).floor() as i32
}

fn apply_main(
    traffic_to_update: &mut Vec<TrafficState>,
    current_traffic: &TrafficState,
    index: usize,
    driving_cars: &mut i32,
    waiting_cars: &mut i32,
    simulation_data: &SimulationData,
) {
    let main_from_prev;
    let main_from_next;
    if !simulation_data.disable_max_passthrough {
        main_from_prev = min(
            simulation_data.main_max_passthrough,
            current_traffic.main_from_prev,
        );
        main_from_next = min(
            simulation_data.main_max_passthrough,
            current_traffic.main_from_next,
        );
    } else {
        main_from_prev = current_traffic.main_from_prev;
        main_from_next = current_traffic.main_from_next;
    }

    *driving_cars += main_from_prev;
    *driving_cars += main_from_next;

    *waiting_cars += current_traffic.main_from_prev - main_from_prev;
    *waiting_cars += current_traffic.main_from_next - main_from_next;
    *waiting_cars += current_traffic.side;

    match traffic_to_update.get_mut(index) {
        Some(next_traffic_current) => {
            next_traffic_current.side += current_traffic.side;
            next_traffic_current.main_from_prev += current_traffic.main_from_prev - main_from_prev;
            next_traffic_current.main_from_next += current_traffic.main_from_next - main_from_next;
        }
        None => {}
    }
    match traffic_to_update.get_mut(index + 1) {
        Some(next_traffic_next) => {
            next_traffic_next.main_from_prev +=
                calc_next(main_from_prev, simulation_data.main_percentage)
        }
        None => {}
    }
    if index > 0 {
        match traffic_to_update.get_mut(index - 1) {
            Some(next_traffic_prev) => {
                next_traffic_prev.main_from_next +=
                    calc_next(main_from_next, simulation_data.main_percentage)
            }
            None => {}
        }
    }
}

fn apply_side(
    traffic_to_update: &mut Vec<TrafficState>,
    current_traffic: &TrafficState,
    index: usize,
    driving_cars: &mut i32,
    waiting_cars: &mut i32,
    simulation_data: &SimulationData,
) {
    let side;
    if !simulation_data.disable_max_passthrough {
        side = min(simulation_data.side_max_passthrough, current_traffic.side);
    } else {
        side = current_traffic.side;
    }

    *driving_cars += side;

    *waiting_cars += current_traffic.side - side;
    *waiting_cars += current_traffic.main_from_prev;
    *waiting_cars += current_traffic.main_from_next;

    match traffic_to_update.get_mut(index) {
        Some(next_traffic_current) => {
            next_traffic_current.side += current_traffic.side - side;
            next_traffic_current.main_from_prev += current_traffic.main_from_prev;
            next_traffic_current.main_from_next += current_traffic.main_from_next;
        }
        None => {}
    }
    match traffic_to_update.get_mut(index + 1) {
        Some(next_traffic_next) => {
            next_traffic_next.main_from_prev +=
                calc_next(side, simulation_data.side_percentage / 2.0)
        }
        None => {}
    }
    if index > 0 {
        match traffic_to_update.get_mut(index - 1) {
            Some(next_traffic_prev) => {
                next_traffic_prev.main_from_next +=
                    calc_next(side, simulation_data.side_percentage / 2.0)
            }
            None => {}
        }
    }
}

fn step(
    simulation_data: &SimulationData,
    traffic_lights: &Vec<BitVec>,
    current_traffic: &Vec<TrafficState>,
    t: usize,
    driving_cars: &mut i32,
    waiting_cars: &mut i32,
) -> Vec<TrafficState> {
    let mut next_traffic = extract_step(&simulation_data.traffic_data, t + 1);
    for (index, traffic) in current_traffic.iter().enumerate() {
        match traffic_lights[index][t] {
            true => apply_main(
                &mut next_traffic,
                traffic,
                index,
                driving_cars,
                waiting_cars,
                simulation_data,
            ),
            false => apply_side(
                &mut next_traffic,
                traffic,
                index,
                driving_cars,
                waiting_cars,
                simulation_data,
            ),
        }
    }
    next_traffic
}

fn fitness(optimization_data: &OptimizationData, driving_cars: i32, waiting_cars: i32) -> i32 {
    if optimization_data.fitness_value == "difference" {
        return driving_cars - waiting_cars;
    } else if optimization_data.fitness_value == "driving_cars" {
        return driving_cars;
    } else if optimization_data.fitness_value == "waiting_cars" {
        return -waiting_cars;
    }
    0
}

pub fn simulate(
    simulation_data: &SimulationData,
    optimization_data: &OptimizationData,
    candidate: &Vec<BitVec>,
) -> i32 {
    let mut driving_cars = 0;
    let mut waiting_cars = 0;
    let mut current_step = extract_step(&simulation_data.traffic_data, 0);
    // println!("Initial Data:");
    // println!("{:?}", simulation_data.traffic_data);
    // println!("Step 0:");
    // println!("{:?}", current_step);
    for t in 0..simulation_data.timesteps {
        current_step = step(
            simulation_data,
            candidate,
            &current_step,
            t,
            &mut driving_cars,
            &mut waiting_cars,
        );
        // println!("Step {}:", t + 1);
        // println!("{:?}", current_step);
    }
    // println!("Current step: {:?}", current_step);
    fitness(optimization_data, driving_cars, waiting_cars)
}

pub fn simulate_population(
    simulation_data: &SimulationData,
    optimization_data: &OptimizationData,
    population: &Vec<Vec<BitVec>>,
) -> Vec<i32> {
    let mut values = vec![0; population.len()];

    for i in 0..population.len() {
        values[i] = simulate(simulation_data, optimization_data, &population[i])
    }

    values
}
