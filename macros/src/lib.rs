// use proc_macro;

// DECLARATIVE MACRO
// a simplified versino of the vec! macro definition
#[macro_export]
macro_rules! vec {
  // similar to match expression, but there is only one branch
  ( $( $x:expr),*) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

// PROCEDURAL MACRO
// acts more like function
// accept code as input, operate on that code, produce code as an output
// must reside in their own crate with a special crate type (for technical reasons)

// CUSTOM DERIVE MACRO
// go to `hello_macro` file to see definition
// go to this `main.rs` to see usage
