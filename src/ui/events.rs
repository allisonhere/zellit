use crate::ui::color_picker::{picker_layout, ColorDragTarget, ColorPickerFocus, ColorPickerMode};
use crate::ui::state::{App, InputMode};
use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

pub fn process_key(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);
    let alt = key.modifiers.contains(KeyModifiers::ALT);

    match app.input_mode {
        InputMode::Preview => {
            // Any navigation clears the save message
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return true,
                KeyCode::Up | KeyCode::Char('k') => {
                    app.prev_element();
                    app.message = None;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.next_element();
                    app.message = None;
                }
                KeyCode::Tab => {
                    app.message = None;
                    if !app.selected_element.is_single_color() {
                        app.selected_attribute.cycle();
                    }
                }
                KeyCode::Char('1') => {
                    app.select_group_index(0);
                    app.message = None;
                }
                KeyCode::Char('2') => {
                    app.select_group_index(1);
                    app.message = None;
                }
                KeyCode::Char('3') => {
                    app.select_group_index(2);
                    app.message = None;
                }
                KeyCode::Char('4') => {
                    app.select_group_index(3);
                    app.message = None;
                }
                KeyCode::Char('5') => {
                    app.select_group_index(5);
                    app.message = None;
                }
                KeyCode::Char('/') => {
                    app.open_field_search();
                }
                KeyCode::Char('c') => {
                    app.message = None;
                    app.open_color_picker();
                }
                KeyCode::Char('s') => {
                    app.message = None;
                    app.open_theme_name_input();
                }
                KeyCode::Char('l') => {
                    app.message = None;
                    app.open_theme_load_dialog();
                }
                KeyCode::Char('a') => {
                    app.message = None;
                    if app.theme.name == "default" {
                        app.sync_theme_name_input();
                        app.input_mode = InputMode::ThemeNameInputApply;
                        app.message = Some(String::from("Name this theme before applying"));
                    } else {
                        app.apply_theme_to_zellij();
                    }
                }
                KeyCode::Char('?') => {
                    app.message = None;
                    app.help_scroll = 0;
                    app.input_mode = InputMode::Help;
                }
                KeyCode::Char('A') => {
                    app.message = None;
                    app.input_mode = InputMode::About;
                }
                KeyCode::Enter => {
                    app.message = None;
                    app.open_color_picker();
                }
                KeyCode::Char('y') => {
                    app.message = None;
                    app.yank_color();
                }
                KeyCode::Char('U') => {
                    app.start_self_update();
                }
                KeyCode::Char('p') => {
                    app.message = None;
                    app.paste_color();
                }
                KeyCode::Char('u') => {
                    app.message = None;
                    app.undo_color();
                }
                _ => {}
            }
        }
        InputMode::ColorPicker => {
            if app.color_editor.is_editing_text() {
                match key.code {
                    KeyCode::Esc => app.color_editor.cancel_text_edit(),
                    KeyCode::Enter => {
                        if app.color_editor.commit_text_edit() {
                            app.apply_current_color();
                        }
                    }
                    KeyCode::Backspace => {
                        app.color_editor.pop_input_char();
                    }
                    KeyCode::Char(c) => {
                        if app.color_editor.push_input_char(c)
                            && matches!(
                                app.color_editor.text_edit.as_ref().map(|edit| edit.target),
                                Some(crate::ui::color_picker::EditableField::Hex)
                            )
                            && app
                                .color_editor
                                .text_edit
                                .as_ref()
                                .map(|edit| edit.value.len())
                                == Some(6)
                        {
                            let _ = app.color_editor.commit_text_edit();
                            app.apply_current_color();
                        }
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc => app.close_color_picker(false),
                    KeyCode::Enter => match app.color_editor.focus {
                        ColorPickerFocus::HexField
                        | ColorPickerFocus::RgbField(_)
                        | ColorPickerFocus::HslFieldValue(_) => app.color_editor.start_editing_focused(),
                        ColorPickerFocus::ModeToggle => app.color_editor.toggle_mode(),
                        _ => app.close_color_picker(true),
                    },
                    KeyCode::BackTab => {
                        app.color_editor.focus_next(true);
                    }
                    KeyCode::Tab => {
                        app.color_editor.focus_next(false);
                    }
                    KeyCode::Char('f') => {
                        if !app.selected_element.is_single_color() {
                            app.switch_editing_attribute();
                        }
                    }
                    KeyCode::Char('m') => {
                        app.color_editor.toggle_mode();
                    }
                    KeyCode::Char('#') => {
                        app.color_editor.start_hex_input();
                    }
                    KeyCode::Char('y') => {
                        app.yank_color();
                    }
                    KeyCode::Up => {
                        let fine = if alt { 0.5 } else if shift { 10.0 } else { 2.0 };
                        match app.color_editor.focus {
                            ColorPickerFocus::RgbSlider(_) if app.color_editor.mode == ColorPickerMode::RgbSliders => {
                                app.color_editor.move_rgb_slider_focus(true);
                            }
                            ColorPickerFocus::HslField => {
                                app.color_editor.nudge_hsl_field(0.0, fine);
                                app.apply_current_color();
                            }
                            ColorPickerFocus::LightnessSlider => {
                                app.color_editor.adjust_focused_numeric(fine);
                                app.apply_current_color();
                            }
                            _ => {
                                if app.color_editor.adjust_focused_numeric(fine) {
                                    app.apply_current_color();
                                }
                            }
                        }
                    }
                    KeyCode::Down => {
                        let fine = if alt { 0.5 } else if shift { 10.0 } else { 2.0 };
                        match app.color_editor.focus {
                            ColorPickerFocus::RgbSlider(_) if app.color_editor.mode == ColorPickerMode::RgbSliders => {
                                app.color_editor.move_rgb_slider_focus(false);
                            }
                            ColorPickerFocus::HslField => {
                                app.color_editor.nudge_hsl_field(0.0, -fine);
                                app.apply_current_color();
                            }
                            ColorPickerFocus::LightnessSlider => {
                                app.color_editor.adjust_focused_numeric(-fine);
                                app.apply_current_color();
                            }
                            _ => {
                                if app.color_editor.adjust_focused_numeric(-fine) {
                                    app.apply_current_color();
                                }
                            }
                        }
                    }
                    KeyCode::Left => {
                        let rgb_delta = if alt { -20.0 } else if shift { -1.0 } else { -5.0 };
                        let hsl_delta = if alt { -10.0 } else if shift { -0.5 } else { -2.0 };
                        match app.color_editor.focus {
                            ColorPickerFocus::ModeToggle => app.color_editor.toggle_mode(),
                            ColorPickerFocus::RgbSlider(_) => {
                                app.color_editor.adjust_rgb_slider_selection(rgb_delta as i32);
                                app.apply_current_color();
                            }
                            ColorPickerFocus::HslField => {
                                app.color_editor.nudge_hsl_field(hsl_delta, 0.0);
                                app.apply_current_color();
                            }
                            _ => {
                                if app.color_editor.adjust_focused_numeric(if matches!(app.color_editor.focus, ColorPickerFocus::RgbField(_)) { rgb_delta } else { hsl_delta }) {
                                    app.apply_current_color();
                                }
                            }
                        }
                    }
                    KeyCode::Right => {
                        let rgb_delta = if alt { 20.0 } else if shift { 1.0 } else { 5.0 };
                        let hsl_delta = if alt { 10.0 } else if shift { 0.5 } else { 2.0 };
                        match app.color_editor.focus {
                            ColorPickerFocus::ModeToggle => app.color_editor.toggle_mode(),
                            ColorPickerFocus::RgbSlider(_) => {
                                app.color_editor.adjust_rgb_slider_selection(rgb_delta as i32);
                                app.apply_current_color();
                            }
                            ColorPickerFocus::HslField => {
                                app.color_editor.nudge_hsl_field(hsl_delta, 0.0);
                                app.apply_current_color();
                            }
                            _ => {
                                if app.color_editor.adjust_focused_numeric(if matches!(app.color_editor.focus, ColorPickerFocus::RgbField(_)) { rgb_delta } else { hsl_delta }) {
                                    app.apply_current_color();
                                }
                            }
                        }
                    }
                    KeyCode::PageUp => {
                        if app.color_editor.adjust_focused_numeric(10.0) {
                            app.apply_current_color();
                        }
                    }
                    KeyCode::PageDown => {
                        if app.color_editor.adjust_focused_numeric(-10.0) {
                            app.apply_current_color();
                        }
                    }
                    _ => {}
                }
            }
        }
        InputMode::ThemeNameInput => match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Preview;
                app.message = Some(String::from("Save cancelled"));
            }
            KeyCode::Enter => {
                app.save_theme_as_input_name();
            }
            KeyCode::Backspace => {
                app.pop_theme_name_char();
            }
            KeyCode::Char(c) => {
                app.push_theme_name_char(c);
            }
            _ => {}
        },
        InputMode::ThemeNameInputApply => match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Preview;
                app.message = Some(String::from("Save cancelled"));
            }
            KeyCode::Enter => {
                app.save_and_apply_theme_as_input_name();
            }
            KeyCode::Backspace => {
                app.pop_theme_name_char();
            }
            KeyCode::Char(c) => {
                app.push_theme_name_char(c);
            }
            _ => {}
        },
        InputMode::ThemeLoad => {
            if app.search_focused {
                match key.code {
                    KeyCode::Enter | KeyCode::Down => {
                        // Commit search — switch to card navigation
                        app.search_focused = false;
                        app.move_theme_selection_to(0);
                    }
                    KeyCode::Up => {
                        app.search_focused = false;
                        app.move_theme_selection_to(app.loadable_themes.len().saturating_sub(1));
                    }
                    KeyCode::Esc => {
                        app.theme_search_query = String::new();
                        app.search_focused = false;
                        app.apply_filter_to_list();
                        app.move_theme_selection_to(0);
                    }
                    KeyCode::Backspace => {
                        app.theme_search_query.pop();
                        app.apply_filter_to_list();
                        app.move_theme_selection_to(0);
                        if app.theme_search_query.is_empty() {
                            app.search_focused = false;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.theme_search_query.push(c);
                        app.apply_filter_to_list();
                        app.move_theme_selection_to(0);
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc => {
                        if !app.theme_search_query.is_empty() {
                            app.theme_search_query = String::new();
                            app.apply_filter_to_list();
                            app.move_theme_selection_to(0);
                        } else {
                            app.cancel_theme_load();
                        }
                    }
                    KeyCode::Enter => {
                        app.load_selected_theme();
                    }
                    KeyCode::Char('a') => {
                        app.apply_selected_theme();
                    }
                    KeyCode::Char('b') => {
                        app.set_theme_filter(crate::ui::state::ThemeFilter::Builtin);
                    }
                    KeyCode::Char('s') => {
                        app.set_theme_filter(crate::ui::state::ThemeFilter::Saved);
                    }
                    KeyCode::Char('r') => {
                        app.begin_rename_selected_theme();
                    }
                    KeyCode::Char('d') => {
                        app.begin_delete_selected_theme();
                    }
                    KeyCode::Up => {
                        app.move_theme_selection_up();
                    }
                    KeyCode::Down => {
                        app.move_theme_selection_down();
                    }
                    KeyCode::Backspace => {
                        if !app.theme_search_query.is_empty() {
                            app.theme_search_query.pop();
                            app.search_focused = true;
                            app.apply_filter_to_list();
                            app.move_theme_selection_to(0);
                        }
                    }
                    KeyCode::Char(c) => {
                        // Any other char starts/resumes search
                        app.search_focused = true;
                        app.theme_search_query.push(c);
                        app.apply_filter_to_list();
                        app.move_theme_selection_to(0);
                    }
                    _ => {}
                }
            }
        }
        InputMode::FieldSearch => match key.code {
            KeyCode::Esc => {
                app.cancel_field_search();
            }
            KeyCode::Enter => {
                app.commit_field_search();
            }
            KeyCode::Backspace => {
                app.pop_field_search_char();
            }
            KeyCode::Up => {
                app.move_field_search(-1);
            }
            KeyCode::Down => {
                app.move_field_search(1);
            }
            KeyCode::Char(c) => {
                app.push_field_search_char(c);
            }
            _ => {}
        },
        InputMode::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                app.input_mode = InputMode::Preview;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.help_scroll = app.help_scroll.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.help_scroll = app.help_scroll.saturating_add(1);
            }
            KeyCode::PageUp => {
                app.help_scroll = app.help_scroll.saturating_sub(8);
            }
            KeyCode::PageDown => {
                app.help_scroll = app.help_scroll.saturating_add(8);
            }
            KeyCode::Home => {
                app.help_scroll = 0;
            }
            KeyCode::End => {
                app.help_scroll = u16::MAX;
            }
            _ => {}
        },
        InputMode::About => {
            app.input_mode = InputMode::Preview;
        }
        InputMode::ThemeLoadRename => match key.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::ThemeLoad;
                app.message = None;
            }
            KeyCode::Enter => {
                app.commit_rename_theme();
            }
            KeyCode::Backspace => {
                app.pop_theme_name_char();
            }
            KeyCode::Char(c) => {
                app.push_theme_name_char(c);
            }
            _ => {}
        },
        InputMode::ThemeLoadDeleteConfirm => match key.code {
            KeyCode::Char('y') => {
                app.confirm_delete_theme();
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.input_mode = InputMode::ThemeLoad;
                app.message = None;
            }
            _ => {}
        },
        InputMode::UpdateRestartConfirm => match key.code {
            KeyCode::Enter | KeyCode::Char('r') | KeyCode::Char('y') => {
                app.confirm_restart();
                return true;
            }
            KeyCode::Char('l') | KeyCode::Char('n') | KeyCode::Esc => {
                app.defer_restart();
            }
            _ => {}
        },
    }
    false
}

