mod rect;
mod style_manager;

use std::{cell::RefCell, fmt, sync::Arc};

pub use rect::Rect;
use style::{Appearance, Dimensions, Position, Size, Unit};
pub use style_manager::STYLESHEET;

pub trait Renderable {
    fn id(&self) -> &String;

    fn classes(&self) -> &Vec<String>;

    fn rect(&self) -> &Rect;

    fn default_rect(&self) -> &Rect;

    fn get_styles(&self) -> (Dimensions, Appearance) {
        let mut styles = self.classes().clone();
        styles.push(self.id().clone());
        STYLESHEET.0.read().unwrap().get_styles(styles)
    }

    fn show(&mut self);
    fn hide(&mut self);
    fn update(&mut self, rect: Rect);
}

pub trait Container<Data>: Renderable + fmt::Debug {
    fn layout(&mut self) -> &mut Layout<Data>;
    fn init(&mut self) -> Result<(), String>;
}

pub trait Component<Data>: Renderable + fmt::Debug {
    fn create(&mut self, data: Data) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub enum Child<Data> {
    Component(Arc<RefCell<dyn Component<Data>>>),
    Container(Arc<RefCell<dyn Container<Data>>>),
}

pub struct LayoutBuilder<Data> {
    children: Vec<Child<Data>>,
}

impl<Data> LayoutBuilder<Data> {
    pub fn add(mut self, child: Child<Data>) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> Layout<Data> {
        Layout {
            children: self.children,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layout<Data> {
    pub children: Vec<Child<Data>>,
}

impl<Data> From<Vec<Child<Data>>> for Layout<Data> {
    fn from(value: Vec<Child<Data>>) -> Self {
       Layout { children: value } 
    }
}

impl<Data> Layout<Data> {
    pub fn new() -> Layout<Data> {
        Layout {
            children: Vec::new(),
        }
    }

    pub fn builder() -> LayoutBuilder<Data> {
        LayoutBuilder {
            children: Vec::new(),
        }
    }

    pub fn update(&mut self, parent: &Rect, pstyle: &Dimensions) {
        let rect = parent.shift(&pstyle.padding.calc(parent.width(), parent.height()));

        let mut previous: Option<(Rect, Size)> = None;
        for child in self.children.iter() {
            match child {
                Child::Component(component) => {
                    let component = &mut *component.borrow_mut();
                    let dimensions = component.get_styles().0;
                    component.update(self.calc(
                        component.rect(),
                        component.default_rect(),
                        &dimensions,
                        &rect,
                        &pstyle.padding,
                        previous,
                    ));
                    previous = Some((component.rect().clone(), dimensions.margin));
                }
                Child::Container(container) => {
                    let container = &mut *container.borrow_mut();
                    let dimensions = container.get_styles().0;
                    let crect = self.calc(
                        container.rect(),
                        container.default_rect(),
                        &dimensions,
                        &rect,
                        &pstyle.padding,
                        previous,
                    );
                    
                    container.layout().update(&crect, &dimensions);
                    previous = Some((crect.clone(), dimensions.margin));
                    container.update(crect);
                }
            }
        }
        // Final child size add or remove scrollbar on parent
    }

    fn calc(
        &self,
        rect: &Rect,
        default_rect: &Rect,
        dimensions: &Dimensions,
        parent_rect: &Rect,
        parent_padding: &Size,
        previous: Option<(Rect, Size)>,
    ) -> Rect {
        let mut add_padding = (false, false);

        let ppadding = parent_padding.calc(parent_rect.width(), parent_rect.height());
        let inset = dimensions
            .inset
            .calc(parent_rect.width(), parent_rect.height());
        let margin = dimensions
            .margin
            .calc(parent_rect.width(), parent_rect.height());

        let width = dimensions.width.as_i32(
            parent_rect.width() - ppadding.1 - ppadding.3 - margin.1 - margin.3,
            match dimensions.position {
                style::Position::Absolute
                    if inset.3 != 0 && inset.1 != 0 && dimensions.width == Unit::Default =>
                {
                    parent_rect.width() - inset.3 - inset.1 - margin.1 - margin.3
                }
                _ => match dimensions.width {
                    Unit::FitConent => {
                        add_padding.0 = true;
                        default_rect.width()
                    }
                    _ => parent_rect.width() - margin.1 - margin.3,
                },
            },
        );

        let height = dimensions.height.as_i32(
            parent_rect.height() - ppadding.0 - ppadding.2,
            match dimensions.position {
                Position::Absolute
                    if inset.0 != 0 && inset.2 != 0 && dimensions.height == Unit::Default =>
                {
                    parent_rect.height() - inset.2 - inset.0 - margin.2 - margin.0
                }
                _ => match dimensions.height {
                    Unit::FitConent | Unit::Default => {
                        add_padding.1 = true;
                        default_rect.height()
                    }
                    _ => parent_rect.height() - margin.0 - margin.2,
                },
            },
        );

        let padding = dimensions.padding.calc(width, height);

        let mut crect = rect.clone();
        crect.left = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.left {
                Unit::Default => match dimensions.inset.right {
                    Unit::Default => margin.3,
                    _ => rect.width() - margin.1 - inset.1 - width,
                },
                _ => margin.3 + inset.3,
            },
            _ => ppadding.3 + margin.3,
        };

        crect.top = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.top {
                Unit::Default => match dimensions.inset.bottom {
                    Unit::Default => margin.0,
                    _ => rect.height() - margin.2 - inset.2 - height,
                },
                _ => margin.0 + inset.0,
            },
            _ => match previous {
                Some((prect, pmargin)) => {
                    let (bottom, pad) = (prect.bottom, pmargin.bottom.as_i32(rect.height(), 0));
                    bottom + pad + margin.0
                }
                None => ppadding.0 + margin.0,
            },
        };

        crect.right = crect.left + width;
        crect.bottom = crect.top + height;

        if add_padding.0 {
            crect.right += padding.3 + padding.1 + 4;
        }

        if add_padding.1 {
            crect.bottom += padding.0 + padding.2;
        }

        match dimensions.max_width {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_width.as_i32(rect.width(), 0);
                if crect.width() > max {
                    crect.right -= crect.width() - max;
                }
            }
        }

        match dimensions.min_width {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_width.as_i32(rect.width(), 0);
                if crect.width() < min {
                    crect.right += min - crect.width();
                }
            }
        };

        match dimensions.max_height {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_height.as_i32(rect.height(), 0);
                if crect.height() > max {
                    crect.bottom -= crect.height() - max;
                }
            }
        }

        match dimensions.min_height {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_height.as_i32(rect.height(), 0);
                if crect.height() < min {
                    crect.bottom += min - crect.height();
                }
            }
        };

        crect
    }
}
