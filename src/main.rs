mod error;

use sdl2::{
	event::Event, keyboard::Keycode,
	pixels::Color, rect::FPoint,
	render::Canvas, video::Window, EventPump,
};

use error::RendererError;

const PI: f32 = 3.1415926535;

fn deg_to_rad(degrees: f32) -> f32 {
	return degrees / 180.0 * PI;
}

#[derive(Debug, Clone)]
struct Mat4x4 {
	pub m: [[f32; 4]; 4]
}

impl Mat4x4 {
	pub fn new() -> Mat4x4 {
		return Mat4x4 { m: [[0.0; 4]; 4] }
	}

	pub fn set(&mut self, r: usize, c: usize, value: f32) {
		assert!(r <= 3 && c <= 3);
		self.m[r][c] = value;
	}

	// multiplies a 1x3 vector by a 4x4 matrix
	pub fn multiply_vector(&self, input: &Vec3D) -> Vec3D {
		let mut res: Vec3D = Vec3D {
			x: input.x * self.m[0][0] + input.y * self.m[1][0] + input.z * self.m[2][0] + self.m[3][0],
			y: input.x * self.m[0][1] + input.y * self.m[1][1] + input.z * self.m[2][1] + self.m[3][1],
			z: input.x * self.m[0][2] + input.y * self.m[1][2] + input.z * self.m[2][2] + self.m[3][2],
		};
		let w: f32 = input.x * self.m[0][3] + input.y * self.m[1][3] + input.z * self.m[2][3] + self.m[3][3];
		if w == 0.0 { return res; }
		res.x /= w;
		res.y /= w;
		res.z /= w;
		return res;
	}
}

#[derive(Debug, Clone)]
struct Vec3D { x: f32, y: f32, z: f32, }

impl Vec3D {
	pub fn add(&mut self, number: f32) {
		self.x += number;
		self.y += number;
		self.z += number;
	}
}

#[derive(Debug, Clone)]
struct Triangle { points: [Vec3D; 3], }

impl Triangle {
	pub fn translate(&mut self, x: f32, y: f32, z: f32) {
		for p in &mut self.points {
			p.x += x;
			p.y += y;
			p.z += z;
		}
	}

	pub fn apply_matrix_transform(&mut self, matrix: &Mat4x4) {
		for p in &mut self.points {
			*p = matrix.multiply_vector(p);
		}
	}
}

#[derive(Debug, Clone)]
struct Mesh { tris: Vec<Triangle>, }

fn initialize_canvas(dimensions: (u32, u32), title: &str) -> anyhow::Result<(Canvas<Window>, EventPump)> {
	let ctx = sdl2::init().or_else(|err| {
		return Err(RendererError::InitError(err));
	})?;

	let video_subsystem = ctx.video().or_else(|err| {
		return Err(RendererError::InitError(err));
	})?;

	let event_pump = ctx.event_pump().or_else(|err| {
		return Err(RendererError::InitError(err));
	})?;

	let mut canvas = video_subsystem.window(title, dimensions.0, dimensions.1)
		.position(400, 250)
		.build()?
		.into_canvas()
		.build()?;

	canvas.set_draw_color(Color::RGB(0, 0, 0));

	return Ok((canvas, event_pump));
}

