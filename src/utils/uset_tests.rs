#[cfg(test)]
mod uset_tests {
    use utils::uset::*;

    use std::collections::HashSet;

    use quickcheck::TestResult;
    use spectral::prelude::*;

    #[test]
    fn set_from_and_to_vec() {
        let v = vec![0, 3, 8, 10];
        let s: USet = USet::from_vec(&v);

        assert_that(&(s.len()))
            .named(&"USet length")
            .is_equal_to(&4);
        assert_that(&(s.capacity()))
            .named(&"USet capacity")
            .is_equal_to(&11);

        assert_that(&(s.contains(0))).is_true();
        assert_that(&(s.contains(3))).is_true();
        assert_that(&(s.contains(8))).is_true();
        assert_that(&(s.contains(10))).is_true();
        assert_that(&(s.contains(9))).is_false();

        let v2 = s.to_vec();

        assert_that!(&v2).is_equal_to(&v);
    }

    fn to_unique_sorted_vec(v: &Vec<usize>) -> Vec<usize> {
        let mut hs = HashSet::new();
        for x in v {
            hs.insert(*x);
        }

        let mut v2: Vec<usize> = hs.into_iter().collect();
        v2.sort();
        v2
    }

    fn vec_compare(va: &[usize], vb: &[usize]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
            va.iter()
                .zip(vb)
                .all(|(&a, &b)| a == b)
    }

    quickcheck! {
        fn from_and_to_vec(v: Vec<usize>) -> TestResult {
            let unique_v = to_unique_sorted_vec(&v);

            if v.len() != unique_v.len() {
                return TestResult::discard()
            }

            let result = USet::from_vec(&unique_v).to_vec();
            TestResult::from_bool(vec_compare(&unique_v, &result))
        }
    }

    #[test]
    fn should_substract() {
        let s1 = USet::from_vec(&[0, 3, 8, 10]);
        let s2 = USet::from_vec(&[3, 8]);

        let s3 = s1.substract(&s2);

        assert_that(&(s3.len())).is_equal_to(&2);
        assert_that(&(s3.contains(0))).is_true();
        assert_that(&(s3.contains(10))).is_true();

        let s4 = s1.substract(&s2);

        assert_that(&(s4.len())).is_equal_to(&2);
        assert_that(&(s4.contains(0))).is_true();
        assert_that(&(s4.contains(10))).is_true();
    }

    #[test]
    fn should_compile() {
        let s4 = vec![0usize, 3, 8, 10];
        for _i in 1..10 {
            let _s5: USet = USet::from(&s4);
        }
    }

    #[test]
    fn should_be_equal() {
        let _s1 = uset![0, 3, 8, 10];
    }

}
