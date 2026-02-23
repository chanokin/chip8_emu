use crate::gpu::Gpu;
use crate::ram::Ram;


pub trait Show {
    fn show(&self, gpu: &Gpu, ram: &Ram) -> Result<(), String>;
}

#[derive(Debug, Default)]
pub struct TextRenderer;

impl Show for TextRenderer {
    fn show(&self, gpu: &Gpu, ram: &Ram) -> Result<(), String> {
        // Implement text rendering logic here

        let output_buffer = match gpu.buffer_as_u8_matrix(ram) {
            Ok(buffer) => buffer,
            Err(e) => return Err(format!("Failed to get GPU buffer: {}", e)),
        };

        if output_buffer.is_empty() {
            return Err("Output buffer is empty".to_string());
        }

        for row in output_buffer {
            let line: String = row.iter().map(|&b| if b > 0 { '█' } else { ' ' }).collect();
            println!("{}", line);
        }
        Ok(())
    }
}