# enum-as-inner

A deriving proc-macro for generating functions to automatically give access to the inner members of enum.

## Basic unnamed field case

The basic case is meant for single item enums, like:

```rust
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum OneEnum {
    One(u32),
}
```

where the inner item can be retrieved with the `as_*()` or with the `into_*()` functions:

```rust
let one = OneEnum::One(1);

assert_eq!(*one.as_one().unwrap(), 1);
assert_eq!(one.into_one().unwrap(), 1);
```

where the result is either a reference for inner items or a tuple containing the inner items.

## Unit case

This will return copy's of the value of the unit variant, as `isize`:

```rust
use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
enum UnitVariants {
    Zero,
    One,
    Two,
}
```

These are not references:

```rust
let unit = UnitVariants::Two;

assert_eq!(unit.as_two().unwrap(), ());
```

Note that for unit enums there is no `into_*()` function generated.

## Mutliple, unnamed field case

This will return a tuple of the inner types:

```rust
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One(u32),
    Two(u32, i32),
    Three(bool, u32, i64),
}
```

And can be accessed like:

```rust
let many = ManyVariants::Three(true, 1, 2);

assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
```

## Multiple, named field case

This will return a tuple of the inner types, like the unnamed option:

```rust
use enum_as_inner::EnumAsInner;

#[derive(Debug, EnumAsInner)]
enum ManyVariants {
    One{ one: u32 },
    Two{ one: u32, two: i32 },
    Three{ one: bool, two: u32, three: i64 },
}
```

And can be accessed like:

```rust
let many = ManyVariants::Three{ one: true, two: 1, three: 2 };

assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
```