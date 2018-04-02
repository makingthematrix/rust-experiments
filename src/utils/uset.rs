#![macro_use]

use std::ops::Range;
use std::cmp::{max, min};

use std::ops::{Add, Sub};

#[allow(unused_macros)]
macro_rules! uset {
    ($($x:expr),*) => (USet::from_vec(&vec![$($x),*]))
}

#[derive(Debug, Default, Clone)]
pub struct USet {
    set: Vec<bool>,
    len: usize,
}

pub struct USetIter<'a> {
    uset: &'a USet,
    index: usize,
    rindex: usize,
}

impl<'a> Iterator for USetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.uset.set.len() - self.rindex;
        while self.index < max {
            let index = self.index;
            self.index += 1;
            if self.uset.set[index] {
                return Some(index);
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for USetIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.uset.set.len();
        while self.rindex < len - self.index {
            let index = len - self.rindex - 1;
            self.rindex += 1;
            if self.uset.set[index] {
                return Some(index);
            }
        }
        None
    }
}

impl USet {
    pub fn default() -> USet {
        USet::new()
    }

    pub fn new() -> USet {
        USet::with_capacity(0)
    }

    pub fn with_capacity(size: usize) -> USet {
        USet {
            set: vec![false; size],
            len: 0,
        }
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

    pub fn pop(&mut self, index: usize) -> Option<usize> {
        let d = self.find_by_index(index);
        if !d.is_none() {
            self.remove(d.unwrap());
        }
        d
    }

    pub fn iter(&self) -> USetIter {
        USetIter {
            uset: self,
            index: 0,
            rindex: 0,
        }
    }

    #[inline]
    pub fn contains(&self, value: usize) -> bool {
        value < self.set.len() && self.set[value]
    }

    fn find_by_index(&self, index: usize) -> Option<usize> {
        let mut it = self.iter();
        for _i in 0..index {
            it.next();
        }
        it.next()
    }

    #[inline]
    fn min(&self) -> Option<usize> {
        self.iter().next()
    }

    #[inline]
    fn max(&self) -> Option<usize> {
        self.iter().rev().next()
    }

    pub fn from_vec(vec: &[usize]) -> USet {
        let &mx = vec.iter().max().unwrap_or(&0);
        let mut set = vec![false; mx + 1];
        vec.iter().for_each(|&i| set[i] = true);
        USet {
            set,
            len: vec.len(),
        }
    }

    pub fn from_range(r: Range<usize>) -> USet {
        let mut set = vec![false; r.end];
        let len = r.len();
        for i in r {
            set[i] = true;
        }
        USet { set, len }
    }

    fn add_set(&self, other: &USet) -> USet {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let min: usize = min(self.min().unwrap(), other.min().unwrap());
            let max: usize = max(self.max().unwrap(), other.max().unwrap());

            let mut set = vec![false; max + 1];
            let mut len = 0usize;

            // TODO: is it possible to simply create the vec from the range?
            set.iter_mut()
                .enumerate()
                .skip(min)
                .take(max + 1)
                .for_each(|(i, value)| {
                    if self.contains(i) || other.contains(i) {
                        *value = true;
                        len += 1;
                    }
                });
            USet { set, len }
        }
    }

    fn sub_set(&self, other: &USet) -> USet {
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
}

impl PartialEq for USet {
    fn eq(&self, other: &USet) -> bool {
        self.len == other.len && self.set == other.set
    }
}

impl Eq for USet {}

impl<'a> Add for &'a USet {
    type Output = USet;
    fn add(self, other: &USet) -> USet {
        self.add_set(other)
    }
}

impl<'a> Sub for &'a USet {
    type Output = USet;
    fn sub(self, other: &USet) -> USet {
        self.sub_set(other)
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

impl Into<Vec<usize>> for USet {
    fn into(self) -> Vec<usize> {
        self.iter().collect()
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
