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
        size: try!(args[0].parse().map_err(|_| "`size` must be a valid integer")),
        rows: try!(args[1].parse().map_err(|_| "`rows` must be a valid integer")),
        rule: try!(args[2].parse().map_err(|_| "`rule` must be 0-255"))
    })
}

fn print_row(row: &Vec<bool>) {
    println!("{}", row.iter().map(|value| if *value { '*' } else { ' ' }).collect::<String>());
}

/*
fn get_rule_map(rule: u8) -> Vec<bool> {
    (..8).iter().map(|i| ((1 << i) & rule) > 0).collect()
}
*/

fn run_cellular_automaton(config: &Config) {
    let mut even = vec![false; config.size];
    even[config.size / 2] = true;
    let mut odd = vec![false; config.size];
    print_row(&even);
    for i in 1..config.rows {

        // Swap buffer each time.
        let (prev, mut next) = if i % 2 == 0 {
            (&even, &odd)
        } else {
            (&odd, &even)
        };

        for (j, element: mut &bool) in next.iter().enumerate() {
        //for j in 0..config.size {
            let a = if j == 0 { 0 } else { prev[j - 1] as u8 };
            let b = prev[i] as u8;
            let c = if j == config.size { 0 } else { prev [j + 1] as u8 };
            
            let pos = a << 2 | b << 1 | c;
            element = (config.rule & (1 << pos)) > 0;
        }
    }
}

fn main() {
    let args = get_config();
    match args {
        Ok(config) => run_cellular_automaton(&config),
        Err(message) => {
            println!("Error {}", message);
            println!("Usage: {} <size> <rows> <rule>", env::args().nth(0).unwrap());
        }
    }
}
