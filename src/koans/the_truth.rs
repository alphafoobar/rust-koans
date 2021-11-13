use speculoos::assert_that;
use speculoos::prelude::*;

#[test]
fn the_truth() {
    assert_that!(true).is_true()
}
