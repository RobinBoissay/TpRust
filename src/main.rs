use image::{DynamicImage, GenericImageView, ImageError, Rgb, RgbImage};
use argh::FromArgs;
use rand::Rng; 

/// Structure des options pour le CLI
#[derive(FromArgs)]
struct Args {
    /// image d'entrée
    #[argh(option, short = 'i')]
    image: String,

    /// fichier de sortie
    #[argh(option, short = 'o', default = "String::from(\"out.png\")")]
    output: String,

    /// opération à appliquer à l'image
    #[argh(option, short = 'r')]
    operation: String,

    /// seuil pour l'opération monochrome (optionnel)
    #[argh(option, short = 't')]
    threshold: Option<f32>,
}

fn main() {
    let args: Args = argh::from_env();

    match process_image(&args.image, &args.output, &args.operation, args.threshold.unwrap_or(0.5)) {
        Ok(_) => println!("Traitement terminé avec succès !"),
        Err(e) => eprintln!("Erreur : {:?}", e),
    }
}

fn process_image(input: &str, output: &str, operation: &str, threshold: f32) -> Result<(), ImageError> {
    let img = image::open(input)?;
    let rgb_image = img.to_rgb8();

    // Différent type de filtres
    let processed_image = match operation {
        "monochrome" => apply_monochrome(rgb_image, threshold),
        "palette" => apply_palette(rgb_image),
        "dithering" => apply_dithering(rgb_image),
        "bayer" => apply_bayer(rgb_image, 1),
        _ => {
            eprintln!("Opération non reconnue : {}", operation);
            return Ok(());
        }
    };

    // Sauvegarde de l'image
    processed_image.save(output)?;
    println!("Image sauvegardée dans {}", output);

    Ok(())
}

/// Appliquer le traitement monochrome
fn apply_monochrome(mut image: RgbImage, threshold: f32) -> RgbImage {
    for pixel in image.pixels_mut() {
        let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
        if brightness as f32 / 255.0 > threshold {
            *pixel = Rgb([255, 255, 255]); 
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
    image
}

/// Appliquer le traitement palette de 8 couleurs
fn apply_palette(mut image: RgbImage) -> RgbImage {
    let palette = [
        Rgb([0, 0, 0]),
        Rgb([255, 255, 255]),
        Rgb([255, 0, 0]),
        Rgb([0, 255, 0]),
        Rgb([0, 0, 255]),
        Rgb([255, 255, 0]),
        Rgb([255, 0, 255]),
        Rgb([0, 255, 255]),
    ];

    for pixel in image.pixels_mut() {
        *pixel = find_closest_color(*pixel, &palette);
    }
    image
}

/// Trouver la couleur la plus proche dans une palette
fn find_closest_color(pixel: Rgb<u8>, palette: &[Rgb<u8>]) -> Rgb<u8> {
    let mut closest_color = palette[0];
    let mut smallest_distance = u32::MAX;

    for &color in palette {
        let distance = color_distance(pixel, color);
        if distance < smallest_distance {
            smallest_distance = distance;
            closest_color = color;
        }
    }

    closest_color
}

/// Calculer la distance entre deux couleurs
fn color_distance(c1: Rgb<u8>, c2: Rgb<u8>) -> u32 {
    let r_diff = (c1[0] as i32 - c2[0] as i32).pow(2);
    let g_diff = (c1[1] as i32 - c2[1] as i32).pow(2);
    let b_diff = (c1[2] as i32 - c2[2] as i32).pow(2);

    (r_diff + g_diff + b_diff) as u32
}

/// Appliquer le tramage aléatoire (dithering)
fn apply_dithering(mut image: RgbImage) -> RgbImage {
    let mut rng = rand::thread_rng();

    for pixel in image.pixels_mut() {
        
        let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);

        // Générer un nombre random entre 0 et 1
        let threshold: f32 = rng.gen();

        // Comparer la luminosité
        if brightness > threshold {
            *pixel = Rgb([255, 255, 255]); 
        } else {
            *pixel = Rgb([0, 0, 0]); 
        }
    }

    image
}

/// Appliquer le tramage ordonné avec la matrice de Bayer
fn apply_bayer(image: RgbImage, order: u32) -> RgbImage {
    let bayer_matrix = generate_bayer_matrix(order);
    let matrix_size = bayer_matrix.len();

    let mut result_image = image.clone();

    for (y, row) in image.enumerate_rows() {
        for (x, _, pixel) in row {
            // Calculer la luminosité moyenne 
            let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
            let normalized_brightness = brightness as f32 / 255.0;

            // Récupérer le seuil dans la matrice de Bayer
            let threshold = bayer_matrix[y as usize % matrix_size][x as usize % matrix_size];

            // Appliquer le tramage en comparant au seuil
            if normalized_brightness > threshold {
                result_image.put_pixel(x, y, Rgb([255, 255, 255]));
            } else {
                result_image.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }

    result_image
}

/// Génère une matrice de Bayer normalisée 
fn generate_bayer_matrix(order: u32) -> Vec<Vec<f32>> {
    if order == 0 {
        return vec![vec![0.0]];
    }

    let smaller_matrix = generate_bayer_matrix(order - 1);
    let size = smaller_matrix.len();
    let mut matrix = vec![vec![0.0; size * 2]; size * 2];

    for y in 0..size {
        for x in 0..size {
            let base_value = smaller_matrix[y][x] * 4.0;
            matrix[y][x] = base_value; // Haut gauche
            matrix[y][x + size] = base_value + 2.0; // Haut droit
            matrix[y + size][x] = base_value + 3.0; // Bas gauche
            matrix[y + size][x + size] = base_value + 1.0; // Bas droit
        }
    }

    let max_value = (size * size * 4) as f32;
    for row in &mut matrix {
        for value in row {
            *value /= max_value; // Normalisation entre 0 et 1
        }
    }

    matrix
}
