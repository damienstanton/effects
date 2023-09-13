use core::marker::PhantomData;

pub struct Kind<T, Op, Pure> {
    pub value: T,
    _f: PhantomData<Op>,
    _a1: PhantomData<Pure>,
}

impl<T, Op, Pure> Kind<T, Op, Pure> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _f: PhantomData,
            _a1: PhantomData,
        }
    }
}

pub trait KindF<T> {
    type Op;
    type Pure;

    fn from_kind(kind: Kind<T, Self::Op, Self::Pure>) -> Self;
    fn as_kind(&self) -> Kind<T, Self::Op, Self::Pure>;
}

impl<T, Op, Pure> KindF<T> for Kind<T, Op, Pure>
where
    T: Copy,
{
    type Op = Box<dyn Fn() -> T>;
    type Pure = T;

    fn from_kind(kind: Kind<T, Self::Op, Self::Pure>) -> Self {
        Self {
            value: kind.value,
            _f: PhantomData,
            _a1: PhantomData,
        }
    }

    fn as_kind(&self) -> Kind<T, Self::Op, Self::Pure> {
        Kind {
            value: self.value,
            _f: PhantomData,
            _a1: PhantomData,
        }
    }
}
