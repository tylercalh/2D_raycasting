use macroquad::prelude::*;
use std::vec::Vec;
use core::f32::consts::PI;

const FOV: usize = 70;
const PLAYER_SPEED: f32 = 90.0;

#[macroquad::main("DOOM")]
async fn main() {
    // Initialize wall segments.
    let wall_l: Segment = Segment::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, screen_height()));
    let wall_t: Segment = Segment::new(Vec2::new(1.0, 1.0), Vec2::new(screen_width(), 1.0));
    let wall_r: Segment = Segment::new(Vec2::new(screen_width(), 1.0), Vec2::new(screen_width(), screen_height()));
    let wall_b: Segment = Segment::new(Vec2::new(1.0, screen_height()), Vec2::new(screen_width(),screen_height()));
    let wall_01: Segment = Segment::new(Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0));
    let wall_02: Segment = Segment::new(Vec2::new(100.0, 100.0), Vec2::new(100.0, 200.0));
    let wall_03: Segment = Segment::new(Vec2::new(200.0, 200.0), Vec2::new(200.0, 100.0));
    let wall_04: Segment = Segment::new(Vec2::new(100.0, 200.0), Vec2::new(200.0, 200.0));
    let wall_11: Segment = Segment::new(Vec2::new(400.0, 200.0), Vec2::new(500.0, 250.0));
    let wall_12: Segment = Segment::new(Vec2::new(400.0, 400.0), Vec2::new(400.0, 500.0));
    let wall_13: Segment = Segment::new(Vec2::new(500.0, 500.0), Vec2::new(500.0, 400.0));
    let wall_14: Segment = Segment::new(Vec2::new(400.0, 500.0), Vec2::new(500.0, 500.0));
    let wall_21: Segment = Segment::new(Vec2::new(60.0, 80.0), Vec2::new(300.0, 500.0));
    let wall_22: Segment = Segment::new(Vec2::new(220.0, 40.0), Vec2::new(60.0, 50.0));

    // Intitialize maps.
    let map_a: Vec<Segment> = vec![wall_l, wall_t, wall_r, wall_b, wall_01, wall_02, wall_03, wall_04, wall_11, wall_12, wall_13, wall_14];
    let map_b: Vec<Segment> = vec![wall_l, wall_t, wall_r, wall_b, wall_21, wall_22];

    let mut ray_cast: RayCast = RayCast::new(&Vec2::new(100.0, 150.0));

    // Time stamps for timed events.
    let mut time_stamp_flicker: f64 = 9.0;
    let mut time_stamp_ten: f64 = 10.0;
    let mut flip: usize = 0b1;

    loop {
        clear_background(BLACK);
        
        let mut mapa_or_mapb = match flip {
            0 => &map_a,
            1 => &map_b,
            _=> &map_a,
        };

        // Every 10 seconds flicker between maps.
        if get_time() > time_stamp_flicker {
            let square_wave: f32 = ((-1.0_f32).powf((9.0 * (get_time() as f32) ).floor()) + 1.0) / 2.0;

            mapa_or_mapb = match square_wave{
                0.0 => &map_a,
                1.0 => &map_b,
                _=> &map_a,
            };
        }
        
        // Every 10 seconds switch maps.
        if  get_time() > time_stamp_ten {
            time_stamp_flicker = get_time() + 9.0;
            time_stamp_ten = get_time() + 10.0;
            flip = !flip & 0b1;
            println!("{} {}", get_time(), flip);
        }

        // Draw current map's walls.
        for bound in mapa_or_mapb.iter() {
            bound.draw(&flip);
        }

        // Obtain raycast collision points with the current map.
        let points = ray_cast.look(mapa_or_mapb);

        ray_cast.draw();

        keyboard_input(&mut ray_cast, &points, &flip);
        draw_minimap(&mapa_or_mapb, &ray_cast, &flip);

        next_frame().await
    }
}

