use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

struct GodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Player {
    base: Base<Node3D>,
    vector: Vec<f32>,
    rng: ThreadRng,
}

#[godot_api]
impl INode3D for Player {
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        let mut rng = rand::rng();
        let count = 1000;
        let vector: Vec<f32> = (0..count)
            .flat_map(|_| {
                let tx = rng.random::<f32>() * 2.0 - 1.0;
                let ty = rng.random::<f32>() * 2.0 - 1.0;
                let tz = rng.random::<f32>() * 2.0 - 1.0;
                let rx = rng.random::<f32>() * PI;
                let ry = rng.random::<f32>() * PI;
                let rz = rng.random::<f32>() * PI;
                Player::euler_to_mat3((tx, ty, tz), (1.0, 1.0, 1.0), (rx, ry, rz))
            })
            .collect();
        Self { base, vector, rng }
    }
}

#[godot_api]
impl Player {
    fn euler_to_mat3(t: (f32, f32, f32), s: (f32, f32, f32), r: (f32, f32, f32)) -> [f32; 12] {
        let (sx, cx) = r.0.sin_cos();
        let (sy, cy) = r.1.sin_cos();
        let (sz, cz) = r.2.sin_cos();

        [
            s.0 * cy * cz,
            -cy * sz,
            sy,
            t.0,
            cx * sz + sx * sy * cz,
            (cx * cz - sx * sy * sz) * s.1,
            -sx * cy,
            t.1,
            sx * sz - cx * sy * cz,
            sx * cz + cx * sy * sz,
            s.2 * cx * cy,
            t.2,
        ]
    }
    #[func]
    fn get_points(&mut self) -> Array<f32> {
        for chunk in self.vector.chunks_mut(12) {
            let s = 1.0;
            let tx = self.rng.random::<f32>() * 2.0 - 1.0;
            let ty = self.rng.random::<f32>() * 2.0 - 1.0;
            let tz = self.rng.random::<f32>() * 2.0 - 1.0;
            let rx = self.rng.random::<f32>() * PI;
            let ry = self.rng.random::<f32>() * PI;
            let rz = self.rng.random::<f32>() * PI;
            chunk.copy_from_slice(&Player::euler_to_mat3(
                (tx, ty, tz),
                (s, s, s),
                (rx, ry, rz),
            ));
        }
        self.vector.to_godot()
    }

    #[func]
    fn benchmark(&mut self) {
        let iterations = 10_000;

        let start = Instant::now();

        for _ in 0..iterations {
            self.get_points();
        }

        let total = start.elapsed();
        let per_iter = total / iterations;

        godot_print!("Total time: {:?}", total);
        godot_print!("Per iteration: {:?}", per_iter);
    }
}
