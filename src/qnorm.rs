pub fn qnorm5(p: f64, mu: f64, sigma: f64, lower_tail: bool, log_p: bool) -> f64 {
    let mut r: f64;
    let mut val: f64;
    if p.is_nan() || mu.is_nan() || sigma.is_nan() {
        return p + mu + sigma;
    }
    if log_p {
        if p > 0 as libc::c_int as f64 {
            return f64::NAN;
        }
        if p == 0 as libc::c_int as f64 {
            return if lower_tail {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as f64 || p > 1 as libc::c_int as f64 {
            return f64::NAN;
        }
        if p == 0 as libc::c_int as f64 {
            return if lower_tail {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as f64 {
            return if lower_tail {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
    }
    if sigma < 0 as libc::c_int as f64 {
        return f64::NAN;
    }
    if sigma == 0 as libc::c_int as f64 {
        return mu;
    }
    let p_ = if log_p {
        if lower_tail {
            p.exp()
        } else {
            -p.exp_m1()
        }
    } else if lower_tail {
        p
    } else {
        0.5f64 - p + 0.5f64
    };
    let q = p_ - 0.5f64;
    if q.abs() <= 0.425f64 {
        r = 0.180625f64 - q * q;
        val = q
            * (((((((r * 2509.0809287301226727f64 + 33430.575583588128105f64) * r
                + 67265.770927008700853f64)
                * r
                + 45921.953931549871457f64)
                * r
                + 13731.693765509461125f64)
                * r
                + 1971.5909503065514427f64)
                * r
                + 133.14166789178437745f64)
                * r
                + 3.387132872796366608f64)
            / (((((((r * 5226.495278852854561f64 + 28729.085735721942674f64) * r
                + 39307.89580009271061f64)
                * r
                + 21213.794301586595867f64)
                * r
                + 5394.1960214247511077f64)
                * r
                + 687.1870074920579083f64)
                * r
                + 42.313330701600911252f64)
                * r
                + 1.0f64);
    } else {
        if q > 0 as libc::c_int as f64 {
            r = if log_p {
                if lower_tail {
                    -p.exp_m1()
                } else {
                    p.exp()
                }
            } else if lower_tail {
                0.5f64 - p + 0.5f64
            } else {
                p
            };
        } else {
            r = p_;
        }
        r = (-if log_p && (lower_tail && q <= 0. || !lower_tail && q > 0.) {
            p
        } else {
            r.ln()
        })
        .sqrt();
        if r <= 5.0f64 {
            r += -1.6f64;
            val = (((((((r * 7.7454501427834140764e-4f64 + 0.0227238449892691845833f64) * r
                + 0.24178072517745061177f64)
                * r
                + 1.27045825245236838258f64)
                * r
                + 3.64784832476320460504f64)
                * r
                + 5.7694972214606914055f64)
                * r
                + 4.6303378461565452959f64)
                * r
                + 1.42343711074968357734f64)
                / (((((((r * 1.05075007164441684324e-9f64 + 5.475938084995344946e-4f64) * r
                    + 0.0151986665636164571966f64)
                    * r
                    + 0.14810397642748007459f64)
                    * r
                    + 0.68976733498510000455f64)
                    * r
                    + 1.6763848301838038494f64)
                    * r
                    + 2.05319162663775882187f64)
                    * r
                    + 1.0f64);
        } else {
            r += -5.0f64;
            val = (((((((r * 2.01033439929228813265e-7f64 + 2.71155556874348757815e-5f64) * r
                + 0.0012426609473880784386f64)
                * r
                + 0.026532189526576123093f64)
                * r
                + 0.29656057182850489123f64)
                * r
                + 1.7848265399172913358f64)
                * r
                + 5.4637849111641143699f64)
                * r
                + 6.6579046435011037772f64)
                / (((((((r * 2.04426310338993978564e-15f64 + 1.4215117583164458887e-7f64) * r
                    + 1.8463183175100546818e-5f64)
                    * r
                    + 7.868691311456132591e-4f64)
                    * r
                    + 0.0148753612908506148525f64)
                    * r
                    + 0.13692988092273580531f64)
                    * r
                    + 0.59983220655588793769f64)
                    * r
                    + 1.0f64);
        }
        if q < 0.0f64 {
            val = -val;
        }
    }
    return mu + sigma * val;
}
