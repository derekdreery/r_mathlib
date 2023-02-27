#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dlnorm(
    mut x: libc::c_double,
    mut meanlog: libc::c_double,
    mut sdlog: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || meanlog.is_nan() as i32 != 0 as libc::c_int
        || sdlog.is_nan() as i32 != 0 as libc::c_int
    {
        return x + meanlog + sdlog;
    }
    if sdlog < 0 as libc::c_int as libc::c_double {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg = b"argument out of domain in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                2 => {
                    msg = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if R_finite(x) == 0 && log(x) == meanlog {
        return 0.0f64 / 0.0f64;
    }
    if sdlog == 0 as libc::c_int as libc::c_double {
        return if log(x) == meanlog {
            1.0f64 / 0.0f64
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if x <= 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    y = (log(x) - meanlog) / sdlog;
    return if log_p != 0 {
        -(0.918938533204672741780329736406f64 + 0.5f64 * y * y + log(x * sdlog))
    } else {
        0.398942280401432677939946059934f64 * exp(-0.5f64 * y * y) / (x * sdlog)
    };
}
