use glam::*;

pub fn parabola_pos(target_pos: Vec3, height: f32, progress: f32) -> Vec3 {
    // start_pos = origine -> Add base_pos to return value
    let lerp_pos = target_pos * progress;
    let y = height * 4_f32 * progress * (1_f32 - progress);
    return Vec3::new(lerp_pos.x, y, lerp_pos.z);
}

pub fn circle_pos(radius: f32, progress: f32) -> Vec2 {
    // progress = (progress + 0.01) % (math.pi * 2)
    let x = progress.cos() * radius;
    let y = progress.sin() * radius;
    return Vec2 { x, y };
}

pub fn sphere_pos(latitude: f32, longitude: f32, radius: f32) -> Vec3 {
    // progress = (progress + 0.01) % (math.pi * 2)
    let x = latitude.sin() * longitude.cos() * radius;
    let y  = latitude.sin() * longitude.sin() * radius;
    let z = latitude.cos() * radius;
    return Vec3::new(x, y, z);
}

pub fn circle_points(radius: f32, segments: u16) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec!{};
    let point_distance = (std::f32::consts::PI * 2.0) / segments as f32;

    for i in 0..segments {
        points.push(circle_pos(radius, point_distance * i as f32));
    }

    return points;
}

// Draw Circle using Vector Basis -> Better when moving the Circle
pub fn vector_basis_circle(radius: f32, segments: u16) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec!{};

    let local_point = CVec2 { x: radius, y: 0.0 };
    
    for i in 0..segments {
        // Calculate the rotation of our basis for this segment
        let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        
        // Transform the point into the world basis
        let rotation = local_point.rotate(angle);
        points.insert(points.len(), rotation);
        
        // println!("Point {}: x: {:.2}, y: {:.2}", i, world_pos.x, world_pos.y);
    }

    return points;
}

// Custom Vector2
struct CVec2 {
    x: f32,
    y: f32,
}

impl CVec2 {
    fn rotate(&self, rotation_rad: f32) -> Vec2 {
        let rotated_x = self.x * rotation_rad.cos() - self.y * rotation_rad.sin();
        let rotated_y = self.x * rotation_rad.sin() + self.y * rotation_rad.cos();
        return Vec2 {
            x: rotated_x,
            y: rotated_y,
        };
    }

    // Test
    fn update_rotation(&mut self, radius: f32, angle: f32) {
        self.x = radius * angle.cos();
        self.y = radius * angle.sin();
    }

    fn set_origin(&self, origin: &Vec2) -> Vec2 {
        return Vec2 {
            x: origin.x + self.x,
            y: origin.y + self.y,
        };
    }

    fn to_world_basis(&self, origin: &CVec2, rotation_rad: f32) -> Vec2 {
        // 1. Rotate the point relative to its own (0,0)
        let rotated_x = self.x * rotation_rad.cos() - self.y * rotation_rad.sin();
        let rotated_y = self.x * rotation_rad.sin() + self.y * rotation_rad.cos();

        // 2. Translate it to the "Origin" of the new basis
        Vec2 {
            x: origin.x + rotated_x,
            y: origin.y + rotated_y,
        }
    }
}