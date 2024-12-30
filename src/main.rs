mod error;

use std::{env::args, vec};

use sdl2::{
	event::Event, keyboard::Keycode,
	pixels::Color, rect::{FPoint, FRect, Point},
	render::Canvas, video::Window, EventPump,
};

use sdl2::gfx::primitives::DrawRenderer;

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
	pub fn add_num_in_place(&mut self, number: f32) {
		self.x += number;
		self.y += number;
		self.z += number;
	}

	pub fn sub_num_in_place(&mut self, number: f32) {
		self.add_num_in_place(-number);
	}

	pub fn add_vec(&self, vector: &Vec3D) -> Vec3D {
		let mut res: Vec3D = self.clone();
		res.add_vec_in_place(vector);
		return res;
	}

	pub fn add_vec_in_place(&mut self, vector: &Vec3D) {
		self.x += vector.x;
		self.y += vector.y;
		self.z += vector.z;
	}

	pub fn sub_vec(&self, vector: &Vec3D) -> Vec3D {
		let mut res: Vec3D = self.clone();
		res.sub_vec_in_place(vector);
		return res;
	}

	pub fn sub_vec_in_place(&mut self, vector: &Vec3D) {
		self.x -= vector.x;
		self.y -= vector.y;
		self.z -= vector.z;
	}

	pub fn scale_in_place(&mut self, scale_factor: f32) {
		self.x *= scale_factor;
		self.y *= scale_factor;
		self.z *= scale_factor;
	}

	pub fn dot_product(&self, vector: &Vec3D) -> f32 {
		return self.x * vector.x + self.y * vector.y + self.z * vector.z;
	}

	pub fn cross_product(&self, vector: &Vec3D) -> Vec3D {
		return Vec3D {
			x: self.y * vector.z - self.z * vector.y,
			y: self.z * vector.x - self.x * vector.z,
			z: self.x * vector.y - self.y * vector.x
		};
	}

	pub fn len(&self) -> f32 {
		return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
	}
}

#[derive(Debug, Clone)]
struct Triangle { points: [Vec3D; 3], color: Color, }

impl Triangle {
	pub fn translate(&mut self, x: f32, y: f32, z: f32) {
		for p in &mut self.points {
			p.add_vec_in_place(&Vec3D { x, y, z, });
		}
	}

	pub fn apply_matrix_transform(&mut self, matrix: &Mat4x4) {
		for p in &mut self.points {
			*p = matrix.multiply_vector(p);
		}
	}

	pub fn normal(&self) -> Vec3D {
		let line1: Vec3D = self.points[1].sub_vec(&self.points[0]);
		let line2: Vec3D = self.points[2].sub_vec(&self.points[0]);

		let mut calc_normal: Vec3D = line1.cross_product(&line2);
		calc_normal.scale_in_place(1.0 / calc_normal.len());
		return calc_normal;
	}
}

#[derive(Debug, Clone)]
struct Mesh { tris: Vec<Triangle>, }

