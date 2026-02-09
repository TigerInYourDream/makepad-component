use makepad_widgets::*;

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
pub enum MpDockSplitAxis {
    #[pick]
    Horizontal,
    Vertical,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::theme_colors::*;

    pub MpDockPanel = {{MpDockPanel}} {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            fn pixel(self) -> vec4 {
                return self.bg_color;
            }
        }
    }

    pub MpDockSplitter = {{MpDockSplitter}} {
        width: Fill
        height: Fill

        first = <View> { width: Fill, height: Fill }

        handle = <View> {
            width: 6, height: Fill
            show_bg: true
            draw_bg: {
                instance hover: 0.0
                instance down: 0.0
                instance handle_color: (BORDER)
                instance handle_color_hover: (PRIMARY)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Thin line in center
                    let line_w = 2.0;
                    sdf.rect(c.x - line_w * 0.5, 0.0, line_w, self.rect_size.y);

                    let hover_amount = max(self.hover, self.down);
                    let col = mix(self.handle_color, self.handle_color_hover, hover_amount);
                    sdf.fill(col);

                    return sdf.result;
                }
            }
            animator: {
                hover = {
                    default: off
                    off = {
                        from: { all: Forward { duration: 0.15 } }
                        apply: { draw_bg: { hover: 0.0 } }
                    }
                    on = {
                        from: { all: Forward { duration: 0.1 } }
                        apply: { draw_bg: { hover: 1.0 } }
                    }
                }
                down = {
                    default: off
                    off = {
                        from: { all: Forward { duration: 0.1 } }
                        apply: { draw_bg: { down: 0.0 } }
                    }
                    on = {
                        from: { all: Forward { duration: 0.05 } }
                        apply: { draw_bg: { down: 1.0 } }
                    }
                }
            }
        }

        second = <View> { width: Fill, height: Fill }
    }
}

// ============================================================
// MpDockPanel
// ============================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpDockPanel {
    #[deref] view: View,
}

impl Widget for MpDockPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================
// MpDockSplitter
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockSplitterAction {
    None,
    SplitRatioChanged { ratio: f64 },
}

#[derive(Live, Widget)]
pub struct MpDockSplitter {
    #[deref] view: View,
    #[live] axis: MpDockSplitAxis,
    #[live(0.5)] split_ratio: f64,
    #[live(100.0)] min_size: f64,
    #[rust] dragging: bool,
    #[rust] drag_start_ratio: f64,
    #[rust] container_size: f64,
    #[animator] animator: Animator,
}

impl LiveHook for MpDockSplitter {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_layout(cx);
    }
}

impl MpDockSplitter {
    fn sync_layout(&mut self, cx: &mut Cx) {
        let handle_size = 6.0;
        match self.axis {
            MpDockSplitAxis::Horizontal => {
                // Side-by-side: flow Right, handle is vertical bar
                self.view.apply_over(cx, live! {
                    flow: Right
                });
                self.view.view(ids!(handle)).apply_over(cx, live! {
                    width: (handle_size),
                    height: Fill,
                    cursor: ColResize,
                });
            }
            MpDockSplitAxis::Vertical => {
                // Stacked: flow Down, handle is horizontal bar
                self.view.apply_over(cx, live! {
                    flow: Down
                });
                self.view.view(ids!(handle)).apply_over(cx, live! {
                    width: Fill,
                    height: (handle_size),
                    cursor: RowResize,
                });
            }
        }
    }

    fn apply_split_sizes(&mut self, cx: &mut Cx, container_dim: f64) {
        let handle_size = 6.0;
        let available = (container_dim - handle_size).max(0.0);
        let first_size = (available * self.split_ratio).max(0.0);
        let second_size = (available - first_size).max(0.0);

        match self.axis {
            MpDockSplitAxis::Horizontal => {
                self.view.view(ids!(first)).apply_over(cx, live! {
                    width: (first_size),
                    height: Fill,
                });
                self.view.view(ids!(second)).apply_over(cx, live! {
                    width: (second_size),
                    height: Fill,
                });
            }
            MpDockSplitAxis::Vertical => {
                self.view.view(ids!(first)).apply_over(cx, live! {
                    width: Fill,
                    height: (first_size),
                });
                self.view.view(ids!(second)).apply_over(cx, live! {
                    width: Fill,
                    height: (second_size),
                });
            }
        }
    }

    fn clamp_ratio(&self, ratio: f64, container_dim: f64) -> f64 {
        let handle_size = 6.0;
        let available = (container_dim - handle_size).max(0.0);
        if available <= 0.0 {
            return 0.5;
        }
        let min_ratio = self.min_size / available;
        let max_ratio = 1.0 - min_ratio;
        if min_ratio >= max_ratio {
            return 0.5;
        }
        ratio.clamp(min_ratio, max_ratio)
    }
}

impl Widget for MpDockSplitter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        let handle_area = self.view.view(ids!(handle)).area();
        match event.hits(cx, handle_area) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
                match self.axis {
                    MpDockSplitAxis::Horizontal => cx.set_cursor(MouseCursor::ColResize),
                    MpDockSplitAxis::Vertical => cx.set_cursor(MouseCursor::RowResize),
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.dragging {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(_fe) => {
                self.dragging = true;
                self.drag_start_ratio = self.split_ratio;
                self.animator_play(cx, ids!(down.on));
                match self.axis {
                    MpDockSplitAxis::Horizontal => cx.set_cursor(MouseCursor::ColResize),
                    MpDockSplitAxis::Vertical => cx.set_cursor(MouseCursor::RowResize),
                }
            }
            Hit::FingerMove(fe) => {
                if self.dragging {
                    let container_rect = self.view.area().rect(cx);
                    let (container_origin, container_dim) = match self.axis {
                        MpDockSplitAxis::Horizontal => (container_rect.pos.x, container_rect.size.x),
                        MpDockSplitAxis::Vertical => (container_rect.pos.y, container_rect.size.y),
                    };
                    self.container_size = container_dim;

                    let handle_size = 6.0;
                    let available = (container_dim - handle_size).max(0.0);
                    if available > 0.0 {
                        let mouse_pos = match self.axis {
                            MpDockSplitAxis::Horizontal => fe.abs.x,
                            MpDockSplitAxis::Vertical => fe.abs.y,
                        };
                        let new_ratio = (mouse_pos - container_origin - handle_size * 0.5) / available;
                        let clamped = self.clamp_ratio(new_ratio, container_dim);
                        if (clamped - self.split_ratio).abs() > 0.001 {
                            self.split_ratio = clamped;
                            self.apply_split_sizes(cx, container_dim);
                            self.redraw(cx);
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                MpDockSplitterAction::SplitRatioChanged { ratio: self.split_ratio },
                            );
                        }
                    }
                }
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
                self.animator_play(cx, ids!(down.off));
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let container_rect = self.view.area().rect(cx);
        let container_dim = match self.axis {
            MpDockSplitAxis::Horizontal => container_rect.size.x,
            MpDockSplitAxis::Vertical => container_rect.size.y,
        };

        // Use stored container_size if we have a valid one, otherwise use rect
        let dim = if container_dim > 0.0 {
            self.container_size = container_dim;
            container_dim
        } else if self.container_size > 0.0 {
            self.container_size
        } else {
            // Fallback: estimate from walk
            0.0
        };

        if dim > 0.0 {
            self.apply_split_sizes(cx, dim);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
