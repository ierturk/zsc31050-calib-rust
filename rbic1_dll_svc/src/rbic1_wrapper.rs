// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

#![allow(warnings)]

extern crate libc;

use libc::{c_char, c_float, c_int, c_long};
use std::ffi::CStr;

use common_lib::types::*;

#[link(name = "RBIC1", kind = "static")]
unsafe extern "C" {
    pub unsafe fn OutLin(
        Zp1m: c_float,
        Zp2m: c_float,
        A: c_float,
        B: c_float,
        C0: *mut c_float,
        C1: *mut c_float,
    ) -> bool;

    pub unsafe fn TLin(
        Ztmed: c_float,
        Ztupp: c_float,
        Tmed: c_float,
        Tupp: c_float,
        Ct0: *mut c_float,
        Ct1: *mut c_float,
    ) -> bool;

    pub unsafe fn OutQuad(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp3m: c_float,
        A: c_float,
        B: c_float,
        M: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C2: *mut c_float,
    ) -> bool;

    pub unsafe fn OutThird(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp3m: c_float,
        Zp4m: c_float,
        A: c_float,
        B: c_float,
        M: c_float,
        M2: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C2: *mut c_float,
        C3: *mut c_float,
    ) -> bool;

    pub unsafe fn TQuad(
        Ztlow: c_float,
        Ztupp: c_float,
        Ztmed: c_float,
        Tlow: c_float,
        Tupp: c_float,
        Tmed: c_float,
        adc_reso: c_int,
        Ct0: *mut c_float,
        Ct1: *mut c_float,
        Ct2: *mut c_float,
    ) -> bool;

    pub unsafe fn OutLinTLin(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp1u: c_float,
        Zp2u: c_float,
        A: c_float,
        B: c_float,
        Ztmed: c_float,
        Ztupp: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C4: *mut c_float,
        C6: *mut c_float,
    ) -> bool;

    pub unsafe fn OutQuadTLin(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp3m: c_float,
        Zp1u: c_float,
        Zp2u: c_float,
        A: c_float,
        B: c_float,
        M: c_float,
        Ztmed: c_float,
        Ztupp: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C2: *mut c_float,
        C4: *mut c_float,
        C6: *mut c_float,
    ) -> bool;

    pub unsafe fn OutLinTQuad(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp1u: c_float,
        Zp2u: c_float,
        Zp1l: c_float,
        Zp2l: c_float,
        A: c_float,
        B: c_float,
        Ztmed: c_float,
        Ztupp: c_float,
        Ztlow: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C4: *mut c_float,
        C6: *mut c_float,
        C5: *mut c_float,
        C7: *mut c_float,
    ) -> bool;

    pub unsafe fn OutQuadTQuad(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp3m: c_float,
        Zp1u: c_float,
        Zp2u: c_float,
        Zp1l: c_float,
        Zp2l: c_float,
        A: c_float,
        B: c_float,
        M: c_float,
        Ztmed: c_float,
        Ztupp: c_float,
        Ztlow: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C2: *mut c_float,
        C4: *mut c_float,
        C6: *mut c_float,
        C5: *mut c_float,
        C7: *mut c_float,
    ) -> bool;

    pub unsafe fn ZMD31050_cal1(
        Zp1m: c_float,
        Zp2m: c_float,
        Zp3m: c_float,
        Zp4m: c_float,
        Zp1u: c_float,
        Zp2u: c_float,
        Zp1l: c_float,
        Zp2l: c_float,
        A: c_float,
        B: c_float,
        M: c_float,
        M2: c_float,
        Ztmed: c_float,
        Ztupp: c_float,
        Ztlow: c_float,
        adc_reso: c_int,
        C0: *mut c_float,
        C1: *mut c_float,
        C2: *mut c_float,
        C3: *mut c_float,
        C4: *mut c_float,
        C6: *mut c_float,
        C5: *mut c_float,
        C7: *mut c_float,
    ) -> c_long;

    pub unsafe fn ZMD31050_sim1(
        c0: c_int,
        c1: c_int,
        c2: c_int,
        c3: c_int,
        c4: c_int,
        c5: c_int,
        c6: c_int,
        c7: c_int,
        adc_reso: c_int,
        range_shift: c_float,
        izmin: c_int,
        izmax: c_int,
        ZT: c_int,
        ZTmin: c_int,
        ZTmax: c_int,
    ) -> c_long;

    pub unsafe fn getMessage() -> *const c_char;

    pub unsafe fn DLLVersion(version: *mut c_char) -> bool;

    pub unsafe static mut msgRBIC1: *const c_char;

    pub unsafe static mut rcRBIC1: c_long;
}

