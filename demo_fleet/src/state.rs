use protologic_core::lowlevel::constants::const_get_turretshellspeed;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use protologic_core::highlevel::queries::*;
use protologic_core::highlevel::actions::*;
use protologic_core::highlevel::wasi::*;

pub struct State
{
    rng: ChaCha20Rng,
    ticks: u64,

    turned: bool,
    burned: bool,
    scan_angle: f32
}

impl State
{
    pub fn new(ticks: u64) -> State
    {
        return State {
            rng: ChaCha20Rng::from_entropy(),
            ticks,
            turned: true,
            burned: true,
            scan_angle: 0f32
        };
    }

    pub fn main(&mut self)
    {
        self.gunnery_test_loop();

        // Turn
        if !self.turned
        {
            unsafe
            {
                // Start turning
                protologic_core::lowlevel::actions::wheel_set_torque(0.5f32, 0f32, 0f32);

                // Wait a bit
                for _ in 0..205 {
                    sched_yield();
                }

                // Counter toruqe to stop turning
                protologic_core::lowlevel::actions::wheel_set_torque(-0.5f32, 0f32, 0f32);

                // Wait a bit
                for _ in 0..205 {
                    sched_yield();
                }

                // Stop applying torque
                protologic_core::lowlevel::actions::wheel_set_torque(0f32, 0f32, 0f32);
            }

            self.turned = true;
        }

        // Burn
        if !self.burned
        {
            engine_set_throttle(1.0);

            // Wait a bit
            for _ in 0..100 {
                sched_yield();
            }

            engine_set_throttle(0.0);
        }

        loop
        {
            self.ticks += 1;
            if self.ticks > 5000 && self.rng.gen_bool(0.001)
            {
                self_destruct();
            }

            // sweep radar
            self.scan_angle = (self.scan_angle + 2f32) % 360f32;
            radar_set_angle(15f32);
            radar_set_elevation(0f32);
            radar_set_bearing(self.scan_angle);
            gun_set_bearing(0, self.scan_angle);
            gun_set_elevation(0, 0f32);
            radar_trigger();
            
            // Wait until the next frame (to get radar results)
            sched_yield();

            // Check if we detected anything
            let mut detected = false;
            let mut dist = 0f32;
            for i in 0..radar_get_target_count()
            {
                let (t, d) = radar_get_target(i);
                if t != RadarTargetType::Asteroid {
                    detected = true;
                    dist = d;
                    println!("Target detected: {:?} @ {}", t, dist);
                    break;
                } else {
                    println!("Asteroid detected: {:?} @ {}", t, dist);
                }
            }

            // Fire the guns if we found something
            if detected
            {
                gun_set_fuse(0, dist / unsafe { const_get_turretshellspeed() });

                // Wait a bit for the guns to slew around
                for _ in 0..125 {
                    sched_yield();
                }

                gun_trigger(0);
                println!("Fire!");
            }
        }
    }

    fn gunnery_test_loop(&mut self)
    {
        let mut dir = 0f32;

        loop {
            radar_set_elevation(0f32);
            radar_set_bearing(dir);
            radar_set_angle(20f32);
            radar_trigger();

            gun_set_elevation(0, 0f32);
            gun_set_bearing(0, dir);

            for i in 0..radar_get_target_count()
            {
                let (t, d) = radar_get_target(i);
                println!("Target detected: {:?}, dir: {}", t, dir);
                gun_set_fuse(0, d / unsafe { const_get_turretshellspeed() });
            }

            if radar_get_target_count() > 0
            {
                for _ in 0..125 {
                    sched_yield();
                }

                gun_trigger(0);
                println!("Fire!");
            }

            for _ in 0..500 {
                sched_yield();
            }

            dir += 15f32;

            if dir > 360f32 {
                self_destruct();
            }
        }
    }

    pub fn trap_handler(&mut self, _trap_code: protologic_core::trap::TrapCode) -> State
    {
        // Handle errors by creating a clean new state and hoping for the best!
        return State::new(self.ticks);
    }
}