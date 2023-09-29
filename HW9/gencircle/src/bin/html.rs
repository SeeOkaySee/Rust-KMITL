use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::error::Error;
use std::fs::File;
use std::io::Write;

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
    let html_file_path = "layers.html";

    // Open an HTML file for writing
    let mut html_file = File::create(html_file_path)?;

    // Generate the HTML table
    html_file.write_all("<html><body><table>\n".as_bytes())?;

    // Write the HTML table header
    html_file.write_all(
        "<tr><th>Layer Name</th><th>Color</th><th>Circle Count</th><th>Average Area</th><th>Minimum Area</th><th>Maximum Area</th></tr>\n"
            .as_bytes(),
    )?;
    
    // Calculate and write the average area for each layer as HTML table rows
    for layer in &layers {
        let mut areas: Vec<f32> = layer.objects.iter().map(|circle| {
            std::f32::consts::PI * circle.radius * circle.radius
        }).collect();

        let average_area = if areas.is_empty() {
            0.0
        } else {
            areas.iter().sum::<f32>() / areas.len() as f32
        };

        let min_area = areas.iter().cloned().fold(std::f32::INFINITY, f32::min);
        let max_area = areas.iter().cloned().fold(std::f32::NEG_INFINITY, f32::max);

        let row = format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>\n",
            &layer.name, &layer.color, layer.objects.len(), average_area, min_area, max_area
        );
        html_file.write_all(row.as_bytes())?;
    }

    // Close the HTML table and file
    html_file.write_all("</table></body></html>\n".as_bytes())?;

    println!("HTML file '{}' has been created.", html_file_path);

    Ok(())
}
