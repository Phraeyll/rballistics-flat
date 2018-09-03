use na::{Rotation3, Vector3};

pub use dragtables::DragTableKind;

use self::constructors::*;
use conversions::*;
use dragtables::*;

use std::f64::consts::{E, PI};

const GRAVITY: f64 = -9.80665; // Local gravity in m/s
const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
const MOLAR_DRY: f64 = 0.0289644; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: f64 = 0.018016; // Molar mass of water vapor (kg/mol)

#[derive(Debug)]
pub struct PointMassModel {
    // Projectile properties
    pub weight: WeightMass, // Weight (grains)
    pub caliber: Length,    // Caliber (inches)
    pub bc: f64,            // Ballistic Coefficient

    // Environmental Conditions
    pub wind_velocity: Vector3<f64>, // Wind Velocity (m/s)
    pub temperature: Temperature,    // Temperature (F)
    pub pressure: Pressure,          // Pressure (InHg)
    pub humidity: f64,               // Humidity (0-1)
    pub g: Vector3<f64>,             // Gravity (m/s^2)

    // Variables for simulation
    initial_angle: f64,             // Initial launch angle (radians), determined by zero function
    pub time_step: f64,             // Timestep for simulation (s)
    pub initial_velocity: Velocity, // Initial velocity (ft/s)
    pub scope_height: Length,       // Scope Height (inches)
    pub los_angle: f64,             // Line of Sight angle (degrees)
    pub drag_table: DragTable,      // Drag Function DragTable

    /*
    Other factors, not calculated yet
    pub ptmp: f64,                   // Powder Temperature (K?)
    pub lat:  f64,                   // Lattitude (Coriolis Effect)
    pub long: f64,                   // Longitude (Coriolis Effect)
    pub dir:  Direction,             // Direction Facing (Coriolis Effect)
    pub spin: f64,                   // Spin drift (Gyroscopic Drift)
    */
}
struct Envelope {
    // Envelope of motion
    time: f64,                  // Position in time (s)
    position: Vector3<f64>,     // Position (m)
    velocity: Vector3<f64>,     // Velocity (m/s)
    acceleration: Vector3<f64>, // Acceleration (m/s^2)
}
pub struct IterPointMassModel<'a> {
    model: &'a PointMassModel,
    envelope: Envelope,
}
pub struct Ballistic {
    angle: f64,
    height: f64,
    time: f64,                  // Position in time (s)
    position: Vector3<f64>,     // Position (m)
    velocity: Vector3<f64>,     // Velocity (m/s)
    acceleration: Vector3<f64>, // Acceleration (m/s^2)
}

trait DragSimulation {
    fn area(&self) -> f64; // Area (meters)
    fn mass(&self) -> f64; // Mass (kgs)
    fn radius(&self) -> f64; // Radius (meters)
    fn sd(&self) -> f64; // Sectional Density
    fn i(&self) -> f64; // Form Factor
    fn rho(&self) -> f64; // Density of air (kg/m^3)
    fn mach(&self) -> f64; // Velocity rel ative to speed of sound
    fn drag_force(&self) -> Vector3<f64>;
}
pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn acceleration(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
    fn relative_position(&self) -> (f64, f64, f64);
}

impl PointMassModel {
    pub fn new(
        weight: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        scope_height: f64,
        los_angle: f64,
        drag_table: DragTableKind,
        time_step: f64,
        wind_velocity: f64,
        wind_angle: f64,
        temperature: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let weight_grains = WeightMass::Grains(weight);
        let diameter_inches = Length::Inches(caliber);
        let initial_velocity_fps = Velocity::Fps(initial_velocity);
        let temperature_f = Temperature::F(temperature);
        let pressure_inhg = Pressure::Inhg(pressure);
        let wind_velocity_mph = Velocity::Mph(wind_velocity);
        let time_step_seconds = Time::Seconds(time_step);
        let scope_height_inches = Length::Inches(scope_height);

        Self {
            weight: weight_grains,
            caliber: diameter_inches,
            bc,

            wind_velocity: construct_velocity(wind_velocity_mph, Wind(wind_angle.to_radians())),
            temperature: temperature_f,
            pressure: pressure_inhg,
            humidity,
            g: Vector3::new(0.0, GRAVITY, 0.0),

            time_step: time_step_seconds.to_seconds().into(),
            initial_angle: 0.0,
            initial_velocity: initial_velocity_fps,
            scope_height: scope_height_inches,
            los_angle,
            drag_table: DragTable::new(drag_table),
        }
    }
    pub fn iter<'a>(&'a self) -> IterPointMassModel {
        let initial_angle_radians = self.initial_angle.to_radians();
        let initial_velocity_fps = self.initial_velocity;
        IterPointMassModel {
            model: self,
            envelope: Envelope {
                position: Vector3::new(0.0, 0.0, 0.0),
                velocity: construct_velocity(
                    initial_velocity_fps,
                    Projectile(initial_angle_radians),
                ),
                acceleration: Vector3::new(0.0, 0.0, 0.0),
                time: 0.0,
            },
        }
    }
    // I have no idea how this should work - approx doesn't seem to work
    pub fn zero(&mut self, zero_distance: f64, angle: f64, prev: f64) {
        let zero_distance_yards = Length::Yards(zero_distance);
        let zero_distance_meters = f64::from(zero_distance_yards.to_meters());
        let mut drop = 0.0;
        for b in self.iter() {
            if b.distance() > zero_distance_meters {
                drop = b.position.y;
                break;
            }
        }
        if relative_eq!(drop, 0.0) {
            return;
        } else {
            if drop.is_sign_negative() {
                if prev.is_sign_positive() {
                    self.initial_angle += angle / 2.0;
                    self.zero(zero_distance, angle / 2.0, drop);
                } else {
                    self.initial_angle += angle;
                    self.zero(zero_distance, angle, drop);
                }
            } else {
                if prev.is_sign_negative() {
                    self.initial_angle -= angle / 2.0;
                    self.zero(zero_distance, angle / 2.0, drop);
                } else {
                    self.initial_angle -= angle;
                    self.zero(zero_distance, angle, drop);
                }
            }
        }
    }
}

