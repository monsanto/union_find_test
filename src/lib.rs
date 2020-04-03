#![allow(unused)]

use std::marker::PhantomData;
mod union_find;
use union_find as uf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lol() {
        new(|mut sys| {
            let i = sys.mk_int();

            assert!(sys.eq(i, i));

            let arr = sys.mk_arrow(i, i);

            assert!(!sys.eq(arr, i));
            assert!(sys.eq(arr, arr));
        })
    }

    // #[test]
    // fn guh() {
    //     new(|mut sys| {
    //         System::new(|mut sys2| {
    //             let v1 = sys.mk_int();

    //             let v2 = sys2.mk_int();
    //             let v3 = sys2.mk_int();

    //             sys.eq(v1, v2);
    //         })
    //     })
    // }

    #[test]
    fn seee() {
        new(|mut sys| {
            let v1 = sys.mk_var();
            let mut sys2 = sys.subsystem();
            let v2 = sys2.mk_var();
            // let sys2 = sys2.finish();
            // let vquant = sys2.quantify(v2);
            // let mut sys3 = sys.subsystem();
            // let v3 = sys3.mk_var();
            // sys3.eq(vquant, v3);
            sys2.eq(v2, v1);
        })
    }

    #[test]
    fn vars() {
        new(|mut sys| {
            let i = sys.mk_int();
            let v1 = sys.mk_var();
            let v2 = sys.mk_var();
            sys.eq(v1, v2);
            sys.eq(i, v1);
            let arr = sys.mk_arrow(i, i);
            assert!(!sys.eq(v2, arr));
        })
    }
}

pub enum Active {}
pub enum Finished {}

type Level = u32;

pub struct System<'id, 'current, State> {
    arena: &'current mut uf::UnionFind<'id, Var<'id>>,
    level: Level,
    // Invariant in 'state
    _marker: PhantomData<*mut State>,
}

#[derive(Copy, Clone)]
enum Head<'id> {
    Int,
    Arrow(uf::Key<'id>, uf::Key<'id>),
}

#[derive(Copy, Clone)]
enum Var<'id> {
    Unsolved(Level),
    Solved(Head<'id>),
}

#[derive(Copy, Clone)]
pub struct Type<'id, 'current> {
    contents: uf::Key<'id>,
    // Covariant in 'current
    _marker: PhantomData<&'current ()>,
}

pub fn new<Out, F: for<'id, 'current> FnOnce(System<'id, 'current, Active>) -> Out>(f: F) -> Out {
    uf::new(|mut uf : uf::UnionFind<Var<'_>>| {
        let sys = System::<Active> {
            arena: &mut uf,
            level: 0,
            _marker: PhantomData,
        };
        f(sys)
    })
}

impl<'id, 'current> System<'id, 'current, Active> {
    fn mk_type<'scope>(&mut self, var: Var<'id>) -> Type<'id, 'scope> {
        Type {
            contents: self.arena.alloc(var),
            _marker: PhantomData,
        }
    }

    pub fn mk_var(&mut self) -> Type<'id, 'current> {
        self.mk_type(Var::Unsolved(self.level))
    }

    pub fn mk_int(&mut self) -> Type<'id, 'static> {
        self.mk_type(Var::Solved(Head::Int))
    }

    pub fn mk_arrow<'scope>(
        &mut self,
        t1: Type<'id, 'scope>,
        t2: Type<'id, 'scope>,
    ) -> Type<'id, 'scope> {
        self.mk_type(Var::Solved(Head::Arrow(t1.contents, t2.contents)))
    }

    pub fn subsystem<'new>(&'new mut self) -> System<'id, 'new, Active> {
        System {
            arena: self.arena,
            level: self.level + 1,
            _marker: PhantomData,
        }
    }

    pub fn finish(self) -> System<'id, 'current, Finished> {
        System {
            arena: self.arena,
            level: self.level,
            _marker: PhantomData,
        }
    }

    fn eq_head(&mut self, ty1: Head<'id>, ty2: Head<'id>) -> bool {
        match (ty1, ty2) {
            (Head::Int, Head::Int) => true,
            (Head::Int, _) => false,
            (Head::Arrow(dom1, rng1), Head::Arrow(dom2, rng2)) => {
                self.eq_ref(dom1, dom2) && self.eq_ref(dom2, rng2)
            }
            (Head::Arrow(..), _) => false,
        }
    }

    fn eq_ref(&mut self, ty1: uf::Key<'id>, ty2: uf::Key<'id>) -> bool {
        unimplemented!()
        // let v1 = self.arena.get(ty1);
        // let v2 = self.arena.get(ty2);
        // match (v1, v2) {
        //     (Var::Unsolved(level1), Var::Unsolved(level2)) => {
        //         let level = std::cmp::min(level1, level2);
        //         self.arena[ty1] = Var::Unsolved(level);
        //         self.arena[ty2] = Var::Unsolved(level);
        //         true
        //     }
        //     (Var::Solved(..), Var::Unsolved(..)) => {
        //         self.arena[ty2] = v1;
        //         true
        //     }
        //     (Var::Unsolved(..), Var::Solved(..)) => {
        //         self.arena[ty1] = v2;
        //         true
        //     }
        //     (Var::Solved(h1), Var::Solved(h2)) => self.eq_head(h1, h2),
        // }
    }

    pub fn eq(&mut self, ty1: Type<'id, 'current>, ty2: Type<'id, 'current>) -> bool {
        self.eq_ref(ty1.contents, ty2.contents)
    }
}

impl<'id, 'current> System<'id, 'current, Finished> {
    pub fn quantify(&self, ty: Type<'id, 'current>) -> Type<'id, 'static> {
        Type {
            contents: ty.contents,
            _marker: PhantomData,
        }
    }
}