impl Mesh {
	pub fn from_obj_file(filename: &str) -> anyhow::Result<Mesh> {
		let mut res: Mesh = Mesh { tris: vec![], };
		let mut vertices: Vec<Vec3D> = vec![];
		let mut normals: Vec<Vec3D> = vec![];

		if let Ok(f) = std::fs::read_to_string(filename) {
			for line in f.split("\n") {
				if line.starts_with("v ") {
					let mut numbers = line.trim().split(" ");
					numbers.next();
					vertices.push(Vec3D {
						x: numbers.next().unwrap().parse::<f32>().unwrap(),
						y: numbers.next().unwrap().parse::<f32>().unwrap(),
						z: numbers.next().unwrap().parse::<f32>().unwrap(),
					});
				} else if line.starts_with("vn") {
					let mut numbers = line.trim().split(" ");
					numbers.next();
					normals.push(Vec3D {
						x: numbers.next().unwrap().parse::<f32>().unwrap(),
						y: numbers.next().unwrap().parse::<f32>().unwrap(),
						z: numbers.next().unwrap().parse::<f32>().unwrap(),
					});
				} else if line.starts_with("f") {
					if !line.contains("/") {
						let mut numbers = line.trim().split(" ");
						numbers.next();
						res.tris.push(Triangle {
							points: [
								vertices[numbers.next().unwrap().parse::<usize>().unwrap() - 1].clone(),
								vertices[numbers.next().unwrap().parse::<usize>().unwrap() - 1].clone(),
								vertices[numbers.next().unwrap().parse::<usize>().unwrap() - 1].clone(),
							],
							color: Color::RGB(0, 0, 0),
						});
					} else {
						let mut numbers = line.trim().split(" ");
						numbers.next();
						let mut i1 = numbers.next()
							.unwrap()
							.split("/")
							.map(|x| x.trim().parse::<usize>().unwrap());
						let mut i2 = numbers.next()
							.unwrap()
							.split("/")
							.map(|x| x.trim().parse::<usize>().unwrap());
						let mut i3 = numbers.next()
							.unwrap()
							.split("/")
							.map(|x| x.trim().parse::<usize>().unwrap());
						res.tris.push(Triangle {
							points: [
								vertices[i1.next().unwrap() - 1].clone(),
								vertices[i2.next().unwrap() - 1].clone(),
								vertices[i3.next().unwrap() - 1].clone(),
							],
							color: Color::RGB(0, 0, 0),
						});
					}
				}
			}
		} else {
			return Err(RendererError::FileReadError(filename.to_string()))?;
		}

		return Ok(res);
	}
}

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
	let args: Vec<String> = args().collect();
	if args.len() != 3 {
		return Err(RendererError::InitError(
			"bad arguments supplied\nneed <filename> and <distance from camera>"
			.to_string()
		))?;
	}
	let loaded_mesh: Mesh = Mesh::from_obj_file(args[1].as_str())?;
	let z_translation: f32 = args[2].parse::<f32>()?;

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

	let mut camera: Vec3D = Vec3D { x: 0.0, y: 0.0, z: 0.0 };

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

		let mut tris_to_draw: Vec<Triangle> = vec![];
		for tri in &loaded_mesh.tris {
			let mut ntri: Triangle = tri.clone();

			ntri.apply_matrix_transform(&rot_z);
			ntri.apply_matrix_transform(&rot_x);

			ntri.translate(0.0, 0.0, z_translation);

			if ntri.normal().dot_product(&ntri.points[0].sub_vec(&camera)) >= 0.0 { continue; }

			let mut light_direction = Vec3D { x: 0.0, y: 0.0, z: -1.0, };
			light_direction.scale_in_place(1.0 / light_direction.len()); // does nothing when it's just a unit vector

			let dot_prod: f32 = ntri.normal().dot_product(&light_direction);

			let color_value: u8 =  (dot_prod * 256.0) as u8;
			ntri.color = Color::RGB(color_value, color_value, color_value);

			ntri.apply_matrix_transform(&projection_matrix);
			ntri.points.iter_mut().for_each(|x: &mut Vec3D| {
				x.x += 1.0;
				x.y += 1.0;
				x.x *= 0.5 * WIDTH as f32;
				x.y *= 0.5 * HEIGHT as f32;
			});

			tris_to_draw.push(ntri);
		}

		tris_to_draw.sort_unstable_by(|a, b| {
			let z1: f32 = (a.points[0].z + a.points[1].z + a.points[2].z) / 3.0;
			let z2: f32 = (b.points[0].z + b.points[1].z + b.points[2].z) / 3.0;
			return z2.partial_cmp(&z1).unwrap();
		});

		for ntri in tris_to_draw {
			canvas.filled_trigon(
				ntri.points[0].x as i16,
				ntri.points[0].y as i16,
				ntri.points[1].x as i16,
				ntri.points[1].y as i16,
				ntri.points[2].x as i16,
				ntri.points[2].y as i16,
				ntri.color,
			).or_else(|err| {
				return Err(RendererError::DrawError(err));
			})?;
		}

		/* end rendering code */

		canvas.present();
		canvas.set_draw_color(Color::RGB(0, 0, 0));

		let frame_time: u64 = frame_start_time.elapsed().as_millis() as u64;
		println!("frame_time: {frame_time}ms");
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