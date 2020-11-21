use crate::base::gwindow::{render_struct, NextPos};
use crate::base::structure::Structure;

pub fn render_element<P, S: Structure>(element: fn(P) -> S, props: P) {
    render_struct(&element(props), NextPos::Bottom);
}
