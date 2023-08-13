fn main() {
    // raw pointers
    {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    // closing unsafe code in safe abstraction
    {
        use std::slice;

        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // accesing or modifying mutable static variables is always unsafe.
    // it's due to inability to guarantee safety in multithreaded scenarios
    {
        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
