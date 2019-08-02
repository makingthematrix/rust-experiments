#![macro_use]

use crate::utils::uset::USet;
use itertools::{Itertools, MinMaxResult};
use std::clone::Clone;
use std::cmp;
use std::fmt;
use std::ops::{Add, BitXor, Mul, Sub};

#[derive(Default, Clone)]
pub struct UMap<T> {
    pub vec: Vec<Option<T>>,
    len: usize,
    offset: usize,
    min: usize,
    max: usize,
}

#[derive(Debug, Clone)]
pub struct UMapIter<'a, T: 'a> {
    handle: &'a UMap<T>,
    index: usize,
    rindex: usize,
}

impl<'a, T> Iterator for UMapIter<'a, T>
where
    T: Clone + PartialEq,
{
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let max = self.handle.vec.len() - self.rindex;
        while self.index < max {
            let index = self.index;
            self.index += 1;
            if let Some(ref value) = self.handle.vec[index] {
                return Some((index + self.handle.offset, value));
            }
        }
        None
    }
}

impl<'a, T> DoubleEndedIterator for UMapIter<'a, T>
where
    T: Clone + PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.handle.vec.len();
        while self.rindex < len - self.index {
            let index = len - self.rindex - 1;
            self.rindex += 1;
            if let Some(ref value) = self.handle.vec[index] {
                return Some((index + self.handle.offset, &value));
            }
        }
        None
    }
}

impl<'a, T> IntoIterator for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Item = (usize, &'a T);
    type IntoIter = UMapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub const INITIAL_CAPACITY: usize = 8;

