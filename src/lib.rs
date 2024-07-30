mod core;
pub use core::*;
mod simp;
pub use simp::*;

use logic_form::Lit;
use std::ffi::{c_int, c_void};

extern "C" {}
