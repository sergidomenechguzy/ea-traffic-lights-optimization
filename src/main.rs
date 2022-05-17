use rand::Rng;
use std::fmt;

struct TrafficState {
    main_from_prev: i32,
    main_from_next: i32,
    side_1: i32,
    side_2: i32,
}

impl fmt::Display for TrafficState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{main_from_prev: {}, main_from_next: {}, side_1: {}, side_2: {}}}",
            self.main_from_prev, self.main_from_next, self.side_1, self.side_2
        )
    }
}

struct Intersection(Vec<TrafficState>);

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\t[");

        for traffic_state in &self.0[0..self.0.len()] {
            writeln!(f, "\t\t{},", traffic_state);
        }
        writeln!(f, "\t],")
    }
}

struct Street(Vec<Intersection>);

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[");

        for intersection in &self.0[0..self.0.len()] {
            write!(f, "{}", intersection);
        }
        writeln!(f, "]")
    }
}

fn random_traffic_value(main: bool) -> i32 {
    let mut rng = rand::thread_rng();
    if main {
        return rng.gen_range(2..10);
    }
    rng.gen_range(0..5)
}

fn build_traffic_state_initial() -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(true),
        main_from_next: random_traffic_value(true),
        side_1: random_traffic_value(false),
        side_2: random_traffic_value(false),
    }
}

fn build_traffic_state_base() -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: 0,
        side_1: random_traffic_value(false),
        side_2: random_traffic_value(false),
    }
}

fn build_traffic_state_first_intersection() -> TrafficState {
    TrafficState {
        main_from_prev: random_traffic_value(true),
        main_from_next: 0,
        side_1: random_traffic_value(false),
        side_2: random_traffic_value(false),
    }
}

fn build_traffic_state_last_intersection() -> TrafficState {
    TrafficState {
        main_from_prev: 0,
        main_from_next: random_traffic_value(true),
        side_1: random_traffic_value(false),
        side_2: random_traffic_value(false),
    }
}

fn generate_data(intersections: i32, time: i32) -> Street {
    let mut data: Street = Street(Vec::new());
    for index in 0..intersections {
        let mut traffic_data: Vec<TrafficState> = Vec::new();

        let random_initial_traffic = build_traffic_state_initial();
        traffic_data.push(random_initial_traffic);

        for _ in 1..time {
            if index == 0 {
                traffic_data.push(build_traffic_state_first_intersection());
            } else if index == intersections - 1 {
                traffic_data.push(build_traffic_state_last_intersection());
            } else {
                traffic_data.push(build_traffic_state_base());
            }
        }
        let new_intersection = Intersection(traffic_data);
        data.0.push(new_intersection);
    }
    data
}

fn main() {
    let intersections = generate_data(4, 4);
    println!("{}", intersections);
}
