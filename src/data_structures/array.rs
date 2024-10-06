// Array implementation using a Vector

pub struct DynamicArray<T> {
    pub data: Vec<T>,
}

impl <T> DynamicArray<T> where T: std::fmt::Debug + PartialEq + Clone, {

    // Create a new Array
    pub fn new() -> Self {
        DynamicArray {
            data: Vec::new(),
        }
    }

    // Insert an element to the end of an array
    pub fn insert(&mut self, element: T){
        self.data.push(element);
    }

    // Insert an element at a specific index 
    pub fn insert_at(&mut self, element: T, index: usize) -> Result<(), String> {
        if index > self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        self.data.insert(index, element);
        Ok(())
    }

    // Remove element at a specific index
    pub fn remove_at(&mut self, index: usize) -> Result<T, String> {
        if index > self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        Ok(self.data.remove(index))
    }

    // Search for an element in the array 
    pub fn search(&self, element: &T) -> Option<usize> {
        self.data.iter().position(|x| x == element)
    }

    // Transverse the array (print all elements)
    pub fn transverse(&self) {
        for (index, element) in self.data.iter().enumerate() {
            println!("Index: {}, Value: {:?}", index, element);
        }
    }

    // Get the size of the array
    pub fn size(&self) -> usize {
        self.data.len()
    }

}

// Add test using the Rust Test framework. 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(1);
        array.insert(6);
        assert_eq!(array.data, vec![8, 1, 6]);
    }

    #[test]
    fn test_insert_at() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(6);
        let result = array.insert_at(1, 1);
        assert!(result.is_ok());
        assert_eq!(array.data, vec![8, 1, 6]);
    }

    #[test]
    fn test_remove_at() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(1);
        array.insert(6);
        let result = array.remove_at(1);
        assert!(result.is_ok());
        assert_eq!(array.data, vec![8, 6]);
    }

    #[test]
    fn test_search() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(1);
        array.insert(6);
        assert_eq!(array.search(&6), Some(2));
        assert_eq!(array.search(&100), None);
    }

    #[test]
    fn test_transverse() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(1);
        array.insert(6);
        array.transverse();
    }

    #[test]
    fn test_size() {
        let mut array = DynamicArray::new();
        array.insert(8);
        array.insert(1);
        array.insert(6);
        assert_eq!(array.size(), 3);
    }
}