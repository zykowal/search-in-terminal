use anyhow::Result;
use std::time::Instant;

use ratatui::widgets::ListState;

use crate::{
    search::{
        engine::{Google, SearchEngine},
        models::{ITEMS_PER_PAGE, RATE_LIMIT_DURATION},
    },
    SearchResult,
};

/// 应用程序状态结构体
#[derive(Debug)]
pub struct App {
    pub input: String,                     // 用户输入的搜索词
    pub search_results: Vec<SearchResult>, // 搜索结果列表
    pub selected_index: usize,             // 当前选中的结果索引
    pub input_mode: bool,                  // 是否处于输入模式
    pub error_message: Option<String>,     // 错误信息
    pub warning_message: Option<String>,   // 警告信息
    pub scroll_offset: usize,              // 滚动偏移量
    pub page: usize,                       // 当前页码
    pub total_pages: usize,                // 总页数
    pub last_search: Option<Instant>,      // 上次搜索时间
    pub is_loading: bool,                  // 是否正在加载
    pub search_engine: SearchEngine,       // 当前选择的搜索引擎
    pub list_state: ListState,             // 列表状态
    pub start: u16,                        // 搜索起始位置
}

impl App {
    /// 创建新的应用程序实例
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            input: String::new(),
            search_results: Vec::new(),
            selected_index: 0,
            input_mode: true,
            error_message: None,
            warning_message: None,
            scroll_offset: 0,
            page: 0,
            total_pages: 0,
            last_search: None,
            is_loading: false,
            search_engine: SearchEngine::Google(Google),
            list_state,
            start: 0,
        }
    }

    /// 设置搜索结果的总页数
    pub fn total_pages(&mut self) {
        if self.search_results.is_empty() {
            self.total_pages = 1;
        } else {
            self.total_pages = (self.search_results.len() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        }
    }

    /// 获取当前页的结果范围
    pub fn current_page_range(&self) -> (usize, usize) {
        if self.search_results.is_empty() {
            return (0, 0);
        }

        let start_index = self.page * ITEMS_PER_PAGE;
        let end_index = if self.page == self.total_pages - 1 {
            self.search_results.len()
        } else {
            start_index + ITEMS_PER_PAGE
        };
        (start_index, end_index)
    }

    /// 切换页面
    pub async fn change_page(&mut self, direction: i32) -> Result<()> {
        if self.search_results.is_empty() {
            return Ok(());
        }

        if let Some(new_page) = (self.page as i32).checked_add(direction) {
            if new_page >= 0 && new_page < self.total_pages as i32 {
                self.page = new_page as usize;
                let (start_index, _) = self.current_page_range();
                self.selected_index = start_index;
                self.scroll_offset = 0;
                self.list_state.select(Some(0));
            } else if new_page >= 0 && direction > 0 {
                // 如果尝试前进到下一页但已经是最后一页，则尝试获取更多结果
                self.next_search().await?;
                if self.page < self.total_pages - 1 {
                    self.page += 1;
                    let (start_index, _) = self.current_page_range();
                    self.selected_index = start_index;
                    self.scroll_offset = 0;
                    self.list_state.select(Some(0));
                }
            }
        }

        Ok(())
    }

    /// 清空输入
    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    /// 清空搜索结果
    pub fn clear_results(&mut self) {
        self.search_results.clear();
        self.selected_index = 0;
        self.page = 0;
        self.total_pages = 0;
        self.scroll_offset = 0;
        self.list_state.select(Some(0));
        self.error_message = None;
        self.warning_message = None;
        self.input.clear();
        self.input_mode = true;
        self.start = 0;
    }

    pub async fn next_search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        self.error_message = None;
        self.warning_message = None;

        // 更新起始位置
        self.start = self.start.saturating_add(10);

        // 根据选择的搜索引擎执行搜索
        let results = match self.search_engine.search(&self.input, self.start).await {
            Ok(results) => Ok(results),
            Err(e) => {
                self.error_message = Some(format!("Next search failed: {}", e));
                Err(e)
            }
        };

        match results {
            Ok(mut results) => {
                // 检查是否有重复的结果
                let existing_urls: std::collections::HashSet<_> =
                    self.search_results.iter().map(|r| &r.url).collect();

                // 只添加不重复的结果
                results.retain(|result| !existing_urls.contains(&result.url));

                if !results.is_empty() {
                    self.search_results.extend(results);
                    self.total_pages();
                } else {
                    self.warning_message = Some("No more results found".to_string());
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Next search failed: {}", e));
            }
        }

        Ok(())
    }

    /// 执行搜索操作
    pub async fn perform_search(&mut self) -> Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        // 检查搜索频率限制
        if let Some(last_search) = self.last_search {
            if last_search.elapsed() < RATE_LIMIT_DURATION {
                self.warning_message =
                    Some("Please wait a moment before searching again".to_string());
                return Ok(());
            }
        }

        self.is_loading = true;
        self.error_message = None;
        self.warning_message = None;
        self.start = 0; // 重置起始位置

        // 根据选择的搜索引擎执行搜索
        let results = match self.search_engine.search(&self.input, self.start).await {
            Ok(results) => Ok(results),
            Err(e) => {
                self.error_message = Some(format!("Search failed: {}", e));
                Err(e)
            }
        };

        match results {
            Ok(results) => {
                self.search_results = results;
                self.selected_index = 0;
                self.page = 0;
                self.scroll_offset = 0;
                self.list_state.select(Some(0));
                self.total_pages();
            }
            Err(e) => {
                self.error_message = Some(format!("Search failed: {}", e));
                self.clear_results();
            }
        }

        self.is_loading = false;
        self.input_mode = false;
        self.last_search = Some(Instant::now());
        Ok(())
    }

    /// 打开选中的URL
    pub fn open_selected_url(&mut self) -> Result<()> {
        let (start_index, _) = self.current_page_range();
        if let Some(selected) = self.list_state.selected() {
            if let Some(result) = self.search_results.get(start_index + selected) {
                open::that(&result.url)?;
            }
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
