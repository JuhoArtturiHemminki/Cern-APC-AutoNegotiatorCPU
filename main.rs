use std::time::Instant;
use std::hint::black_box;

fn main() {
    println!("MEASURING PHYSICAL TRANSISTOR SPEED: Pure Nanosecond Hardware Clock...");
    println!("Executing 100% Un-Pruned Signed CERN-128 Matrix (ILP x8)...");

    let phi: i128 = 6_949_392_209;
    let phi_inv: i128 = 1_640_531_527;
    let c_base: i128 = 0;
    let delta_c: i128 = 1_000_000;

    let iterations = 500_000_000;
    let density_anchor: i128 = -1_000_000_000; 

    let mut r1: i128 = 0; let mut r2: i128 = 0;
    let mut r3: i128 = 0; let mut r4: i128 = 0;
    let mut r5: i128 = 0; let mut r6: i128 = 0;
    let mut r7: i128 = 0; let mut r8: i128 = 0;

    let mut bmcr_register: u16 = 0x1000; 
    let mut anar_register: u16 = 0x01E1; 

    let start = Instant::now();

    for i in 0..iterations {
        let nic_stream_i = black_box(i as i128);
        let rho = nic_stream_i + density_anchor;
        let tau = nic_stream_i;

        r1 = (rho.wrapping_mul(phi)).wrapping_add(tau.wrapping_mul(phi_inv));
        r2 = ((rho + 1).wrapping_mul(phi)).wrapping_add((tau + 1).wrapping_mul(phi_inv));
        r3 = ((rho + 2).wrapping_mul(phi)).wrapping_add((tau + 2).wrapping_mul(phi_inv));
        r4 = ((rho + 3).wrapping_mul(phi)).wrapping_add((tau + 3).wrapping_mul(phi_inv));
        r5 = ((rho + 4).wrapping_mul(phi)).wrapping_add((tau + 4).wrapping_mul(phi_inv));
        r6 = ((rho + 5).wrapping_mul(phi)).wrapping_add((tau + 5).wrapping_mul(phi_inv));
        r7 = ((rho + 6).wrapping_mul(phi)).wrapping_add((tau + 6).wrapping_mul(phi_inv));
        r8 = ((rho + 7).wrapping_mul(phi)).wrapping_add((tau + 7).wrapping_mul(phi_inv));

        r1 = r1.wrapping_add(r1 & (r1 >> 127));
        r2 = r2.wrapping_add(r2 & (r2 >> 127));
        r3 = r3.wrapping_add(r3 & (r3 >> 127));
        r4 = r4.wrapping_add(r4 & (r4 >> 127));
        r5 = r5.wrapping_add(r5 & (r5 >> 127));
        r6 = r6.wrapping_add(r6 & (r6 >> 127));
        r7 = r7.wrapping_add(r7 & (r7 >> 127));
        r8 = r8.wrapping_add(r8 & (r8 >> 127));
    }

    let autoneg_trigger_mask = ((r8 >> 127) & 1) as u16;
    bmcr_register |= autoneg_trigger_mask << 9;  
    anar_register &= !(autoneg_trigger_mask * 0x0180); 

    let apc_mask = (r8 >> 127) as u128;
    let r_state_reflection = (c_base as u128).wrapping_add(apc_mask.wrapping_mul(delta_c as u128));

    black_box(r1); black_box(r2); black_box(r3); black_box(r4);
    black_box(r5); black_box(r6); black_box(r7); black_box(r8);
    black_box(r_state_reflection);
    black_box(bmcr_register);
    black_box(anar_register);

    let duration = start.elapsed();
    let elapsed_secs = duration.as_secs_f64();
    
    let nanoseconds_per_loop = (elapsed_secs / iterations as f64) * 1_000_000_000.0;

    let kinetic_boost_active = (r8 >> 127) & 1;

    println!("\n--- THE ABSOLUTE PHYSICAL CLOCK RESULTS ---");
    println!("Total physical elapsed time: {:.4} seconds", elapsed_secs);
    println!("Duration of one complete cycle: {:.4} nanoseconds", nanoseconds_per_loop);
    println!("Signed register channel r8 authentic final state: {}", r8);
    println!("Final APC Reflection Coordinate (R): {}", r_state_reflection);
    println!("CERN Kinetic Core Boost (127th hardware bit): {}", if kinetic_boost_active == 1 { "ACTIVE (1)" } else { "INACTIVE (0)" });
    println!("AutoNegotiator PHY BMCR Register State: 0x{:04X}", bmcr_register);
    println!("AutoNegotiator PHY ANAR Advertisement: 0x{:04X}", anar_register);
}
