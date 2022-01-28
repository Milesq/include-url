# include-url

This crate provides `include_url` macro, which works like `include_str` from `std` with 2 differences
- Data is taken from URL, not local files
- The data from given URL is treated as a Rust code, rather than raw data


## Examples
```rs
use include_url::include_raw_url;

include_raw_url!("https://raw.githubusercontent.com/Milesq/include-url/main/assets/add.rs");

fn main() {
  println!("{}", add(2, 3)); // prints 5
}
```
