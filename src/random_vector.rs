extern crate rand;

use rand::prelude::*;

pub struct RandomVector<T>{
    pub vector: Vec<T>,
}

impl<T> RandomVector<T>{
    pub fn new() -> RandomVector<T>
    {
        RandomVector {
            vector: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        item: T
    )
    {
        // push the item to the end of the vector
        self.vector.push(item);

        // swap last item with random item
        let max_index =  self.vector.len() - 1;
        if max_index != 0 {
            self.vector.swap(thread_rng().gen_range(0, max_index), max_index);
        }
    }
}