use crate::WebState;

pub trait WebStateAction<S: WebState> {
    fn apply(&self, state: &mut S);
}
