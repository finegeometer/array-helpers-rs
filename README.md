# array-helpers
Basic length-generic operations on arrays.

This crate provides utilities for initializing, consuming, and transforming arrays.
All methods are length-generic.

This crate uses the extremely new `const_generics` feature.
As such, it requires nightly Rust.

```
let arr: [[usize; 3]; 2] = array_helpers::new(|i| array_helpers::new(|j| 3*i + j));
assert_eq!(arr, [[0,1,2],[3,4,5]]);

let arr = arr.transpose();
assert_eq!(arr, [[0,3],[1,4],[2,5]]);

let arr = arr.map(|[a,b]| (a,b));
assert_eq!(arr, [(0,3),(1,4),(2,5)]);

let (arr1, arr2) = arr.unzip();
assert_eq!(arr1, [0,1,2]);
assert_eq!(arr2, [3,4,5]);
```



## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.