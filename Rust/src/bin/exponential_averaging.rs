use std::io::{self, Write};

fn read_input<T: std::str::FromStr>(prompt: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<T>().expect("Failed to parse input")
}

fn main() {
    let alpha: f32 = read_input("Enter the value of alpha: ");
    let cpu_bursts: Vec<f32> = read_input::<String>("Enter the CPU bursts (comma separated): ")
        .split(',')
        .map(|s: &str| s.trim().parse::<f32>().expect("Failed to parse CPU burst"))
        .collect();
    let prediction: f32 = read_input("Enter the initial prediction: ");

    let mut predictions: Vec<f32> = vec![prediction];

    for &burst in &cpu_bursts {
        predictions.push(
            (1.0 - alpha)
                .mul_add(
                    *predictions.last().expect("No last value found"),
                    alpha * burst,
                )
                .floor(),
        );
    }

    println!("Predicted CPU bursts: {predictions:?}");
}
