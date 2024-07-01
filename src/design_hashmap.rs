pub struct HashEntry {
    pub key: i32,
    pub value: i32,
}

pub type Chain = Vec<HashEntry>;

pub struct MyHashMap {
    pub buckets: Vec<Chain>,
    pub len: usize,
    pub capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    pub fn new() -> Self {
        let capacity = 64;

        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }

        Self {
            buckets,
            len: 0,
            capacity,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.len == self.capacity / 2 {
            self.resize();
        }

        let index = (key % self.capacity as i32) as usize;
        self.len += 1;

        if let Some(i) = self.buckets[index].iter_mut().position(|e| e.key == key) {
            self.buckets[index][i].value = value;
        } else {
            self.buckets[index].push(HashEntry { key, value });
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let index = key % self.capacity as i32;

        self.buckets[index as usize]
            .iter()
            .position(|e| e.key == key)
            .map(|i| self.buckets[index as usize].get(i).unwrap().value)
            .unwrap_or(-1)
    }

    pub fn remove(&mut self, key: i32) {
        let index = key % self.capacity as i32;
        let chain = &mut self.buckets[index as usize];

        chain
            .iter_mut()
            .position(|e| e.key == key)
            .map(|i| chain.remove(i));
    }

    fn resize(&mut self) {
        self.capacity *= 2;
        let mut buckets: Vec<Vec<HashEntry>> = Vec::with_capacity(self.capacity);
        for _ in 0..self.capacity {
            buckets.push(Vec::new());
        }

        for chain in &self.buckets {
            for entry in chain {
                let index = entry.key % self.capacity as i32;
                buckets[index as usize].push(HashEntry {
                    key: entry.key,
                    value: entry.value,
                });
            }
        }

        self.buckets = buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut map = MyHashMap::new();

        map.put(1, 1);
        map.put(2, 2);

        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);

        map.put(2, 1);

        assert_eq!(map.get(2), 1);

        map.remove(2);

        assert_eq!(map.get(2), -1);
    }
}
