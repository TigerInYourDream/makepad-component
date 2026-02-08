// Calendar grid rendering implementation for A2uiSurface
//
// Renders a weekly calendar/planner as a colored grid with rows for
// different time slots (header, morning, afternoon, evening, budget).
// Supports click interaction: cells are hit-testable, clicked cells
// get highlighted, and a userAction is emitted with (row, col).

impl A2uiSurface {
    /// Render a calendar grid component
    pub(crate) fn render_calendar(
        &mut self,
        cx: &mut Cx2d,
        calendar: &CalendarComponent,
        data_model: &DataModel,
    ) {
        let num_cols = calendar.columns;
        let num_rows = calendar.row_labels.len();
        if num_cols == 0 || num_rows == 0 {
            return;
        }

        // Extract cells data path (we need the path string itself, not the resolved value)
        let cells_path = match &calendar.cells {
            crate::a2ui::value::StringValue::Path { path } => path.clone(),
            crate::a2ui::value::StringValue::Literal { literal_string } => literal_string.clone(),
        };

        // Use a reasonable total width
        let total_width = 900.0_f64;
        let col_width = total_width / num_cols as f64;

        // Snapshot interactive state to avoid borrow issues
        let selected_cell = self.calendar_selected_cell;
        let hovered_idx = self.calendar_hovered_idx;
        let cell_start_idx = self.calendar_cell_areas.len();

        // Outer container: Flow::Down
        cx.begin_turtle(
            Walk::new(Size::Fixed(total_width), Size::fit()),
            Layout {
                flow: Flow::Down,
                ..Layout::default()
            },
        );

        // === Title row ===
        if let Some(title_sv) = &calendar.title {
            let title_text = resolve_string_value_scoped(
                title_sv,
                data_model,
                self.current_scope.as_deref(),
            );
            if !title_text.is_empty() {
                let title_color = vec4(0.08, 0.10, 0.18, 1.0);
                self.draw_calendar_cell.color = title_color;
                self.draw_calendar_cell.begin(
                    cx,
                    Walk::new(Size::Fixed(total_width), Size::Fixed(45.0)),
                    Layout {
                        align: Align { x: 0.5, y: 0.5 },
                        ..Layout::default()
                    },
                );
                self.draw_calendar_header_text.text_style.font_size = 16.0;
                self.draw_calendar_header_text.draw_walk(
                    cx,
                    Walk::fit(),
                    Align::default(),
                    &title_text,
                );
                self.draw_calendar_cell.end(cx);
            }
        }

        // === Data rows ===
        let mut cell_counter = cell_start_idx;

        for row_idx in 0..num_rows {
            let color_hint = calendar
                .row_color_hints
                .get(row_idx)
                .map(|s| s.as_str())
                .unwrap_or("");

            let base_row_color = Self::calendar_row_color(color_hint);
            let row_height = match color_hint {
                "header" => 55.0,
                "budget" => 40.0,
                _ => 70.0,
            };

            let row_label = calendar
                .row_labels
                .get(row_idx)
                .cloned()
                .unwrap_or_default();

            // Row container: Flow::Right
            cx.begin_turtle(
                Walk::new(Size::Fixed(total_width), Size::Fixed(row_height)),
                Layout {
                    flow: Flow::right(),
                    ..Layout::default()
                },
            );

            for col_idx in 0..num_cols {
                // Determine cell color: highlight if selected or hovered
                let is_selected = selected_cell == Some((row_idx, col_idx));
                let is_hovered = hovered_idx == Some(cell_counter);

                let cell_color = if is_selected {
                    // Bright highlight for selected cell
                    vec4(0.231, 0.510, 0.965, 1.0) // #3B82F6 blue
                } else if is_hovered {
                    // Subtle brighten for hovered cell
                    Self::brighten_color(base_row_color, 0.15)
                } else {
                    base_row_color
                };

                self.draw_calendar_cell.color = cell_color;
                self.draw_calendar_cell.begin(
                    cx,
                    Walk::new(Size::Fixed(col_width), Size::Fixed(row_height)),
                    Layout {
                        flow: Flow::Down,
                        padding: Padding {
                            left: 4.0,
                            right: 4.0,
                            top: 3.0,
                            bottom: 3.0,
                        },
                        ..Layout::default()
                    },
                );

                if color_hint == "header" {
                    // Header row: show column headers + subtitles
                    let header = calendar
                        .column_headers
                        .get(col_idx)
                        .cloned()
                        .unwrap_or_default();
                    let subtitle = calendar
                        .column_subtitles
                        .get(col_idx)
                        .cloned()
                        .unwrap_or_default();

                    self.draw_calendar_header_text.text_style.font_size = 13.0;
                    self.draw_calendar_header_text.draw_walk(
                        cx,
                        Walk::fit(),
                        Align::default(),
                        &header,
                    );

                    if !subtitle.is_empty() {
                        self.draw_calendar_text.text_style.font_size = 10.0;
                        self.draw_calendar_text.color = vec4(0.7, 0.8, 0.9, 1.0);
                        self.draw_calendar_text.draw_walk(
                            cx,
                            Walk::fit(),
                            Align::default(),
                            &subtitle,
                        );
                    }
                } else {
                    // Data rows: show row_label + cell data
                    let line1_path = format!("{}/{}/{}/line1", cells_path, row_idx, col_idx);
                    let line2_path = format!("{}/{}/{}/line2", cells_path, row_idx, col_idx);

                    let line1 = data_model
                        .get_string(&line1_path)
                        .unwrap_or("")
                        .to_string();
                    let line2 = data_model
                        .get_string(&line2_path)
                        .unwrap_or("")
                        .to_string();

                    // Row label (time slot indicator)
                    if !row_label.is_empty() {
                        self.draw_calendar_text.text_style.font_size = 9.0;
                        self.draw_calendar_text.color = vec4(0.6, 0.7, 0.8, 0.8);
                        self.draw_calendar_text.draw_walk(
                            cx,
                            Walk::fit(),
                            Align::default(),
                            &row_label,
                        );
                    }

                    // Line 1 (main text - location/activity)
                    if !line1.is_empty() {
                        self.draw_calendar_header_text.text_style.font_size = 11.0;
                        self.draw_calendar_header_text.draw_walk(
                            cx,
                            Walk::fit(),
                            Align::default(),
                            &line1,
                        );
                    }

                    // Line 2 (detail text)
                    if !line2.is_empty() {
                        self.draw_calendar_text.text_style.font_size = 9.0;
                        self.draw_calendar_text.color = vec4(0.7, 0.8, 0.9, 0.9);
                        self.draw_calendar_text.draw_walk(
                            cx,
                            Walk::fit(),
                            Align::default(),
                            &line2,
                        );
                    }
                }

                self.draw_calendar_cell.end(cx);

                // Store the cell area for hit testing
                let cell_area = self.draw_calendar_cell.area();
                self.calendar_cell_areas.push(cell_area);
                self.calendar_cell_meta.push((row_idx, col_idx));
                cell_counter += 1;
            }

            cx.end_turtle();
        }

        // === Footer row ===
        if let Some(footer_sv) = &calendar.footer {
            let footer_text = resolve_string_value_scoped(
                footer_sv,
                data_model,
                self.current_scope.as_deref(),
            );
            if !footer_text.is_empty() {
                let footer_color = vec4(0.08, 0.10, 0.18, 1.0);
                self.draw_calendar_cell.color = footer_color;
                self.draw_calendar_cell.begin(
                    cx,
                    Walk::new(Size::Fixed(total_width), Size::Fixed(40.0)),
                    Layout {
                        align: Align { x: 0.5, y: 0.5 },
                        ..Layout::default()
                    },
                );
                self.draw_calendar_header_text.text_style.font_size = 14.0;
                self.draw_calendar_header_text.draw_walk(
                    cx,
                    Walk::fit(),
                    Align::default(),
                    &footer_text,
                );
                self.draw_calendar_cell.end(cx);
            }
        }

        // === Detail panel for selected cell ===
        if let Some((sel_row, sel_col)) = selected_cell {
            self.render_calendar_detail(cx, calendar, data_model, &cells_path, sel_row, sel_col, total_width);
        }

        cx.end_turtle();
    }

