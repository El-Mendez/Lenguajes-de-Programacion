pub trait Visitable<T> {
    fn accept(&self, visitor: &mut impl Visitor<Self, T>) where Self: Sized {
        visitor.visit(self);
    }
}


pub trait Visitor<T, E = ()> where T: Visitable<E> + Sized {
    fn visit(&mut self, visitable: &T) -> E;
}
