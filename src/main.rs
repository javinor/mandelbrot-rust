use mandelbrot::{run, Config};

fn main() {
    let config = Config {
        top: 1.5,
        right: 0.85,
        bottom: -1.5,
        left: -2.15,

        width: 80,
        height: 40,

        max_iterations: 50,
        escape_radius: 8.0,
    };

    run(config);
}
