use std::usize;

use super::HashMap;

pub enum Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    entry: &'a mut (K, V),
}
pub struct VacantEntry<'a, K, V> {
    key: K,
    map: &'a mut HashMap<K, V>,
    bucket: usize,
}

mod test {
    use crate::datastructures::HashMap;

    #[test]
    fn or_insert() {
        let mut map = HashMap::<&str, u32>::new();
        map.entry("poneyland").or_insert(3);
        assert_eq!(map["poneyland"], 3);

        *map.entry("poneyland").or_insert(10) *= 2;
        assert_eq!(map["poneyland"], 6);
    }

    #[test]
    fn or_insert_with() {
        let mut map = HashMap::new();
        let value = "hoho";

        map.entry("poneyland").or_insert_with(|| value);

        assert_eq!(map["poneyland"], "hoho");
    }

    #[test]
    fn or_insert_with_key() {
        let mut map: HashMap<&str, usize> = HashMap::new();

        map.entry("poneyland")
            .or_insert_with_key(|key| key.chars().count());

        assert_eq!(map["poneyland"], 9);
    }

    #[test]
    fn key() {
        assert!(false);
    }

    #[test]
    fn and_modify() {
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 42);

        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 43);
    }
}
