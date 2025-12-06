use image::ImageReader;
use jpegxl_rs::encoder_builder;
use jpegxl_rs::encode::{EncoderResult, EncoderSpeed};

#[tauri::command]
pub fn process_file_array(file_paths: Vec<String>) -> Result<String, String> {
    let mut encoder = encoder_builder()
        .lossless(true)
        .speed(EncoderSpeed::Tortoise)
        .build()
        .unwrap();

    // FORGET ABOUT WHAT WAS WRONG, FIGURE IT OUT YOURSELF, FIX IT
    for file in &file_paths {
        let open_file = ImageReader::open(file).unwrap().decode().unwrap().to_rgba16();

        println!("Processing file: {}", file);
        let buff: EncoderResult<f32> = encoder.encode(&open_file, open_file.width(), open_file.height()).unwrap();

        write_file(file, &buff.data).unwrap();
    }
    Ok(format!(
        "Processed {} files successfully!",
        file_paths.len()
    ))
}

#[tauri::command]
fn write_file(file_path_name: &str, file_bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(file_path_name, file_bytes).map_err(|e| e.to_string()).unwrap();
    Ok(())
}