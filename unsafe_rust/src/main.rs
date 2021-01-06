use std::slice;

fn main() {
    {
        // DEREFERENCING A RAW POINTER

        let mut num = 5;

        //immutable raw pointer from reference
        let r1 = &num as *const i32;
        // mutable raw pointer from reference
        let r2 = &mut num as *mut i32;

        // Creating a raw pointer to an arbitrary location in memory.
        // who knows what is there or not there: it is a bad idea
        let address = 0x012345usize;
        let r = address as *const i32;

        // dereferencing raw pointers and reading the data they point to requires an unsafe block
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    {
        // CALLING UNSAFE FUNCTIONS OR METHODS
        unsafe fn dangerous() {}

        unsafe { dangerous() }
    }

    // CREATING SAFE ABSTRACTIONS OVER UNSAFE CODE

    {
        //HOW TO USE split_at_mut
        let mut r = vec![1, 2, 3, 4, 5, 6];

        let v_ref = &mut r[..];

        let (a, b) = v_ref.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    {
        // WILL NOT COMPILE BECAUSE SECOND MUTABLE BORROW OCCURS
        // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        //     let len = slice.len();

        //     assert!(mid <= len);

        //     (&mut slice[..mid], &mut slice[mid..])
        // }

        //WHY THE FOLLOWING IS OK
        //...By looking at the code and by adding the assertion that mid must be less than or equal to len, we can tell that all the raw pointers used within the unsafe block will be valid pointers to data within the slice. This is an acceptable and appropriate use of unsafe.
        //...
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
    }

    {
        // UNSAFE "from_raw_parts_mut" usage that will crash
        // this leads to a "segmentation fault"

        let new_address = 0x01234usize;
        let r_new_address = new_address as *mut i32;

        let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r_new_address, 10000) };

        // println!("The crashing slice: {:?}", slice);
    }

    {
        // USING "EXTERN"
        // Creates a "Foreign Function Interface" (FFI)
        // The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI.
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }

        // CALLING RUST FROM ANOTHER LANGUAGE
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!")
        }
    }

    {
        //GLOBAL AKA STATIC VARIABLES

        static HELLO_WORLD: &str = "Hello, world!";

        println!("static var = {}", HELLO_WORLD);

        // Static variables have a fixed address in memory.
        // accessing immutable static variables is safe
        // whereas accessing MUTABLE static variables is unsafe

        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER : {}", COUNTER);
        }
    }

    {
        // OTHER "unsafe" usages
        // "unsafe impl" - thus needing to uphold the invariants that the compiler can't verify
        // "union" - when interfacing with "C" code.
    }
}
