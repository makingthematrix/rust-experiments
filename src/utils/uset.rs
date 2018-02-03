use std::ops::Range;

extern crate rand;
use self::rand::{Rng, ThreadRng};

#[derive(Debug)]
pub struct USet {
    set: Vec<bool>,
    len: usize,
}

impl USet {
    pub fn new(size: usize) -> USet {
        USet {
            set: vec![false; size],
            len: 0,
        }
    }

    pub fn from_vec(vec: &[usize]) -> USet {
        let &mx = vec.iter().max().unwrap_or(&0);
        let mut s = vec![false; mx + 1];
        vec.iter().for_each(|&i| s[i] = true);
        USet {
            set: s,
            len: vec.len(),
        }
    }

    pub fn from_range(r: Range<usize>) -> USet {
        let mut s = vec![false; r.end];
        let len = r.len();
        for i in r {
            s[i] = true;
        }
        USet { set: s, len: len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.set.len()
    }

    pub fn add(&mut self, el: usize) {
        if el >= self.set.len() {
            self.set.resize(el + 1, false);
        }
        if !self.set[el] {
            self.set[el] = true;
            self.len += 1;
        }
    }

    pub fn remove(&mut self, el: usize) {
        if el < self.set.len() && self.set[el] {
            self.set[el] = false;
            self.len -= 1
        }
    }

    pub fn pop_random(&mut self, rnd: &mut ThreadRng) -> Option<usize> {
        if self.len > 0 {
            let index = rnd.gen_range(0, self.len);
            self.pop(index)
        } else {
            None
        }
    }

    pub fn pop(&mut self, index: usize) -> Option<usize> {
        let d = self.find(index);
        if !d.is_none() {
            self.remove(d.unwrap());
        }
        d
    }

    pub fn find(&self, index: usize) -> Option<usize> {
        if index < self.len {
            let mut c = index;
            for (i, &b) in self.set.iter().enumerate() {
                if b {
                    if c == 0 {
                        return Some(i);
                    }
                    c -= 1;
                }
            }
            None
        } else {
            None
        }
    }

    pub fn substract(&self, other: &USet) -> USet {
        let mut s = self.set.clone();
        let mut size = self.len();
        other
            .set
            .iter()
            .take(s.len())
            .enumerate()
            .for_each(|(i, &v)| {
                if v && s[i] {
                    s[i] = false;
                    size -= 1;
                }
            });

        USet { set: s, len: size }
    }

    pub fn to_vec(&self) -> Vec<usize> {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(i) } else { None })
            .collect()
    }

    pub fn contains(&self, value: usize) -> bool {
        self.set[value]
    }
}

use std::ops::Sub;

impl<'a> Sub for &'a USet {
    type Output = USet;
    fn sub(self, other: &USet) -> USet {
        self.substract(other)
    }
}

impl<'a> From<&'a [usize]> for USet {
    fn from(slice: &'a [usize]) -> Self {
        USet::from_vec(slice)
    }
}

impl From<Vec<usize>> for USet {
    fn from(vec: Vec<usize>) -> Self {
        USet::from_vec(&vec)
    }
}

impl<'a> From<&'a Vec<usize>> for USet {
    fn from(vec: &'a Vec<usize>) -> Self {
        USet::from_vec(vec)
    }
}

impl From<Range<usize>> for USet {
    fn from(r: Range<usize>) -> Self {
        USet::from_range(r)
    }
}
