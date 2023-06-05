use std::{collections::HashSet, fmt};

use super::layout::Layout;
use crate::{Rect, STYLESHEET};
use style::{Appearance, Dimensions};

pub trait Renderable {
    fn id(&self) -> &String;

    fn classes(&self) -> &HashSet<String>;

    fn rect(&self) -> &Rect;

    fn default_rect(&self) -> &Rect;

    fn get_styles(&self) -> (Dimensions, Appearance) {
        let mut styles = self.classes().clone();
        styles.insert(self.id().clone());
        // println!("{:?} {:?}", styles, STYLESHEET.0.read());
        STYLESHEET.get().get_styles(styles)
    }

    fn show(&mut self);
    fn hide(&mut self);
    fn update(&mut self, rect: Rect) -> (i32, i32);
}

pub trait Container<Data, Error>: Renderable + fmt::Debug {
    fn layout(&mut self) -> &mut Layout<Data, Error>;
    fn init(&mut self) -> Result<(), Error>;
}

pub trait Component<Data, Error>: Renderable + fmt::Debug {
    fn create(&mut self, data: Data) -> Result<(), Error>;
}
