use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ============================================================
    // MpSelect - Custom select/combobox component
    // ============================================================

    pub MpSelect = {{MpSelectWidget}} {
        width: 200
        height: Fit

        flow: Right,
        align: { y: 0.5 },
        padding: { left: 12, right: 12, top: 8, bottom: 8 },

        draw_bg: {
            color: #FFFFFF
        }

        draw_text: {
            color: #0a0a0a
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
        }

        popup_draw_bg: {
            color: #FFFFFF
        }

        item_draw_bg: {
            color: #00000000
        }

        item_draw_text: {
            color: #0a0a0a
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
        }

        search_draw_bg: {
            color: #f5f5f5
        }

        search_draw_text: {
            color: #0a0a0a
            text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
        }

        clear_draw_bg: {
            color: #00000000
        }

        placeholder: "Select..."
        searchable: false
        clearable: false
        disabled: false
        popup_width: 0.0

    }

    // Size variants
    pub MpSelectSmall = <MpSelect> {
        padding: { left: 8, right: 8, top: 4, bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
        }
        item_draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
        }
        search_draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
        }
    }

    pub MpSelectLarge = <MpSelect> {
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 16.0 }
        }
        item_draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 16.0 }
        }
        search_draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 15.0 }
        }
    }
}

// ============================================================
// Data Types
// ============================================================

#[derive(Clone, Debug)]
pub struct SelectItemData {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}

// ============================================================
// Actions
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpSelectAction {
    None,
    Changed(usize, String),
    Cleared,
    Opened,
    Closed,
}

// ============================================================
// MpSelectWidget
// ============================================================

#[derive(Live, Widget)]
pub struct MpSelectWidget {
    #[redraw] #[live] draw_bg: DrawColor,
    #[live] draw_text: DrawText,
    #[walk] walk: Walk,
    #[layout] layout: Layout,

    #[live] popup_draw_bg: DrawColor,
    #[live] item_draw_bg: DrawColor,
    #[live] item_draw_text: DrawText,
    #[live] search_draw_bg: DrawColor,
    #[live] search_draw_text: DrawText,
    #[live] clear_draw_bg: DrawColor,

    #[live] placeholder: String,
    #[live] searchable: bool,
    #[live] clearable: bool,
    #[live] disabled: bool,
    #[live(0.0)] popup_width: f64,

    #[rust] opened: bool,
    #[rust] selected_index: Option<usize>,
    #[rust] selected_label: String,
    #[rust] items: Vec<SelectItemData>,
    #[rust] filtered_indices: Vec<usize>,
    #[rust] search_text: String,
    #[rust] hover_item_index: Option<usize>,
    #[rust] keyboard_focus_index: Option<usize>,

    #[rust(DrawList2d::new(cx))] draw_list: DrawList2d,

    #[rust] item_areas: Vec<Area>,
    #[rust] popup_area: Area,
    #[rust] search_area: Area,
    #[rust] clear_area: Area,
}

impl LiveHook for MpSelectWidget {}

