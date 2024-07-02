pub struct MyHashSet {
    set: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    pub fn new() -> Self {
        let mut set = Vec::new();
        for _ in 0..64 {
            set.push(Vec::new());
        }

        Self { set }
    }

    pub fn add(&mut self, key: i32) {
        let index = key % 64;
        let chain = &mut self.set[index as usize];

        if !chain.contains(&key) {
            chain.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let index = key % 64;
        let chain = &mut self.set[index as usize];

        if let Some(i) = chain.iter().position(|e| *e == key) {
            chain.swap_remove(i as usize);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = key % 64;
        let chain = &self.set[index as usize];

        chain.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn case_1() {
        let mut hashset = MyHashSet::new();

        hashset.add(1);
        hashset.add(2);

        assert!(hashset.contains(1));
        assert!(!hashset.contains(3));

        hashset.add(2);

        assert!(hashset.contains(2));

        hashset.remove(2);

        assert!(!hashset.contains(2));
    }
}
