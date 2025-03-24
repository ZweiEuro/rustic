#[cfg(test)]
pub mod test {
    use super::GenVec;

    #[test]
    fn gen_vec() {
        let mut gv = GenVec::new();

        // Insert
        let a = gv.insert("a");
        let b = gv.insert("b");
        let c = gv.insert("c");
        assert_eq!(gv.get(&a), Some(&"a"));
        assert_eq!(gv.get(&b), Some(&"b"));
        assert_eq!(gv.get(&c), Some(&"c"));
        assert_eq!(gv.len(), 3);

        // Remove
        gv.remove(&a);
        assert_eq!(gv.get(&a), None);
        assert_eq!(gv.len(), 2);

        // Re-insert
        let d = gv.insert("d");

        assert_eq!(a.index, d.index);
        assert_ne!(a.generation, d.generation);

        // Re-remove and re-re-insert
        gv.remove(&d);
        let e = gv.insert("e");
        assert_eq!(a.index, e.index);
        assert_ne!(a.generation, e.generation);
    }
}

pub struct Index {
    pub index: usize,
    pub generation: u32,
}

enum Entry<T> {
    Free { next_free: usize },
    Occupied { value: T },
}

struct GenEntry<T> {
    entry: Entry<T>,
    generation: u32,
}

pub struct GenVec<T> {
    data: Vec<GenEntry<T>>,
    free_head: usize,
    len: usize,
}

impl<T> GenVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            free_head: 0,
            len: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Index {
        let key = if let Some(GenEntry { entry, generation }) = self.data.get_mut(self.free_head) {
            // Update
            if let Entry::Free { next_free } = entry {
                let key = Index {
                    index: self.free_head,
                    generation: *generation,
                };
                self.free_head = *next_free;
                *entry = Entry::Occupied { value };
                key
            } else {
                // We have found an occupied entry, what?!
                panic!("corrupt free list");
            }
        } else {
            // Insert
            let generation = 0;
            let key = Index {
                index: self.data.len(),
                generation,
            };
            let entry = Entry::Occupied { value };
            let gen_entry = GenEntry { entry, generation };
            self.data.push(gen_entry);
            self.free_head = key.index + 1;
            key
        };

        self.len = self.len + 1;
        key
    }

    pub fn get(&self, key: &Index) -> Option<&T> {
        let GenEntry { entry, generation } = &self.data[key.index];

        if let Entry::Occupied { value } = entry {
            if *generation == key.generation {
                return Some(value);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &Index) {
        let GenEntry { entry, generation } = &mut self.data[key.index];

        if let Entry::Occupied { .. } = entry {
            if *generation != key.generation {
                // Trying to remove an older generation
                return;
            }

            *generation += 1;
            *entry = Entry::Free {
                next_free: self.free_head,
            };
            self.free_head = key.index;
            self.len = self.len - 1;
        } else {
            // If we get there it mean's that the user is trying to remove an already
            // removed key, just do nothing.
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
