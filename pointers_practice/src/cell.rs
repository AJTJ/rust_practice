use std::cell::UnsafeCell;

// The cell type allows you to modify a value through a shared reference because no other threads have a reference to it, so you can't have multiple concurrent modifications, and because you've never given out a reference INTO the value you store and therefore you can replace it without any problems.
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}
unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
    // pub fn get(&self) -> &T {
    //     unsafe { &*self.value.get() }
    // }
}

#[cfg(test)]
mod test {
    use super::Cell;
    use std::sync::Arc;
    use std::thread;

    // does not work because we are trying to mutate the cell from two different threads
    #[test]
    fn bad() {
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        let x2 = Arc::clone(&x);
        thread::spawn(move || x1.set(43));
        thread::spawn(move || x2.set(44));
    }

    #[test]
    fn bad2() {
        let x = Cell::new(String::from("hello"));
        // the "first" variable would normally NOT be valid because Cell will NEVER give out a reference, it only ever gives out a copy. Otherwise first becomes invalidated after we call set on the cell.
        // with Cell, it only ever returns a COPY
        let first = x.get();
        x.set(String::new());
        x.set(String::from("world"));
        eprintln!("{}", first);
    }
}