pub unsafe fn out_lin(
    zp1m: f32,
    zp2m: f32,
    a: f32,
    b: f32,
) -> Result<(f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let ret = unsafe { OutLin(zp1m, zp2m, a, b, &mut c0, &mut c1) };
    Ok((c0, c1, ret))
}

pub unsafe fn t_lin(
    ztmed: f32,
    ztupp: f32,
    tmed: f32,
    tupp: f32,
) -> Result<(f32, f32, bool), RBIC1ServiceErrorType> {
    let mut ct0 = 0.0;
    let mut ct1 = 0.0;
    let ret = unsafe { TLin(ztmed, ztupp, tmed, tupp, &mut ct0, &mut ct1) };
    Ok((ct0, ct1, ret))
}

pub unsafe fn out_quad(
    zp1m: f32,
    zp2m: f32,
    zp3m: f32,
    a: f32,
    b: f32,
    m: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c2 = 0.0;
    let ret = unsafe {
        OutQuad(
            zp1m, zp2m, zp3m, a, b, m, adc_reso, &mut c0, &mut c1, &mut c2,
        )
    };
    Ok((c0, c1, c2, ret))
}

pub unsafe fn out_third(
    zp1m: f32,
    zp2m: f32,
    zp3m: f32,
    zp4m: f32,
    a: f32,
    b: f32,
    m: f32,
    m2: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c2 = 0.0;
    let mut c3 = 0.0;
    let ret = unsafe {
        OutThird(
            zp1m, zp2m, zp3m, zp4m, a, b, m, m2, adc_reso, &mut c0, &mut c1, &mut c2, &mut c3,
        )
    };
    Ok((c0, c1, c2, c3, ret))
}

pub unsafe fn t_quad(
    ztlow: f32,
    ztupp: f32,
    ztmed: f32,
    tlow: f32,
    tupp: f32,
    tmed: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut ct0 = 0.0;
    let mut ct1 = 0.0;
    let mut ct2 = 0.0;

    let ret = unsafe {
        TQuad(
            ztlow, ztupp, ztmed, tlow, tupp, tmed, adc_reso, &mut ct0, &mut ct1, &mut ct2,
        )
    };
    Ok((ct0, ct1, ct2, ret))
}

pub unsafe fn out_lin_t_lin(
    zp1m: f32,
    zp2m: f32,
    zp1u: f32,
    zp2u: f32,
    a: f32,
    b: f32,
    ztmed: f32,
    ztupp: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c4 = 0.0;
    let mut c6 = 0.0;

    let ret = unsafe {
        OutLinTLin(
            zp1m, zp2m, zp1u, zp2u, a, b, ztmed, ztupp, adc_reso, &mut c0, &mut c1, &mut c4,
            &mut c6,
        )
    };
    Ok((c0, c1, c4, c6, ret))
}

pub unsafe fn out_quad_t_lin(
    zp1m: f32,
    zp2m: f32,
    zp3m: f32,
    zp1u: f32,
    zp2u: f32,
    a: f32,
    b: f32,
    m: f32,
    ztmed: f32,
    ztupp: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c2 = 0.0;
    let mut c4 = 0.0;
    let mut c6 = 0.0;

    let ret = unsafe {
        OutQuadTLin(
            zp1m, zp2m, zp3m, zp1u, zp2u, a, b, m, ztmed, ztupp, adc_reso, &mut c0, &mut c1,
            &mut c2, &mut c4, &mut c6,
        )
    };
    Ok((c0, c1, c2, c4, c6, ret))
}

