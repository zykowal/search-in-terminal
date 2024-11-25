use std::time::Duration;

// 每页显示的搜索结果数量
pub const ITEMS_PER_PAGE: usize = 10;
// 事件轮询超时时间
pub const POLL_TIMEOUT: Duration = Duration::from_millis(100);
// 搜索频率限制时间
pub const RATE_LIMIT_DURATION: Duration = Duration::from_secs(1);

/// 搜索结果结构体
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,       // 搜索结果标题
    pub url: String,         // 搜索结果URL
    pub description: String, // 搜索结果描述
}
