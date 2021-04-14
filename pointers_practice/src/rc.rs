// WHAT IS RC
// A pointer to some type T stored on the heap
// Stored on the heap
// Not stored on the stack because
// Rc is a single-threaded reference counting pointer
// disallows mutation by default
// if u need mutability put a Cell/RefCell inside the Rc

// SIMILAR TO REFCELL
// because it keeps count of references
// BUT never provides mutability

// WHAT IT DOES
// Allows you to have multiple shared references to a thing and deallocates when it goes away
// Usefor for: data structures where one element is present in multiple places, such as databases.

// WHAT IT DOES NOT DO
// Not thread-safe
// therefore not safe to send to different threads (it is not Send)

// WEAK VS STRONG POINTERS
// Strong pointers will deallocate memory when they all disappear
// Weak pointers will not deallocate memory when they are gone

// OTHER NOTES
// Mutable reference: guarantees that nothing else is trying to modify it
// Mutable pointer: only carries semantics to call itself something, there are no implications or guarantees

// SYNCHRONOUS
// since rc is not thread-safe, there is ARC
// ARC is basically the same thing as RC but it uses thread-safe operations for managing the reference count
// it is both Send and Sync

use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    // NonNull is !Send by default
    inner: NonNull<RcInner<T>>,
    // WHY PHANTOMDATA
    // Tells rust that when you drop an Rc, that an RcInner<T> might be dropped and that it needs to be checked.
    // without this the Rc would be broken
    // this is only needed when T is not static, but we would like this to work with all types
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        Rc {
            // SAFETY: Box does not give us a null pointer.
            // it gives us a heap allocation
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    // cloning the reference count but not cloning the actual value
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // dropping inner before because it references what is inside the box EVEN THOUGH inner is not actually dropped until the end of this scope.
            drop(inner);
            // SAFETY: we are the ONLY Rc left, and we are being dropped. Therefore, after us, there will be no Rc's and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            //there are other Rcs, so don't drop the Box!
            inner.refcount.set(c - 1)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn bad() {
//         let (y, x);
//         x = String::from("foo");
//         y = Rc::new(&x);
//     }
// }
