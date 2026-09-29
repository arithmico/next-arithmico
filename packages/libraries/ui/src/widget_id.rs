use std::cell::RefCell;

thread_local! {
    static NEXT_WIDGET_ID: RefCell<usize> = const { RefCell::new(0) };
}

pub fn use_widget_id() -> usize {
    NEXT_WIDGET_ID.with_borrow_mut(|id| {
        let return_id = *id;
        *id += 1;
        return_id
    })
}
