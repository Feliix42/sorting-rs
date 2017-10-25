pub trait Panicsort {
    fn panicsort(&self);
}


impl<T: PartialOrd> Panicsort for Vec<T> {
    fn panicsort(&self) {
        for i in 1..self.len() {
            if !self[i].ge(&self[i - 1]) {
                panic!("Vector is not sorted!");
            }
        }
    }
}
