
#[derive(Debug, Clone, Hash)]
pub struct UnmovingVec<T>{
    list: Vec<T>,
    whats_free: Vec<bool>,
    /// This is a stack of free indices. When an item is removed, it's index is pushed onto this stack.
    free_stack: Vec<usize>,
}

impl <T>Default for UnmovingVec<T>{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UnmovingVec<T>{
    pub fn new() -> UnmovingVec<T> {
        UnmovingVec {
            list: vec![],
            whats_free: vec![],
            free_stack: vec![]
        }
    }

    pub fn get(&self, index: usize) -> Option<&T>{
        if !self.whats_free[index]{
            self.list.get(index)
        }else{
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T>{
        if !self.whats_free[index]{
            self.list.get_mut(index)
        }else{
            None
        }
    }
    pub fn remove(&mut self, index: usize) -> &T {
        assert!(!self.whats_free[index], "Tried to remove item that was already removed.");
        self.whats_free[index] = true;
        self.free_stack.push(index);
        &self.list[index]
    }
    pub fn capacity(&self) -> usize{
        return self.list.len();
    }

    pub fn push(&mut self, item: T) -> usize {
        match self.free_stack.pop(){
            Some(index) => {
                self.whats_free[index] = false;
                self.list[index] = item;
                index
            },
            None => {
                let index = self.list.len();
                self.whats_free.push(false);
                self.list.push(item);
                index
            }
        }
    }
}