use core::time::Duration;

use std::sync::RwLock;
use std::time::SystemTime;

static FLAT_VALUE: RwLock<FlatValue> = RwLock::new(FlatValue::Unspecified(der::asn1::Null {}));
static ENCODED: RwLock<[u8; 1024]> = RwLock::new([0; 1024]);

use der::Encode;

#[derive(der::Choice)]
pub enum FlatValue {
    Unspecified(der::asn1::Null),
    Boolean(bool),
    Integer(i64),
    Real(f64),
    GeneralizedTime(SystemTime),
}

impl FlatValue {
    pub fn to_der_bytes(&self) -> Result<Vec<u8>, &'static str> {
        self.to_der().map_err(|_| "unable to serialize")
    }
}

impl FlatValue {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_real(&self) -> bool {
        matches!(self, Self::Real(_))
    }

    pub fn is_time(&self) -> bool {
        matches!(self, Self::GeneralizedTime(_))
    }
}

impl FlatValue {
    pub fn boolean_value(&self) -> Result<bool, &'static str> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err("not boolean"),
        }
    }

    pub fn integer_value(&self) -> Result<i64, &'static str> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err("not integer"),
        }
    }

    pub fn real_value(&self) -> Result<f64, &'static str> {
        match self {
            Self::Real(f) => Ok(*f),
            _ => Err("not real"),
        }
    }

    pub fn unixtime_value(&self) -> Result<f64, &'static str> {
        let stime: SystemTime = match self {
            Self::GeneralizedTime(s) => Ok(*s),
            _ => Err("not time"),
        }?;
        let dur: Duration = stime
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| "invalid time")?;
        Ok(dur.as_secs_f64())
    }
}

pub fn try_read<F, V>(read2val: &F) -> Result<V, &'static str>
where
    F: Fn(&FlatValue) -> Result<V, &'static str>,
{
    let guard = FLAT_VALUE.try_read().map_err(|_| "unable to read lock")?;
    let fval: &FlatValue = &guard;
    read2val(fval)
}

pub fn bool2u8(b: bool) -> u8 {
    match b {
        true => 0xff,
        false => 0x00,
    }
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn is_boolean() -> u8 {
    try_read(&|fval: &FlatValue| Ok(fval.is_boolean()))
        .map(bool2u8)
        .unwrap_or(0x00)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn is_integer() -> u8 {
    try_read(&|fval: &FlatValue| Ok(fval.is_integer()))
        .map(bool2u8)
        .unwrap_or(0x00)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn is_real() -> u8 {
    try_read(&|fval: &FlatValue| Ok(fval.is_real()))
        .map(bool2u8)
        .unwrap_or(0x00)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn is_time() -> u8 {
    try_read(&|fval: &FlatValue| Ok(fval.is_time()))
        .map(bool2u8)
        .unwrap_or(0x00)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn bool_value() -> u8 {
    try_read(&|fval: &FlatValue| fval.boolean_value())
        .map(bool2u8)
        .unwrap_or(0x00)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn integer_value() -> i64 {
    try_read(&|fval: &FlatValue| fval.integer_value()).unwrap_or_default()
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn real_value() -> f64 {
    try_read(&|fval: &FlatValue| fval.real_value()).unwrap_or(f64::NAN)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn unixtime_seconds() -> f64 {
    try_read(&|fval: &FlatValue| fval.unixtime_value()).unwrap_or(f64::NAN)
}

pub fn try_write<F, V>(f: &F, val: V) -> Result<(), &'static str>
where
    F: Fn(&mut FlatValue, V) -> Result<(), &'static str>,
{
    let mut guard = FLAT_VALUE.try_write().map_err(|_| "unable to write lock")?;
    let fv: &mut FlatValue = &mut guard;
    f(fv, val)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn bool_set_true() -> i32 {
    try_write(
        &|fval: &mut FlatValue, val: bool| {
            *fval = FlatValue::Boolean(val);
            Ok(())
        },
        true,
    )
    .map(|_| 0)
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn bool_set_false() -> i32 {
    try_write(
        &|fval: &mut FlatValue, val: bool| {
            *fval = FlatValue::Boolean(val);
            Ok(())
        },
        false,
    )
    .map(|_| 0)
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn integer_set(i: i64) -> i32 {
    try_write(
        &|fval: &mut FlatValue, val: i64| {
            *fval = FlatValue::Integer(val);
            Ok(())
        },
        i,
    )
    .map(|_| 0)
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn real_set(f: f64) -> i32 {
    try_write(
        &|fval: &mut FlatValue, val: f64| {
            *fval = FlatValue::Real(val);
            Ok(())
        },
        f,
    )
    .map(|_| 0)
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn unixtime_seconds_set(f: f64) -> i32 {
    try_write(
        &|fval: &mut FlatValue, val: f64| {
            let dur: Duration = Duration::from_secs_f64(val);
            let tim: SystemTime = SystemTime::UNIX_EPOCH
                .checked_add(dur)
                .ok_or("invalid time")?;
            *fval = FlatValue::GeneralizedTime(tim);
            Ok(())
        },
        f,
    )
    .map(|_| 0)
    .unwrap_or(-1)
}

pub fn _offset() -> Result<*const u8, &'static str> {
    let guard = ENCODED.try_read().map_err(|_| "unable to read lock")?;
    let a: &[u8; 1024] = &guard;
    let s: &[u8] = a;
    Ok(s.as_ptr())
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn offset() -> *const u8 {
    _offset().unwrap_or(std::ptr::null())
}

pub fn _encode() -> Result<usize, &'static str> {
    let mut guard = ENCODED.try_write().map_err(|_| "unable to write lock")?;
    let a: &mut [u8; 1024] = &mut guard;
    let s: &mut [u8] = a;

    let der_bytes: Vec<u8> = try_read(&|fv: &FlatValue| fv.to_der_bytes())?;
    let src: &[u8] = &der_bytes;
    let ls: usize = src.len();
    let ld: usize = s.len();
    let len: usize = ls.min(ld);

    let limited_s: &[u8] = &src[..len];
    let limited_d: &mut [u8] = &mut s[..len];
    limited_d.copy_from_slice(limited_s);

    Ok(len)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn encode() -> i32 {
    _encode().map(|u| u as i32).unwrap_or(-1)
}
