use makepad_widgets::*;

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
}

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
