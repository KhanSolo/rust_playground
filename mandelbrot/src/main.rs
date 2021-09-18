fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64, x_max: f64,
    y_min: f64, y_max: f64,
    width: usize, height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z_re = 0.0_f64;
    let mut z_im = 0.0_f64;
    for i in 0..=max_iters {
        let norm = z_re.hypot(z_im);
        if norm > 2.0 {
            return i;
        }
        //z = z * z + c;
        let (sq_re, sq_im) = complex_mul(z_re, z_im, z_re, z_im);
        let (ad_re, ad_im) = complex_add(sq_re, sq_im, cx, cy);
        z_re = ad_re;
        z_im = ad_im;
    }    
    max_iters
}

fn complex_add(re1: f64, im1: f64, re2: f64, im2: f64) -> (f64, f64){
    (
        re1 + re2,
        im1 + im2
    )
}

fn complex_mul(re1: f64, im1: f64, re2: f64, im2: f64) -> (f64, f64){
    (
        re1 * re2 - im1 * im2,
        re1 * im2 + re2 * im1
    )
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=1 => ' ',
                2..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.2, 0.75, -1.25, 1.25, 100, 48);
    render_mandelbrot(mandelbrot);
}
