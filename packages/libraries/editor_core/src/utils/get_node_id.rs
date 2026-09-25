use std::cell::RefCell;

thread_local! {
    static NEXT_NODE_ID: RefCell<usize> = const { RefCell::new(0) };
}

pub fn get_node_id() -> usize {
    NEXT_NODE_ID.with_borrow_mut(|next_node_id| {
        let id = *next_node_id;
        *next_node_id += 1;
        id
    })
}
