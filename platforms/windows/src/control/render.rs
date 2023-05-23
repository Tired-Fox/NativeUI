trait OldRender {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String> {
        let mut add_padding = (false, false);
        let dimensions = self.style().0;

        let parent_padding = parent
            .1
             .0
            .padding
            .calc(parent.0.width(), parent.0.height());

        let inset = dimensions.inset.calc(parent.0.width(), parent.0.height());
        let margin = self
            .style()
            .0
            .margin
            .calc(parent.0.width(), parent.0.height());

        let width = dimensions.width.as_i32(
            parent.0.width() - parent_padding.1 - parent_padding.3 - margin.1 - margin.3,
            match dimensions.position {
                style::Position::Absolute
                    if inset.3 != 0 && inset.1 != 0 && dimensions.width == Unit::Default =>
                {
                    parent.0.width() - inset.3 - inset.1 - margin.1 - margin.3
                }
                _ => match dimensions.width {
                    Unit::FitConent => {
                        add_padding.0 = true;
                        self.text_rect.width()
                    }
                    _ => {
                        parent.0.width() - parent_padding.1 - parent_padding.3 - margin.1 - margin.3
                    }
                },
            },
        );

        let height = dimensions.height.as_i32(
            parent.0.height() - parent_padding.0 - parent_padding.2,
            match dimensions.position {
                Position::Absolute
                    if inset.0 != 0 && inset.2 != 0 && dimensions.height == Unit::Default =>
                {
                    parent.0.height() - inset.2 - inset.0 - margin.2 - margin.0
                }
                _ => match dimensions.height {
                    Unit::FitConent | Unit::Default => {
                        add_padding.1 = true;
                        self.text_rect.height()
                    }
                    _ => {
                        parent.0.height()
                            - margin.0
                            - margin.2
                            - parent_padding.0
                            - parent_padding.2
                    }
                },
            },
        );

        let padding = self.style().0.padding.calc(width, height);

        self.rect.left = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.left {
                Unit::Default => match dimensions.inset.right {
                    Unit::Default => margin.3,
                    _ => parent.0.width() - margin.1 - inset.1 - width,
                },
                _ => margin.3 + inset.3,
            },
            _ => parent_padding.3 + margin.3,
        };

        self.rect.top = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.top {
                Unit::Default => match dimensions.inset.bottom {
                    Unit::Default => margin.0,
                    _ => parent.0.height() - margin.2 - inset.2 - height,
                },
                _ => margin.0 + inset.0,
            },
            _ => match previous {
                Some(prev) => {
                    prev.0.bottom
                        + prev.1 .0.margin.bottom.as_i32(parent.0.height(), 0)
                        + parent_padding.0
                        + margin.0
                }
                None => parent_padding.0 + margin.0,
            },
        };

        self.rect.right = self.rect.left + width;
        self.rect.bottom = self.rect.top + height;

        if add_padding.0 {
            self.rect.right += padding.3 + padding.1 + 4;
        }

        if add_padding.1 {
            self.rect.bottom += padding.0 + padding.2;
        }

        match dimensions.max_width {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_width.as_i32(parent.0.width(), 0);
                if self.rect.width() > max {
                    self.rect.right -= self.rect.width() - max;
                }
            }
        }

        match dimensions.min_width {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_width.as_i32(parent.0.width(), 0);
                if self.rect.width() < min {
                    self.rect.right += min - self.rect.width();
                }
            }
        };

        match dimensions.max_height {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_height.as_i32(parent.0.height(), 0);
                if self.rect.height() > max {
                    self.rect.bottom -= self.rect.height() - max;
                }
            }
        }

        match dimensions.min_height {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_height.as_i32(parent.0.height(), 0);
                if self.rect.height() < min {
                    self.rect.bottom += min - self.rect.height();
                }
            }
        };

        self.ns_rect = self.rect.clone();
        self.ns_rect.top += padding.0;
        self.ns_rect.right -= padding.1;
        self.ns_rect.bottom -= padding.2;
        self.ns_rect.left += padding.3;
        update_pos(self);

        Ok(())
    }
}
