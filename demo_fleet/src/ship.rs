use protologic_core::RadarTargetType;
use protologic_core::actions;
use protologic_core::constants;
use protologic_core::highlevel::queries::*;
use protologic_core::highlevel::actions::*;
use protologic_core::lowlevel::queries::RadarGetContactInfo;
use protologic_core::radio::*;
use protologic_core::wait::*;

use crate::turn_and_stop;

pub fn run()
{
    // Setup some state
    let scan_angle: f32 = 45.0;
    let mut scan_elevation: f32 = 0.0;
    let mut contacts: Vec<RadarGetContactInfo> = Vec::new();
    let mut handles: Vec<DebugShapeHandle> = Vec::new();

    // Reconfigure launch cells (high thrust nuclear tipped missiles)
    for i in 0 .. constants::ship_missile_launcher_count() {
        actions::missilelauncher_configure(i, protologic_core::MissileEngineType::HighThrust, protologic_core::MissileWarheadType::Nuclear);
    }

    // Turn upwards
    turn_and_stop(1.0, 0.0, 0.0, 900);

    // Burn
    engine_set_throttle(1.0);
    wait_ticks(650);
    engine_set_throttle(0.0);

    // Turn along long axis
    turn_and_stop(0.0, 0.0, -1.0, 1850);

    // Launch one missile from each launch cell
    for i in 0 .. constants::ship_missile_launcher_count() {
        actions::missilelauncher_trigger(i);
        wait_ticks(50);
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
        scan_elevation = (scan_elevation + 0.5f32) % 90f32;
        radar_set_angle(scan_angle);
        radar_set_elevation(scan_elevation);
        radar_trigger();

        // keep guns in sync with radar direction
        let gun_dir = scan_elevation + scan_angle / 2.0;
        gun_set_elevation(0, gun_dir);
        gun_set_elevation(1, gun_dir);
        gun_set_elevation(2, gun_dir);
        gun_set_elevation(3, gun_dir);
        
        // Wait until the next frame (to get radar results)
        wait_ticks(1);

        // Check if we detected anything
        let pos = vehicle_get_position();
        let mut detected = false;
        let mut dist = 0f32;
        radar_get_contacts(&mut contacts);
        for tgt in contacts.iter()
        {
            if tgt.get_target_type() == RadarTargetType::SpaceBattleShip
            {
                detected = true;
                dist = ((pos.0 - tgt.x).powf(2.0) + (pos.1 - tgt.y).powf(2.0) + (pos.2 - tgt.z).powf(2.0)).sqrt();
                println!("Target detected: {:?} @ d:{} b:{}", tgt.get_target_type(), dist, scan_elevation);
                scan_elevation -= scan_angle * 4.0;

                // Send a message over the radio with the location
                radio_transmit(crate::radio::pack_message(pos), 999999f32);

                // Draw a line to the target
                handles.clear();
                handles.push(debug_line_create(tgt.x, tgt.y, tgt.z, pos.0, pos.1, pos.2, 0.8, 0.8, 0.8));
                handles.push(debug_sphere_create(tgt.x, tgt.y, tgt.z, 200.0, 0.7, 0.7, 0.7));

                break;
            }
        }

        // Fire the guns if we found something
        if detected
        {
            wait_ticks(10);
            let fuse = dist / constants::turret_shell_speed() - 0.1;
            gun_set_fuse(0, fuse * 1.05);
            gun_set_fuse(1, fuse * 1.0);
            gun_set_fuse(2, fuse * 0.95);
            gun_set_fuse(3, fuse * 0.9);

            // wait a short time between each fire so the shells don';'t set each other off
            gun_trigger(0);
            wait_ticks(255);
            gun_trigger(1);
            wait_ticks(255);
            gun_trigger(2);
            wait_ticks(255);
            gun_trigger(3);
            wait_ticks(255);
        }
    }
}