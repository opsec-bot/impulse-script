pub struct ScopeSensitivityCalculator {
    pub fov: f64,
    pub sens: f64,
    pub x_factor: f64,
    pub x1modifier: f64,
    pub x25modifier: f64,
}

impl ScopeSensitivityCalculator {
    pub fn new() -> Self {
        Self {
            fov: 0.0,
            sens: 0.0,
            x_factor: 0.0,
            x1modifier: 0.0,
            x25modifier: 0.0,
        }
    }

    fn calculate_ads(&self, modifier: f64, fov_multiplier: f64, ads_multiplier: f64) -> i32 {
        let fov_adjustment = (fov_multiplier * self.fov).to_radians().tan() / (self.fov.to_radians().tan());
        ((modifier / (ads_multiplier / fov_adjustment) * self.x_factor * ads_multiplier * self.sens).round()) as i32
    }

    pub fn get_rcs_values(
        &mut self,
        fov: f64,
        sens: f64,
        x1modifier: f64,
        x25modifier: f64,
        x_factor: f64,
    ) -> [i32; 2] {
        self.fov = fov;
        self.sens = sens;
        self.x_factor = x_factor;
        self.x1modifier = x1modifier;
        self.x25modifier = x25modifier;

        let x1_ads = self.calculate_ads(self.x1modifier, 0.9, 0.6);
        let x25_ads = self.calculate_ads(self.x25modifier, 0.42, 0.42);

        let x1_rcs = CursorMovementCalculator::calculate_cursor_movement(x1_ads);
        let x25_rcs = CursorMovementCalculator::calculate_cursor_movement(x25_ads);

        [x1_rcs, x25_rcs]
    }
}

pub struct CursorMovementCalculator;

impl CursorMovementCalculator {
    pub fn calculate_cursor_movement(new_sensitivity: i32) -> i32 {
        let k = 8.0 * 3.0;
        let cursor_movement = k / (new_sensitivity as f64);
        cursor_movement.round() as i32
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
