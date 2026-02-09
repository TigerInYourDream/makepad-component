// ============================================================================
// A2UI Component Tools Definition
// ============================================================================

fn get_a2ui_tools() -> Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "create_text",
                "description": "Create a text/label component to display static or dynamic text",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID (e.g., 'title', 'label-1')"},
                        "text": {"type": "string", "description": "Static text to display"},
                        "dataPath": {"type": "string", "description": "JSON pointer for dynamic text binding (e.g., '/user/name')"},
                        "style": {"type": "string", "enum": ["h1", "h3", "caption", "body"], "description": "Text style: h1=large title, h3=subtitle, caption=small, body=normal"}
                    },
                    "required": ["id"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_button",
                "description": "Create a clickable button that triggers an action",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Button text label"},
                        "action": {"type": "string", "description": "Action name triggered on click (e.g., 'submit', 'cancel')"},
                        "primary": {"type": "boolean", "description": "If true, button is highlighted as primary action"}
                    },
                    "required": ["id", "label", "action"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_textfield",
                "description": "Create a text input field for user input",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for data binding (e.g., '/form/email')"},
                        "placeholder": {"type": "string", "description": "Placeholder text shown when empty"}
                    },
                    "required": ["id", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_checkbox",
                "description": "Create a checkbox toggle for boolean values",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Label text next to checkbox"},
                        "dataPath": {"type": "string", "description": "JSON pointer for boolean binding (e.g., '/settings/darkMode')"}
                    },
                    "required": ["id", "label", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_slider",
                "description": "Create a slider for numeric value selection",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for numeric binding (e.g., '/volume')"},
                        "min": {"type": "number", "description": "Minimum value"},
                        "max": {"type": "number", "description": "Maximum value"},
                        "step": {"type": "number", "description": "Step increment (default: 1)"}
                    },
                    "required": ["id", "dataPath", "min", "max"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_card",
                "description": "Create a card container with visual styling (elevation, border)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "childId": {"type": "string", "description": "ID of the child component inside the card"}
                    },
                    "required": ["id", "childId"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_column",
                "description": "Create a vertical layout container (stacks children top to bottom)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_row",
                "description": "Create a horizontal layout container (arranges children left to right)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "set_data",
                "description": "Set initial data value in the data model. For simple values use stringValue/numberValue/booleanValue. For nested objects (like calendar cells), use mapValue with a JSON object.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "JSON pointer path (e.g., '/volume', '/calendar/cells/1/0')"},
                        "stringValue": {"type": "string", "description": "String value to set"},
                        "numberValue": {"type": "number", "description": "Number value to set"},
                        "booleanValue": {"type": "boolean", "description": "Boolean value to set"},
                        "mapValue": {"type": "object", "description": "Nested object value (e.g., {\"line1\": \"Title\", \"line2\": \"Subtitle\", \"description\": \"Details\"})"}
                    },
                    "required": ["path"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_chart",
                "description": "Create a chart. Types: bar, line, pie, area (filled line), scatter (series[0]=X,series[1]=Y), radar (labels=axes, values per axis), gauge (series[0].values[0]=value, maxValue=max), bubble (series[0]=X,[1]=Y,[2]=Size), candlestick (4 series: open,high,low,close), heatmap (series=rows, labels=columns), treemap (series[0]=sizes), chord (labels=entities, series=flow matrix rows: series[i].values[j]=flow from i to j), sankey (labels=node names, series[0]=source indices, series[1]=target indices, series[2]=flow values).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "chartType": {"type": "string", "enum": ["bar", "line", "pie", "area", "scatter", "radar", "gauge", "bubble", "candlestick", "heatmap", "treemap", "chord", "sankey"], "description": "Chart type"},
                        "title": {"type": "string", "description": "Chart title displayed above the chart"},
                        "labels": {"type": "array", "items": {"type": "string"}, "description": "Category labels / axis names / column headers"},
                        "values": {"type": "array", "items": {"type": "number"}, "description": "Data values for a single series"},
                        "series": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string", "description": "Series name (for legend)"},
                                    "values": {"type": "array", "items": {"type": "number"}, "description": "Data values"}
                                },
                                "required": ["values"]
                            },
                            "description": "Multiple data series (alternative to 'values' for multi-series charts)"
                        },
                        "colors": {"type": "array", "items": {"type": "string"}, "description": "Color palette as hex strings"},
                        "width": {"type": "number", "description": "Chart width in pixels (default: 400)"},
                        "height": {"type": "number", "description": "Chart height in pixels (default: 300)"},
                        "maxValue": {"type": "number", "description": "Max value for gauge chart (default: 100)"}
                    },
                    "required": ["id", "chartType", "labels"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "render_ui",
                "description": "Finalize and render the UI with the specified root component. Call this LAST after creating all components.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "rootId": {"type": "string", "description": "ID of the root component (usually a column or row)"},
                        "title": {"type": "string", "description": "Optional title for the UI surface"}
                    },
                    "required": ["rootId"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "generate_music",
                "description": "Generate AI music using Mureka API. Returns a job ID that will be polled for completion. The music generation takes about 45 seconds.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "prompt": {"type": "string", "description": "Description of the music to generate (e.g., 'relaxing piano melody', 'upbeat electronic dance')"},
                        "instrumental": {"type": "boolean", "description": "If true, generate instrumental music without lyrics. Default true."}
                    },
                    "required": ["prompt"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_audio_player",
                "description": "Create an audio player component to play music. Use this after generate_music returns an audio URL.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "url": {"type": "string", "description": "Audio URL to play"},
                        "title": {"type": "string", "description": "Song title"},
                        "artist": {"type": "string", "description": "Artist name (optional)"}
                    },
                    "required": ["id", "url", "title"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_calendar",
                "description": "Create a calendar/grid component for weekly planners, schedules, timetables. Renders as a table with column headers, row labels, and cells bound to data model. Each cell has line1 (main text) and line2 (sub text). Click a cell to see its detail (description, time, tips). After creating, use set_calendar_data to populate ALL cells in one call. Row 0 is typically empty (header spacer). Set /calendar/grandTotal for footer text.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "title": {"type": "string", "description": "Calendar title"},
                        "columns": {"type": "integer", "description": "Number of columns (default 7 for weekly view)"},
                        "columnHeaders": {"type": "array", "items": {"type": "string"}, "description": "Column header labels (e.g., ['Mon 2/10', 'Tue 2/11', ...])"},
                        "columnSubtitles": {"type": "array", "items": {"type": "string"}, "description": "Column subtitle labels (e.g., ['Shibuya', 'Asakusa', ...])"},
                        "rowLabels": {"type": "array", "items": {"type": "string"}, "description": "Row labels (e.g., ['', '🌅 AM', '☀️ PM', '🌙 Eve', '💰'])"},
                        "rowColorHints": {"type": "array", "items": {"type": "string"}, "description": "Row color hints: 'header', 'morning', 'afternoon', 'evening', 'budget'"},
                        "cellsDataPath": {"type": "string", "description": "Data model path for cell data (default: '/calendar/cells')"},
                        "footerDataPath": {"type": "string", "description": "Data model path for footer text (default: '/calendar/grandTotal')"}
                    },
                    "required": ["id", "title", "columnHeaders", "rowLabels"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "set_calendar_data",
                "description": "Bulk-set ALL calendar cell data in one call. This is much more efficient than calling set_data for each cell individually. Provide a 2D array of rows, where each row is an array of cell objects. Each cell object has: line1 (main text), line2 (sub text), time (time range), description (detailed description shown in detail panel), tips (travel tips shown in detail panel). Row 0 should be empty cells for header spacer. IMPORTANT: You MUST provide time, description, and tips for content rows (row 1+) so the detail panel shows rich information when a cell is clicked.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "rows": {
                            "type": "array",
                            "description": "2D array: rows[rowIndex][colIndex] = cell object. Row 0 = header spacer (empty cells). Row 1+ = content rows.",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "line1": {"type": "string", "description": "Main text (e.g., location name)"},
                                        "line2": {"type": "string", "description": "Sub text (e.g., activity type)"},
                                        "time": {"type": "string", "description": "Time range (e.g., '9:00 - 12:00')"},
                                        "description": {"type": "string", "description": "Detailed description for detail panel"},
                                        "tips": {"type": "string", "description": "Travel tips for detail panel"}
                                    }
                                }
                            }
                        },
                        "footer": {"type": "string", "description": "Footer text (e.g., '💰 GRAND TOTAL: $670')"},
                        "cellsDataPath": {"type": "string", "description": "Data model path for cells (default: '/calendar/cells')"},
                        "footerDataPath": {"type": "string", "description": "Data model path for footer (default: '/calendar/grandTotal')"}
                    },
                    "required": ["rows"]
                }
            }
        }
    ])
}


