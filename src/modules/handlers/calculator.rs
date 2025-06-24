use std::f64::consts::PI;

pub struct ScopeSensitivityCalculator {
    fov: f64,
    sens: f64,
    xfactor: f64,
    x1modifier: f64,
    x25modifier: f64,
}

impl ScopeSensitivityCalculator {
    pub fn new() -> Self {
        Self {
            fov: 0.0,
            sens: 0.0,
            xfactor: 0.0,
            x1modifier: 0.0,
            x25modifier: 0.0,
        }
    }

    fn calculate_ads(&self, modifier: f64, fov_multiplier: f64, ads_multiplier: f64) -> i32 {
        let fov_adjustment = ((fov_multiplier * self.fov) * PI / 180.0 / 2.0).tan()
            / ((self.fov * PI / 180.0 / 2.0).tan());
        ((modifier / (ads_multiplier / fov_adjustment) * self.xfactor * ads_multiplier * self.sens).round()) as i32
    }

    pub fn calculate_ads_values(&self) -> (i32, i32) {
        let x1_ads = self.calculate_ads(self.x1modifier, 0.9, 0.6);
        let x25_ads = self.calculate_ads(self.x25modifier, 0.42, 0.42);
        (x1_ads, x25_ads)
    }

    pub fn calculate_cursor_movement(&self, new_sensitivity: i32) -> i32 {
        let sensitivity = 8.0;
        let movement = 3.0;
        let k = sensitivity * movement;
        (k / new_sensitivity as f64).round() as i32
    }

    pub fn get_rcs_values(&mut self, fov: f64, sens: f64, x1modifier: f64, x25modifier: f64, xfactor: f64) -> Vec<i32> {
        self.fov = fov;
        self.sens = sens;
        self.xfactor = xfactor;
        self.x1modifier = x1modifier;
        self.x25modifier = x25modifier;

        let (x1_ads, x25_ads) = self.calculate_ads_values();
        let x1_rcs = self.calculate_cursor_movement(x1_ads);
        let x25_rcs = self.calculate_cursor_movement(x25_ads);

        vec![x1_rcs, x25_rcs]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcs_values() {
        let mut calc = ScopeSensitivityCalculator::new();
        let rcs = calc.get_rcs_values(90.0, 7.0, 58.0, 146.0, 0.02);
        println!("{:?}", rcs);
    }
} 

fn main() {
    let mut calc = ScopeSensitivityCalculator::new();
    let result = calc.get_rcs_values(90.0, 7.0, 58.0, 146.0, 0.02);
    println!("{:?}", result);
}