fn main() -> anyhow::Result<()> {
	let cube_mesh: Mesh = Mesh { tris: vec![
		// south
		Triangle { points: [Vec3D { x: 0.0, y: 0.0, z: 0.0, }, Vec3D { x: 0.0, y: 1.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 0.0, }], },
		Triangle { points: [Vec3D { x: 0.0, y: 0.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 0.0, }, Vec3D { x: 1.0, y: 0.0, z: 0.0, }], },

		// east
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 1.0, }], },
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 1.0, }, Vec3D { x: 1.0, y: 0.0, z: 1.0, }], },

		// north
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 1.0, }, Vec3D { x: 1.0, y: 1.0, z: 1.0, }, Vec3D { x: 0.0, y: 1.0, z: 1.0, }], },
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 1.0, z: 1.0, }, Vec3D { x: 0.0, y: 0.0, z: 1.0, }], },

		// west
		Triangle { points: [Vec3D { x: 0.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 1.0, z: 1.0, }, Vec3D { x: 0.0, y: 1.0, z: 0.0, }], },
		Triangle { points: [Vec3D { x: 0.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 1.0, z: 0.0, }, Vec3D { x: 0.0, y: 0.0, z: 0.0, }], },

		// top
		Triangle { points: [Vec3D { x: 0.0, y: 1.0, z: 0.0, }, Vec3D { x: 0.0, y: 1.0, z: 1.0, }, Vec3D { x: 1.0, y: 1.0, z: 1.0, }], },
		Triangle { points: [Vec3D { x: 0.0, y: 1.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 0.0, }, Vec3D { x: 1.0, y: 1.0, z: 0.0, }], },

		// bottom
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 0.0, z: 0.0, }], },
		Triangle { points: [Vec3D { x: 1.0, y: 0.0, z: 1.0, }, Vec3D { x: 0.0, y: 0.0, z: 0.0, }, Vec3D { x: 1.0, y: 0.0, z: 0.0, }], },
	]};

	const FRAMERATE_DELAY_MS: u64 = ((1.0 / 144.0) * 1000.0) as u64 ;

	const WIDTH: u32 = 800;
	const HEIGHT: u32 = 600;

	// projection matrix setup
	const ASPECT_RATIO: f32 = HEIGHT as f32 / WIDTH as f32;
	const ZFAR: f32 = 1000.0;
	const ZNEAR: f32 = 0.1;
	const Z_SCALING_FACTOR: f32 = ZFAR / (ZFAR - ZNEAR);
	let fov: f32 = 90.0;
	let inv_tan_half_fov: f32 = 1.0 / (deg_to_rad(fov / 2.0)).tan();

	let mut projection_matrix: Mat4x4 = Mat4x4::new();
	projection_matrix.set(0, 0, ASPECT_RATIO * inv_tan_half_fov);
	projection_matrix.set(1, 1, inv_tan_half_fov);
	projection_matrix.set(2, 2, Z_SCALING_FACTOR);
	projection_matrix.set(2, 3, 1.0);
	projection_matrix.set(3, 2, -ZNEAR * Z_SCALING_FACTOR);

	let (mut canvas, mut events) = initialize_canvas((WIDTH, HEIGHT), "window")?;
	let mut elapsed_time: f32 = 0.0;
	'running: loop {
		let frame_start_time = std::time::Instant::now();
		canvas.clear();
		canvas.set_draw_color(Color::RGB(255, 255, 255));
		for event in events.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { break 'running; }
				_ => {}
			}
		}

		/* rendering code */

		// set up rotation matrices
		let mut rot_z: Mat4x4 = Mat4x4::new();
		rot_z.set(0, 0, elapsed_time.cos());
		rot_z.set(0, 1, elapsed_time.sin());
		rot_z.set(1, 0, -elapsed_time.sin());
		rot_z.set(1, 1, elapsed_time.cos());
		rot_z.set(2, 2, 1.0);
		rot_z.set(3, 3, 1.0);

		let mut rot_x: Mat4x4 = Mat4x4::new();
		rot_x.set(0, 0, 1.0);
		rot_x.set(1, 1, (elapsed_time * 0.5).cos());
		rot_x.set(1, 2, (elapsed_time * 0.5).sin());
		rot_x.set(2, 1, -(elapsed_time * 0.5).sin());
		rot_x.set(2, 2, (elapsed_time * 0.5).cos());
		rot_x.set(3, 3, 1.0);

		for tri in &cube_mesh.tris {
			let mut ntri: Triangle = tri.clone();

			ntri.apply_matrix_transform(&rot_z);
			ntri.apply_matrix_transform(&rot_x);

			ntri.translate(0.0, 0.0, 3.0);

			ntri.apply_matrix_transform(&projection_matrix);
			ntri.points.iter_mut().for_each(|x: &mut Vec3D| {
				x.add(1.0);
				x.x *= 0.5 * WIDTH as f32;
				x.y *= 0.5 * HEIGHT as f32;
			});
			for i in 0..3 {
				canvas.draw_fline(
					FPoint::new(ntri.points[i].x, ntri.points[i].y),
					FPoint::new(ntri.points[(i + 1) % 3].x, ntri.points[(i + 1) % 3].y)
				).or_else(|err| {
					return Err(RendererError::DrawError(err));
				})?;
			}
		}

		/* end rendering code */

		canvas.present();
		canvas.set_draw_color(Color::RGB(0, 0, 0));

		let frame_time: u64 = frame_start_time.elapsed().as_millis() as u64;
		if frame_time < FRAMERATE_DELAY_MS {
			std::thread::sleep(
				std::time::Duration::from_millis(FRAMERATE_DELAY_MS - frame_time)
			);
			elapsed_time += FRAMERATE_DELAY_MS as f32 / 1000.0;
		} else {
			elapsed_time += frame_time as f32 / 1000.0;
		}
	}
	return Ok(());
}