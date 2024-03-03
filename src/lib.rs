mod core;
pub use core::*;
mod simp;
use satif::{SatifSat, SatifUnsat};
pub use simp::*;

use logic_form::Lit;
use std::ffi::{c_int, c_void};

extern "C" {
    fn solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
}

pub struct Sat {
    solver: *mut c_void,
}

impl SatifSat for Sat {
    fn lit_value(&self, lit: Lit) -> Option<bool> {
        let res = unsafe { solver_model_value(self.solver, lit.into()) };
        assert!(res == 0 || res == 1);
        Some(res == 0)
    }
}

pub struct Unsat {
    solver: *mut c_void,
}

impl SatifUnsat for Unsat {
    fn has(&self, lit: Lit) -> bool {
        unsafe { solver_conflict_has(self.solver, (!lit).into()) }
    }
}