fn build_component_json(name: &str, args: &Value) -> Option<Value> {
    let id = args.get("id").and_then(|v| v.as_str()).unwrap_or("comp");

    match name {
        "create_text" => {
            let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let usage_hint = args.get("usage_hint").and_then(|v| v.as_str());
            let mut text_obj = json!({
                "text": {"literalString": text}
            });
            if let Some(hint) = usage_hint {
                text_obj["usageHint"] = json!(hint);
            }
            Some(json!({"id": id, "component": {"Text": text_obj}}))
        }
        "create_audio_player" => {
            let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let title = args.get("title").and_then(|v| v.as_str()).unwrap_or("Audio");
            let artist = args.get("artist").and_then(|v| v.as_str());
            let mut audio_obj = json!({
                "url": {"literalString": url},
                "title": {"literalString": title}
            });
            if let Some(a) = artist {
                audio_obj["artist"] = json!({"literalString": a});
            }
            Some(json!({"id": id, "component": {"AudioPlayer": audio_obj}}))
        }
        "create_column" => {
            let children = args.get("children").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            Some(json!({"id": id, "component": {"Column": {"children": {"explicitList": children}}}}))
        }
        "create_row" => {
            let children = args.get("children").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            Some(json!({"id": id, "component": {"Row": {"children": {"explicitList": children}}}}))
        }
        "create_button" => {
            let label = args.get("label").and_then(|v| v.as_str()).unwrap_or("Button");
            let label_id = format!("{}-label", id);
            Some(json!({"id": id, "component": {"Button": {"child": label_id, "primary": true}}}))
        }
        "create_card" => {
            let child = args.get("child").and_then(|v| v.as_str()).unwrap_or("");
            Some(json!({"id": id, "component": {"Card": {"child": child}}}))
        }
        "create_calendar" => {
            let title = args.get("title").and_then(|v| v.as_str()).unwrap_or("Calendar");
            let columns = args.get("columns").and_then(|v| v.as_u64()).unwrap_or(7);
            let column_headers = args.get("columnHeaders").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            let column_subtitles = args.get("columnSubtitles").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            let row_labels = args.get("rowLabels").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            let row_color_hints = args.get("rowColorHints").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            let cells_path = args.get("cellsDataPath").and_then(|v| v.as_str()).unwrap_or("/calendar/cells");
            let footer_path = args.get("footerDataPath").and_then(|v| v.as_str()).unwrap_or("/calendar/grandTotal");

            let mut cal_obj = json!({
                "title": {"literalString": title},
                "columns": columns,
                "columnHeaders": column_headers,
                "rowLabels": row_labels,
                "cells": {"path": cells_path},
                "footer": {"path": footer_path}
            });
            if !column_subtitles.is_empty() {
                cal_obj["columnSubtitles"] = json!(column_subtitles);
            }
            if !row_color_hints.is_empty() {
                cal_obj["rowColorHints"] = json!(row_color_hints);
            }
            Some(json!({"id": id, "component": {"Calendar": cal_obj}}))
        }
        "render_ui" => {
            // This is the final layout - handled separately
            None
        }
        _ => None
    }
}

