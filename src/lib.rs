pub struct Config {
  pub top: f64,
  pub right: f64,
  pub bottom: f64,
  pub left: f64,

  pub width: u32,
  pub height: u32,

  pub max_iterations: u32,
  pub escape_radius: f64,
}

pub fn run(config: Config) {
  for row in 0..config.height {
    for col in 0..config.width {
      let c_0 = pixel_to_complex(&config, row, col);
      let exit_iteration = calc_exit_iteration(&config, c_0);
      match exit_iteration {
        None => print!("*"),
        Some(_) => print!(" "),
      }
    }
    println!("");
  }
}

fn calc_exit_iteration(config: &Config, c: (f64, f64)) -> Option<u32> {
  let mut zx = 0.0;
  let mut zy = 0.0;
  let mut next_x: f64;
  let mut next_y: f64;
  let escape_distance = config.escape_radius * config.escape_radius;

  for i in 0..config.max_iterations {
    if zx * zx + zy * zy > escape_distance {
      // smoothening the image:
      // i - Math.log(zx*zx+zy*zy)/Math.log(64);
      return Some(i);
    }

    next_x = zx * zx - zy * zy + c.0;
    next_y = 2. * zx * zy + c.1;

    zx = next_x;
    zy = next_y;
  }

  return None;
}

fn pixel_to_complex(config: &Config, row: u32, col: u32) -> (f64, f64) {
  let dx = (config.right - config.left) / (config.width as f64);
  let dy = (config.top - config.bottom) / (config.height as f64);

  let re = config.left + (col as f64 * dx);
  let im = config.top - (row as f64 * dy);

  (re, im)
}
