
pub mod folding {
    pub fn fold1<I, B, F>(mut ls: I, f: F) -> B
    where
        I: Iterator<Item = B>,
        F: FnMut(B, B) -> B,
    {
        let head = ls.next()
            .expect("Can't fold1 over an empty list");
        ls.fold(head, f)
    }    
}
