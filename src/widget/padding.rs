// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A widget that just adds padding during layout.

use crate::{
    Action, BaseState, BoxConstraints, Env, Event, EventCtx, LayoutCtx, PaintCtx, Point, Rect,
    Size, WidgetBase, WidgetInner,
};

pub struct Padding {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,

    child: WidgetBase<Box<dyn WidgetInner>>,
}

impl Padding {
    /// Create widget with uniform padding.
    pub fn uniform(padding: f64, child: impl Into<WidgetBase<Box<dyn WidgetInner>>>) -> Padding {
        Padding {
            left: padding,
            right: padding,
            top: padding,
            bottom: padding,
            child: child.into(),
        }
    }
}

impl WidgetInner for Padding {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, _base_state: &BaseState, env: &Env) {
        self.child.paint_with_offset(paint_ctx, env, ());
    }

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, env: &Env) -> Size {
        let hpad = self.left + self.right;
        let vpad = self.top + self.bottom;
        let min = Size::new(bc.min.width - hpad, bc.min.height - vpad);
        let max = Size::new(bc.max.width - hpad, bc.max.height - vpad);
        let child_bc = BoxConstraints::new(min, max);
        let size = self.child.layout(layout_ctx, &child_bc, env, ());
        let origin = Point::new(self.left, self.top);
        self.child
            .set_layout_rect(Rect::from_origin_size(origin, size));
        Size::new(size.width + hpad, size.height + vpad)
    }

    fn event(&mut self, event: &Event, ctx: &mut EventCtx, env: &Env) -> Option<Action> {
        self.child.event(event, ctx, env, ())
    }
}
