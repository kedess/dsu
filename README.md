## dsu
A disjoint set system is a data structure that allows you to administer a set of elements divided into disjoint subsets.
Initially, there are several elements, each of which is in a separate (own) set. Two sets can be combined in one operation,
and you can also request which set this element is in. The algorithmic complexity of both operations averages O(1)

### Usage example:
```rust
use dsu::Dsu;

fn main() {
    let mut dsu = Dsu::new(10);
    
    dsu.union(1, 2);
    dsu.union(2, 3);
    dsu.union(2, 7);

    assert_eq!(dsu.lookup(2).unwrap(), dsu.lookup(7).unwrap());
    assert_eq!(dsu.lookup(1).unwrap(), dsu.lookup(3).unwrap());
    assert_ne!(dsu.lookup(1).unwrap(), dsu.lookup(8).unwrap());
    assert_eq!(dsu.lookup(9).unwrap(), 9);
}
```

### Cargo.toml
```bash
[dependencies]
dsu = {git = "https://github.com/mingendo/dsu.git", branch="main"}
```
