use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

struct Bouncer {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    start_x: i32,
    bouncer: Rc<RefCell<image::DynamicImage>>,
    hit_box: Rectangle,
}

impl Bouncer {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Bouncer {
        let start_x = x;
        let image_path = Path::new("../assests/bouncer.png");
        let bouncer = Rc::new(RefCell::new(image::open(image_path).expect("Failed to load bouncer image")));
        let hit_box = Rectangle::new(x, y, width, height);
        
        Bouncer {
            x,
            y,
            width,
            height,
            start_x,
            bouncer,
            hit_box,
        }
    }
    
    fn draw(&self, mut ctx: image::RgbImage) {
        let bouncer_image = self.bouncer.borrow();
        let bouncer_image = bouncer_image.deref();
        
        imageproc::drawing::draw_hollow_rectangle_mut(&mut ctx, self.hit_box, image::Rgb([0, 0, 0]));
        imageproc::drawing::draw_filled_rect_mut(&mut ctx, self.hit_box.inner(), image::Rgb([0, 0, 0]));
        ctx.copy_from(bouncer_image, self.x as u32 - 5, self.y as u32 - 2);
    }
    
    fn set(&mut self, camera_x: i32) -> i32 {
        self.x = self.start_x - camera_x;
        self.hit_box.x = self.x;
        
        self.x
    }
    
    fn get_hit_box(&self) -> Rectangle {
        self.hit_box
    }
}