pub unsafe fn out_lin_t_quad(
    zp1m: f32,
    zp2m: f32,
    zp1u: f32,
    zp2u: f32,
    zp1l: f32,
    zp2l: f32,
    a: f32,
    b: f32,
    ztmed: f32,
    ztupp: f32,
    ztlow: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c4 = 0.0;
    let mut c6 = 0.0;
    let mut c5 = 0.0;
    let mut c7 = 0.0;

    let ret = unsafe {
        OutLinTQuad(
            zp1m, zp2m, zp1u, zp2u, zp1l, zp2l, a, b, ztmed, ztupp, ztlow, adc_reso, &mut c0,
            &mut c1, &mut c4, &mut c6, &mut c5, &mut c7,
        )
    };
    Ok((c0, c1, c4, c6, c5, c7, ret))
}

pub unsafe fn out_quad_t_quad(
    zp1m: f32,
    zp2m: f32,
    zp3m: f32,
    zp1u: f32,
    zp2u: f32,
    zp1l: f32,
    zp2l: f32,
    a: f32,
    b: f32,
    m: f32,
    ztmed: f32,
    ztupp: f32,
    ztlow: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, f32, f32, f32, bool), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c2 = 0.0;
    let mut c4 = 0.0;
    let mut c6 = 0.0;
    let mut c5 = 0.0;
    let mut c7 = 0.0;

    let ret = unsafe {
        OutQuadTQuad(
            zp1m, zp2m, zp3m, zp1u, zp2u, zp1l, zp2l, a, b, m, ztmed, ztupp, ztlow, adc_reso,
            &mut c0, &mut c1, &mut c2, &mut c4, &mut c6, &mut c5, &mut c7,
        )
    };
    Ok((c0, c1, c2, c4, c6, c5, c7, ret))
}

pub unsafe fn zmd31050_cal1(
    zp1m: f32,
    zp2m: f32,
    zp3m: f32,
    zp4m: f32,
    zp1u: f32,
    zp2u: f32,
    zp1l: f32,
    zp2l: f32,
    a: f32,
    b: f32,
    m: f32,
    m2: f32,
    ztmed: f32,
    ztupp: f32,
    ztlow: f32,
    adc_reso: i32,
) -> Result<(f32, f32, f32, f32, f32, f32, f32, f32, i32), RBIC1ServiceErrorType> {
    let mut c0 = 0.0;
    let mut c1 = 0.0;
    let mut c2 = 0.0;
    let mut c3 = 0.0;
    let mut c4 = 0.0;
    let mut c6 = 0.0;
    let mut c5 = 0.0;
    let mut c7 = 0.0;
    let mut ret = 0;

    unsafe {
        let ret = ZMD31050_cal1(
            zp1m, zp2m, zp3m, zp4m, zp1u, zp2u, zp1l, zp2l, a, b, m, m2, ztmed, ztupp, ztlow,
            adc_reso, &mut c0, &mut c1, &mut c2, &mut c3, &mut c4, &mut c6, &mut c5, &mut c7,
        );
    }

    Ok((c0, c1, c2, c3, c4, c6, c5, c7, ret))
}

pub unsafe fn zmd31050_sim1(
    c0: i32,
    c1: i32,
    c2: i32,
    c3: i32,
    c4: i32,
    c5: i32,
    c6: i32,
    c7: i32,
    adc_reso: i32,
    range_shift: f32,
    izmin: i32,
    izmax: i32,
    zt: i32,
    ztmin: i32,
    ztmax: i32,
) -> Result<i32, RBIC1ServiceErrorType> {
    unsafe {
        Ok(ZMD31050_sim1(
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
            c6,
            c7,
            adc_reso,
            range_shift,
            izmin,
            izmax,
            zt,
            ztmin,
            ztmax,
        ))
    }
}

pub unsafe fn dll_version() -> Result<String, RBIC1ServiceErrorType> {
    let mut version = [0 as c_char; 256];

    match unsafe { DLLVersion(version.as_mut_ptr()) } {
        true => {
            let c_str = unsafe { CStr::from_ptr(version.as_ptr()) };
            Ok(c_str.to_string_lossy().into_owned())
        }
        false => Err(RBIC1ServiceErrorType::OperationFailed),
    }
}

pub unsafe fn get_message() -> Result<String, RBIC1ServiceErrorType> {
    let message = unsafe { CStr::from_ptr(getMessage()) };
    Ok((message.to_string_lossy().into_owned()))
}
