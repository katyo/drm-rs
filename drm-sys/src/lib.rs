#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate core;
extern crate libc;

#[cfg(feature = "use_bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "use_bindgen"),
          target_os="linux",
          target_arch="x86_64"))]
include!(concat!("platforms/linux/x86_64/bindings.rs"));

#[cfg(all(not(feature = "use_bindgen"),
          target_os="linux",
          target_arch="i686"))]
include!(concat!("platforms/linux/i686/bindings.rs"));

#[cfg(all(not(feature = "use_bindgen"),
          target_os="linux",
          target_arch="arm"))]
include!(concat!("platforms/linux/arm/bindings.rs"));

#[cfg(all(not(feature = "use_bindgen"),
          target_os="freebsd",
          target_arch="x86_64"))]
include!(concat!("platforms/freebsd/x86_64/bindings.rs"));

pub mod fourcc;
