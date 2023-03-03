use std::collections::HashSet;

#[derive(Clone)]
pub struct Cupboard<T : Clone>{
    vec: Vec<Shelf<T>>,
    set: HashSet<Shelf<T>>,

}

#[derive(Clone)]
pub struct Shelf<T : Clone>{
    data: T,
    on_loan_box: Option<Box<T>>
}