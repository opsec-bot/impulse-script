use std::collections::HashMap;

pub struct ScopeSensitivityCalculator {
    pub fov: f64,
    pub sens: f64,
    pub x_factor: f64,
    pub x1modifier: f64,
    pub x15modifier: f64,
    pub x2modifier: f64,
    pub x25modifier: f64,
    pub x3modifier: f64,
    pub x4modifier: f64,
}

impl ScopeSensitivityCalculator {
    pub fn new(
        fov: f64,
        sens: f64,
        x_factor: f64,
        x1modifier: f64,
        x15modifier: f64,
        x2modifier: f64,
        x25modifier: f64,
        x3modifier: f64,
        x4modifier: f64
    ) -> Self {
        Self {
            fov,
            sens,
            x_factor,
            x1modifier,
            x15modifier,
            x2modifier,
            x25modifier,
            x3modifier,
            x4modifier,
        }
    }

    fn calculate_ads(&self, modifier: f64, fov_multiplier: f64, ads_multiplier: f64) -> i32 {
        let fov_adjustment =
            (fov_multiplier * self.fov).to_radians().tan() / self.fov.to_radians().tan();
        (
            (modifier / (ads_multiplier / fov_adjustment)) *
            self.x_factor *
            ads_multiplier *
            self.sens
        ).round() as i32
    }

    pub fn calculate_ads_values(&self) -> HashMap<String, i32> {
        let mut map = HashMap::new();
        map.insert("x1 ADS".to_string(), self.calculate_ads(self.x1modifier, 0.9, 0.6));
        map.insert("x15 ADS".to_string(), self.calculate_ads(self.x15modifier, 0.59, 0.59));
        map.insert("x2 ADS".to_string(), self.calculate_ads(self.x2modifier, 0.49, 0.49));
        map.insert("x25 ADS".to_string(), self.calculate_ads(self.x25modifier, 0.42, 0.42));
        map.insert("x3 ADS".to_string(), self.calculate_ads(self.x3modifier, 0.35, 0.35));
        map.insert("x4 ADS".to_string(), self.calculate_ads(self.x4modifier, 0.3, 0.3));
        map
    }
}

pub struct CursorMovementCalculator;

impl CursorMovementCalculator {
    pub fn calculate_cursor_movement(new_sensitivity: i32, dpi: i32) -> i32 {
        // SENSITIVITY = 8, MOVEMENT = 3, k = SENSITIVITY * MOVEMENT
        let k = 8.0 * 3.0;
        let cursor_movement = k / (new_sensitivity as f64);
        cursor_movement.round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recoil_output() {
        let calculator = ScopeSensitivityCalculator::new(
            90.0,
            7.0,
            0.02,
            58.0,
            100.0,
            123.0,
            146.0,
            177.0,
            200.0
        );

        let ads_values = calculator.calculate_ads_values();
        for (scope, ads_val) in ads_values.iter() {
            let recoil = CursorMovementCalculator::calculate_cursor_movement(*ads_val, 800);
            println!("{}: {}\nRecoil Amount: {}", scope, ads_val, recoil);
        }
    }
}
