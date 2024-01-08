use nalgebra::{Matrix, SquareMatrix, Vector, Vector2};

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

pub struct Player {
    position: Vector2<f64>,
    direction: Vector2<f64>,
    cam_plane: Vector2<f64>,
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Vertical,
    Horizontal,
}

impl Player {
    fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
            cam_plane: Vector2::new(0.0, -1.0),
        }
    }

    fn at_position(x_pos: f64, y_pos: f64) -> Self {
        let mut p = Self::new();
        p.position.x = x_pos;
        p.position.y = y_pos;
        p
    }

    fn render_frame(&mut self) {
        for i in 0..SCREEN_WIDTH {
            let x = 2.0 * (i as f64) / (SCREEN_WIDTH as f64) - 1.0;
            // double rayDirX = dirX + planeX * cameraX;
            // double rayDirY = dirY + planeY * cameraX;
            let ray_dir = Vector2::new(
                self.direction.x + self.cam_plane.x * x,
                self.direction.y + self.cam_plane.y * x,
            );

            // let (hit_pos, side) = self.get_collision(ray_dir, self.map)
        }
    }

    /// Find position and side of wall hit of first wall hit by ray
    /// Uses a modified version of the dda algorithm
    fn get_collision(&self, dir: Vector2<f64>, map: Vec<Vec<i32>>) -> (Vector2<f64>, Side) {
        let point2 = dir * 2.0;
        let diff = point2 - dir;

        let m = (diff.y / diff.x).abs();
        let mut curr_position = self.position;

        let x_step = if diff.x < 0.0 { -1.0 } else { 1.0 };
        let y_step = if diff.y < 0.0 { -1.0 } else { 1.0 };

        let step = if m <= 1.0 {
            Vector2::new(x_step, y_step * m)
        } else {
            Vector2::new(x_step / m, y_step)
        };

        while map[curr_position.y.round() as usize][curr_position.x.round() as usize] == 0 {
            curr_position += step;
        }

        let side = if curr_position.x == curr_position.x.floor() {
            Side::Horizontal
        } else {
            Side::Vertical
        };

        return (curr_position, side);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_map() -> Vec<Vec<i32>> {
        vec![
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ]
    }

    #[test]
    fn get_collision_location_slope_of_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(1.0, 1.0), map);
        assert_eq!(pos, Vector2::new(3.0, 3.0));
    }

    #[test]
    fn get_collision_location_slope_less_than_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(2.0, 1.0), map);
        assert_eq!(pos, Vector2::new(3.0, 2.0));
    }

    #[test]
    fn get_collision_location_slope_greater_than_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(1.0, 2.0), map);
        assert_eq!(pos, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn get_collision_location_slope_of_neg_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(1.0, -1.0), map);
        assert_eq!(pos, Vector2::new(2.0, 0.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(2.0, -1.0), map);
        assert_eq!(pos, Vector2::new(3.0, 0.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 2.0);

        let (pos, side) = player.get_collision(Vector2::new(0.5, -1.0), map);
        assert_eq!(pos, Vector2::new(2.0, 0.0));
    }

    #[test]
    fn get_collision_location_slope_of_neg_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-1.0, 1.0), map);
        assert_eq!(pos, Vector2::new(0.0, 2.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-1.0, 2.0), map);
        assert_eq!(pos, Vector2::new(1.0, 3.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-4.0, 2.0), map);
        assert_eq!(pos, Vector2::new(0.0, 2.0));
    }

    #[test]
    fn get_collision_location_slope_of_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-1.0, -1.0), map);
        assert_eq!(pos, Vector2::new(0.0, 0.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-1.0, -2.0), map);
        assert_eq!(pos, Vector2::new(0.5, 0.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, side) = player.get_collision(Vector2::new(-2.0, -1.0), map);
        assert_eq!(pos, Vector2::new(0.0, 0.5));
    }

    #[test]
    fn get_collision_location_slope_0() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, _) = player.get_collision(Vector2::new(1.0, 0.0), map);
        assert_eq!(pos, Vector2::new(3.0, 1.0));
    }

    #[test]
    fn get_collision_location_slope_undefined() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (pos, _) = player.get_collision(Vector2::new(0.0, 1.0), map);
        assert_eq!(pos, Vector2::new(1.0, 3.0));
    }

    #[test]
    fn get_collision_picks_horizontal_side_x_pos() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (_, side) = player.get_collision(Vector2::new(0.4, 1.0), map);
        assert_eq!(side, Side::Vertical);
    }

    #[test]
    fn get_collision_picks_vertical_side() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (_, side) = player.get_collision(Vector2::new(0.4, 1.0), map);
        assert_eq!(side, Side::Vertical);
    }

    #[test]
    fn get_collision_picks_horizontal_side() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (_, side) = player.get_collision(Vector2::new(1.0, 2.0), map);
        assert_eq!(side, Side::Horizontal);
    }
}
