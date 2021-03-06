use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One { one: u32 },
    Two { one: u32, two: i32 },
    Three { one: bool, two: u32, three: i64 },
}

#[test]
fn test_one_named() {
    let many = ManyVariants::One { one: 1 };

    assert!(many.as_one().is_some());
    assert!(many.as_two().is_none());
    assert!(many.as_three().is_none());

    assert_eq!(*many.as_one().unwrap(), 1_u32);
}

#[test]
fn test_two_named() {
    let many = ManyVariants::Two { one: 1, two: 2 };

    assert!(many.as_one().is_none());
    assert!(many.as_two().is_some());
    assert!(many.as_three().is_none());

    assert_eq!(many.as_two().unwrap(), (&1_u32, &2_i32));
    assert_eq!(many.into_two().unwrap(), (1_u32, 2_i32));
}

#[test]
fn test_three_named() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert!(many.as_one().is_none());
    assert!(many.as_two().is_none());
    assert!(many.as_three().is_some());

    assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
    assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
}
