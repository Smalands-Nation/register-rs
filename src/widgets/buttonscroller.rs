use {
    super::{Clickable, Widget},
    iced::Element,
};
/*
 * on_back
 * on_foward
 * elements per view
 *
 *
 * set of elements
 */

pub struct ButtonScroller<M, E: Widget<M>> {
    fwd: Option<M>,
    back: Option<M>,
    inner: Vec<E>,
    view_size: usize,
}

impl<M, E> ButtonScroller<M, E>
where
    M: Clone,
{
    pub fn new() -> Self {}

    pub fn new_from(inner: Vec<E>) -> Self {
        Self{
            fwd: None,
            back: None,
            inner,
            view_size: 0,
        }
    }

    pub fn view_size()

    pub fn on_foward(mut self, msg: M) -> Self {
        self.fwd = Some(msg);
        self
    }

    pub fn on_back(mut self, msg: M) -> Self {
        self.back = Some(msg);
        self
    }
}
