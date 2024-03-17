const GRAVITY: f64 = 9.81; // Gravitational constant (m/s^2)

struct RocketEngine {
    current_fuel_mass: f64, // Current mass of fuel in kg
    altitude: f64, // Current altitude in meters
    velocity: f64, // Current velocity in m/s
    acceleration: f64, // Current acceleration in m/s^2
    time_elapsed: f64, // Time elapsed in seconds
}

impl RocketEngine {
    fn new(initial_fuel_mass: f64) -> RocketEngine {
        RocketEngine {
            current_fuel_mass: initial_fuel_mass,
            altitude: 0.0,
            velocity: 0.0,
            acceleration: 0.0,
            time_elapsed: 0.0,
        }
    }

    fn calculate_thrust(&self) -> f64 {
        // Placeholder for thrust calculation
        1000.0 // Example value, replace with actual formula
    }

    fn calculate_drag(&self) -> f64 {
        // Placeholder for drag calculation
        0.0 // Example value, replace with actual formula
    }

    fn update_state(&mut self, time_step: f64) {
        let thrust = self.calculate_thrust();
        let drag = self.calculate_drag();
        let net_force = thrust - drag - self.current_fuel_mass * GRAVITY;

        self.acceleration = net_force / (self.current_fuel_mass * GRAVITY);
        self.velocity += self.acceleration * time_step;
        self.altitude += self.velocity * time_step;
        self.current_fuel_mass -= 0.1 * time_step; // Example fuel consumption
        self.time_elapsed += time_step;
    }

    fn print_state(&self) {
        println!("Time: {:.2} s", self.time_elapsed);
        println!("Altitude: {:.2} m", self.altitude);
        println!("Velocity: {:.2} m/s", self.velocity);
        println!("Acceleration: {:.2} m/s^2", self.acceleration);
        println!("Fuel Mass: {:.2} kg", self.current_fuel_mass);
    }
}

fn main() {
    println!("Rocket Engine Simulation");

    // Initialize the rocket engine
    let initial_fuel_mass = 1000.0; // kg
    let mut rocket = RocketEngine::new(initial_fuel_mass);

    // Simulation parameters
    let time_step = 1.0; // seconds
    let simulation_duration = 10.0; // seconds

    // Run the simulation
    while rocket.time_elapsed < simulation_duration && rocket.current_fuel_mass > 0.0 {
        rocket.update_state(time_step);
        rocket.print_state();
        println!("--------------------------------");
    }

    println!("Simulation ended.");
}
