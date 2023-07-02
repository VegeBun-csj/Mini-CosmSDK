use std::{
    cell::RefCell,
    collections::{ BTreeMap},
    ops::Bound,
};

use crate::DB;

pub struct MemDB {
    store: RefCell<BTreeMap<Vec<u8>, Vec<u8>>>, // we use a refcell because the set method on the DB trait doesn't take a mutable ref
}

impl MemDB {
    pub fn new() -> MemDB {
        MemDB {
            store: RefCell::new(BTreeMap::new()),
        }
    }
}

impl DB for MemDB {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.store.borrow().get(key).cloned()
    }

    fn put(&self, key: Vec<u8>, value: Vec<u8>) {
        self.store.borrow_mut().insert(key, value);
    }

    fn iterator<'a>(&'a self) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a> {
        Box::new(
            self.store
                .borrow()
                .clone()
                .into_iter()
                .map(|(key, value)| (key.into_boxed_slice(), value.into_boxed_slice())),
        )
    }

    fn prefix_iterator<'a>(
        &'a self,
        prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a> {
        let start = Bound::Included(prefix.clone());
        let end = prefix_end_bound(prefix);

        let mut pairs = Vec::new();

        for (k, v) in self.store.borrow().range((start, end)) {
            //println!("Found: {}: {}", k, v);
            let pair = (k.clone().into_boxed_slice(), v.clone().into_boxed_slice());
            pairs.push(pair)
        }

        Box::new(pairs.into_iter())
    }
}

/// Returns the Bound on a range query for a given prefix
///
/// That is the smallest x such that, prefix + y < x for all y. If
/// no such x exists (i.e. prefix = vec![255; N]; for some N) it returns Bound::Unbounded
fn prefix_end_bound(mut prefix: Vec<u8>) -> Bound<Vec<u8>> {
    loop {
        let last = prefix.last_mut();

        match last {
            None => return Bound::Unbounded,
            Some(last) => {
                if *last != 255 {
                    *last += 1;
                    return Bound::Excluded(prefix);
                }
                prefix.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterator_works() {
        let db = MemDB::new();
        db.put(vec![1], vec![1]);
        db.put(vec![2], vec![2]);
        let got_pairs: Vec<(Box<[u8]>, Box<[u8]>)> = db.iterator().collect();

        let expected_pairs: Vec<(Box<[u8]>, Box<[u8]>)> = vec![
            (vec![1].into_boxed_slice(), vec![1].into_boxed_slice()),
            (vec![2].into_boxed_slice(), vec![2].into_boxed_slice()),
        ];

        assert_eq!(expected_pairs.len(), got_pairs.len());
        assert!(got_pairs.iter().all(|e| { expected_pairs.contains(e) }));
    }

    #[test]
    fn prefix_iterator_works() {
        let db = MemDB::new();
        db.put(vec![1, 1], vec![1]);
        db.put(vec![2, 1], vec![2]);
        db.put(vec![3, 1], vec![3]);
        db.put(vec![4, 1], vec![4]);

        let got_pairs: Vec<(Box<[u8]>, Box<[u8]>)> = db.prefix_iterator(vec![2]).collect();

        println!("got pairs: {:?}", got_pairs);

        let expected_pairs: Vec<(Box<[u8]>, Box<[u8]>)> =
            vec![(vec![2, 1].into_boxed_slice(), vec![2].into_boxed_slice())];

        assert_eq!(expected_pairs.len(), got_pairs.len());
        assert!(got_pairs.iter().all(|e| { expected_pairs.contains(e) }));
    }
}
