# ASCII Art Generator

A Rust implementation of an ASCII art generator that converts images into text-based art using ASCII characters. This project transforms images into beautiful terminal-based artwork by mapping pixel brightness to carefully selected ASCII characters.

## About

This is an educational project based on [Programming Projects for Advanced Beginners #1: ASCII art](https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/) by Robert Heaton. The project demonstrates image processing, pixel manipulation, and terminal output formatting in Rust.

## Features

- Convert JPEG images to ASCII art
- Automatic image resizing to fit terminal dimensions
- High-quality image resampling using Lanczos3 filtering
- Brightness-based character mapping for accurate representation
- Terminal-optimized output with proper aspect ratio correction

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd ascii_art
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the program with an image file path as an argument:

```bash
cargo run --release <path-to-image.jpg>
```

For example:
```bash
cargo run --release ascii-pineapple.jpg
```

Or run the compiled binary directly:
```bash
./target/release/ascii_art <path-to-image.jpg>
```

## How It Works

1. **Image Loading**: The program reads a JPEG image file using the `image` crate
2. **Resizing**: The image is resized to 50x40 pixels (configurable) using Lanczos3 filtering for high-quality downscaling
3. **Pixel Extraction**: RGB pixel values are extracted into a 2D matrix
4. **Brightness Calculation**: Each pixel's RGB values are averaged to compute brightness (0-255)
5. **ASCII Mapping**: Brightness values are linearly mapped to ASCII characters from a carefully ordered string
6. **Output**: Each character is printed 3 times horizontally to correct for terminal character aspect ratios

## Technical Details

- **Brightness Calculation**: Uses simple averaging `(R + G + B) / 3`
- **Character Mapping**: Maps brightness values (0-255) to 64 ASCII characters using linear interpolation
- **Aspect Ratio Correction**: Prints each character 3 times horizontally to compensate for terminal characters being taller than they are wide

## Future Improvements

Potential enhancements based on the original project:
- [ x ] Support for different brightness calculation methods (luminosity, min/max)
- Color output support
- [ x ] Brightness inversion option
- [ x ] Command-line arguments for customization (inversion option, brightness method, etc.)
- Support for additional image formats

## License

This is an educational project. Please refer to the original project guide for more information and extensions.

## References

- [Programming Projects for Advanced Beginners #1: ASCII art](https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/) by Robert Heaton

