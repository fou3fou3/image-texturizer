# ASCII Art Generator

This is a simple command-line application that converts an image into ASCII art based on the intensity of its grayscale pixels. It uses Rust and the `clap` and `image` crates.

---

## Features
- Convert images into ASCII art.
- Rescale images to a desired size for better control over the output.
- Adjust the contrast of the image before conversion.

---

## Prerequisites

### Dependencies
Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/) (with `cargo` package manager)
- Required Rust crates:
  - `clap`
  - `image`

You can install the dependencies using Cargo:
```bash
cargo install clap image
```

---

## How to Compile

Clone the repository:
```bash
git clone github.com/fou3fou3/image-texturizer
cd image-texturizer
```

Build the project:
```bash
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

---

## How to Run

Run the program with the following command:
```bash
./target/release/<binary-name> -i <path-to-image> -d <rate>
```

### Arguments
- `--image-path` (required): The path to the image file you want to convert.
- `--descale-rate` (optional): The percentage to resize the image (default is 8).

### Example
Convert an image located at `./images/example.png`:
```bash
./target/release/<binary-name> --image-path ./images/example.png --descale-rate 10
```

---

## How It Works

1. **Load the Image:**
   The program uses the `image` crate to load the image file from the specified path.

2. **Resize the Image:**
   The image is resized based on the `--descale-rate` parameter to reduce its dimensions. This is done using the Lanczos3 filter for better quality.

3. **Convert to Grayscale:**
   The resized image is converted into a grayscale image with an adjustable contrast level (fixed to 20.0 in this implementation).

4. **Map Intensity to ASCII:**
   Each pixel's intensity is mapped to a specific ASCII character based on predefined ranges:
   - `90..=255`: (Space)
   - `80..=89`: `.`
   - `60..=79`: `+`
   - `40..=59`: `*`
   - `20..=39`: `&`
   - `1..=19`: `/`
   - `0`: `@`

5. **Print ASCII Art:**
   The ASCII characters are printed line by line to the terminal.

---

## Example Output
For an image input, the output will resemble:
```
@@@@@@@@@@@@@@@@
@@@@&&&&&&&&&&@@
@@&&......&&&&@@
@@&&++++++&&&&@@
@@&&********&&@@
@@@@@@@@@@@@@@@@
```

---

## Notes
- Large images may require a lower `--descale-rate` value to fit properly in the terminal.
- The default contrast adjustment is set to `20.0`, but you can modify it in the code if needed.

---

## Contributing
Feel free to submit issues or pull requests for improvements and new features.

---

## License
This project is licensed under the MIT License. See the LICENSE file for details.