impl<T> UMap<T>
where
    T: Clone + PartialEq,
{
    pub fn new() -> Self {
        UMap::with_capacity(0)
    }

    pub fn with_capacity(size: usize) -> Self {
        UMap {
            vec: vec![None; size],
            len: 0,
            offset: 0,
            min: 0,
            max: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.vec.len()
    }

    pub fn trim(&mut self) {
        if !self.is_empty() {
            let mut vec = vec![None; self.max - self.min + 1];
            for key in self.min..=self.max {
                vec[key - self.min] = self.get(key);
            }
            self.vec = vec;
            self.offset = self.min;
        }
    }

    pub fn put(&mut self, key: usize, value: &T) {
        match key {
            _ if self.capacity() == 0 => {
                self.vec = vec![None; INITIAL_CAPACITY];
                self.vec[0] = Some(value.clone());
                self.min = key;
                self.len += 1;
                self.max = key;
                self.offset = key;
            }
            _ if self.is_empty() => {
                self.vec[0] = Some(value.clone());
                self.min = key;
                self.len = 1;
                self.max = key;
                self.offset = key;
            }
            _ if key < self.offset => {
                let mut vec = vec![None; self.max - key + 1];
                vec[0] = Some(value.clone());
                for i in self.min..=self.max {
                    vec[i - key] = self.get(i);
                }
                self.vec = vec;
                self.len += 1;
                self.min = key;
                self.offset = key;
            }
            _ if key > self.offset + self.capacity() => {
                self.vec.resize(key + 1 - self.offset, None);
                self.vec[key - self.offset] = Some(value.clone());
                self.len += 1;
                self.max = key;
            }
            _ if self.vec[key - self.offset].is_none() => {
                self.vec[key - self.offset] = Some(value.clone());
                self.len += 1;
                if key < self.min {
                    self.min = key
                } else if key > self.max {
                    self.max = key
                }
            }
            _ => {}
        }
    }

    pub fn contains(&self, key: usize) -> bool {
        key >= self.min && key <= self.max && self.vec[key - self.offset].is_some()
    }

    pub fn get_ref(&self, key: usize) -> Option<&T> {
        if key >= self.min && key <= self.max {
            if let Some(&Some(ref v)) = self.vec.get(key - self.offset) {
                Some(&v)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get(&self, key: usize) -> Option<T> {
        if key >= self.min && key <= self.max {
            unsafe { self.vec.get_unchecked(key - self.offset).clone() }
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        match key {
            _ if key < self.min || key > self.max => None,
            _ if !self.contains(key) => None,
            _ if self.len == 1 => {
                let t = self.vec[key - self.offset].clone();
                self.vec[key - self.offset] = None;
                self.max = 0;
                self.min = 0;
                self.len = 0;
                self.offset = 0;
                t
            }
            _ if key > self.min && key < self.max => {
                let t = self.vec[key - self.offset].clone();
                self.vec[key - self.offset] = None;
                self.len -= 1;
                t
            }
            _ if key == self.min => {
                let t = self.vec[key - self.offset].clone();
                self.vec[key - self.offset] = None;
                self.len -= 1;
                self.min = (self.min..self.max)
                    .find(|&i| self.vec[i - self.offset].is_some())
                    .unwrap_or(self.max);
                t
            }
            _ if key == self.max => {
                let t = self.vec[key - self.offset].clone();
                self.vec[key - self.offset] = None;
                self.len -= 1;
                self.max = (self.min..self.max)
                    .rev()
                    .find(|&i| self.vec[i - self.offset].is_some())
                    .unwrap_or(self.min);
                t
            }
            _ => None,
        }
    }

    pub fn to_set(&self) -> USet {
        let set: Vec<bool> = self.vec.iter().map(Option::is_some).collect();
        USet::from_fields(set, self.len)
    }

    pub fn pop(&mut self, index: usize) -> Option<T> {
        let d = self.find_by_index(index);
        if let Some((key, value)) = d {
            self.remove(key);
            Some(value.clone())
        } else {
            None
        }
    }

    fn find_by_index(&self, index: usize) -> Option<(usize, T)> {
        if index >= self.len {
            None
        } else {
            let mut it = self.iter();
            for _i in 0..index {
                it.next();
            }
            it.next().map(|(key, value)| (key, value.clone()))
        }
    }

    pub fn iter(&self) -> UMapIter<T> {
        UMapIter {
            handle: self,
            index: 0,
            rindex: 0,
        }
    }

    pub fn min(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.min)
        }
    }

    pub fn max(&self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            Some(self.max)
        }
    }

    fn make_from_slice(slice: &[(usize, T)]) -> (usize, usize, usize, Vec<Option<T>>) {
        match slice.iter().minmax_by_key(|(ref key, _)| *key) {
            MinMaxResult::NoElements => (0, 0, 0, Vec::<Option<T>>::new()),
            MinMaxResult::OneElement((ref key, value)) => {
                (*key, *key, 1, vec![Some(value.clone()); 1])
            }
            MinMaxResult::MinMax(&(min, _), &(max, _)) => {
                let len = slice.len();
                let capacity = cmp::max(INITIAL_CAPACITY, max + 1 - min);
                let mut vec = vec![None; capacity];
                slice
                    .iter()
                    .for_each(|(key, value)| vec[*key - min] = Some(value.clone()));
                (min, max, len, vec)
            }
        }
    }

    pub fn from_slice(slice: &[(usize, T)]) -> Self {
        if slice.is_empty() {
            UMap::new()
        } else {
            let (min, max, len, new_vec) = UMap::make_from_slice(slice);
            UMap {
                vec: new_vec,
                len,
                offset: min,
                min,
                max,
            }
        }
    }

    fn debug_compare(self: &Self, other: &UMap<T>) {
        // don't perform operation on maps if they have different elements at the same places - clearly something's messed up
        debug_assert!(self
            .iter()
            .zip(other.iter())
            .find(|&((i1, ref v1), (i2, ref v2))| i1 == i2 && v1 != v2)
            .is_none());
    }

    pub fn add_all(&mut self, slice: &[(usize, T)]) {
        if !slice.is_empty() {
            if self.is_empty() {
                let (min, max, len, new_vec) = UMap::make_from_slice(slice);
                self.min = min;
                self.max = max;
                self.offset = min;
                self.len = len;
                self.vec = new_vec;
            } else {
                let (min, max) = match slice.iter().minmax_by_key(|&(key, _)| *key) {
                    MinMaxResult::NoElements => (0, 0), // should not happen
                    MinMaxResult::OneElement(&(min, _)) => (min, min),
                    MinMaxResult::MinMax(&(min, _), &(max, _)) => (min, max),
                };

                if min >= self.min && max <= self.max {
                    slice.iter().for_each(|(ref key, value)| {
                        if self.vec[*key - self.offset].is_none() {
                            self.vec[*key - self.offset] = Some(value.clone());
                            self.len += 1;
                        }
                    })
                } else {
                    let new_min = cmp::min(self.min, min);
                    let new_max = cmp::max(self.max, max);
                    let mut new_vec = vec![None; new_max - new_min + 1];
                    self.iter()
                        .skip(self.min - self.offset)
                        .take(self.max - self.min + 1)
                        .for_each(|(key, value)| new_vec[key - new_min] = Some(value.clone()));
                    slice.iter().for_each(|(ref key, value)| {
                        if new_vec[*key - new_min].is_none() {
                            new_vec[*key - new_min] = Some(value.clone());
                            self.len += 1;
                        }
                    });
                    self.min = new_min;
                    self.offset = new_min;
                    self.max = new_max;
                    self.vec = new_vec;
                }
            }
        }
    }

    fn union(&self, other: &Self) -> Self {
        if self.is_empty() {
            if other.is_empty() {
                UMap::new()
            } else {
                other.clone()
            }
        } else if other.is_empty() {
            if self.is_empty() {
                UMap::new()
            } else {
                self.clone()
            }
        } else {
            let min: usize = cmp::min(self.min, other.min);
            let max: usize = cmp::max(self.max, other.max);

            let mut vec = vec![None; max + 1 - min];
            let mut len = 0usize;

            vec.iter_mut().enumerate().for_each(|(key, value)| {
                println!(
                    "key: {}, #1 contains: {}, #2 contains: {}",
                    key,
                    self.contains(key + self.offset),
                    other.contains(key + other.offset)
                );
                if self.contains(key + min) {
                    *value = self.get(key + min);
                    len += 1;
                } else if other.contains(key + min) {
                    *value = other.get(key + min);
                    len += 1;
                }
            });

            UMap {
                vec,
                len,
                offset: min,
                min,
                max,
            }
        }
    }

    fn difference(&self, other: &UMap<T>) -> Self {
        let mut vec = self.vec.clone();
        let mut len = self.len;

        other.iter().for_each(|(key, _)| {
            if self.contains(key) {
                vec[key - self.offset] = None;
                len -= 1;
            }
        });

        if len == 0 {
            UMap::new()
        } else {
            let min = vec
                .iter()
                .enumerate()
                .find_map(|(key, b)| if b.is_some() { Some(key) } else { None })
                .unwrap()
                + self.offset;
            let max = vec
                .iter()
                .enumerate()
                .rev()
                .find_map(|(key, b)| if b.is_some() { Some(key) } else { None })
                .unwrap()
                + self.offset;
            UMap {
                vec,
                len,
                offset: self.offset,
                min,
                max,
            }
        }
    }

    fn common_part(&self, other: &UMap<T>) -> Self {
        if self.is_empty() || other.is_empty() {
            UMap::new()
        } else {
            let rough_range = cmp::max(self.min, other.min)..=cmp::min(self.max, other.max);
            let mn = rough_range
                .clone()
                .find(|&key| self.contains(key) && other.contains(key));
            let mx = rough_range
                .clone()
                .rev()
                .find(|&key| self.contains(key) && other.contains(key));
            if let Some(min) = mn {
                if let Some(max) = mx {
                    let mut vec = vec![None; max + 1 - min];
                    let mut len = 0usize;
                    for key in min..=max {
                        if self.contains(key) && other.contains(key) {
                            vec[key - min] = self.get(key);
                            len += 1;
                        }
                    }
                    UMap {
                        vec,
                        len,
                        offset: min,
                        min,
                        max,
                    }
                } else {
                    UMap::new()
                }
            } else {
                UMap::new()
            }
        }
    }

    fn xor_set(&self, other: &UMap<T>) -> Self {
        if self.is_empty() && other.is_empty() {
            UMap::new()
        } else if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let rough_range = cmp::min(self.min, other.min)..=cmp::max(self.max, other.max);
            let mn = rough_range.clone().find(|&key| {
                (self.contains(key) && !other.contains(key)) || (!self.contains(key) && other.contains(key))
            });
            let mx = rough_range.clone().rev().find(|&key| {
                (self.contains(key) && !other.contains(key)) || (!self.contains(key) && other.contains(key))
            });
            if let Some(min) = mn {
                if let Some(max) = mx {
                    let mut vec = vec![None; max + 1 - min];
                    let mut len = 0usize;
                    for key in min..=max {
                        if self.contains(key) && !other.contains(key) {
                            vec[key - min] = self.get(key);
                            len += 1;
                        } else if !self.contains(key) && other.contains(key) {
                            vec[key - min] = other.get(key);
                            len += 1;
                        }
                    }
                    UMap {
                        vec,
                        len,
                        offset: min,
                        min,
                        max,
                    }
                } else {
                    UMap::new()
                }
            } else {
                UMap::new()
            }
        }
    }
}

impl<T> PartialEq for UMap<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
            && self.min == other.min
            && self.max == other.max
            && self
                .vec
                .iter()
                .skip(self.min - self.offset)
                .take(self.max + 1 - self.min)
                .zip(
                    other
                        .vec
                        .iter()
                        .skip(other.min - other.offset)
                        .take(other.max + 1 - other.min),
                )
                .find(|&(a, b)| *a != *b)
                .is_none()
    }
}

