// Calendar rendering bridge: converts A2UI CalendarComponent + DataModel
// into MpCalendar config/cells and delegates drawing to the standalone widget.

use crate::widgets::calendar::{CalendarCellData, CalendarConfig};

impl A2uiSurface {
    /// Render a calendar component by delegating to MpCalendar widget
    pub(crate) fn render_calendar(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        calendar: &CalendarComponent,
        data_model: &DataModel,
    ) {
        let num_cols = calendar.columns;
        let num_rows = calendar.row_labels.len();
        if num_cols == 0 || num_rows == 0 {
            return;
        }

        // Extract cells data path
        let cells_path = match &calendar.cells {
            crate::a2ui::value::StringValue::Path { path } => path.clone(),
            crate::a2ui::value::StringValue::Literal { literal_string } => literal_string.clone(),
        };

        // Build CalendarConfig from A2UI CalendarComponent
        let config = CalendarConfig {
            title: calendar
                .title
                .as_ref()
                .map(|sv| {
                    resolve_string_value_scoped(sv, data_model, self.current_scope.as_deref())
                })
                .unwrap_or_default(),
            footer: calendar
                .footer
                .as_ref()
                .map(|sv| {
                    resolve_string_value_scoped(sv, data_model, self.current_scope.as_deref())
                })
                .unwrap_or_default(),
            column_headers: calendar.column_headers.clone(),
            column_subtitles: calendar.column_subtitles.clone(),
            row_labels: calendar.row_labels.clone(),
            row_color_hints: calendar.row_color_hints.clone(),
        };

        // Build cells data from DataModel
        let mut cells = Vec::with_capacity(num_rows);
        for row_idx in 0..num_rows {
            let mut row_cells = Vec::with_capacity(num_cols);
            for col_idx in 0..num_cols {
                let cell_base = format!("{}/{}/{}", cells_path, row_idx, col_idx);
                row_cells.push(CalendarCellData {
                    line1: data_model
                        .get_string(&format!("{}/line1", cell_base))
                        .unwrap_or("")
                        .to_string(),
                    line2: data_model
                        .get_string(&format!("{}/line2", cell_base))
                        .unwrap_or("")
                        .to_string(),
                    time: data_model
                        .get_string(&format!("{}/time", cell_base))
                        .unwrap_or("")
                        .to_string(),
                    description: data_model
                        .get_string(&format!("{}/description", cell_base))
                        .unwrap_or("")
                        .to_string(),
                    tips: data_model
                        .get_string(&format!("{}/tips", cell_base))
                        .unwrap_or("")
                        .to_string(),
                });
            }
            cells.push(row_cells);
        }

        // Ensure calendar widget exists
        let cal = self.ensure_calendar(cx);

        // Set data
        cal.set_config(config);
        cal.set_all_cells(cells);

        // Draw
        let _ = cal.draw_walk(cx, scope, Walk::fit());
    }
}
