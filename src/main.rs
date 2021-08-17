use std::time::{Duration, Instant};

fn save_results(seed: usize, previous_results: Vec<usize>) {
    let mut file = match File::create("samples/{seed}")

}

fn main() {
    std::fs::create_dir_all("samples/");
    let mut seed = 0;
    let mut previous_results: Vec<usize> = Vec::new();
    loop {
        let start = Instant::now();
        let mut value = seed;
        previous_results.clear();
        loop {
            value = (value & 1 == 1)
                .then(|| (value * 3) + 1)
                .unwrap_or(value / 2);

            previous_results.push(value);

            if value == 1 {
                break;
            }
        }
        save_results(seed, previous_results);
        println!("Seed {:?} took {:?}", seed, start.elapsed());
        seed += 1;
    }
}
