extern crate ods;

use ods::array_stack::*;

fn main() {
    println!("Hello, world!");
}

#[test]
pub fn test_array_stack() {
    {
        let mut stack = ArrayStack::new(3);
        stack.add(0, 1);
        stack.add(1, 2);
        stack.add(2, 3);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.into_vec(), vec![1, 2, 3]);
    }

    {
        let mut stack = ArrayStack::new(3);
        stack.add(0, 1);
        stack.add(1, 2);
        stack.add(2, 3);
        stack.set(1, 100);

        assert_eq!(*stack.get(1).unwrap(), 100);
    }
}

#[test]
pub fn test_fast_array_stack() {
    {
        let mut stack = FastArrayStack::new(3);
        stack.add(0, 1);
        stack.add(1, 2);
        stack.add(2, 3);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.into_vec(), vec![1, 2, 3]);
    }
}
