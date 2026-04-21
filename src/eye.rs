use crate::simulation::Food;
use nalgebra as na;
use std::f32::consts::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(
                &na::Vector2::y(),
                &vec,
            ).angle();

            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use test_case::test_case;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, 5);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            let actual_vision: Vec<String> = actual_vision
                .iter()
                .map(|cell| {
                    if *cell >= 0.7 {
                        "#".to_string()
                    } else if *cell >= 0.3 {
                        "+".to_string()
                    } else if *cell > 0.0 {
                        ".".to_string()
                    } else {
                        " ".to_string()
                    }
                })
                .collect();

            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    // -----------------------------------------------
    // | FOV range tests                             |
    // -----------------------------------------------

    #[test_case(1.0, "  +  ")] // food clearly in range
    #[test_case(0.9, "  +  ")]
    #[test_case(0.5, "     ")] // food just out of range
    #[test_case(0.1, "     ")] // food out of range
    fn test_fov_range(fov_range: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_range,
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            expected_vision,
        }
        .run()
    }

    // -----------------------------------------------
    // | FOV angle tests                             |
    // -----------------------------------------------

    #[test_case(0.25 * PI, "  +  ")] // narrow FOV
    #[test_case(0.50 * PI, "  +  ")] // food comes into view
    #[test_case(1.00 * PI, "  +  ")] // food visible
    #[test_case(2.00 * PI, "  +  ")] // full 360, food visible
    fn test_fov_angle(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_range: 1.0,
            fov_angle,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            expected_vision,
        }
        .run()
    }
}