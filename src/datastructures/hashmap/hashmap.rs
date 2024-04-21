use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

const INIT_BUCKETS: usize = 4;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    fn key_to_index<Q: ?Sized>(key: &Q, len: usize) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % len as u64) as usize
    }

    fn resize(&mut self) {
        let new_size = match self.buckets.len() {
            0 => INIT_BUCKETS,
            l => l * 2,
        };

        let mut new_buckets: Vec<Vec<(K, V)>> = Vec::with_capacity(new_size);

        (0..new_size).for_each(|_| new_buckets.push(Vec::with_capacity(new_size)));

        for mut bucket in self.buckets.drain(..) {
            for (key, value) in bucket.drain(..) {
                let index = Self::key_to_index(&key, new_buckets.len());
                new_buckets[index].push((key, value));
            }
        }

        let _ = std::mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let index = Self::key_to_index(&key, self.buckets.len());
        let bucket = self.buckets.get_mut(index).unwrap();

        if let Some((_, prev_v)) = bucket.iter_mut().find(|(k, _)| *k == key) {
            Some(std::mem::replace(prev_v, value))
        } else {
            bucket.push((key, value));
            self.items += 1;
            None
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::key_to_index(key, self.buckets.len());
        self.buckets[index]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::key_to_index(key, self.buckets.len());
        self.buckets[index]
            .iter_mut()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::key_to_index(key, self.buckets.len());

        self.buckets[index]
            .iter()
            .position(|(k, _)| k.borrow() == key)
            .map(|pos| {
                self.items -= 1;
                self.buckets[index].swap_remove(pos).1
            })
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = Self::key_to_index(key, self.buckets.len());

        self.buckets[index]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .is_some()
    }
}

mod test {
    use crate::HashMap;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert(5, "A Dance with Dragons");

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&5), Some(&"A Dance with Dragons"));

        let adwd = map.remove(&5);

        assert_eq!(map.len(), 0);
        assert_eq!(adwd, Some("A Dance with Dragons"));

        assert_eq!(map.get(&5), None);
    }

    #[test]
    fn rust_doc_example() {
        let mut book_reviews = HashMap::new();

        // Review some books.
        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        book_reviews.insert(
            "Grimms' Fairy Tales".to_string(),
            "Masterpiece.".to_string(),
        );
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );
        book_reviews.insert(
            "The Adventures of Sherlock Holmes".to_string(),
            "Eye lyked it alot.".to_string(),
        );

        //  Check for a specific one.
        //  When collections store owned values (String), they can still be
        //  queried using references (&str).
        if !book_reviews.contains_key("Les Misérables") {
            println!(
                "We've got {} reviews, but Les Misérables ain't one.",
                book_reviews.len()
            );
        }

        // oops, this review has a lot of spelling mistakes, let's delete it.
        book_reviews.remove("The Adventures of Sherlock Holmes");

        // Look up the values associated with some keys.
        let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        for &book in &to_find {
            match book_reviews.get(book) {
                Some(review) => println!("{book}: {review}"),
                None => println!("{book} is unreviewed."),
            }
        }

        // Look up the value for a key (will panic if the key is not found).
        // println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

        // Iterate over everything.
        for (book, review) in &book_reviews {
            println!("{book}: \"{review}\"");
        }
    }
}
