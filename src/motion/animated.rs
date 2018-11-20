use core::*;

pub trait Animated {
    fn positions(&self, time: usize) -> Refs<Vec<Point>>;

    fn colors(&self, time: usize) -> Refs<Vec<Color>>;

    fn indices(&self) -> Vec<u16>;
    //todo: indices(&self, time)

    fn size(&self) -> u16;
}