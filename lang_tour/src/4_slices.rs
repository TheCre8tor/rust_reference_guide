//! slices.rs

pub fn run() {
    /* NOTE: Slices are a generic way to get a view into a collection type.

    Most use cases are to get a read only access to a certain range of items
    in a collection type.

    A slice is basically a pointer or a reference that points to a continuous
    range in an existing collection type that's owned by some other variable.
    Under the hood, slices are fat pointers to existing data somewhere in the
    stack or the heap. By fat pointer, it means that they also have information
    on how many elements they are pointing to, along with the pointer to the
    data.

    Slices are denoted by &[T] , where T is any type. They are quite similar to
    arrays in terms of usage */

    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        // Un-mutably acquired slice --> Read only
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }

    {
        // Mutably acquired slice --> Read & Write
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}