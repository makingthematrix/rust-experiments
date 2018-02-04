#[cfg(test)]
mod cities_tests {
    use cities::*;

    use spectral::prelude::*;

    #[test]
    fn should_generate_unsorted_array() {
        let v = gen_unshuffled(20, 3, 0.0, 3);
        assert_that!(v).has_length(20);
        assert_that!(v[0]).is_equal_to(0);
        assert_that(v.iter().max().unwrap()).is_less_than(20);
    }

    #[test]
    fn should_generate_city_array() {
        let v = gen_cities(20, 3, 0.0, 3);
        assert_that!(v).has_length(20);
        assert_that(v.iter().max().unwrap()).is_less_than(20);
    }
}
