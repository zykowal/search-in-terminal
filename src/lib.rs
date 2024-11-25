pub mod core;
pub mod error;
pub mod search;
pub mod ui;

pub use core::app::App;
pub use core::config::CONFIG;
pub use error::types::SearchError;
pub use search::models::SearchResult;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::stdout;

pub async fn run() -> Result<()> {
    // 启用原始模式，禁用控制台的行缓存，直接将键盘输入传递给程序
    enable_raw_mode()?;
    // 获取标准输出流
    let mut stdout = stdout();
    // 进入交替屏幕模式，这样我们可以自由地在控制台上绘制窗口
    execute!(stdout, EnterAlternateScreen)?;
    // 创建一个 CrosstermBackend，用于将控制台作为一个 backend
    let backend = CrosstermBackend::new(stdout);
    // 创建一个 Terminal，用于在控制台上绘制窗口
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = ui::terminal::run_app(&mut terminal, &mut app).await;

    // 退出原始模式，恢复控制台的行缓存
    disable_raw_mode()?;
    // 退出交替屏幕模式，恢复控制台的默认屏幕
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    // 显示光标
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {}", err);
    }

    Ok(())
}
