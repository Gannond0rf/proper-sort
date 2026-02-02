# Proper Sort

Provides natural sorting for strings containing numbers or sizes. Eg: S, M, L, XL, Extra Large etc.
Useful for sorting product data for a web store or other line of business apps.
Case is also considered when sorting. Eg: A, B, a would be sorted as A, a, B (this feature will only work correctly with ascii data).

## Example

```rust
let mut data = vec![
    "T-Shirt L Black",
    "T-Shirt XS Black",
    "T-Shirt Extra Large Black",
    "T-Shirt Medium Black",
    "Crank 180mm Blue",
    "Crank 172.5mm Blue",
    "Crank 175mm Blue",
    "Crank 170mm Blue",
];

data.sort_by(|a, b| proper_sort::compare(a, b));

assert_eq!(data, vec![
    "Crank 170mm Blue",
    "Crank 172.5mm Blue",
    "Crank 175mm Blue",
    "Crank 180mm Blue",
    "T-Shirt XS Black",
    "T-Shirt Medium Black",
    "T-Shirt L Black",
    "T-Shirt Extra Large Black",
]);
```