use std::sync::Mutex;
use std::sync::OnceLock;

use protologic_core::RadarTargetType;
use protologic_core::lowlevel::constants::const_get_turretshellspeed;
use protologic_core::highlevel::queries::*;
use protologic_core::highlevel::actions::*;
use protologic_core::highlevel::wasi::*;

pub struct State
{
    turned: bool,
    turned2: bool,
    burned: bool,
    scan_elevation: f32,
    scan_angle: f32,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

impl State
{
    /// The "state" of the program is stored in a mutable static object. Get access to the
    /// state through this method. This allows state to persist between calls of main/trap_handler.
    pub fn get_state_singleton() -> &'static Mutex<State>
    {
        return STATE.get_or_init(|| Mutex::new(State::new()));
    }

    fn new() -> State
    {
        return State {
            turned: false,
            turned2: false,
            burned: false,
            scan_elevation: 0f32,
            scan_angle: 10f32,
        };
    }

    pub fn tick(&mut self)
    {
        // Turn upwards
        if !self.turned
        {
            Self::turn_and_stop(1.0, 0.0, 0.0, 900);
            self.turned = true;
        }

        // Burn
        if !self.burned
        {
            engine_set_throttle(1.0);
            Self::wait_ticks(550);
            engine_set_throttle(0.0);
        }

        // Turn along long axis
        if !self.turned2
        {
            Self::turn_and_stop(0.0, 0.0, -1.0, 1900);
            self.turned2 = true;
        }

        // Set bearing forward
        radar_set_bearing(270f32);
        gun_set_bearing(0, 270f32);
        gun_set_bearing(1, 270f32);
        gun_set_bearing(2, 270f32);
        gun_set_bearing(3, 270f32);

        loop
        {
            // sweep radar
            self.scan_elevation = (self.scan_elevation + 0.5f32) % 90f32;
            radar_set_angle(self.scan_angle);
            radar_set_elevation(self.scan_elevation);
            radar_trigger();

            // keep guns in sync with radar direction
            let gun_dir = self.scan_elevation + self.scan_angle / 2.0;
            gun_set_elevation(0, gun_dir);
            gun_set_elevation(1, gun_dir);
            gun_set_elevation(2, gun_dir);
            gun_set_elevation(3, gun_dir);
            
            // Wait until the next frame (to get radar results)
            Self::wait_ticks(1);

            // Check if we detected anything
            let pos = ship_get_position();
            let mut detected = false;
            let mut dist = 0f32;
            for i in 0..radar_get_contact_count()
            {
                let tgt = radar_get_contact(i);
                if tgt.get_target_type() == RadarTargetType::SpaceBattleShip
                {
                    detected = true;
                    dist = ((pos.0 - tgt.x).powf(2.0) + (pos.1 - tgt.y).powf(2.0) + (pos.2 - tgt.z).powf(2.0)).sqrt();
                    println!("Target detected: {:?} @ d:{} b:{}", tgt.get_target_type(), dist, self.scan_elevation);
                    self.scan_elevation -= self.scan_angle * 4.0;
                    break;
                }
            }

            // Fire the guns if we found something
            if detected
            {
                Self::wait_ticks(10);
                let fuse = dist / unsafe { const_get_turretshellspeed() } - 0.1;
                gun_set_fuse(0, fuse * 1.05);
                gun_set_fuse(1, fuse * 1.0);
                gun_set_fuse(2, fuse * 0.95);
                gun_set_fuse(3, fuse * 0.9);

                // wait a short time between each fire so the shells don';'t set each other off
                gun_trigger(0);
                Self::wait_ticks(125);
                gun_trigger(1);
                Self::wait_ticks(125);
                gun_trigger(2);
                Self::wait_ticks(125);
                gun_trigger(3);
                println!("Fire!");
            }
        }
    }

    fn wait_ticks(ticks: u32)
    {
        for _ in 0..ticks {
            sched_yield();
        }
    }

    fn turn_and_stop(x: f32, y: f32, z: f32, ticks: u32)
    {
        // Start turning
        wheel_set_torque(x, y, z);

        // Wait a bit
        Self::wait_ticks(ticks);

        // Counter toruqe to stop turning
        wheel_set_torque(-x, -y, -z);

        // Wait a bit
        Self::wait_ticks(ticks);

        // Stop applying torque
        wheel_set_torque(0f32, 0f32, 0f32);
    }

    pub fn trap_handler(&mut self, _trap_code: protologic_core::trap::TrapCode) -> State
    {
        // Handle errors by creating a clean new state and hoping for the best!
        let mut replacement = State::new();
        replacement.burned = true;
        replacement.turned = true;
        replacement.turned2 = true;
        return replacement;
    }
}