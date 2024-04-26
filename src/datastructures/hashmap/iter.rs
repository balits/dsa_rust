use std::hash::Hash;

use crate::datastructures::hashmap::HashMap;

pub struct Pairs<'m, K, V> {
    map: &'m HashMap<K, V>,
    bucket: usize,
    pair: usize,
}

impl<'m, K, V> Iterator for Pairs<'m, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'m K, &'m V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.pair) {
                    Some((k, v)) => {
                        self.pair += 1;
                        return Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.pair = 0;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'m, K, V> IntoIterator for &'m HashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'m K, &'m V);
    type IntoIter = Pairs<'m, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Pairs {
            map: self,
            bucket: 0,
            pair: 0,
        }
    }
}

pub struct IntoPairs<K, V> {
    map: HashMap<K, V>,
    bucket: usize,
}

impl<K, V> Iterator for IntoPairs<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get_mut(self.bucket) {
                Some(bucket) => match bucket.pop() {
                    Some(pair) => return Some(pair),
                    None => {
                        self.bucket += 1;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = IntoPairs<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoPairs {
            map: self,
            bucket: 0,
        }
    }
}

/// An iterator over references to keys
pub struct Keys<'m, K, V> {
    map: &'m HashMap<K, V>,
    bucket: usize,
    pair: usize,
}

impl<'m, K, V> Iterator for Keys<'m, K, V>
where
    K: Eq + Hash,
{
    type Item = &'m K;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.pair) {
                    Some((k, _)) => {
                        self.pair += 1;
                        return Some(k);
                    }
                    None => {
                        self.bucket += 1;
                        self.pair = 0;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'m, K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn keys(&'m self) -> Keys<'m, K, V> {
        Keys {
            map: self,
            bucket: 0,
            pair: 0,
        }
    }
}

/// An iterator over references to values
pub struct Values<'m, K, V> {
    map: &'m HashMap<K, V>,
    bucket: usize,
    pair: usize,
}

impl<'m, K, V> Iterator for Values<'m, K, V>
where
    K: Eq + Hash,
{
    type Item = &'m V;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.pair) {
                    Some((_, v)) => {
                        self.pair += 1;
                        return Some(v);
                    }
                    None => {
                        self.bucket += 1;
                        self.pair = 0;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'m, K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn values(&'m self) -> Values<'m, K, V> {
        Values {
            map: self,
            bucket: 0,
            pair: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BOOKS: &'static [(u8, &'static str)] = &[
        (1, "A Game of Thrones"),
        (2, "A Clash of Kings"),
        (3, "A Storm of Swords"),
        (4, "A Feast for Crows"),
        (5, "A Dance with Dragons"),
    ];

    #[test]
    fn iter_pairs() {
        let mut map = HashMap::new();
        for (k, v) in BOOKS {
            map.insert(*k, v.to_string());
        }

        // (k,v) are only references, we dont own them
        for (k, v) in &map {
            assert!(BOOKS.contains(&(*k, v.as_str())));
            // println!("#{i}: {k} - {v}");
        }

        // now we have moved k,v out from the map, we took ownership of them
        // for implicitly calls map.into_iter()
        for (k, v) in map {
            assert!(BOOKS.contains(&(k, v.as_str())));
        }
    }

    #[test]
    fn iter_keys() {
        let mut map = HashMap::new();
        for (k, v) in BOOKS {
            map.insert(*k, v.to_string());
        }

        let keys_only: Vec<u8> = BOOKS.iter().map(|(k, v)| *k).collect();
        for k in map.keys() {
            // println!("#{i}: {k} - {v}");
            assert!(keys_only.contains(&k));
        }
    }

    #[test]
    fn iter_values() {
        let mut map = HashMap::new();
        for (k, v) in BOOKS {
            map.insert(*k, v.to_string());
        }

        let titles: Vec<&str> = BOOKS.iter().map(|(_, v)| *v).collect();
        for v in map.values() {
            // println!("#{i}: {k} - {v}");
            assert!(titles.contains(&v.as_str()));
        }
    }
}
