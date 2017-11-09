extern crate rand;

pub trait Bogosort {
    fn bogosort(&mut self);
}

fn is_sorted<T: PartialOrd>(vec: &Vec<T>) -> bool {
    for i in 1..vec.len() {
        if !vec[i].ge(&vec[i - 1]) {
            return false;
        }
    }

    true
}

impl<T: PartialOrd> Bogosort for Vec<T> {
    fn bogosort(&mut self) {
        while !is_sorted(self) {
            let mut new_vec: Vec<T> = Vec::new();
            while self.len() > 0 {
                let length = self.len();
                new_vec.push(self.remove(rand::random::<usize>() % length));
            }
            self.append(&mut new_vec);
        }
    }
}
