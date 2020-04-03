use std::marker::PhantomData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        new(|mut uf: UnionFind<u32>| {
            let k1 = uf.alloc(0);
            let k2 = uf.alloc(0);

            assert!(!uf.equivalent(k1, k2));

            uf.union_uncond(k1, k2, 1);

            assert!(uf.equivalent(k1, k2));
            assert_eq!(uf.get(k1), 1);
            assert_eq!(uf.get(k2), 1);
        })
    }
}

// TODO: path compression, more efficient implementation in general

pub struct UnionFind<'id, V> {
    table: Vec<Value<'id, V>>,
    // Invariant in 'id
    _marker: PhantomData<*mut &'id ()>,
}

#[derive(Copy, Clone)]
pub struct Key<'id> {
    key: usize,
    _marker: PhantomData<*mut &'id ()>,
}

type CanonicalKey<'id> = Key<'id>;

#[derive(Copy, Clone)]
enum Value<'id, V> {
    Leaf(V),
    Branch(Key<'id>),
}

pub fn new<Out, V, F: for<'id> FnOnce(UnionFind<'id, V>) -> Out>(f: F) -> Out {
    f(UnionFind {
        table: Vec::new(),
        _marker: PhantomData,
    })
}

impl<'id, V: Copy> UnionFind<'id, V> {
    pub fn alloc(&mut self, v: V) -> Key<'id> {
        let key = Key {
            key: self.table.len(),
            _marker: PhantomData,
        };
        self.table.push(Value::Leaf(v));
        key
    }

    fn get_canonical(&mut self, key: Key<'id>) -> (CanonicalKey<'id>, V) {
        match self.table[key.key] {
            Value::Leaf(v) => (key, v),
            Value::Branch(next_key) => {
                let (canon_key, v) = self.get_canonical(next_key);
                self.table[key.key] = Value::Branch(canon_key);
                (canon_key, v)
            }
        }
    }

    pub fn get(&mut self, key: Key<'id>) -> V {
        self.get_canonical(key).1
    }

    pub fn union_uncond(&mut self, k1: Key<'id>, k2: Key<'id>, v: V) {
        self.union(k1, k2, (), |_, _| (Some(v), ()))
    }

    // Should it even be possible to back out of union? What does this mean for type errors?
    pub fn union<Out, F: FnOnce(V, V) -> (Option<V>, Out)>(
        &mut self,
        k1: Key<'id>,
        k2: Key<'id>,
        default: Out,
        f: F,
    ) -> Out {
        let (k1, v1) = self.get_canonical(k1);
        let (k2, v2) = self.get_canonical(k2);

        if (k1.key == k2.key) {
            default
        } else {
            // TODO: union-by-rank
            let (should_union, result) = f(v1, v2);

            if let Some(v) = should_union {
                self.table[k1.key] = Value::Leaf(v);
                self.table[k2.key] = Value::Branch(k1);
            }

            result
        }
    }

    pub fn equivalent(&mut self, k1: Key<'id>, k2: Key<'id>) -> bool {
        self.get_canonical(k1).0.key == self.get_canonical(k2).0.key
    }
}
