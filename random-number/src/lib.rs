/*!
# Random Number

Generate random numbers quickly.

### The `random!` Marco

```rust
#[macro_use] extern crate random_number;

let n: u8 = random!();
println!("{}", n); // 0 ~ 255

let n: f64 = random!();
println!("{}", n); // 0.0 ~ 1.0

let n: u8 = random!(..=10);
println!("{}", n); // 0 ~ 10

let n: u8 = random!(..=9);
println!("{}", n); // 0 ~ 9

let n: u8 = random!(10..);
println!("{}", n); // 10 ~ 255

let n: i8 = random!(-2..=12);
println!("{}", n); // -2 ~ 12

let n: u8 = random!(12, 20);
println!("{}", n); // 12 ~ 20

let n: u8 = random!(20, 12);
println!("{}", n); // 12 ~ 20
```

The random number generator can be reused by adding it to the `random!` macro as the last argument.

```rust
#[macro_use] extern crate random_number;

let mut rng = random_number::rand::thread_rng();

let n: u8 = random!(rng);
println!("{}", n); // 0 ~ 255

let n: u8 = random!(..=10, rng);
println!("{}", n); // 0 ~ 10

let n: u8 = random!(20, 12, rng);
println!("{}", n); // 12 ~ 20
```

### The `random_ranged` Function

If the **range** is not literal, for example, a variable, `var_range`, storing an instance that implements the `RangeBounds` trait, the `var_range` variable cannot be used in the `random!` macro.

```rust,ignore
#[macro_use] extern crate random_number;

let var_range = 1..=10;

let n: u8 = random!(var_range); // compile error
```

In this case, use the `random_ranged` function instead.

```rust
extern crate random_number;

let var_range = 1..=10;

let n: u8 = random_number::random_ranged(var_range);
println!("{}", n); // 1 ~ 10
```
*/
#![no_std]
pub extern crate rand;

extern crate random_number_macro_impl;

#[macro_use]
extern crate proc_macro_hack;

mod bounded;
mod functions;
mod macros;

pub use bounded::Bounded;
pub use functions::*;

/**
Generate a random number or random numbers.

## Examples

```rust
extern crate random_number;

let f: f64 = random_number::random!();

assert!(0.0 <= f && f <= 1.0);

let i: u8 = random_number::random!(..);

assert!(0 <= i && i <= 255);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let f: f64 = random_number::random!(thread_rng);

assert!(0.0 <= f && f <= 1.0);

let i: u8 = random_number::random!(.., thread_rng);

assert!(0 <= i && i <= 255);
```

```rust
extern crate random_number;

let i: u8 = random_number::random!(..=10);

assert!(0 <= i && i <= 10);

let i: i8 = random_number::random!(..=0);

assert!(-128 <= i && i <= 0);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let i: u8 = random_number::random!(..=10, thread_rng);

assert!(0 <= i && i <= 10);

let i: i8 = random_number::random!(..=0, thread_rng);

assert!(-128 <= i && i <= 0);
```

```rust
extern crate random_number;

let i: u8 = random_number::random!(10..);

assert!(10 <= i && i <= 255);

let i: i8 = random_number::random!(0..);

assert!(0 <= i && i <= 127);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let i: u8 = random_number::random!(10.., thread_rng);

assert!(10 <= i && i <= 255);

let i: i8 = random_number::random!(0.., thread_rng);

assert!(0 <= i && i <= 127);
```

```rust
extern crate random_number;

let i: u8 = random_number::random!(..10);

assert!(0 <= i && i <= 9);

let i: i8 = random_number::random!(..0);

assert!(-128 <= i && i <= -1);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let i: u8 = random_number::random!(..10, thread_rng);

assert!(0 <= i && i <= 9);

let i: i8 = random_number::random!(..0, thread_rng);

assert!(-128 <= i && i <= -1);
```

```rust
extern crate random_number;

let i: u8 = random_number::random!(1..10);

assert!(1 <= i && i <= 10);

let i: i8 = random_number::random!(-2..=12);

assert!(-2 <= i && i <= 12);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let i: u8 = random_number::random!(1..10, thread_rng);

assert!(1 <= i && i <= 10);

let i: i8 = random_number::random!(-2..=12, thread_rng);

assert!(-2 <= i && i <= 12);
```

```rust
extern crate random_number;

let i: u8 = random_number::random!(12, 20);

assert!(12 <= i && i <= 20);

let i: i8 = random_number::random!(20, 12);

assert!(12 <= i && i <= 20);
```

```rust
extern crate random_number;

use random_number::rand;

let mut thread_rng = rand::thread_rng();

let i: u8 = random_number::random!(12, 20, thread_rng);

assert!(12 <= i && i <= 20);

let i: i8 = random_number::random!(20, 12, thread_rng);

assert!(12 <= i && i <= 20);
```
*/
#[proc_macro_hack]
pub use random_number_macro_impl::random;
