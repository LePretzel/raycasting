use nalgebra::{distance, Matrix, SquareMatrix, Vector, Vector2};

pub struct Player {
    position: Vector2<f64>,
    direction: Vector2<f64>,
    cam_plane: Vector2<f64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Vertical,
    Horizontal,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
            cam_plane: Vector2::new(0.0, 1.0),
        }
    }

    pub fn at_position(x_pos: f64, y_pos: f64) -> Self {
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

    /// Find position and side of wall of first wall hit by ray
    /// Uses a modified version of the dda algorithm
    /// Returns vector from player position to collision position
    fn get_collision(&self, dir: Vector2<f64>, map: &Vec<Vec<i32>>) -> (Vector2<f64>, Side) {
        let point2 = dir * 2.0;
        let diff = point2 - dir;
        let m = (diff.y / diff.x).abs();

        let delta_dist_x = Vector2::new(1.0, m).magnitude();
        let delta_dist_y = Vector2::new(1.0, 1.0 / m).magnitude();

        let x_step = if diff.x < 0.0 { -1.0 } else { 1.0 };
        let y_step = if diff.y < 0.0 { -1.0 } else { 1.0 };

        let mut curr_distance = Vector2::new(0.0, 0.0);
        let mut map_position = Vector2::new(self.position.x.floor(), self.position.y.floor());
        curr_distance.x += if x_step > 0.0 {
            (1.0 - self.position.x + map_position.x) * delta_dist_x
        } else {
            (self.position.x - map_position.x) * delta_dist_x
        };
        curr_distance.y += if y_step > 0.0 {
            (1.0 - self.position.y + map_position.y) * delta_dist_y
        } else {
            (self.position.y - map_position.y) * delta_dist_y
        };

        let mut total_distance = 0.0;
        let mut side = Side::Horizontal;
        while map[map_position.y as usize][map_position.x as usize] == 0 {
            if curr_distance.x < curr_distance.y {
                side = Side::Vertical;
                total_distance = curr_distance.x;
                curr_distance.x += delta_dist_x;
                map_position.x += x_step;
            } else {
                side = Side::Horizontal;
                total_distance = curr_distance.y;
                curr_distance.y += delta_dist_y;
                map_position.y += y_step;
            }
        }

        let hit_vector = total_distance * dir.normalize();

        return (hit_vector, side);
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

    fn from_origin(player: Player, from_player: Vector2<f64>) -> Vector2<f64> {
        from_player + player.position
    }

    #[test]
    fn get_collision_location_slope_of_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(1.0, 1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(3.0, 3.0));
    }

    #[test]
    fn get_collision_location_slope_less_than_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(2.0, 1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(3.0, 2.0));
    }

    #[test]
    fn get_collision_location_slope_greater_than_1_pos_x() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(1.0, 2.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(2.0, 3.0));
    }

    #[test]
    fn get_collision_location_slope_of_neg_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(1.0, -1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 1.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 2.0);

        let (hit, _) = player.get_collision(Vector2::new(2.0, -1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(3.0, 1.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_pos_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(1.0, 2.0);

        let (hit, _) = player.get_collision(Vector2::new(0.5, -1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.5, 1.0));
    }

    #[test]
    fn get_collision_location_slope_of_neg_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(-1.0, 1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 2.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(-1.0, 2.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 3.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_neg_x_pos_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(-4.0, 2.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 1.5));
    }

    #[test]
    fn get_collision_location_slope_of_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 2.0);

        let (hit, _) = player.get_collision(Vector2::new(-1.0, -1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 1.0));
    }

    #[test]
    fn get_collision_location_abs_slope_greater_than_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 2.0);

        let (hit, _) = player.get_collision(Vector2::new(-1.0, -2.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.5, 1.0));
    }

    #[test]
    fn get_collision_location_abs_slope_less_than_1_neg_x_neg_y() {
        let map = simple_map();
        let player = Player::at_position(2.0, 2.0);

        let (hit, _) = player.get_collision(Vector2::new(-2.0, -1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 1.5));
    }

    #[test]
    fn get_collision_location_slope_0() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(1.0, 0.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(3.0, 1.0));
    }

    #[test]
    fn get_collision_location_slope_undefined() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (hit, _) = player.get_collision(Vector2::new(0.0, 1.0), &map);
        assert_eq!(from_origin(player, hit), Vector2::new(1.0, 3.0));
    }

    #[test]
    fn get_collision_picks_vertical_side() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (_, side) = player.get_collision(Vector2::new(1.0, 0.0), &map);
        assert_eq!(side, Side::Vertical);
    }

    #[test]
    fn get_collision_picks_horizontal_side() {
        let map = simple_map();
        let player = Player::at_position(1.0, 1.0);

        let (_, side) = player.get_collision(Vector2::new(0.2, 1.0), &map);
        assert_eq!(side, Side::Horizontal);
    }
}