impl<T> Eq for UMap<T> where T: Clone + PartialEq {}

impl<'a, T> Add for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn add(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.union(other)
    }
}

impl<'a, T> Sub for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn sub(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.difference(other)
    }
}

impl<'a, T> Mul for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn mul(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.common_part(other)
    }
}

impl<'a, T> BitXor for &'a UMap<T>
where
    T: Clone + PartialEq,
{
    type Output = UMap<T>;
    fn bitxor(self, other: &UMap<T>) -> UMap<T> {
        self.debug_compare(other);
        self.xor_set(other)
    }
}

impl<'a, T> From<&'a [(usize, T)]> for UMap<T>
where
    T: Clone + PartialEq,
{
    fn from(slice: &'a [(usize, T)]) -> Self {
        UMap::from_slice(slice)
    }
}

impl<T> From<Vec<(usize, T)>> for UMap<T>
where
    T: Clone + PartialEq,
{
    fn from(vec: Vec<(usize, T)>) -> Self {
        UMap::from_slice(&vec)
    }
}

impl<T> Into<Vec<(usize, T)>> for UMap<T>
where
    T: Clone + PartialEq,
{
    fn into(self) -> Vec<(usize, T)> {
        self.iter()
            .map(|(key, value)| (key, value.clone()))
            .collect()
    }
}

impl<T> fmt::Debug for UMap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UMap(").unwrap();
        for item in &self.vec {
            if let Some(entry) = item {
                write!(f, "{:?}", entry).unwrap();
            }
        }
        write!(f, ")").unwrap();
        Ok(())
    }
}
