use crate::prelude::*;

pub enum BrightnessMethod {
    Average,
    MinMax,
    Luminosity
}

pub fn get_brightness_method(method: &str) -> BrightnessMethod {
    match method {
        "min_max" => BrightnessMethod::MinMax,
        "luminosity" => BrightnessMethod::Luminosity,
        _ => BrightnessMethod::Average,
    }
}

pub fn brightness_map(color: &Rgb, method: &BrightnessMethod) -> u8 {
    let Rgb{r , g, b} = color;

    // converting channels to f32 (preventing overflow)
    let r = *r as f32;
    let g = *g as f32;
    let b = *b as f32;

    match method {
        BrightnessMethod::Average => ((r + g + b) / 3.0) as u8,
        BrightnessMethod::MinMax => {
            let min = [r, g, b].into_iter()
                .min_by(|a, b| a.partial_cmp(b)
                .unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(r);

            let max = [r, g, b].into_iter()
                .max_by(|a, b| a.partial_cmp(&b)
                .unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(r);

            ((min + max) / 2.0) as u8 
        },
        BrightnessMethod::Luminosity => (0.21 * r + 0.72 * g + 0.07 * b) as u8
    }
}

// Map brightness value (0-255) to ASCII character index (0-63)
pub fn linear_map(value: usize, invert: bool) -> usize {
    let value = if invert {
        255 - value
    }else {
        value
    };

    (value * 63) / 255
}