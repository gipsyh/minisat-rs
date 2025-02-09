use logic_form::{Lit, Var};
use satif::Satif;
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
    fn solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
}

pub struct Solver {
    solver: *mut c_void,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { solver_new() },
        }
    }
}

impl Satif for Solver {
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

    fn solve(&mut self, assumps: &[Lit]) -> bool {
        unsafe { solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _) }
    }

    fn sat_value(&mut self, lit: Lit) -> Option<bool> {
        let res = unsafe { solver_model_value(self.solver, Into::<u32>::into(lit) as _) };
        assert!(res == 0 || res == 1);
        Some(res == 0)
    }

    fn unsat_has(&mut self, lit: Lit) -> bool {
        unsafe { solver_conflict_has(self.solver, Into::<u32>::into(!lit) as _) }
    }

    fn simplify(&mut self) -> Option<bool> {
        if !unsafe { solver_simplify(self.solver) } {
            return Some(false);
        }
        None
    }
}

impl Solver {
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
        (0..out_len)
            .map(|i| unsafe { *out_ptr.add(i as usize) })
            .collect()
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
    use logic_form::LitVec;
    let mut solver = Solver::new();
    let lit0: Lit = solver.new_var().into();
    let lit1: Lit = solver.new_var().into();
    let lit2: Lit = solver.new_var().into();
    solver.add_clause(&LitVec::from([lit0, !lit2]));
    solver.add_clause(&LitVec::from([lit1, !lit2]));
    solver.add_clause(&LitVec::from([!lit0, !lit1, lit2]));
    if solver.solve(&[lit2]) {
        assert!(solver.sat_value(lit0).unwrap());
        assert!(solver.sat_value(lit1).unwrap());
        assert!(solver.sat_value(lit2).unwrap());
    } else {
        panic!();
    }
    solver.add_clause(&LitVec::from([!lit0]));
    if !solver.solve(&[lit2]) {
        assert!(solver.unsat_has(lit2));
    } else {
        panic!();
    }
}
