use crate::complex::{ Complex, ZERO };
use std::f64::consts::PI;

struct Spinner {
    phase: f64,
    frequency: f64,
    amplitude: f64,
}

impl Spinner {
    fn sample(&self, t: f64) -> Complex {
        self.amplitude * Complex::exp_i(self.frequency * t + self.phase)
    }
}

pub struct Spirograph {
    spinners: Vec<Spinner>,
}

impl Spirograph {
    pub fn new(inner_circle_ratio: f64, pen_ratio: f64, frequency: f64, pen_frequency_ratio: f64) -> Spirograph {
        let main = Spinner {
            frequency,
            amplitude: 1.0,
            phase: 0.0,
        };
        let inner = Spinner {
            frequency,
            amplitude: inner_circle_ratio,
            phase: PI,
        };
        let pen = Spinner {
            frequency: -pen_frequency_ratio * frequency,
            amplitude: inner_circle_ratio * pen_ratio,
            phase: 0.0,
        };
        Spirograph { spinners: vec![main, inner, pen] }
    }

    pub fn offset_phase(&mut self, offset: f64) {
        for spinner in &mut self.spinners {
            spinner.phase += offset;
        }
    }

    pub fn offset_amplitude(&mut self, offset: f64) {
        for spinner in &mut self.spinners {
            spinner.amplitude += offset;
        }
    }

    // return each partial sum, effectively sampling each spinner along the way
    // to the pen
    pub fn debug_sample(&self, t: f64) -> Vec<Complex> {
        let mut samples = Vec::new();
        let mut sum = ZERO;
        samples.push(sum);
        for spinner in &self.spinners {
            sum = sum + spinner.sample(t);
            samples.push(sum);
        }
        samples
    }

    pub fn sample(&self, t: f64) -> Complex {
        self.spinners.iter()
            .map(|spinner| spinner.sample(t))
            .fold(ZERO, std::ops::Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // pen at the center of the spinning disc
        let spiro = Spirograph::new(0.5, 0.0, 2.0 * PI, 0.0);
        assert_eq!(spiro.sample(0.0), Complex { re: 0.5, im: 0.0 });
        assert_eq!(spiro.sample(0.25), Complex { re: 0.0, im: 0.5 });
        assert_eq!(spiro.sample(0.5), Complex { re: -0.5, im: 0.0 });
        assert_eq!(spiro.sample(0.75), Complex { re: 0.0, im: -0.5 });
    }
}