impl<'a> Iterator for IterPointMassModel<'a> {
    type Item = Ballistic;
    fn next(&mut self) -> Option<Self::Item> {
        self.envelope.acceleration = self.drag_force() / self.mass() + self.model.g;
        self.envelope.position = self.envelope.position
            + self.envelope.velocity * self.model.time_step
            + self.envelope.acceleration * (self.model.time_step.powf(2.0) / 2.0);
        self.envelope.velocity =
            self.envelope.velocity + self.envelope.acceleration * self.model.time_step;
        self.envelope.time += self.model.time_step;

        Some(Ballistic {
            angle: self.model.los_angle.to_radians(),
            height: self.model.scope_height.to_meters().into(),
            time: self.envelope.time,
            position: self.envelope.position,
            velocity: self.envelope.velocity,
            acceleration: self.envelope.acceleration,
        })
    }
}

impl<'a> DragSimulation for IterPointMassModel<'a> {
    fn area(&self) -> f64 {
        PI * self.radius().powf(2.0)
    }
    fn mass(&self) -> f64 {
        self.model.weight.to_kgs().into()
    }
    fn radius(&self) -> f64 {
        f64::from(self.model.caliber.to_meters()) / 2.0
    }
    fn sd(&self) -> f64 {
        f64::from(self.model.weight.to_lbs()) / f64::from(self.model.caliber.to_inches()).powf(2.0)
    }
    fn i(&self) -> f64 {
        self.sd() / self.model.bc
    }
    fn rho(&self) -> f64 {
        let celsius = f64::from(self.model.temperature.to_celsius());
        let kelvin = f64::from(self.model.temperature.to_kelvin());
        let pa = f64::from(self.model.pressure.to_pascals());
        let pv = self.model.humidity
            * 611.21
            * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
        let pd = pa - pv;
        ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
    }
    fn mach(&self) -> f64 {
        let pa = f64::from(self.model.pressure.to_pascals());
        let c = (1.4 * (pa / self.rho())).sqrt();
        self.envelope.velocity.norm() / c
    }
    fn drag_force(&self) -> Vector3<f64> {
        let cd = self.model.drag_table.lerp(self.mach()) * self.i();
        let vv = self.envelope.velocity - self.model.wind_velocity;
        -(self.rho() * self.area() * vv * vv.norm() * cd) / 2.0
    }
}

impl Output for Ballistic {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.velocity.norm()).to_fps())
    }
    fn acceleration(&self) -> f64 {
        f64::from(Acceleration::Mps2(self.acceleration.norm()).to_fps2())
    }
    fn distance(&self) -> f64 {
        f64::from(Length::Meters(self.position.x).to_yards())
    }
    fn drop(&self) -> f64 {
        f64::from(Length::Meters(self.position.y).to_inches())
    }
    fn windage(&self) -> f64 {
        f64::from(Length::Meters(self.position.z).to_inches())
    }
    fn relative_position(&self) -> (f64, f64, f64) {
        let angle = -self.angle;
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let height = Vector3::new(0.0, f64::from(self.height), 0.0);
        let position = rotation * self.position - height;
        let drop = f64::from(Length::Meters(position.y));
        (
            f64::from(Length::Meters(position.x).to_yards()),
            f64::from(Length::Meters(drop).to_inches()),
            f64::from(Length::Meters(position.z).to_inches()),
        )
    }
}

mod constructors {
    pub use self::AngleKind::*;

    use conversions::*;
    use na::{Rotation3, Vector3};

    pub enum AngleKind {
        Projectile(f64),
        Wind(f64),
    }

    pub fn construct_velocity(vel: Velocity, vk: AngleKind) -> Vector3<f64> {
        let (axis, angle) = match vk {
            Projectile(deg) => {
                // Rotation along z axis is pitch, projectile up/down relative to x/y plane
                (Vector3::z_axis(), deg)
            }
            Wind(deg) => {
                // Rotation along y axis is yaw, wind left/right relative to x/z plane
                (Vector3::y_axis(), deg)
            }
        };
        let velocity_mps = vel.to_mps().into();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let velocity = Vector3::new(velocity_mps, 0.0, 0.0);
        rotation * velocity
    }
}
