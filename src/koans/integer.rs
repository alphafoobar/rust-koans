// As the name implies, unsigned integers (u8, u16, u32, u64) cannot be negative
#[test]
fn unsigned_ints() {
    assert_that!(u8::MIN).is_equal_to(0);
}

// Unsigned integers can be reduced only as far as their minimum value of 0
#[test]
fn sub_unsigned_int() {
    let mut num: u8 = 10;
    num -= 10;
    assert_that!(num).is_equal_to(u8::MIN);
}

// Signed integers(i8, i16, i32, i64), on the other hand, can be negative
#[test]
fn signed_ints() {
    assert_that!(i8::MIN).is_less_than(0);
}

// Signed integers can be reduced below zero, as far as their minimum value.
// hint: The maximum magnitude for a negative integer is greater than
// that of a positive integer
#[test]
fn sub_signed_int() {
    let mut num: i8 = 0;
    let negative: i8 = -128;
    num += negative;
    assert_that!(num).is_equal_to(i8::MIN);
}

// Addition of positive integers works much the same for signed and unsigned numbers
#[test]
fn add_numbers() {
    let mut sig: i8 = 0;
    let mut unsig: u8 = 0;
    sig += i8::MAX;
    unsig += u8::MAX;
    assert_that!(sig).is_equal_to(i8::MAX);
    assert_that!(unsig).is_equal_to(u8::MAX);
}

// Like any variable in Rust, integers are immutable unless declared otherwise
#[test]
fn mutating_ints() {
    let mut num: i8 = 1;
    num += 2;
    assert_that!(num).is_equal_to(3);
}

// While regular immutable variables cannot be changed, mutable versions of them can be
#[test]
fn referencing_values() {
    let num: i8 = 1;
    let mut mut_num = num;
    mut_num += 1;
    assert_that!(num).is_not_equal_to(mut_num);
}
