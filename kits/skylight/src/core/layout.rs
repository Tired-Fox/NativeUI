use std::collections::HashMap;

use style::{Prop, Size};

use super::Rect;

fn calculate_rect(
    rect: &mut Rect,
    previous: &(Rect, HashMap<String, Prop>),
    parent: &(Rect, HashMap<String, Prop>),
) {
    let mut padding: (i32, i32, i32, i32) = (0, 0, 0, 0);
    if previous.1.contains_key("padding") {
        match previous.get("padding").unwrap() {
            Prop::Padding(top, right, bottom, left) => {
                let width = parent.0.width();
                let height = parent.0.height();

                match top {
                    Size::PX(px) => padding.0 = px,
                    Size::Percent(percent) => padding.0 = (height * percent) as i32
                }
                padding = (top, right, bottom, left);
            }
            _ => (),
        }
    }
    previous.1.get("")
    // TODO:
    // - Get padding
    // - Get margin
    // - Calc base width
    // - Calc base height
    // - Calc position with padding and margin
}
