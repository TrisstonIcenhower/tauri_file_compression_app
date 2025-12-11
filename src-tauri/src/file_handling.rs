use image::ImageReader;
use jpegxl_rs::encode::{EncoderFrame, EncoderResult, EncoderSpeed};
use jpegxl_rs::encoder_builder;

#[tauri::command]
pub fn process_file_array(file_paths: Vec<String>) -> Result<String, String> {
    for file in &file_paths {
        println!("Processing file: {}", file);
        let open_file = ImageReader::open(file)
            .map_err(|e| format!("Failed to open file {}: {}", file, e))?
            .decode()
            .map_err(|e| format!("Failed to decode file {}: {}", file, e))?;

        let img_alpha = open_file.has_alpha();

        let img_bytes = convert_file_to_bytes(img_alpha, &open_file);

        let img_frame: EncoderFrame<u16> = EncoderFrame::new(&img_bytes).num_channels(if img_alpha { 4 } else { 3 });

        println!("Opened file {}: ", file);
        let mut encoder = encoder_builder()
            .has_alpha(img_alpha)
            .lossless(false)
            .speed(EncoderSpeed::Tortoise)
            .build()
            .map_err(|e| format!("Encoder failed to build on file {}: {}", file, e))?;
    
        println!("Encoder built successfully for file {}", file);
        let buffer: EncoderResult<f32> = encoder
            .encode_frame(&img_frame, open_file.width(), open_file.height())
            .map_err(|e| format!("Encoder failed to encode frame on file {}: {}", file, e))?;

        println!("Encoded file {}: {} bytes", file, buffer.data.len());
        
        let jxl_file_name = create_jxl_from_file_name(file);
        if write_file(&jxl_file_name, &buffer.data).is_err() {
            return Err(format!("Failed to write JXL file for {}", file));
        }
        else{
            println!("Successfully wrote JXL file: {}", jxl_file_name);
        }
    }
    Ok(format!(
        "Processed {} files successfully!",
        file_paths.len()
    ))
}

fn convert_file_to_bytes(has_alpha: bool, img: &image::DynamicImage) -> Vec<u16> {
    if has_alpha {
        img.to_rgba16().into_raw()
    } else {
        img.to_rgb16().into_raw()
    }
}

fn create_jxl_from_file_name(file_name: &str) -> String {
    let mut jxl_file_name = String::from(file_name);
    
    if let Some(pos) = jxl_file_name.rfind('.') {
        jxl_file_name.replace_range(pos.., ".jxl");
    } else {
        jxl_file_name.push_str(".jxl");
    }
    jxl_file_name
}

fn write_file(file_path_name: &str, file_bytes: &[u8]) -> std::io::Result<()> {
    std::fs::write(file_path_name, file_bytes)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(())
}
