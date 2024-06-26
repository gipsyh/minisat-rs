use crate::{Sat, Unsat};
use logic_form::{Lit, Var};
use satif::{SatResult, Satif};
use std::ffi::{c_int, c_void};

extern "C" {
    fn solver_new() -> *mut c_void;
    fn solver_free(s: *mut c_void);
    fn solver_new_var(s: *mut c_void) -> c_int;
    fn solver_num_var(s: *mut c_void) -> c_int;
    fn solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int) -> bool;
    fn solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> bool;
    fn solver_simplify(s: *mut c_void) -> bool;
    fn solver_release_var(s: *mut c_void, lit: c_int);
    fn solver_set_random_seed(s: *mut c_void, seed: f64);
    fn solver_set_rnd_init_act(s: *mut c_void, enable: bool);
    fn solver_set_polarity(s: *mut c_void, var: c_int, pol: c_int);
    fn solver_implies(
        s: *mut c_void,
        assumps: *mut c_int,
        len: c_int,
        out_len: *mut c_int,
    ) -> *mut c_void;

}

pub struct Solver {
    solver: *mut c_void,
}

impl Satif for Solver {
    type Sat = Sat;

    type Unsat = Unsat;

    fn new() -> Self {
        Self {
            solver: unsafe { solver_new() },
        }
    }

    fn new_var(&mut self) -> Var {
        Var::new(unsafe { solver_new_var(self.solver) } as usize)
    }

    fn num_var(&self) -> usize {
        unsafe { solver_num_var(self.solver) as _ }
    }

    fn add_clause(&mut self, clause: &[Lit]) {
        if !unsafe { solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _) } {
            println!("warning: minisat add_clause fail");
        }
    }

    fn solve(&mut self, assumps: &[Lit]) -> satif::SatResult<Self::Sat, Self::Unsat> {
        if unsafe { solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _) } {
            SatResult::Sat(Sat {
                solver: self.solver,
            })
        } else {
            SatResult::Unsat(Unsat {
                solver: self.solver,
            })
        }
    }
}

impl Solver {
    pub fn simplify(&mut self) {
        if !unsafe { solver_simplify(self.solver) } {
            println!("warning: minisat simplify fail");
        }
    }

    pub fn release_var(&mut self, lit: Lit) {
        unsafe { solver_release_var(self.solver, Into::<u32>::into(lit) as _) }
    }

    pub fn set_polarity(&mut self, var: Var, pol: Option<bool>) {
        let pol = match pol {
            Some(true) => 0,
            Some(false) => 1,
            None => 2,
        };
        unsafe { solver_set_polarity(self.solver, var.into(), pol) }
    }

    pub fn set_random_seed(&mut self, seed: f64) {
        unsafe { solver_set_random_seed(self.solver, seed) }
    }

    pub fn set_rnd_init_act(&mut self, enable: bool) {
        unsafe { solver_set_rnd_init_act(self.solver, enable) }
    }

    pub fn implies(&self, assumps: &[Lit]) -> Vec<Lit> {
        let mut out_len = 0;
        let out_ptr: *mut Lit = unsafe {
            solver_implies(
                self.solver,
                assumps.as_ptr() as _,
                assumps.len() as _,
                &mut out_len,
            ) as _
        };
        unsafe { Vec::from_raw_parts(out_ptr, out_len as _, out_len as _) }
    }

    /// # Safety
    /// unsafe get sat model
    pub unsafe fn get_model(&self) -> Sat {
        Sat {
            solver: self.solver,
        }
    }

    /// # Safety
    /// unsafe get unsat core
    pub unsafe fn get_conflict(&self) -> Unsat {
        Unsat {
            solver: self.solver,
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { solver_free(self.solver) }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    use logic_form::Clause;
    use satif::{SatifSat, SatifUnsat};
    let mut solver = Solver::new();
    let lit0: Lit = solver.new_var().into();
    let lit1: Lit = solver.new_var().into();
    let lit2: Lit = solver.new_var().into();
    solver.add_clause(&Clause::from([lit0, !lit2]));
    solver.add_clause(&Clause::from([lit1, !lit2]));
    solver.add_clause(&Clause::from([!lit0, !lit1, lit2]));
    match solver.solve(&[lit2]) {
        SatResult::Sat(model) => {
            assert!(model.lit_value(lit0).unwrap());
            assert!(model.lit_value(lit1).unwrap());
            assert!(model.lit_value(lit2).unwrap());
        }
        SatResult::Unsat(_) => todo!(),
    }
    solver.add_clause(&Clause::from([!lit0]));
    match solver.solve(&[lit2]) {
        SatResult::Sat(_) => panic!(),
        SatResult::Unsat(conflict) => {
            assert!(conflict.has(lit2));
        }
    }
}
