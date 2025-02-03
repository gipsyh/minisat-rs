#include <iostream>
#include <vector>
#include "minisat/core/Solver.h"
#include "minisat/simp/SimpSolver.h"
using namespace Minisat;

class BindingSolver : public Solver {
    public:
	bool add_clause(int *clause, int len)
	{
		add_tmp.clear();
		add_tmp.growTo(len);
		Lit *cls = (Lit *)clause;
		for (int i = 0; i < len; ++i)
			add_tmp[i] = cls[i];
		return addClause_(add_tmp);
	}

	bool solve(int *assumps, int len)
	{
		budgetOff();
		assumptions.clear();
		assumptions.growTo(len);
		Lit *asp = (Lit *)assumps;
		for (int i = 0; i < len; ++i)
			assumptions[i] = asp[i];
		return solve_() == l_True;
	}
};

extern "C" {
void *solver_new()
{
	return new BindingSolver();
}

void solver_free(void *s)
{
	BindingSolver *slv = (BindingSolver *)s;
	delete slv;
}

int solver_new_var(void *s)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->newVar();
}

int solver_num_var(void *s)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->nVars();
}

bool solver_add_clause(void *s, int *clause, int len)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->add_clause(clause, len);
}

bool solver_solve(void *s, int *assumps, int len)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->solve(assumps, len);
}

int solver_model_value(void *s, int lit)
{
	BindingSolver *slv = (BindingSolver *)s;
	return toInt(slv->modelValue(toLit(lit)));
}

bool solver_conflict_has(void *s, int lit)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->conflict.has(toLit(lit));
}

bool solver_simplify(void *s)
{
	BindingSolver *slv = (BindingSolver *)s;
	return slv->simplify();
}

void solver_release_var(void *s, int lit)
{
	BindingSolver *slv = (BindingSolver *)s;
	slv->releaseVar(toLit(lit));
}

void solver_set_polarity(void *s, int var, int pol)
{
	BindingSolver *slv = (BindingSolver *)s;
	slv->setPolarity(var, toLbool(pol));
}

void solver_set_random_seed(void *s, double seed)
{
	BindingSolver *slv = (BindingSolver *)s;
	slv->random_seed = seed;
}

void solver_set_rnd_init_act(void *s, bool enable)
{
	BindingSolver *slv = (BindingSolver *)s;
	slv->rnd_init_act = enable;
}

void *solver_implies(void *s, int *assumps, int len, int *out_len)
{
	BindingSolver *slv = (BindingSolver *)s;
	vec<Lit> a;
	Lit *asp = (Lit *)assumps;
	for (int i = 0; i < len; ++i)
		a.push(asp[i]);
	vec<Lit> *out = new vec<Lit>();
	slv->implies(a, *out);
	*out_len = out->size();
	return &(*out)[0];
}
}

class BindingSimpSolver : public SimpSolver {
    public:
	bool add_clause(int *clause, int len)
	{
		add_tmp.clear();
		add_tmp.growTo(len);
		Lit *cls = (Lit *)clause;
		for (int i = 0; i < len; ++i)
			add_tmp[i] = cls[i];
		return addClause_(add_tmp);
	}
};

extern "C" {
void *simp_solver_new()
{
	return new BindingSimpSolver();
}

void simp_solver_free(void *s)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	delete slv;
}

int simp_solver_new_var(void *s)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	return slv->newVar();
}

int simp_solver_num_var(void *s)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	return slv->nVars();
}

bool simp_solver_add_clause(void *s, int *clause, int len)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	return slv->add_clause(clause, len);
}

void simp_solver_set_frozen(void *s, int var, bool frozen)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	slv->setFrozen(var, frozen);
}

bool simp_solver_eliminate(void *s, bool turn_off_elim)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	return slv->eliminate(turn_off_elim);
}

void *simp_solver_clauses(void *s, int *len)
{
	BindingSimpSolver *slv = (BindingSimpSolver *)s;
	std::vector<void *> *clauses = new std::vector<void *>();
	for (Minisat::ClauseIterator c = slv->clausesBegin(); c != slv->clausesEnd(); ++c) {
		const Minisat::Clause &cls = *c;
		std::vector<Lit> *cls_ = new std::vector<Lit>;
		for (int i = 0; i < cls.size(); ++i)
			cls_->push_back(cls[i]);
		clauses->push_back(cls_->data());
		clauses->push_back((void *)cls_->size());
	}
	for (Minisat::TrailIterator c = slv->trailBegin(); c != slv->trailEnd(); ++c) {
		std::vector<Lit> *cls_ = new std::vector<Lit>;
		cls_->push_back(*c);
		clauses->push_back(cls_->data());
		clauses->push_back((void *)cls_->size());
	}
	*len = clauses->size();
	return clauses->data();
}
}
