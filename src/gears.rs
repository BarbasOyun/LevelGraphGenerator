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
