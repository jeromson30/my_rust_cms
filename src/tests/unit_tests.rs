#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
        assert_eq!(10 * 5, 50);
        assert_eq!(100 / 4, 25);
    }

    #[test]
    fn test_string_operations() {
        let hello = "Hello";
        let world = "World";
        let combined = format!("{}, {}!", hello, world);
        assert_eq!(combined, "Hello, World!");
    }

    #[test]
    fn test_vector_operations() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_option_handling() {
        let some_value = Some(42);
        let none_value: Option<i32> = None;
        
        assert!(some_value.is_some());
        assert!(none_value.is_none());
        assert_eq!(some_value.unwrap_or(0), 42);
        assert_eq!(none_value.unwrap_or(0), 0);
    }

    #[test]
    fn test_result_handling() {
        let success: Result<i32, &str> = Ok(42);
        let error: Result<i32, &str> = Err("Something went wrong");
        
        assert!(success.is_ok());
        assert!(error.is_err());
        assert_eq!(success.unwrap_or(0), 42);
        assert_eq!(error.unwrap_or(0), 0);
    }
}