    /// Render the detail panel below the calendar showing the full day schedule
    fn render_calendar_detail(
        &mut self,
        cx: &mut Cx2d,
        calendar: &CalendarComponent,
        data_model: &DataModel,
        cells_path: &str,
        sel_row: usize,
        sel_col: usize,
        total_width: f64,
    ) {
        let num_rows = calendar.row_labels.len();
        let content_width = total_width - 32.0;

        // Day header
        let day_name = calendar.column_headers.get(sel_col)
            .cloned().unwrap_or_else(|| format!("Day {}", sel_col + 1));
        let day_subtitle = calendar.column_subtitles.get(sel_col)
            .cloned().unwrap_or_default();

        // Detail card container
        self.draw_calendar_cell.color = vec4(0.10, 0.12, 0.22, 1.0);
        self.draw_calendar_cell.begin(
            cx,
            Walk {
                width: Size::Fixed(total_width),
                height: Size::fit(),
                margin: Margin { left: 0.0, right: 0.0, top: 12.0, bottom: 0.0 },
                ..Walk::default()
            },
            Layout {
                flow: Flow::Down,
                padding: Padding { left: 16.0, right: 16.0, top: 14.0, bottom: 14.0 },
                spacing: 4.0,
                ..Layout::default()
            },
        );

        // Title: "Day X — Area"
        let title = if day_subtitle.is_empty() {
            day_name.clone()
        } else {
            format!("{} | {}", day_name, day_subtitle)
        };
        self.draw_calendar_header_text.text_style.font_size = 18.0;
        self.draw_calendar_header_text.color = vec4(1.0, 1.0, 1.0, 1.0);
        self.draw_calendar_header_text.draw_walk(cx, Walk::fit(), Align::default(), &title);

        // Subtitle: "Full Day Itinerary"
        self.draw_calendar_text.text_style.font_size = 11.0;
        self.draw_calendar_text.color = vec4(0.5, 0.6, 0.7, 0.8);
        self.draw_calendar_text.draw_walk(cx, Walk::fit(), Align::default(), "Full Day Itinerary");

        // Divider line
        self.draw_divider.draw_walk(cx, Walk {
            width: Size::Fixed(content_width),
            height: Size::Fixed(1.0),
            margin: Margin { top: 4.0, bottom: 8.0, left: 0.0, right: 0.0 },
            ..Walk::default()
        });

        // Readable time labels and emoji for each slot
        let readable_labels = ["", "Morning", "Afternoon", "Evening", "Budget Summary"];
        let slot_emoji = ["", "  ", "  ", "  ", "  "];

        // Each time slot for this day (skip header row 0)
        for row_idx in 1..num_rows {
            let color_hint = calendar.row_color_hints.get(row_idx)
                .map(|s| s.as_str()).unwrap_or("");

            let cell_base = format!("{}/{}/{}", cells_path, row_idx, sel_col);
            let line1 = data_model.get_string(&format!("{}/line1", cell_base))
                .unwrap_or("").to_string();
            let line2 = data_model.get_string(&format!("{}/line2", cell_base))
                .unwrap_or("").to_string();
            let time_range = data_model.get_string(&format!("{}/time", cell_base))
                .unwrap_or("").to_string();
            let description = data_model.get_string(&format!("{}/description", cell_base))
                .unwrap_or("").to_string();
            let tips = data_model.get_string(&format!("{}/tips", cell_base))
                .unwrap_or("").to_string();

            if line1.is_empty() && line2.is_empty() && description.is_empty() {
                continue;
            }

            let is_selected_slot = row_idx == sel_row;
            let time_label = readable_labels.get(row_idx).copied().unwrap_or("Other");
            let emoji = slot_emoji.get(row_idx).copied().unwrap_or("");

            // Slot section container
            cx.begin_turtle(
                Walk {
                    width: Size::Fixed(content_width),
                    height: Size::fit(),
                    margin: Margin { top: 2.0, bottom: 6.0, left: 0.0, right: 0.0 },
                    ..Walk::default()
                },
                Layout {
                    flow: Flow::Down,
                    padding: Padding { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 },
                    spacing: 3.0,
                    ..Layout::default()
                },
            );

            // --- Slot header line: emoji + time label + time range ---
            let header_text = if time_range.is_empty() {
                format!("{}{}", emoji, time_label)
            } else {
                format!("{}{}    {}", emoji, time_label, time_range)
            };
            self.draw_calendar_header_text.text_style.font_size = 15.0;
            self.draw_calendar_header_text.color = if is_selected_slot {
                // Bright accent for selected slot
                match color_hint {
                    "morning" => vec4(1.0, 0.75, 0.3, 1.0),    // warm gold
                    "afternoon" => vec4(1.0, 0.85, 0.4, 1.0),   // bright gold
                    "evening" => vec4(0.4, 0.75, 1.0, 1.0),     // sky blue
                    "budget" => vec4(0.4, 0.9, 0.5, 1.0),       // green
                    _ => vec4(0.4, 0.75, 1.0, 1.0),
                }
            } else {
                match color_hint {
                    "morning" => vec4(0.8, 0.6, 0.35, 1.0),
                    "afternoon" => vec4(0.85, 0.7, 0.4, 1.0),
                    "evening" => vec4(0.5, 0.65, 0.85, 1.0),
                    "budget" => vec4(0.5, 0.75, 0.5, 1.0),
                    _ => vec4(0.6, 0.7, 0.85, 1.0),
                }
            };
            self.draw_calendar_header_text.draw_walk(
                cx, Walk::fit(), Align::default(), &header_text,
            );

            // --- Location name (bold) ---
            if !line1.is_empty() {
                let location = if line2.is_empty() {
                    line1.clone()
                } else {
                    format!("{} - {}", line1, line2)
                };
                self.draw_calendar_header_text.text_style.font_size = 14.0;
                self.draw_calendar_header_text.color = if is_selected_slot {
                    vec4(1.0, 1.0, 1.0, 1.0)
                } else {
                    vec4(0.85, 0.9, 0.95, 1.0)
                };
                self.draw_calendar_header_text.draw_walk(
                    cx, Walk::fit(), Align::default(), &location,
                );
            }

            // --- Description (detail text) ---
            if !description.is_empty() {
                self.draw_calendar_text.text_style.font_size = 12.0;
                self.draw_calendar_text.color = if is_selected_slot {
                    vec4(0.85, 0.9, 0.95, 0.95)
                } else {
                    vec4(0.65, 0.72, 0.8, 0.9)
                };
                self.draw_calendar_text.draw_walk(
                    cx, Walk::fit(), Align::default(), &description,
                );
            }

            // --- Tips (lighter, smaller) ---
            if !tips.is_empty() {
                let tips_text = format!("Tip: {}", tips);
                self.draw_calendar_text.text_style.font_size = 11.0;
                self.draw_calendar_text.color = if is_selected_slot {
                    vec4(0.6, 0.85, 0.6, 0.9)   // green for tips when selected
                } else {
                    vec4(0.45, 0.65, 0.45, 0.8)  // dim green
                };
                self.draw_calendar_text.draw_walk(
                    cx, Walk::fit(), Align::default(), &tips_text,
                );
            }

            cx.end_turtle();

            // Thin divider between slots
            if row_idx < num_rows - 1 {
                self.draw_divider.draw_walk(cx, Walk {
                    width: Size::Fixed(content_width - 24.0),
                    height: Size::Fixed(1.0),
                    margin: Margin { top: 0.0, bottom: 0.0, left: 12.0, right: 0.0 },
                    ..Walk::default()
                });
            }
        }

        self.draw_calendar_cell.end(cx);
    }

    /// Get the background color for a calendar row based on its color hint
    fn calendar_row_color(hint: &str) -> Vec4 {
        match hint {
            "header" => vec4(0.102, 0.165, 0.290, 1.0),    // #1a2a4a navy
            "morning" => vec4(0.239, 0.169, 0.122, 1.0),   // #3d2b1f warm sunrise
            "afternoon" => vec4(0.290, 0.208, 0.125, 1.0), // #4a3520 warm gold
            "evening" => vec4(0.102, 0.165, 0.227, 1.0),   // #1a2a3a cool blue
            "budget" => vec4(0.165, 0.165, 0.102, 1.0),    // #2a2a1a dark gold
            _ => vec4(0.133, 0.133, 0.200, 1.0),           // default
        }
    }

    /// Brighten a color by a factor (for hover effect)
    fn brighten_color(color: Vec4, amount: f32) -> Vec4 {
        vec4(
            (color.x + amount).min(1.0),
            (color.y + amount).min(1.0),
            (color.z + amount).min(1.0),
            color.w,
        )
    }
}
