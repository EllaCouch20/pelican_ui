#[cfg(any(target_os = "ios", target_os = "android"))]
pub const IS_MOBILE: bool = true;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub const IS_MOBILE: bool = false;