use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn save_results(seed: usize, previous_results: Vec<usize>) {
    let path = &format!("samples/{}.txt", seed);
    let path = Path::new(path);

    // Open a file in write-only mode, returns `io::Result<File>`
    File::create(path).expect("Couldn't create file");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    
    for i in &previous_results {
        writeln!(file, "{:?}", &i.to_le_bytes()[0]);
    }
}

fn main() {
    std::fs::create_dir_all("samples/");
    let mut seed = 1;
    loop {
        let start = Instant::now();
        let mut value = seed;
        let mut previous_results: Vec<usize> = Vec::new();
        loop {
            value = (value & 1 == 1)
                .then(|| (value * 3) + 1)
                .unwrap_or(value / 2);

            previous_results.push(value);
            // Immediatly break if we hit the number 1, as we know, this is a dead lock
            if value == 1 {
                break;
            }
        }
        println!("Seed {:?} took {:?}", seed, start.elapsed());
        save_results(seed, previous_results);
        seed += 1;
    }
}
