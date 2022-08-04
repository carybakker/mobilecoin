// Copyright (c) 2018-2022 The MobileCoin Foundation

use libc::ssize_t;
use mc_util_ffi::{FfiOptOwnedStr, FfiOptRefPtr, FfiOptStr, FfiOwnedStr, FfiStr};

/// This trait facilitates converting one type into another when returning a
/// value from an FFI function. This could be either a return value or an out
/// parameter.
///
/// In the event of an `Result`-style error or a caught panic, the value
/// returned from a call to `error_value()` will be used. This value is often a
/// sentinel value that foreign code can use to differentiate between a
/// successful return and a failure, such as `null` in the case of a pointer, or
/// `-1` in the case of an `int`.
///
/// This provides the most benefit when the error value does not also represent
/// a valid success value, but that in itself is not a requirement, as long as
/// there is another way to detect that an error has occurred (such as an error
/// out-parameter that's only set to a non-null value if an error has occurred).
pub(crate) trait IntoFfi<T>: Sized {
    fn error_value() -> T;

    fn into_ffi(self) -> T;
}

impl<T: IntoFfi<I>, I> IntoFfi<I> for Option<T> {
    #[inline]
    fn error_value() -> I {
        T::error_value()
    }

    #[inline]
    fn into_ffi(self) -> I {
        if let Some(s) = self {
            s.into_ffi()
        } else {
            T::error_value()
        }
    }
}

macro_rules! impl_into_ffi_using_default {
    ($($Type:ty),+) => {
        $(
            impl IntoFfi<$Type> for $Type {
                #[inline]
                fn error_value() -> Self {
                    Default::default()
                }

                #[inline]
                fn into_ffi(self) -> Self {
                    self
                }
            }
        )+
    }
}

impl_into_ffi_using_default![(), bool];

impl IntoFfi<bool> for () {
    #[inline]
    fn error_value() -> bool {
        false
    }

    #[inline]
    fn into_ffi(self) -> bool {
        true
    }
}

pub struct RngSeed32([u8; 32]);
impl AsRef<[u8]> for RngSeed32 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

pub struct McU128 {
    pub bytes: [u8; 16]    
}

impl McU128 {
    pub fn from_u128(val: u128) -> McU128 {
        McU128 {
            bytes: val.to_be_bytes(),
        }
    }

    pub fn to_u128(&self) -> u128 {
        u128::from_be_bytes(self.bytes)
    }
}

// impl_into_ffi_using_default![McU128];

impl IntoFfi<McU128> for McU128 {
    #[inline]
    fn error_value() -> McU128 {
        McU128 {
            bytes: [u8::MAX; 16],
        }
    }

    #[inline]
    fn into_ffi(self) -> McU128 {
        self
    }
}


impl_into_ffi_using_default![u64];

impl IntoFfi<i64> for i64 {
    #[inline]
    fn error_value() -> i64 {
        -1
    }

    #[inline]
    fn into_ffi(self) -> i64 {
        self
    }
}

impl IntoFfi<ssize_t> for ssize_t {
    #[inline]
    fn error_value() -> ssize_t {
        -1
    }

    #[inline]
    fn into_ffi(self) -> ssize_t {
        self
    }
}

impl IntoFfi<FfiOptOwnedStr> for FfiOwnedStr {
    #[inline]
    fn error_value() -> FfiOptOwnedStr {
        FfiOptOwnedStr::null()
    }

    #[inline]
    fn into_ffi(self) -> FfiOptOwnedStr {
        self.into()
    }
}

impl IntoFfi<FfiOptOwnedStr> for FfiOptOwnedStr {
    #[inline]
    fn error_value() -> FfiOptOwnedStr {
        FfiOptOwnedStr::null()
    }

    #[inline]
    fn into_ffi(self) -> FfiOptOwnedStr {
        self
    }
}

pub(crate) trait FromFfi<T>: Sized {
    fn from_ffi(src: T) -> Self;
}

pub(crate) trait FfiInto<U>: Sized {
    fn ffi_into(self) -> U;
}

impl<T, U> FfiInto<U> for T
where
    U: FromFfi<T>,
{
    #[inline]
    fn ffi_into(self) -> U {
        <U as FromFfi<T>>::from_ffi(self)
    }
}

pub(crate) trait TryFromFfi<T>: Sized {
    type Error: Sized;
    fn try_from_ffi(src: T) -> Result<Self, Self::Error>;
}

pub(crate) trait FfiTryInto<U>: Sized {
    type Error: Sized;
    fn ffi_try_into(self) -> Result<U, Self::Error>;
}

impl<T, U> FfiTryInto<U> for T
where
    U: TryFromFfi<T>,
{
    type Error = U::Error;

    #[inline]
    fn ffi_try_into(self) -> Result<U, Self::Error> {
        <U as TryFromFfi<T>>::try_from_ffi(self)
    }
}

impl<T, U> TryFromFfi<Option<T>> for Option<U>
where
    U: TryFromFfi<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_from_ffi(src: Option<T>) -> Result<Self, Self::Error> {
        src.map(U::try_from_ffi).transpose()
    }
}

impl<'a, U> TryFromFfi<FfiOptStr<'a>> for Option<U>
where
    U: TryFromFfi<FfiStr<'a>>,
{
    type Error = <U as TryFromFfi<FfiStr<'a>>>::Error;

    #[inline]
    fn try_from_ffi(src: FfiOptStr<'a>) -> Result<Self, Self::Error> {
        src.as_option().map(U::try_from_ffi).transpose()
    }
}

impl<'a, T, U> TryFromFfi<FfiOptRefPtr<'a, T>> for Option<U>
where
    U: TryFromFfi<&'a T>,
{
    type Error = <U as TryFromFfi<&'a T>>::Error;

    #[inline]
    fn try_from_ffi(src: FfiOptRefPtr<'a, T>) -> Result<Self, Self::Error> {
        src.as_ref().map(U::try_from_ffi).transpose()
    }
}

pub(crate) trait TryIntoFfi<T>: Sized {
    type Error: Sized;
    fn try_into_ffi(self) -> Result<T, Self::Error>;
}

pub(crate) trait FfiTryFrom<U>: Sized {
    type Error: Sized;
    fn ffi_try_from(src: U) -> Result<Self, Self::Error>;
}

impl<T, U> TryIntoFfi<U> for T
where
    U: FfiTryFrom<T>,
{
    type Error = U::Error;

    #[inline]
    fn try_into_ffi(self) -> Result<U, Self::Error> {
        <U as FfiTryFrom<T>>::ffi_try_from(self)
    }
}
