use lexer::Span;

use crate::Tracable;

pub trait CombineHulls {
    fn combine_hulls(&self) -> Option<Span>;
}

impl<T: Tracable> CombineHulls for Vec<T> {
    fn combine_hulls(&self) -> Option<Span> {
        let mut hull = None;
        for tracable in self.iter() {
            match (&mut hull, tracable.trace().hull()) {
                (_, None) => (),
                (None, Some(span)) => {
                    hull = Some(span);
                }
                (Some(left), Some(right)) => hull = Some(left.hull(&right)),
            }
        }
        hull
    }
}

impl<T1: Tracable, T2: Tracable> CombineHulls for (T1, T2) {
    fn combine_hulls(&self) -> Option<Span> {
        let left_hull = self.0.hull();
        let right_hull = self.1.hull();
        match (left_hull, right_hull) {
            (None, None) => None,
            (None, Some(right)) => Some(right),
            (Some(left), None) => Some(left),
            (Some(left), Some(right)) => Some(left.hull(&right)),
        }
    }
}
