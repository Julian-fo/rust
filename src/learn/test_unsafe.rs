#[cfg(test)]
mod tests {
    use std::vec;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn unsafe_define() {
        let mut x = 42;
        let ptr_x = &mut x as *mut i32; // *mut i32 is a raw pointer to a mutable i32, as: turns a reference into a raw pointer
        let const_ptr_x = &x as *const i32; // *const i32 is a raw pointer to an immutable i32, as: turns a reference into a raw pointer
        unsafe {
            *ptr_x = 13;
            assert_eq!(13, *const_ptr_x);
        }
    }

    #[test]
    fn unsafe_offset() {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr();
        let offset = 2;

        println!("first element: {}", unsafe { *ptr }); // unsafe block is required to dereference a raw pointer

        // ptr.offset: offset is used to move the raw pointer to the next element
        println!("second element: {}", unsafe { *ptr.offset(1) }); 
        unsafe {
            let value = *ptr.offset(offset);
            assert_eq!(3, value);
        }
    }

    #[test]
    // This test demonstrates how to use unsafe code to allocate and deallocate memory
    // The alloc and dealloc functions are used to allocate and deallocate memory
    // Layout is used to specify the size and alignment of the memory block
    fn unsafe_with_alloc() {
        // alloc and dealloc are used to allocate and deallocate memory
        use std::alloc::{alloc, dealloc, Layout};

        unsafe {
            // Layout is used to specify the size and alignment of the memory block
            // new is a constructor for Layout
            // ::<i32> is a type parameter for new, turbofish syntax is used to specify the type parameter
            let layout = Layout::new::<i32>();

            // alloc returns a raw pointer to the allocated memory
            let ptr = alloc(layout) as *mut i32;

            if ptr.is_null() {
                panic!("failed to allocate memory");
            }

            // dereference the raw pointer to write to the allocated memory
            *ptr = 42;
            assert_eq!(42, *ptr);

            // dealloc is used to deallocate memory
            // dealloc takes a raw pointer to the allocated memory and the Layout used to allocate the memory
            // dealloc doesn't deallocate the memory, it only marks the memory as available for reuse
            // The memory is not zeroed out
            // ptr: *mut u8 is a raw pointer to a mutable u8, why is it not *mut i32? 
            // the reason is that dealloc takes a raw pointer to the allocated memory, which is a raw pointer to a mutable u8
            dealloc(ptr as *mut u8, layout);
        }
    }

    #[test]
    fn split_vector_with_error() {
        let mut v = vec![1, 2, 3, 4, 5];

        let (left, right) = unsafe_split_at_mut(&mut v, 3);
        assert!(left == [1, 2, 3]);
        assert!(right == [4, 5]);
    }


    // why is this function unsafe?
    // The function is unsafe because it uses raw pointers
    // The function uses raw pointers to create mutable slices from a mutable vector
    // The function panics if mid is greater than the length of the vector
    // The function uses std::slice::from_raw_parts_mut to create mutable slices from raw pointers
    fn unsafe_split_at_mut(v: &mut Vec<i32>, mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = v.len();

        // as_mut_ptr returns a raw pointer to the first element of the vector
        // as_mut_ptr is used to create raw pointers from mutable references
        let ptr = v.as_mut_ptr();

        assert!(mid <= len);

        // split_at_mut panics if mid is greater than the length of the vector
        // split_at_mut returns a tuple of mutable slices
        // The first slice contains the first mid elements
        // The second slice contains the remaining elements
        // error[E0499]: cannot borrow `*v` as mutable more than once at a time
        // (&mut v[..mid], &mut v[mid..])

        unsafe {
            (
                // from_raw_parts_mut is used to create a mutable slice from a raw pointer and a length
                // params: raw pointer, length
                // returns: mutable slice
                std::slice::from_raw_parts_mut(ptr, mid),
                // ptr.add: add is used to move the raw pointer to the next element
                // what's the different with ptr.offset ?
                // add returns a raw pointer to the element at the specified index
                // add doesn't check for overflow
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}