impl MpSelectWidget {
    fn rebuild_filtered_indices(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
        } else {
            let search_lower = self.search_text.to_lowercase();
            self.filtered_indices = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.label.to_lowercase().contains(&search_lower))
                .map(|(i, _)| i)
                .collect();
        }
    }

    fn open_popup(&mut self, cx: &mut Cx) {
        if self.disabled || self.opened {
            return;
        }
        self.opened = true;
        self.search_text.clear();
        self.rebuild_filtered_indices();
        self.keyboard_focus_index = self.selected_index.and_then(|sel| {
            self.filtered_indices.iter().position(|&i| i == sel)
        });
        self.hover_item_index = None;
        self.draw_list.redraw(cx);
        self.redraw(cx);
        cx.sweep_lock(self.draw_bg.area());
    }

    fn close_popup(&mut self, cx: &mut Cx) {
        if !self.opened {
            return;
        }
        self.opened = false;
        self.search_text.clear();
        self.hover_item_index = None;
        self.keyboard_focus_index = None;
        self.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area());
    }

    fn select_item(&mut self, cx: &mut Cx, item_index: usize, uid: WidgetUid, path: &HeapLiveIdPath) {
        if item_index >= self.items.len() || self.items[item_index].disabled {
            return;
        }
        self.selected_index = Some(item_index);
        self.selected_label = self.items[item_index].label.clone();
        let value = self.items[item_index].value.clone();
        cx.widget_action(uid, path, MpSelectAction::Changed(item_index, value));
        self.close_popup(cx);
    }

    fn draw_popup(&mut self, cx: &mut Cx2d) {
        self.item_areas.clear();

        let trigger_rect = self.draw_bg.area().rect(cx);
        let popup_width = if self.popup_width > 0.0 {
            self.popup_width
        } else {
            trigger_rect.size.x
        };

        self.draw_list.begin_overlay_reuse(cx);
        let pass_size = cx.current_pass_size();
        cx.begin_root_turtle(pass_size, Layout::flow_down());

        let popup_walk = Walk::new(Size::Fixed(popup_width), Size::fit())
            .with_abs_pos(dvec2(
                trigger_rect.pos.x,
                trigger_rect.pos.y + trigger_rect.size.y + 4.0,
            ));

        let popup_layout = Layout::flow_down()
            .with_padding(Padding { left: 4.0, top: 4.0, right: 4.0, bottom: 4.0 });

        self.popup_draw_bg.begin(cx, popup_walk, popup_layout);

        // Search input
        if self.searchable {
            let search_bg_walk = Walk::new(Size::fill(), Size::Fixed(32.0))
                .with_margin_bottom(4.0);

            let search_layout = Layout::flow_right()
                .with_align_y(0.5)
                .with_padding(Padding { left: 8.0, top: 0.0, right: 8.0, bottom: 0.0 });

            self.search_draw_bg.begin(cx, search_bg_walk, search_layout);

            let search_display = if self.search_text.is_empty() {
                "Search..."
            } else {
                &self.search_text
            };

            self.search_draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), search_display);

            self.search_draw_bg.end(cx);
            self.search_area = self.search_draw_bg.area();
        }

        // Items
        let item_layout = Layout::flow_right()
            .with_align_y(0.5)
            .with_padding(Padding { left: 8.0, top: 6.0, right: 8.0, bottom: 6.0 });

        for (fi, &item_idx) in self.filtered_indices.clone().iter().enumerate() {
            let item = &self.items[item_idx];

            let is_hover = self.hover_item_index == Some(fi);
            let is_selected = self.selected_index == Some(item_idx);

            let _ = is_hover;
            let _ = is_selected;

            let item_walk = Walk::fill_fit();

            // Show checkmark prefix for selected items
            let label = if is_selected {
                format!("✓ {}", item.label)
            } else {
                format!("   {}", item.label)
            };

            self.item_draw_bg.begin(cx, item_walk, item_layout);
            self.item_draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), &label);
            self.item_draw_bg.end(cx);

            self.item_areas.push(self.item_draw_bg.area());
        }

        // "No results" message
        if self.filtered_indices.is_empty() && !self.search_text.is_empty() {
            let no_result_layout = Layout::flow_right()
                .with_align_x(0.5)
                .with_align_y(0.5)
                .with_padding(Padding { left: 8.0, top: 12.0, right: 8.0, bottom: 12.0 });

            self.item_draw_bg.begin(cx, Walk::fill_fit(), no_result_layout);
            self.item_draw_text
                .draw_walk(cx, Walk::fit(), Align::default(), "No results found");
            self.item_draw_bg.end(cx);
        }

        self.popup_draw_bg.end(cx);
        self.popup_area = self.popup_draw_bg.area();

        cx.end_pass_sized_turtle();
        self.draw_list.end(cx);
    }
}

