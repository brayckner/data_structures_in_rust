use data_structures::array::DynamicArray;

mod data_structures {
    pub mod array;
    pub mod linked_list;
    pub mod stack;
    pub mod queue;
    pub mod hash_map;
    pub mod binary_tree;
}

mod algorithms {
    pub mod searching;
    pub mod sorting;
    pub mod graph;
    pub mod dynamic_programming;
}

fn main() {
    println!("Testing Array");
    let mut myArray = DynamicArray::new();
    let termToSearch = "Andrew";

    // Inserting Items
    println!("========== INSERTING ==========");
    myArray.insert("Bray");
    myArray.insert("Mya");
    myArray.insert("Jack");
    myArray.insert("Andrew");

    // Size of Array
    println!("========== Size ==========");
    myArray.size();

    // Transversing Array 
    println!("========== Transverse ==========");
    myArray.transverse();

    println!("========== INSERTING at Index ==========");
    myArray.insert_at("Ryan", 2);

    println!("========== New Size and Transverse ==========");
    myArray.size();
    myArray.transverse();

    println!("========== Searching ==========");
    let search_result = myArray.search(&termToSearch);
    println!("Search Result: {:?}", search_result);

    println!("========== Deleting MYA ==========");
    myArray.remove_at(1);
    
    println!("========== Final Transverse ==========");
    myArray.transverse();
}
