use std::f64::consts::PI;

use nalgebra::{matrix, Matrix, SquareMatrix, Vector, Vector2};

pub struct Player {
    position: Vector2<f64>,
    direction: Vector2<f64>,
    cam_plane: Vector2<f64>,
    move_speed: f64,
    rotate_speed: f64,
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
            move_speed: 3.0,
            rotate_speed: 3.0,
        }
    }

    pub fn at_position(x_pos: f64, y_pos: f64) -> Self {
        let mut p = Self::new();
        p.position.x = x_pos;
        p.position.y = y_pos;
        p
    }

    pub fn get_wall_distances(
        &mut self,
        map: &Vec<Vec<i32>>,
        view_width: u32,
    ) -> (Vec<f64>, Vec<Side>) {
        let mut distances = Vec::new();
        let mut sides = Vec::new();
        for i in 0..view_width {
            let offset = 2.0 * (i as f64) / (view_width as f64) - 1.0;
            let ray_dir = self.direction + self.cam_plane * offset;

            let (hit_vec, side) = self.get_collision(ray_dir, &map);
            let hit_cam_plane_proj = hit_vec.dot(&self.cam_plane) * self.cam_plane;
            let perp_distance = (hit_vec - hit_cam_plane_proj).magnitude();
            distances.push(perp_distance);
            sides.push(side)
        }
        (distances, sides)
    }

    pub fn move_player(&mut self, map: &Vec<Vec<i32>>, delta: f64, is_moving_forward: bool) {
        let dir = if is_moving_forward { 1.0 } else { -1.0 };
        let move_vector = self.direction * dir * self.move_speed * delta;
        let new_position = self.position + move_vector;
        if map[new_position.y as usize][new_position.x as usize] == 0 {
            self.position = new_position;
        }
    }

    pub fn rotate_player(&mut self, delta: f64, is_rotating_clockwise: bool) {
        let direction = if is_rotating_clockwise { 1.0 } else { -1.0 };
        let angle = PI / 2.0 * self.rotate_speed * delta * direction;
        let rot_matrix = matrix![angle.cos(), -angle.sin();
                                 angle.sin(), angle.cos()];
        self.direction = rot_matrix * (self.direction);
        self.cam_plane = rot_matrix * (self.cam_plane);
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
