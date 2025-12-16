
pub struct Filterer<T> {
    conditon: Box<dyn FnMut(&T) -> bool>
}

impl <T> Filterer<T> {
    fn new<F> (f: F) -> Self 
    where F: FnMut(&T) -> bool + 'static
    {
        Self { conditon: Box::new(f) }
    }

    fn apply <'a>(&'a mut self, iter: impl Iterator <Item=&'a T>+'a) -> impl Iterator <Item=&'a T> + 'a {
        iter.filter(move |value| (self.conditon)(value))
    }
}

// let mut filterer = Filterer::new(|x: &i32| *x > 10);
//
//     // Apply it to an iterator
//     let iter = filterer.apply(data.iter());
