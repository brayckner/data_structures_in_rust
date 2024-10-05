// Array implementation using a Vector

use std::error::Error;

pub struct ArrayImplementation<T> {
    pub data: Vec<T>,
}

impl <T> ArrayImplementation<T> where T: std::fmt::Debug + PartialEq + Clone, {

    // Create a new Array
    pub fn new() -> Self {
        ArrayImplementation {
            data: Vec::new(),
        }
    }

    // Insert an element to the end of an array
    pub fn insert(&mut self, element: T){
        self.data.push(element);
    }

    // Insert an element at a specific index 
    pub fn insert_at_index(&mut self, element: T, index: usize) -> Result<(), String> {
        if index > self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        self.data.insert(index, element);
        Ok(())
    }

    // Remove element at a specific index
    pub fn remove_at_index(&mut self, index: usize) -> Result<T, String> {
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