// 终端搜索应用程序
// 一个基于终端的Google搜索工具，支持实时搜索结果显示和交互式导航

use anyhow::Result;
use search_in_terminal::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
