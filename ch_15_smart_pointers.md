file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch15-06-reference-cycles.html

# Files
`box_practice`
`drop_trait`
`interior_mutability`
`Rc_and_RefCell`
`tree_data`
# Smart Pointers

## Notes
- Both `String` and `Vec<T>` are smart pointers. Because they own some memory and allow you to manipulate it.
- Smart pointers are usually implemented using `structs` but they also implement the `Deref` and `Drop` traits.

### Box
- `Box<>` is a pointer that provides `indirection` by placing data on the `heap` rather than the `stack`.
- "In computer programming, indirection (also called dereferencing) is the ability to reference something using a name, reference, or container instead of the value itself. The most common form of indirection is the act of manipulating a value through its memory address."

### Deref trait
- see `box_practice`
- the `*` operator calls the `deref` method
- it still returns a *reference* to a value because we do not want to move the value out `self`

### Deref coercion
- Ex, converting `&String` to `&str` implicitly
- Saves a lot of code writing and has no runtime penalty.

### Deref coercion with mutability
- Three cases where deref coercion exists:
```
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
```
- The first and second cases are the same (except for mutability)
- The third case defines that you can coerce from a mutable to an immutable reference, but the reverse is *not* allowed, since you can only have one mutable reference, but you can have many immutable references.

### Drop Trait
- See `drop_trait`
- A Trait that allows you to customize what happens when a value is about to go out of scope.
- called automatically
- the `drop()` function (included in prelude) can be used to manually drop.

### Reference Counted Smart Pointer
- `Rc<T>`
- Basically used to count the amount of references to a value, thus determining if the value is still in use or not.
- Only useful for single-thread scenarios
- calling `Rc::clone(&a)` (unlike calling `a.clone()`) does not make a deep copy of all the data. It only increases the reference count, which is much more efficient.

## `RefCell<T>`
- `interior_mutability` file.
  - allows you to mutate data even when there area immutable references to that data.
  - Only shows errors at runtime, which can inhibit the development process

### When to use the various tools
- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

## Using `Rc<T>` and `RefCell<T>>` together
- `rc_and_refcell`
- Multiple owners for the same data that can also be mutable.

## Creating and avoiding Reference Cycle memory leaks
- `reference_cycles`