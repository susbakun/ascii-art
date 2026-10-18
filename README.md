# ASCII Art Generator

A Rust implementation of an ASCII art generator that converts images and videos into text-based art using ASCII characters. This project transforms images into terminal-based artwork by mapping pixel brightness to carefully selected ASCII characters, with optional color output.

## About

This is an educational project based on [Programming Projects for Advanced Beginners #1: ASCII art](https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/) by Robert Heaton. The project demonstrates image processing, pixel manipulation, and terminal output formatting in Rust.

## Features

- Convert JPEG images to ASCII art
- Play videos as ASCII art in the terminal (requires ffmpeg)
- Color output using the original pixel colors, or a green "matrix" style
- Multiple brightness calculation methods (average, min/max, luminosity)
- Brightness inversion option
- High-quality image resampling using Lanczos3 filtering
- Terminal-optimized output with proper aspect ratio correction

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)
- [ffmpeg](https://ffmpeg.org/download.html) installed and available on your `PATH` (only needed for video)
- A terminal with truecolor support for color output (most modern terminals have it)

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

### Images

```bash
cargo run --release -- -p <path-to-image.jpg>
```

For example:

```bash
cargo run --release -- -p ascii-pineapple.jpg
```

Or run the compiled binary directly:

```bash
./target/release/ascii_art -p <path-to-image.jpg>
```

### Video

Video playback needs **ffmpeg** installed on your machine. Pass the video file with `-v`:

```bash
cargo run --release -- -v <path-to-video.mp4>
```

The video is played at 15 fps in a 50x40 pixel frame (150 columns wide because of the aspect ratio correction), so make your terminal at least 150 columns wide. Press `Ctrl+C` to exit. Audio is not played.

### Options

| Flag             | Description                                                        |
| ---------------- | ------------------------------------------------------------------ |
| `-p`, `--photo`  | Image file to convert                                              |
| `-v`, `--video`  | Video file to play (requires ffmpeg)                               |
| `-m`, `--method` | Brightness method: `average` (default), `min_max`, or `luminosity` |
| `-i`, `--invert` | Invert the brightness mapping                                      |
| `-g`, `--green`  | Print in green                                                     |
| `-c`, `--color`  | Print in the original image colors                                 |
| `-h`, `--help`   | Print the help menu                                                |

Examples:

```bash
cargo run --release -- -f photo.jpg -c -m luminosity
cargo run --release -- -v clip.mp4 -c
```

## How It Works

1. **Loading**: Images are read using the `image` crate. For video, `ffmpeg` decodes the file, resizes each frame, and pipes raw RGB pixels to the program.
2. **Resizing**: Images are resized to 50x40 pixels using Lanczos3 filtering. For video, ffmpeg scales the frames to the same size at 15 fps.
3. **Pixel Extraction**: RGB pixel values are extracted into a 2D matrix
4. **Brightness Calculation**: Each pixel's brightness (0-255) is computed using the selected method
5. **ASCII Mapping**: Brightness values are linearly mapped to ASCII characters from a carefully ordered string
6. **Output**: Each character is printed 3 times horizontally to correct for terminal character aspect ratios, colored with the original pixel color if color mode is on. For video, each frame is drawn over the previous one by moving the cursor back to the top-left.

## Technical Details

- **Brightness Calculation**: Average `(R + G + B) / 3` by default, with min/max and luminosity as alternatives
- **Character Mapping**: Maps brightness values (0-255) to 64 ASCII characters using linear interpolation
- **Aspect Ratio Correction**: Prints each character 3 times horizontally to compensate for terminal characters being taller than they are wide
- **Color Output**: Uses 24-bit truecolor terminal colors via the `crossterm` crate
- **Video**: Frames are streamed from ffmpeg as raw `rgb24` bytes, so no temporary files are written to disk

## Future Improvements

Potential enhancements based on the original project:

- [x] Support for different brightness calculation methods (luminosity, min/max)
- [x] Command-line arguments for customization (inversion option, brightness method, etc.)
- [x] Brightness inversion option
- [x] Color output support
- [x] Video support
- [ ] Support for additional image formats
- [x] Configurable frame rate

## References

- [Programming Projects for Advanced Beginners #1: ASCII art](https://robertheaton.com/2018/06/12/programming-projects-for-advanced-beginners-ascii-art/) by Robert Heaton
