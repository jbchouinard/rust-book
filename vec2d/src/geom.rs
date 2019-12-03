pub trait Area {
    type Output;

    fn area(self) -> Self::Output;
}
