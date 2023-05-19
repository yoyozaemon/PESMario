pub trait Obstacle {
    fn get_hit_box(&self, box: &dyn Obstacle) -> Rectangle;
    fn draw(&self, g2d: &mut Graphics2D);
    fn set(&self, camera_x: i32) -> i32;
}

