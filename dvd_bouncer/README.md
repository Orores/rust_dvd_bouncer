# DVD Bouncer

This project simulates the classic DVD logo bouncing animation using Rust and the ggez game development library. The logo changes color each time it bounces off a screen edge, offering a nostalgic visual experience.

## Prerequisites

Ensure the following are installed on your system before running the project:

1. **Rust and Cargo**: 

   Install Rust and Cargo using the following command in your terminal:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Follow the on-screen instructions to complete the installation.

2. **Update Rust**:

   Keep your `cargo` and `rustc` up to date:

   ```bash
   rustup update
   ```

## Project Setup

1. **Clone the Repository**:

   Clone the project repository and navigate into it:

   ```bash
   git clone https://github.com/yourusername/dvd_bouncer.git
   cd dvd_bouncer
   ```

2. **Dependencies**:

   Ensure the necessary dependencies are listed in your `Cargo.toml`:

   ```toml
   [dependencies]
   ggez = "0.7"
   rand = "0.8"
   nalgebra = "0.29"
   mint = "0.5"
   ```

3. **Build the Project**:

   Compile the project and download dependencies:

   ```bash
   cargo build
   ```

## Running the DVD Bouncer

1. **Execute the Program**:

   Run the animation using:

   ```bash
   cargo run
   ```

2. **Enjoy the Animation**:

   A window will open displaying the bouncing DVD logo. Watch as the logo changes color with each bounce.

## Troubleshooting

- **Missing Libraries**: If you face issues related to missing libraries, verify that your Rust toolchain is installed correctly and up-to-date.
- **Linux Dependencies**: On Linux, you may need additional packages for ggez, such as `libalsa`, `libudev`, or `libx11`.

## Additional Resources

- [Rust Language](https://www.rust-lang.org/)
- [ggez Library](https://ggez.rs/)

This README provides all the necessary steps to set up and run the DVD Bouncer project, ensuring a seamless experience for new Rust users.