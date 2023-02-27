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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn qt(_: libc::c_double, _: libc::c_double, _: libc::c_int, _: libc::c_int) -> libc::c_double;
    fn pnt(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn qnt(
    mut p: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut accu: libc::c_double = 1e-13f64;
    static mut Eps: libc::c_double = 1e-11f64;
    let mut ux: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut nx: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return p + df + ncp;
    }
    if df <= 0.0f64 {
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
    if ncp == 0.0f64 && df >= 1.0f64 {
        return qt(p, df, lower_tail, log_p);
    }
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_0: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_0 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_0 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_0 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_0 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_0, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_1: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_1 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_1 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_1 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
    }
    if R_finite(df) == 0 {
        return qnorm5(p, ncp, 1.0f64, lower_tail, log_p);
    }
    p = if log_p != 0 {
        if lower_tail != 0 {
            exp(p)
        } else {
            -expm1(p)
        }
    } else if lower_tail != 0 {
        p
    } else {
        0.5f64 - p + 0.5f64
    };
    if p > 1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64 {
        return 1.0f64 / 0.0f64;
    }
    pp = fmin2(
        1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64,
        p * (1 as libc::c_int as libc::c_double + Eps),
    );
    ux = fmax2(1.0f64, ncp);
    while ux < 1.7976931348623157e+308f64
        && pnt(ux, df, ncp, TRUE as libc::c_int, FALSE as libc::c_int) < pp
    {
        ux *= 2 as libc::c_int as libc::c_double;
    }
    pp = p * (1 as libc::c_int as libc::c_double - Eps);
    lx = fmin2(-1.0f64, -ncp);
    while lx > -1.7976931348623157e+308f64
        && pnt(lx, df, ncp, TRUE as libc::c_int, FALSE as libc::c_int) > pp
    {
        lx *= 2 as libc::c_int as libc::c_double;
    }
    loop {
        nx = 0.5f64 * (lx + ux);
        if pnt(nx, df, ncp, TRUE as libc::c_int, FALSE as libc::c_int) > p {
            ux = nx;
        } else {
            lx = nx;
        }
        if !(ux - lx > accu * fmax2(fabs(lx), fabs(ux))) {
            break;
        }
    }
    return 0.5f64 * (lx + ux);
}
