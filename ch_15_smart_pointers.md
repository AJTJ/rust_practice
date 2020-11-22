file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch15-03-drop.html

# Files
`box_practice`
# Smart Pointers

## Notes
- Both `String` and `Vec<T>` are smart pointers. Because they own some memory and allow you to manipulate it.
- Smart pointers are usually implemented using `structs` but they also implement the `Deref` and `Drop` traits.

## Box
- `Box<>` is a pointer that provides `indirection` by placing data on the `heap` rather than the `stack`.

## Deref trait
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

