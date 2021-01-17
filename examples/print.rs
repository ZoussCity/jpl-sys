use jpl_sys::*;

fn main() {
    unsafe {
        let j = jpl_init_ephemeris(
            b"JPLEPH\0".as_ptr() as _,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if !j.is_null() {
            let t = 2451544.5;
            let mut rrd1 = [0.0_f64; 6];
            let mut rrd2 = [0.0_f64; 6];
            let mut rrd3 = [0.0_f64; 6];
            if jpl_pleph(j, t, 10, 3, rrd1.as_mut_ptr(), 0) == 0
                && jpl_pleph(j, t, 15, 0, rrd2.as_mut_ptr(), 0) == 0
                && jpl_pleph(j, t, 14, 10, rrd3.as_mut_ptr(), 0) == 0
            {
                println!(
                    " Position: {:#?}\nLibration: {:#?}\n Nutation: {:#?}",
                    &rrd1[..3],
                    &rrd2[..3],
                    &rrd3[..2],
                );
            }

            jpl_close_ephemeris(j);
        }
    }
}