// Handle keyboard inputs.
fn keyboard_input(ray_cast: &mut RayCast, points: &Vec<Vec2>, flip: &usize) {
    let fov: f32 = FOV as f32;
    let middle: Vec2 = ray_cast.rays[(fov/2.0) as usize].direction;

    if is_key_down(KeyCode::W) {
        let middle = middle.normalize();
        let x = ray_cast.position.x + (middle.x * PLAYER_SPEED * get_frame_time());
        let y = ray_cast.position.y + (middle.y * PLAYER_SPEED * get_frame_time());
        ray_cast.translate((&x, &y));
    }

    if is_key_down(KeyCode::S) {
        let middle = middle.normalize();
        let x = ray_cast.position.x - (middle.x * PLAYER_SPEED * get_frame_time());
        let y = ray_cast.position.y - (middle.y * PLAYER_SPEED * get_frame_time());
        ray_cast.translate((&x, &y));
    }

    if is_key_down(KeyCode::A) {
        ray_cast.rotate(-0.02);
    }

    if is_key_down(KeyCode::D) {
        ray_cast.rotate(0.02);
    }

    if is_key_down(KeyCode::Space) {
        clear_background(BLACK);

        let w = screen_width() / fov;


        let mut index: f32 = 0.0;
        for point in points.iter() {

            // Calculate point's distance from camera plane using point's euclidean distance.
            let mut distance: f32 = point.distance(ray_cast.position);

            let mut angle_a: f32 = middle.angle_between(ray_cast.rays[index as usize].direction);
            angle_a = angle_a * 180.0/PI;
            let mut angle_b: f32 = 90.0 - angle_a;
            angle_b = angle_b * PI/180.0;
            distance = distance * angle_b.sin();

            // Use distance to determine height and brightness of point in 3D perspective. 
            let height = (-(screen_height())/screen_width()) * distance.clamp(0.0, screen_width()) + screen_height();
            let brightness = -(distance.clamp(0.0, screen_width())) / screen_width() + 0.90;
            let color: Color = 
                match flip {
                    0 => Color::new(0.0, brightness, 0.25, brightness),
                    1 => Color::new(brightness, 0.0, 0.25, brightness),
                    _=> Color::new(0.0, brightness, 0.25, brightness),
                };

            draw_rectangle(index * w, (screen_height() - height) / 2.0, w, height, color);
            //draw_rectangle_lines(index * w, (screen_height() - height) / 2.0, w, height, 1.0, Color::new(0.0, brightness, 0.0, 1.0));
            index += 1.0;
        }
    }


}

fn draw_minimap(bounds: &Vec<Segment>, ray_cast: &RayCast, flip: &usize) {
    let scale: f32 = 0.25;
    let w: f32 = screen_width() * scale;
    let h: f32 = screen_height() * scale;
    let xoff: f32 = 20.0;
    let yoff: f32 = 20.0;

    let mut x1: f32 = screen_width() - w - xoff;
    let mut x2: f32 = w;
    let mut y1: f32 = yoff;
    let mut y2: f32 = h;

    draw_rectangle(x1, y1, x2, y2, BLACK);

    let color: Color = match flip {
        0 => GREEN,
        1 => RED,
        _=> BLUE, 
    };

    for bound in bounds.iter() {
        x1 = bound.p_a.x * scale + screen_width() - w - xoff;
        y1 = bound.p_a.y * scale + yoff;
        x2 = bound.p_b.x * scale + screen_width() - w - xoff;
        y2 = bound.p_b.y * scale + yoff;
        draw_line(x1, y1, x2, y2, 1.0, color);
    }

    x1 = ray_cast.position.x * scale + screen_width() - w - xoff;
    y1 = ray_cast.position.y * scale + yoff;
    draw_circle(x1, y1, 3.0, WHITE);
}


// SEGMENT
    #[derive(Copy, Clone)]
    struct Segment {
        p_a: Vec2,
        p_b: Vec2,
    }
    impl Segment {
        fn new(p1: Vec2, p2: Vec2) -> Segment {
            Segment {
                p_a: p1,
                p_b: p2,
            }
        }

        fn draw(&self, flip: &usize) {
            match flip {
                0 => draw_line(self.p_a.x, self.p_a.y, self.p_b.x, self.p_b.y, 4.0, GREEN),
                1 =>  draw_line(self.p_a.x, self.p_a.y, self.p_b.x, self.p_b.y, 4.0, RED),
                _=>  draw_line(self.p_a.x, self.p_a.y, self.p_b.x, self.p_b.y, 4.0, RED),
            };
        }
    }