pub fn process_mouse(app: &mut App, mouse: MouseEvent) {
    if app.input_mode != InputMode::ColorPicker {
        return;
    }
    let Ok((w, h)) = crossterm::terminal::size() else {
        return;
    };
    let rects = picker_layout(
        ratatui::layout::Rect::new(0, 0, w, h),
        app.color_editor.mode,
        app.selected_element.preferred_picker_anchor(),
    );
    let point_focus = app.color_editor.focus_for_point(&rects, mouse.column, mouse.row);

    let update_drag = |app: &mut App,
                       target: ColorDragTarget,
                       column: u16,
                       row: u16,
                       rects: &crate::ui::color_picker::PickerRects| {
        match target {
            ColorDragTarget::HslField => {
                if rects.main_view.width > 0 && rects.main_view.height > 0 {
                    let x = column.saturating_sub(rects.main_view.x).min(rects.main_view.width.saturating_sub(1));
                    let y = row.saturating_sub(rects.main_view.y).min(rects.main_view.height.saturating_sub(1));
                    let x_frac = x as f32 / rects.main_view.width.saturating_sub(1).max(1) as f32;
                    let y_frac = y as f32 / rects.main_view.height.saturating_sub(1).max(1) as f32;
                    app.color_editor.update_from_hsl_field(x_frac, y_frac);
                    app.apply_current_color();
                }
            }
            ColorDragTarget::LightnessSlider => {
                if rects.aux_slider.height > 0 {
                    let y = row.saturating_sub(rects.aux_slider.y).min(rects.aux_slider.height.saturating_sub(1));
                    let y_frac = y as f32 / rects.aux_slider.height.saturating_sub(1).max(1) as f32;
                    app.color_editor.update_lightness_from_frac(y_frac);
                    app.apply_current_color();
                }
            }
        }
    };

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(focus) = point_focus {
                match focus {
                    ColorPickerFocus::ModeToggle => {
                        let mid = rects.mode_switch.x + rects.mode_switch.width / 2;
                        if mouse.column >= mid && app.color_editor.mode != ColorPickerMode::HslField {
                            app.color_editor.toggle_mode();
                        } else if mouse.column < mid && app.color_editor.mode != ColorPickerMode::RgbSliders {
                            app.color_editor.toggle_mode();
                        }
                    }
                    ColorPickerFocus::HslField => {
                        app.color_editor.set_drag_target(Some(ColorDragTarget::HslField));
                        update_drag(app, ColorDragTarget::HslField, mouse.column, mouse.row, &rects);
                    }
                    ColorPickerFocus::LightnessSlider => {
                        app.color_editor.set_drag_target(Some(ColorDragTarget::LightnessSlider));
                        update_drag(app, ColorDragTarget::LightnessSlider, mouse.column, mouse.row, &rects);
                    }
                    other => app.color_editor.set_focus(other),
                }
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if let Some(target) = app.color_editor.drag_target {
                update_drag(app, target, mouse.column, mouse.row, &rects);
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            app.color_editor.set_drag_target(None);
        }
        _ => {}
    }
}

pub fn run(mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    app.start_update_check();
    crossterm::style::force_color_output(true);

    let mut terminal = ratatui::init();
    terminal.show_cursor()?;

    use crossterm::event::{self as ct_event};
    use crossterm::execute;
    use std::time::Duration;

    execute!(terminal.backend_mut(), crossterm::event::EnableMouseCapture)?;

    loop {
        app.poll_update_channel();
        terminal.draw(|frame| app.render(frame))?;
        if ct_event::poll(Duration::from_millis(100))? {
            match ct_event::read()? {
                ct_event::Event::Key(key) => {
                    if process_key(&mut app, key) {
                        break;
                    }
                }
                ct_event::Event::Mouse(mouse) => process_mouse(&mut app, mouse),
                _ => {}
            }
        }
    }

    execute!(terminal.backend_mut(), crossterm::event::DisableMouseCapture)?;
    ratatui::restore();

    if app.restart_after_exit {
        let exe = std::env::current_exe()?;
        let args: Vec<_> = std::env::args_os().skip(1).collect();
        std::process::Command::new(exe).args(args).spawn()?;
    }

    Ok(())
}
