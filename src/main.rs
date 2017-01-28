use std::env;

struct Config {
    size: usize,
    rows: usize,
    rule: u8
}

fn get_config() -> Result<Config, &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("Please supply three arguments")
    };
    Ok(Config {
        size: try!(args[0].parse().map_err(|_| "`size` must be a positive integer")),
        rows: try!(args[1].parse().map_err(|_| "`rows` must be a positive integer")),
        rule: try!(args[2].parse().map_err(|_| "`rule` must be 0-255"))
    })
}

fn print_row(row: &Vec<bool>) {
    println!("{}", row.iter().map(|value|
        if *value { '░' } else { ' ' }).collect::<String>()
    );
}

fn initial_state(size: usize) -> Vec<bool> {
    let mut state = vec![false; size];
    state[size / 2] = true;
    state
}

fn next_state(rule: u8, state: &Vec<bool>) -> Vec<bool> {
    let size = state.len();
    let mut result = Vec::with_capacity(size);
    for i in 0..state.len() {
        let a = if i == 0 { state[size - 1] } else { state[i - 1] } as u8;
        let b = state[i] as u8;
        let c = if i == size - 1 { state[0] } else { state[i + 1] } as u8;

        let pos = a << 2 | b << 1 | c;
        result.push(rule & (1 << pos) > 0);
    }
    result
}

fn run(config: &Config) {
    let mut state = initial_state(config.size);
    print_row(&state);
    for _ in 2..config.rows {
        state = next_state(config.rule, &state);
        print_row(&state);
    }
}

fn main() {
    let args = get_config();
    match args {
        Ok(config) => run(&config),
        Err(message) => {
            println!("Error {}", message);
            println!("Usage: {} <size> <rows> <rule>", env::args().nth(0).unwrap());
        }
    }
}