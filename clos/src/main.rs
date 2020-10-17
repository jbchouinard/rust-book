struct Memoized<F, A, R>
where
    F: Fn(A) -> R,
{
    func: F,
    value: Option<R>,
}

impl<F, A, R> Memoized<F, A, R>
where
    F: Fn(A) -> R,
{
    pub fn new(func: F) -> Self {
        Self {
            func: func,
            value: None,
        }
    }
    pub fn value(&mut self, arg: A) -> R {
        if self.value.is_none() {
            self.value = self.func(arg);
        }
        self.value
    }
}

fn main() {}
