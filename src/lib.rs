pub struct Dsu {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            parents: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    pub fn lookup(&mut self, key: usize) -> Option<usize> {
        if key >= self.parents.len() {
            return None;
        }
        if key == self.parents[key] {
            return Some(key);
        }
        self.parents[key] = self.lookup(self.parents[key]).unwrap();
        Some(self.parents[key])
    }

    pub fn union(&mut self, first: usize, second: usize) -> bool {
        if let (Some(first), Some(second)) = (self.lookup(first), self.lookup(second)) {
            if first != second {
                if self.ranks[first] < self.ranks[second] {
                    self.ranks.swap(first, second);
                }
                self.parents[second] = first;
                if self.ranks[first] == self.ranks[second] {
                    self.ranks[first] += 1;
                }
            }
            return true;
        }
        false
    }
}
