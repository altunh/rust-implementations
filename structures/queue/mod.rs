pub enum QueueError {
    EmptyQueue,
    FullQueue,
}

pub trait Queue<E> {
    fn add(&mut self, element: E) -> Result<(), QueueError>;

    fn remove(&mut self) -> Result<E, QueueError>;

    fn peek(&self) -> Result<Option<&E>, QueueError>;
}
