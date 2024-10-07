// Array implementation using a Vector

pub struct DynamicArray<T> {
    pub data: Vec<T>,
}

impl <T> DynamicArray<T> where T: std::fmt::Debug + PartialEq + Clone, {

    pub fn new() -> Self {
        DynamicArray {
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, element: T){
        self.data.push(element);
    }

    pub fn insert_at(&mut self, element: T, index: usize) -> Result<(), String> {
        if index > self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        self.data.insert(index, element);
        Ok(())
    }

    pub fn remove_at(&mut self, index: usize) -> Result<T, String> {
        if index > self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        Ok(self.data.remove(index))
    }

    pub fn search(&self, element: &T) -> Option<usize> {
        self.data.iter().position(|x| x == element)
    }

    pub fn transverse(&self) {
        for (index, element) in self.data.iter().enumerate() {
            println!("Index: {}, Value: {:?}", index, element);
        }
    }

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
        array.insert(10);
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