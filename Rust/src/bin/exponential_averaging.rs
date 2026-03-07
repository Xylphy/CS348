use rust::read_input;

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
