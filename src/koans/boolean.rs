// Booleans can have two values, true or false.
// Two equal values will return true when compared with the == operator
#[test]
fn truth() {
    assert_that!(true).is_true();
}

// Likewise, two unequal values will return false when compared with ==
// The != operator can be used to return true for an inequality
#[test]
fn falsehood() {
    assert_that!(false != true).is_true();
    assert_that!(false).is_false();
}

// Strings can also be compared and will return a boolean
#[test]
fn string_equality() {
    assert_that!("Stuff" == "Stuff").is_true();
    assert_that!("Stuff").is_equal_to("Stuff");
}

// Integers can be compared as long as they are of the same type
#[test]
fn int_equality() {
    let num: i8 = 5;
    assert_that!(num == 5).is_true();
}
