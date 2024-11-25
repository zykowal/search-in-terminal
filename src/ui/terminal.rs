use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

use crate::{search::models::POLL_TIMEOUT, App};

/// 渲染用户界面
pub fn ui(frame: &mut Frame, app: &mut App) {
    // 将 Frame 分割成四个部分：输入框、loading指示器、搜索结果列表和状态栏
    // 每个部分的高度分别是3行、1行、至少5行和3行
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 输入框
            Constraint::Length(1), // loading指示器
            Constraint::Min(5),    // 搜索结果列表
            Constraint::Length(3), // 状态栏
        ])
        .split(frame.area());

    let input_content = if app.is_loading {
        // 如果正在搜索，显示一个Loading指示器
        Line::from(vec![
            Span::raw(&app.input), // 显示当前的输入
            Span::styled(
                " [Searching...]",                  // 加一个Loading指示器
                Style::default().fg(Color::Yellow), // 并且高亮显示
            ),
        ])
    } else {
        // 如果不是正在搜索，显示一个竖线
        Line::from(vec![
            Span::raw(&app.input), // 显示当前的输入
            Span::styled(
                "|",                                // 加一个竖线
                Style::default().fg(Color::Yellow), // 并且高亮显示
            ),
        ])
    };

    let input = Paragraph::new(input_content).block(Block::default().borders(Borders::ALL).title(
        if app.input_mode {
            "Input (Press Esc to stop editing)"
        } else {
            "Input (Press 'i' to edit)"
        },
    ));
    frame.render_widget(input, chunks[0]);

    // Search stats
    let stats_text = if !app.search_results.is_empty() {
        format!(" Found {} results", app.search_results.len())
    } else if app.is_loading {
        " Searching...".to_string()
    } else {
        " No results yet".to_string()
    };

    let stats = Paragraph::new(Line::from(vec![
        Span::styled(
            "●",
            Style::default().fg(if app.is_loading {
                Color::Yellow
            } else if !app.search_results.is_empty() {
                Color::Green
            } else {
                Color::Gray
            }),
        ),
        Span::raw(stats_text),
        Span::raw(" | "),
        Span::styled(
            format!(
                "Engine: {} (Press 'e' to change)",
                app.search_engine.as_str()
            ),
            Style::default().fg(Color::Cyan),
        ),
    ]));
    frame.render_widget(stats, chunks[1]);

    // Results area
    let (start_index, end_index) = app.current_page_range();
    let results: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .skip(start_index)
        .take(end_index - start_index)
        .map(|(i, result)| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    format!("{}. {}", i + 1, &result.title),
                    Style::default().fg(if i == app.selected_index {
                        Color::Gray
                    } else {
                        Color::default()
                    }),
                )),
                Line::from(Span::styled(&result.url, Style::default().fg(Color::Blue))),
                Line::from(Span::raw(&result.description)),
                Line::from(""),
            ])
        })
        .collect();

    let results_list = List::new(results)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "Results (Page {}/{})",
            app.page + 1,
            app.total_pages,
        )))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_stateful_widget(results_list, chunks[2], &mut app.list_state);
    // Status area
    if let Some(warn) = &app.warning_message {
        let warning_message = Paragraph::new(warn.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Warning"));
        frame.render_widget(warning_message, chunks[3]);
    } else if let Some(error) = &app.error_message {
        let error_message = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        frame.render_widget(error_message, chunks[3]);
    } else {
        let help = if app.input_mode {
            "Ctrl+U: Clear Input | Press Esc to exit input mode | Enter to search"
        } else {
            "j/k: Navigate | h/l: Change Page | r: Clear Results | Enter: Open URL | i: Input | q: Quit | e: Change Engine"
        };
        let status = Paragraph::new(help)
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Help"));
        frame.render_widget(status, chunks[3]);
    }
}

/// 运行应用程序
pub async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(POLL_TIMEOUT)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') if !app.input_mode => break,
                        KeyCode::Char('i') if !app.input_mode => {
                            app.input_mode = true;
                        }
                        KeyCode::Esc if app.input_mode => {
                            app.input_mode = false;
                        }
                        KeyCode::Char('h')
                            if app.input_mode
                                && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            app.input.pop();
                        }
                        KeyCode::Char('u')
                            if app.input_mode
                                && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            app.clear_input();
                        }
                        KeyCode::Char('h')
                            if app.input_mode
                                && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            app.input.pop();
                        }
                        KeyCode::Char('r') if !app.input_mode => {
                            app.clear_results();
                        }
                        KeyCode::Char('e') if !app.input_mode => {
                            app.search_engine = app.search_engine.next();
                            if !app.input.is_empty() {
                                app.perform_search().await?;
                            }
                        }
                        KeyCode::Char(c) if app.input_mode => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace if app.input_mode => {
                            app.input.pop();
                        }
                        KeyCode::Enter if app.input_mode => {
                            app.perform_search().await?;
                        }
                        KeyCode::Enter if !app.input_mode => {
                            app.open_selected_url()?;
                        }
                        KeyCode::Char('j') | KeyCode::Down if !app.input_mode => {
                            let i = match app.list_state.selected() {
                                Some(i) => i.saturating_add(1),
                                None => 0,
                            };
                            app.list_state.select(Some(i));
                        }
                        KeyCode::Char('k') | KeyCode::Up if !app.input_mode => {
                            let i = match app.list_state.selected() {
                                Some(i) => i.saturating_sub(1),
                                None => 0,
                            };
                            app.list_state.select(Some(i));
                        }
                        KeyCode::Char('h') | KeyCode::Left if !app.input_mode => {
                            app.change_page(-1).await?;
                        }
                        KeyCode::Char('l') | KeyCode::Right if !app.input_mode => {
                            app.change_page(1).await?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
