use style::{Dimensions, Appearance};

use super::Rect;

fn calculate_rect(
    rect: &mut Rect,
    previous: &(Rect, (Dimensions, Appearance)),
    parent: &(Rect, (Dimensions, Appearance)),
) {
    // let mut padding: (i32, i32, i32, i32) = (0, 0, 0, 0);
    // if previous.1.contains_key("padding") {
    //     match previous.1.get("padding").unwrap() {
    //         Prop::Padding(top, right, bottom, left) => {
    //             let width = parent.0.width();
    //             let height = parent.0.height();

    //             match top {
    //                 Size::PX(px) => padding.0 = px.to_owned(),
    //                 Size::Percent(percent) => padding.0 = (height as f32 * percent) as i32
    //             }


    //             match right {
    //                 Size::PX(px) => padding.1 = px.to_owned(),
    //                 Size::Percent(percent) => padding.1 = (height as f32 * percent) as i32
    //             }


    //             match bottom {
    //                 Size::PX(px) => padding.2 = px.to_owned(),
    //                 Size::Percent(percent) => padding.2 = (height as f32 * percent) as i32
    //             }


    //             match left {
    //                 Size::PX(px) => padding.3 = px.to_owned(),
    //                 Size::Percent(percent) => padding.3 = (height as f32 * percent) as i32
    //             }
    //         }
    //         _ => (),
    //     }
    // }

    // TODO:
    // - Get padding
    // - Get margin
    // - Calc base width
    // - Calc base height
    // - Calc position with padding and margin
}
