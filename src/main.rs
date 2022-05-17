use rand::Rng;
use std::fmt;

#[derive(Clone)]
struct TrafficState {
    main_from_prev: i32,
    main_from_next: i32,
    side: i32,
}

impl fmt::Debug for TrafficState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n{{main_from_prev: {}, main_from_next: {}, side: {}}}",
            self.main_from_prev, self.main_from_next, self.side
        )
    }
}

fn random_traffic_value(main: bool) -> i32 {
    let mut rng = rand::thread_rng();
    if main {
        return rng.gen_range(8..20);
    }
    rng.gen_range(3..10)
}

fn build_traffic_state_initial() -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(true),
        main_from_next: random_traffic_value(true),
        side: random_traffic_value(false),
    }
}

fn build_traffic_state_base() -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: 0,
        side: random_traffic_value(false),
    }
}

fn build_traffic_state_first_intersection() -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(true),
        main_from_next: 0,
        side: random_traffic_value(false),
    }
}

fn build_traffic_state_last_intersection() -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: random_traffic_value(true),
        side: random_traffic_value(false),
    }
}

fn generate_data(intersections: usize, timesteps: usize) -> Vec<Vec<TrafficState>> {
    let mut data: Vec<Vec<TrafficState>> = Vec::new();
    for index in 0..intersections {
        let mut traffic_data: Vec<TrafficState> = Vec::new();

        let random_initial_traffic = build_traffic_state_initial();
        traffic_data.push(random_initial_traffic);

        for _ in 1..timesteps {
            if index == 0 {
                traffic_data.push(build_traffic_state_first_intersection());
            } else if index == intersections - 1 {
                traffic_data.push(build_traffic_state_last_intersection());
            } else {
                traffic_data.push(build_traffic_state_base());
            }
        }
        let new_intersection = traffic_data;
        data.push(new_intersection);
    }
    data
}

fn extract_step(traffic_data: &Vec<Vec<TrafficState>>, t: usize) -> Vec<TrafficState> {
    let mut step_data: Vec<TrafficState> = Vec::new();
    for intersection in traffic_data.iter() {
        match intersection.get(t) {
            Some(traffic_data) => step_data.push(traffic_data.clone()),
            None => {}
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
    car_count: &mut i32,
) {
    *car_count += current_traffic.main_from_prev;
    *car_count += current_traffic.main_from_next;
    match traffic_to_update.get_mut(index) {
        Some(next_traffic_current) => {
            next_traffic_current.side += current_traffic.side;
        }
        None => {}
    }
    match traffic_to_update.get_mut(index + 1) {
        Some(next_traffic_next) => {
            next_traffic_next.main_from_prev += calc_next(current_traffic.main_from_prev, 0.8)
        }
        None => {}
    }
    if index > 0 {
        match traffic_to_update.get_mut(index - 1) {
            Some(next_traffic_prev) => {
                next_traffic_prev.main_from_next += calc_next(current_traffic.main_from_next, 0.8)
            }
            None => {}
        }
    }
}

fn apply_side(
    traffic_to_update: &mut Vec<TrafficState>,
    current_traffic: &TrafficState,
    index: usize,
    car_count: &mut i32,
) {
    *car_count += current_traffic.side;
    match traffic_to_update.get_mut(index) {
        Some(next_traffic_current) => {
            next_traffic_current.main_from_prev += current_traffic.main_from_prev;
            next_traffic_current.main_from_next += current_traffic.main_from_next;
        }
        None => {}
    }
    match traffic_to_update.get_mut(index + 1) {
        Some(next_traffic_next) => {
            next_traffic_next.main_from_prev += calc_next(current_traffic.side, 0.3)
        }
        None => {}
    }
    if index > 0 {
        match traffic_to_update.get_mut(index - 1) {
            Some(next_traffic_prev) => {
                next_traffic_prev.main_from_next += calc_next(current_traffic.side, 0.3)
            }
            None => {}
        }
    }
}

fn step(
    traffic_data: &Vec<Vec<TrafficState>>,
    current_traffic: &Vec<TrafficState>,
    t: usize,
    car_count: &mut i32,
) -> Vec<TrafficState> {
    let mut next_traffic = extract_step(traffic_data, t + 1);
    for (index, traffic) in current_traffic.iter().enumerate() {
        apply_main(&mut next_traffic, traffic, index, car_count);
    }
    next_traffic
}

fn simulate(traffic_data: &Vec<Vec<TrafficState>>, timesteps: usize) {
    let mut cars = 0;
    let mut current_step = extract_step(traffic_data, 0);
    for t in 0..timesteps {
        println!("Step {}:", t);
        println!("{:?}", current_step);
        current_step = step(traffic_data, &current_step, t, &mut cars);
    }
    println!("Cars transported: {}", cars);
}

fn main() {
    let intersection: usize = 4;
    let timesteps: usize = 4;
    let traffic_data = generate_data(intersection, timesteps);
    println!("{:?}", traffic_data);
    simulate(&traffic_data, timesteps);
}
