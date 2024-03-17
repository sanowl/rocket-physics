use std::f64::consts::PI;

const AIR_DENSITY_SEA_LEVEL: f64 = 1.225; // Air density at sea level (kg/m^3)
const GRAVITY: f64 = 9.81; // Gravitational constant (m/s^2)

struct RocketEngine {
    initial_fuel_mass: f64, // Initial mass of fuel in kg
}

impl RocketEngine {
    fn new(initial_fuel_mass: f64) -> RocketEngine {
        RocketEngine {
            initial_fuel_mass,
        }
    }

    fn simulate(&mut self, time_step: f64) {
        let mut altitude = 0.0;
        let mut velocity = 0.0;
        let mut fuel_mass = self.initial_fuel_mass;

        for time in (0..=10).map(|t| t as f64 * time_step) {
            let air_density = air_density_at_altitude(altitude);
            let drag_force = calculate_drag_force(velocity, air_density);
            let lift_force = calculate_lift_force(velocity, air_density);

            let acceleration = (thrust() - drag_force - GRAVITY - lift_force) / rocket_mass();
            velocity += acceleration * time_step;
            altitude += velocity * time_step;

            if fuel_mass > 0.0 {
                fuel_mass -= fuel_consumption_rate() * time_step;
            }

            println!("Time: {:.2} s", time);
            println!("Altitude: {:.2} m", altitude);
            println!("Velocity: {:.2} m/s", velocity);
            println!("Fuel Mass: {:.2} kg", fuel_mass);
            println!("--------------------------------");
        }

        println!("Simulation ended.");
    }
}

fn air_density_at_altitude(altitude: f64) -> f64 {
    AIR_DENSITY_SEA_LEVEL * ((1.0 - 2.25577e-5 * altitude).powf(5.25588))
}

fn calculate_drag_force(velocity: f64, air_density: f64) -> f64 {
    // Placeholder calculation, replace with actual drag force model
    0.5 * air_density * velocity.powi(2) * 0.2 * 1.0 // Placeholder coefficient
}

fn calculate_lift_force(_velocity: f64, _air_density: f64) -> f64 {
    // Placeholder calculation, replace with actual lift force model
    0.0 // No lift force for now
}

fn thrust() -> f64 {
    // Placeholder value for thrust, replace with actual thrust calculation
    20000.0 // N
}

fn fuel_consumption_rate() -> f64 {
    // Placeholder value for fuel consumption rate, replace with actual value
    10.0 // kg/s
}

fn rocket_mass() -> f64 {
    // Placeholder value for rocket mass, replace with actual value
    1000.0 // kg
}

fn main() {
    let mut rocket = RocketEngine::new(1000.0); // Initial fuel mass of 1000 kg
    rocket.simulate(1.0); // Simulate with a time step of 1 second
}