impl Widget for MpSelectWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        // Handle popup events when open
        if self.opened {
            if let Event::MouseDown(e) = event {
                let in_popup = self.popup_draw_bg.area().clipped_rect(cx).contains(e.abs);
                let in_trigger = self.draw_bg.area().clipped_rect(cx).contains(e.abs);
                if !in_popup && !in_trigger {
                    self.close_popup(cx);
                    cx.widget_action(uid, &scope.path, MpSelectAction::Closed);
                    return;
                }
            }

            if let Event::MouseMove(e) = event {
                let mut new_hover = None;
                for (fi, area) in self.item_areas.iter().enumerate() {
                    if area.clipped_rect(cx).contains(e.abs) {
                        if let Some(&item_idx) = self.filtered_indices.get(fi) {
                            if !self.items[item_idx].disabled {
                                new_hover = Some(fi);
                            }
                        }
                        break;
                    }
                }
                if new_hover != self.hover_item_index {
                    self.hover_item_index = new_hover;
                    self.keyboard_focus_index = new_hover;
                    self.draw_list.redraw(cx);
                }
            }

            if let Event::MouseUp(e) = event {
                for (fi, area) in self.item_areas.iter().enumerate() {
                    if area.clipped_rect(cx).contains(e.abs) {
                        if let Some(&item_idx) = self.filtered_indices.get(fi) {
                            self.select_item(cx, item_idx, uid, &scope.path);
                            cx.widget_action(uid, &scope.path, MpSelectAction::Closed);
                            return;
                        }
                    }
                }
            }

            if let Event::KeyDown(ke) = event {
                match ke.key_code {
                    KeyCode::Escape => {
                        self.close_popup(cx);
                        cx.widget_action(uid, &scope.path, MpSelectAction::Closed);
                        return;
                    }
                    KeyCode::ArrowDown => {
                        let len = self.filtered_indices.len();
                        if len > 0 {
                            let next = match self.keyboard_focus_index {
                                Some(i) if i + 1 < len => i + 1,
                                Some(i) => i,
                                None => 0,
                            };
                            self.keyboard_focus_index = Some(next);
                            self.hover_item_index = Some(next);
                            self.draw_list.redraw(cx);
                        }
                    }
                    KeyCode::ArrowUp => {
                        let len = self.filtered_indices.len();
                        if len > 0 {
                            let prev = match self.keyboard_focus_index {
                                Some(i) if i > 0 => i - 1,
                                Some(i) => i,
                                None => 0,
                            };
                            self.keyboard_focus_index = Some(prev);
                            self.hover_item_index = Some(prev);
                            self.draw_list.redraw(cx);
                        }
                    }
                    KeyCode::ReturnKey => {
                        if let Some(fi) = self.keyboard_focus_index {
                            if let Some(&item_idx) = self.filtered_indices.get(fi) {
                                self.select_item(cx, item_idx, uid, &scope.path);
                                cx.widget_action(uid, &scope.path, MpSelectAction::Closed);
                                return;
                            }
                        }
                    }
                    _ => {}
                }
            }

            if self.searchable {
                if let Event::TextInput(te) = event {
                    self.search_text.push_str(&te.input);
                    self.rebuild_filtered_indices();
                    self.keyboard_focus_index = if self.filtered_indices.is_empty() {
                        None
                    } else {
                        Some(0)
                    };
                    self.hover_item_index = self.keyboard_focus_index;
                    self.draw_list.redraw(cx);
                }
                if let Event::KeyDown(ke) = event {
                    if ke.key_code == KeyCode::Backspace && !self.search_text.is_empty() {
                        self.search_text.pop();
                        self.rebuild_filtered_indices();
                        self.keyboard_focus_index = if self.filtered_indices.is_empty() {
                            None
                        } else {
                            Some(0)
                        };
                        self.hover_item_index = self.keyboard_focus_index;
                        self.draw_list.redraw(cx);
                    }
                }
            }
        }

        // Handle trigger button events
        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                if !self.disabled {
                    if self.clearable && self.selected_index.is_some() {
                        if self.clear_area.clipped_rect(cx).contains(fe.abs) {
                            self.selected_index = None;
                            self.selected_label.clear();
                            cx.widget_action(uid, &scope.path, MpSelectAction::Cleared);
                            self.redraw(cx);
                            return;
                        }
                    }

                    cx.set_key_focus(self.draw_bg.area());
                    if self.opened {
                        self.close_popup(cx);
                        cx.widget_action(uid, &scope.path, MpSelectAction::Closed);
                    } else {
                        self.open_popup(cx);
                        cx.widget_action(uid, &scope.path, MpSelectAction::Opened);
                    }
                }
            }
            Hit::FingerHoverIn(_) => {
                if !self.disabled {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerHoverOut(_) => {}
            Hit::KeyFocusLost(_) => {
                if self.opened {
                    self.close_popup(cx);
                }
                self.redraw(cx);
            }
            Hit::KeyFocus(_) => {}
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let display_text = if self.selected_label.is_empty() {
            self.placeholder.clone()
        } else {
            self.selected_label.clone()
        };

        self.draw_bg.begin(cx, walk, self.layout);

        let text_walk = Walk::new(Size::fill(), Size::fit());
        self.draw_text.draw_walk(cx, text_walk, Align::default(), &display_text);

        if self.clearable && self.selected_index.is_some() {
            let clear_walk = Walk::fixed(16.0, 16.0).with_margin_right(4.0);
            self.clear_draw_bg.draw_walk(cx, clear_walk);
            self.clear_area = self.clear_draw_bg.area();

            let clear_rect = self.clear_area.rect(cx);
            let clear_text_walk = Walk::fixed(clear_rect.size.x, clear_rect.size.y)
                .with_abs_pos(clear_rect.pos);
            self.draw_text
                .draw_walk(cx, clear_text_walk, Align { x: 0.5, y: 0.5 }, "×");
        }

        self.draw_text
            .draw_walk(cx, Walk::fixed(12.0, 16.0), Align { x: 0.5, y: 0.5 }, "▾");

        self.draw_bg.end(cx);

        if self.opened {
            self.draw_popup(cx);
        }

        DrawStep::done()
    }
}

// ============================================================
// Ref extensions
// ============================================================

impl MpSelectWidgetRef {
    pub fn set_items(&self, cx: &mut Cx, items: Vec<SelectItemData>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.items = items;
            inner.filtered_indices = (0..inner.items.len()).collect();
            inner.redraw(cx);
        }
    }

    pub fn set_labels(&self, cx: &mut Cx, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.items = labels.into_iter().map(|l| SelectItemData {
                value: l.clone(), label: l, disabled: false,
            }).collect();
            inner.filtered_indices = (0..inner.items.len()).collect();
            inner.redraw(cx);
        }
    }

    pub fn set_selected_index(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            if index < inner.items.len() {
                inner.selected_index = Some(index);
                inner.selected_label = inner.items[index].label.clone();
                inner.redraw(cx);
            }
        }
    }

    pub fn clear_selection(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.selected_index = None;
            inner.selected_label.clear();
            inner.redraw(cx);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<(usize, String)> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let MpSelectAction::Changed(idx, val) = item.cast() {
                    return Some((idx, val));
                }
            }
        }
        None
    }

    pub fn cleared(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpSelectAction>(), MpSelectAction::Cleared);
            }
        }
        false
    }

    pub fn opened(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpSelectAction>(), MpSelectAction::Opened);
            }
        }
        false
    }

    pub fn closed(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpSelectAction>(), MpSelectAction::Closed);
            }
        }
        false
    }
}