// RAY
    struct Ray {
        position: Vec2,
        direction: Vec2,
    }

    impl Ray {
        fn new(src: Vec2, dir: Vec2) -> Ray {
            Ray {
                position: src,
                direction: dir.normalize(),
            }
        }

        fn draw(&self) {
            draw_line(
                self.position.x,
                self.position.y,
                self.position.x + self.direction.x * 1.0,
                self.position.y + self.direction.y * 1.0,
                1.0,
                YELLOW
            );
        }

        // Returns the point where self intersects the segment.
        // If no intersection, returns None.
        fn cast(&self, seg: &Segment) -> Option<Vec2> {
            let x1 = seg.p_a.x;
            let y1 = seg.p_a.y;
            let x2 = seg.p_b.x;
            let y2 = seg.p_b.y;

            let x3 = self.position.x;
            let y3 = self.position.y;
            let x4 = self.position.x + self.direction.x;
            let y4 = self.position.y + self.direction.y;

            let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            if den == 0.0 {
                return None;
            }

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
            let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
            
            if t > 0.0 && t < 1.0 && u > 0.0 {
                let intersection: Vec2 = 
                    Vec2::new(
                        x1 + t * (x2 - x1),
                        y1 + t * (y2 - y1)
                    );
                
                Some(intersection)
            } else {
                None
            }
            
        }
    }


//RAYCAST
    struct RayCast {
        position: Vec2,
        rays: Vec<Ray>,
    }
    impl RayCast {
        fn new(pos: &Vec2) -> RayCast {
            // Create Vec of rays for the player FOV.
            let mut r: Vec<Ray> = Vec::new();
            
            for degree in 0..=FOV {
                let rad: f32 = degree as f32 * (PI / 180.0);
                
                let dir: Vec2 = Vec2::new(
                    rad.cos(),
                    rad.sin()
                ).normalize();

                let ray: Ray = Ray::new(*pos, dir);
                r.push(ray);
            }

            RayCast {
                position: *pos,
                rays: r,
            }
        }

        fn draw(&self) {
            draw_circle(self.position.x, self.position.y, 5.0, GREEN);
            for i in 0..self.rays.len() {
                self.rays[i].draw();
            }
        }

        fn translate(&mut self, pos: (&f32, &f32)) {
            self.position = Vec2::new(*pos.0, *pos.1);
            for i in 0..self.rays.len() {
                self.rays[i].position = Vec2::new(*pos.0, *pos.1);
            }
        }

        fn rotate(&mut self, rotation: f32) {
            for ray in self.rays.iter_mut() {
                ray.direction = Vec2::from_angle(rotation).rotate(ray.direction);
            }
        }

        // Return a Vec of distances between raycast's position and a map of segments.
        fn look(&self, segs: &Vec<Segment>) -> Vec<Vec2> {
            let mut points: Vec<Vec2> = Vec::new();

            for i in 0..self.rays.len() {
                let mut min: f32 = f32::INFINITY;
                let mut draw: bool = false;
                let mut intersection_point: Vec2 = Vec2::new(self.position.x, self.position.y);

                for seg in segs.iter() {
                    let hit: Option<Vec2> = self.rays[i].cast(seg);
                    match hit {
                        Some(point) => 
                            {
                                let distance = self.rays[i].position.distance(point);
                                // Only record intersection point distance with closest segment. 
                                if distance < min {
                                    min = distance;
                                    draw = true;
                                    intersection_point = Vec2::new(point.x, point.y);
                                }
                                
                            },
                        None => (),
                    }
                }
                if draw {
                    draw_line(
                        self.position.x,
                        self.position.y,
                        intersection_point.x,
                        intersection_point.y,
                        1.0,
                        DARKGRAY);

                    draw_circle(intersection_point.x, intersection_point.y, 3.0, BLUE);

                    points.push(intersection_point);
                }

            }

            points
        }
    }