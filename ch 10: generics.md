https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

# Generics

check the `generics` file

- rust performs _Monomorphization_ on generic code.

# Traits

- Like `interfaces` in other languages.

## coherance
- We cannot implement **external** traits on **external** types.
  - This is known as the `orphan rune`
    - One or the other MUST be local to our crate.
    - This ensures other people's code cannot break our code and vice versa.