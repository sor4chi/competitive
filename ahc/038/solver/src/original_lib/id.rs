#[derive(Clone)]
pub struct IncrementalIDGenerator<T> {
    current: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> IncrementalIDGenerator<T> {
    pub fn new() -> Self {
        IncrementalIDGenerator {
            current: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_start(start: usize) -> Self {
        IncrementalIDGenerator {
            current: start,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn generate(&mut self) -> T
    where
        T: From<usize>,
    {
        let id = self.current;
        self.current += 1;
        T::from(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generator() {
        let mut id_gen = IncrementalIDGenerator::<usize>::new();
        assert_eq!(id_gen.generate(), 0);
        assert_eq!(id_gen.generate(), 1);
        assert_eq!(id_gen.generate(), 2);
        assert_eq!(id_gen.generate(), 3);
    }
}
