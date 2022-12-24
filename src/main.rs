use std::io;

/// A calculator to calculate our weight on Mars
fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input);
    let mut weight_on_mars:f32 = calculate_our_weight_on_mars(87.0);

println!("Weight on Mars: {}kg",weight_on_mars);
}

fn calculate_our_weight_on_mars(weight:f32) -> f32{
    (weight/9.81)*3.711
}