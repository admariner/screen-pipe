use std::path::PathBuf;
use std::time::Instant;

use screenpipe_vision::perform_ocr_tesseract;

#[test]
fn test_ocr_output() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting test_ocr_output");

    // Use an absolute path that works in both local and CI environments
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("testing_OCR.png");
    println!("Path to testing_OCR.png: {:?}", path);
    let image = image::open(&path).expect("Failed to open image");

    // Start timing
    let start = Instant::now();

    let (text, json_output) = perform_ocr_tesseract(&image);

    // Stop timing
    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();

    // Calculate average confidence score

    // println!("TSV:");
    // println!("{}", tsv_output);
    // println!("Text:");
    // println!("{}", text);
    // println!("json_output:");
    // println!("{}", json_output);
    // println!("Data output:");
    // println!("{:?}", data_output);

    println!("Time taken for OCR: {:.1} seconds", duration_secs);

    // Print character lengths
    println!("Character length of OCR text: {}", text.len());
    // println!("Character length of TSV output: {}", tsv_output.len());
    println!("Character length of JSON output: {}", json_output.len());

    assert!(!text.is_empty(), "OCR text should not be empty");
    // assert!(!tsv_output.is_empty(), "TSV output should not be empty");

    Ok(())
}
