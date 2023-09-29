use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::error::Error;
use std::fs::File;
use csv::Writer;

#[derive(Clone)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

#[derive(Clone)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}

fn gen_circle<R: Rng>(rng: &mut R, n: i32) -> Vec<Circle> {
    let mut result = Vec::new();
    for _i in 1..=n {
        let circle = Circle{
            x: rng.gen_range(-100. ..=100.),
            y: rng.gen_range(-100. ..=100.),
            radius: rng.gen_range(-10. ..=20.),
        };
        result.push(circle);
    }
    result
}

fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: i32) -> Vec<Layer> {
    let mut result = Vec::new();

    for i in 1..=n {
        let text = format!("Layer {}",i);
        let hex = format!("#{:02X}{:02X}{:02X}{:02X}",
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255));

        let p = rng.gen_range(20..=50);
        let mut rng = rand::thread_rng();
        let circles = gen_circle(&mut rng, p);

        result.push(
            Layer {
                name: text,
                color: hex,
                objects: circles, 
            }
        )
    }
    result
}

fn cal_average_area(layers: &[Layer]) -> Vec<(String, f32)> {
    layers
        .iter()
        .map(|layer| {
            let total_area: f32 = layer.objects.iter().map(|circle| {
                std::f32::consts::PI * circle.radius * circle.radius
            }).sum();

            let average_area = if layer.objects.is_empty() {
                0.0
            } else {
                total_area / layer.objects.len() as f32
            };

            (layer.name.clone(), average_area)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define the number of layers you want to generate
    let n = 10; // Change this to your desired number of layers

    // Create a seeded RNG for consistent results
    let seed: [u8; 32] = [1; 32]; // You can choose any seed value
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    // Generate the list of layers
    let layers = gen_obj_layer_list(&mut rng, n);

    // Specify the CSV file path
    let csv_file_path = "layers.csv";

    // Open a CSV file for writing
    let file = File::create(csv_file_path)?;

    // Create a CSV writer
    let mut writer = Writer::from_writer(file);

    // Write the CSV header
    writer.write_record(&["Layer Name", "Color", "Circle Count", "Average Area"])?;

    // Calculate and write the average area for each layer
    for layer in &layers {
        let average_area = cal_average_area(&[layer.clone()])[0].1;
        writer.write_record(&[&layer.name, &layer.color, &layer.objects.len().to_string(), &average_area.to_string()])?;
    }

    println!("CSV file '{}' has been created.", csv_file_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_gen_obj_layer_list() {
        let seed: [u8; 32] = [1; 32]; 
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // Test generating an empty list
        let empty_list = gen_obj_layer_list(&mut rng, 0);
        assert!(empty_list.is_empty());

        // Test generating a list with a single layer
        let single_layer_list = gen_obj_layer_list(&mut rng, 1);
        assert_eq!(single_layer_list.len(), 1);
        assert!(single_layer_list[0].objects.len() >= 20 && single_layer_list[0].objects.len() <= 50);

        // Test generating a list with multiple layers
        let multi_layer_list = gen_obj_layer_list(&mut rng, 5);
        assert_eq!(multi_layer_list.len(), 5);
        for layer in &multi_layer_list {
            assert!(layer.objects.len() >= 20 && layer.objects.len() <= 50);
        }
    }

    #[test]
    fn test_cal_average_area() {
        let layers = vec![
            Layer {
                name: String::from("Layer 1"),
                color: String::from("#FF0000"),
                objects: vec![
                    Circle { x: 0.0, y: 0.0, radius: 1.0 },
                    Circle { x: 0.0, y: 0.0, radius: 2.0 },
                ],
            },
            Layer {
                name: String::from("Layer 2"),
                color: String::from("#00FF00"),
                objects: vec![
                    Circle { x: 0.0, y: 0.0, radius: 3.0 },
                    Circle { x: 0.0, y: 0.0, radius: 4.0 },
                ],
            },
        ];

        let result = cal_average_area(&layers);

        // Check the result
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("Layer 1".to_string(), 7.853982)); 
        assert_eq!(result[1], ("Layer 2".to_string(), 39.26991)); 
    }
}
