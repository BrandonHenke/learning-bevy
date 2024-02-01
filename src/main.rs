use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng,Rng};
use rand::distributions::{Distribution, Uniform};

// const EPSILON_0: f32 = 8.85E-12;
const TAU: f32 = 6.28;
const EPSILON_0: f32 = 2.*TAU*1E-2;
const SCALE: f32 = 1.;

#[derive(Component, Default)]
struct Momentum(Vec3);

#[derive(Component, Default)]
struct Mass(f32);

#[derive(Component, Default)]
struct Charge(f32);

#[derive(Bundle)]
struct Boid {
	mass: Mass,
	charge: Charge,
	material_mesh: MaterialMesh2dBundle<ColorMaterial>,
	momentum: Momentum
}

fn main() {
	// println!("Random number: {}", between.sample(&mut rng))

	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, add_boids)
		.add_systems(FixedUpdate,(
				sprite_acceleration,
				sprite_velocity,
			))
		.run();
}

fn add_boids(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let mut rng = thread_rng();
	let between: Uniform<f32> = Uniform::new_inclusive(-1., 1.);
	println!("Once.");
	commands.spawn(Camera2dBundle::default());
	let mass_pos = 5.;
	let mass_neg = 2.;
	for _ in 0..20 {
		commands.spawn(
			Boid {
				mass: Mass(mass_pos),
				charge: Charge(1.),
				material_mesh: MaterialMesh2dBundle {
					mesh: meshes.add(shape::Circle::new(mass_pos*SCALE).into()).into(),
					material: materials.add(ColorMaterial::from(Color::RED)),
					transform: Transform::from_xyz(
						between.sample(&mut rng)*100.,
						between.sample(&mut rng)*100.,
						0.),
					..default()
				},
				momentum: Momentum(Vec3::new(
					0.,
					0.,
					0.
				))
			}
		);
	}

	for _ in 0..40 {
		commands.spawn(
			Boid {
				mass: Mass(mass_neg),
				charge: Charge(-1.),
				material_mesh: MaterialMesh2dBundle {
					mesh: meshes.add(shape::Circle::new(mass_neg*SCALE).into()).into(),
					material: materials.add(ColorMaterial::from(Color::BLUE)),
					transform: Transform::from_xyz(
						between.sample(&mut rng)*100.,
						between.sample(&mut rng)*100.,
						0.),
					..default()
				},
				momentum: Momentum(Vec3::new(
					0.,
					0.,
					0.
				))
			}
		);
	}
}

fn sprite_acceleration(mut query: Query<(&Mass, &Charge, &GlobalTransform, &mut Momentum)>) {
	let mut iter = query.iter_combinations_mut();

	while let Some([
		(Mass(m1), Charge(c1), transform1, mut mom1),
		(Mass(m2), Charge(c2), transform2, mut mom2)
		]) = iter.fetch_next()
	{
		let dist = transform1.translation() - transform2.translation();
		let dist_mag = dist.length();
		let f = 1./(2. * TAU * EPSILON_0) * (c1*c2/dist_mag + 2.*(m1+m2)*SCALE/(dist_mag*dist_mag)) * dist.normalize();
		let dp1 = f;
		let dp2 = -f;

		mom1.0 += dp1;
		mom2.0 += dp2;
		// println!("{}",f)
	}
	// println!("Changing momenta.")
}

fn sprite_velocity(time: Res<Time>, mut query: Query<(&Mass, &mut Transform, &Momentum)>) {
	let dt = time.delta_seconds();
	for (mass, mut trans, mom) in &mut query {
		// let new_pos = trans.translation + ;
		trans.translation += 1./(mass.0) * mom.0 * dt;
	}
	// println!("Changing positions.")
}