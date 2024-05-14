use plotters::prelude::*;

#[derive(Debug)]
struct C32 {
    real: f32,
    imaginary: f32,
}

impl C32 {
    fn magnitude(&self) -> f32 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }
    fn argument(&self) -> f32 {
        f32::atan2(self.imaginary, self.real)
    }
    fn distance(&self, other: &C32) -> f32 {
        ((self.real - other.real).powi(2) + (self.imaginary - other.imaginary).powi(2)).sqrt()
    }
}


fn main() {
    // z^n = c
    let c = C32 {
        real: 100.0,
        imaginary: 0.0,
    };
    let n = 5; // >= 2!

    let roots = roots_of_unity(n, c);

    let (max_r, max_i) = roots.iter().fold((f32::NEG_INFINITY, f32::NEG_INFINITY), |(max_r, max_i), c32| {
        (max_r.max(c32.real), max_i.max(c32.imaginary))
    });

    let dimensions = max_r.max(max_i) * 1.2;
    let path = "src/roots_of_unity.png";
    plot_data(path, &roots, dimensions).unwrap();
}

fn plot_data(path: &str, data: &Vec<C32>, dimensions: f32) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(path, (500, 500)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("real vs complex", ("sans-serif", 25).into_font())
        .margin(10)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(-dimensions..dimensions, -dimensions..dimensions)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        data.iter().map(|point| {
            Circle::new((point.real, point.imaginary), 5, RED.filled())
        })
    )?;

    Ok(())
}

fn roots_of_unity(n: i32, c: C32) -> Vec<C32> {
    let mut roots: Vec<C32> = vec![];
    let magnitude = c.magnitude().powf(1.0/n as f32);
    let argument = c.argument();
    for i in 0..n {
        let angle = (std::f32::consts::TAU * i as f32 + argument) / n as f32;
        let complex_number = C32 {
            real: magnitude * angle.cos(),
            imaginary: magnitude * angle.sin(),
        };
        roots.push(complex_number);
    }
    roots
}
