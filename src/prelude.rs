use crate::lhk::KindF;
pub trait Effect<T, F>
where
    F: KindF<T>,
{
    type F: KindF<T>;
}

// TODO: read `HandlerFunction`
