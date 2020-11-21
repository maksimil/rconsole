use crate::render::{render_struct, NextPos};
use crate::structure::Structure;

pub fn render_element<P, S: Structure>(element: fn(P) -> S, props: P) {
    render_struct(&element(props), NextPos::Bottom);
}
