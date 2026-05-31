use clap::{Parser, Subcommand};

// 导入 lib 中的公共接口
use clap_todo::{修改任务内容, 列出任务, 删除任务, 标记完成状态, 添加任务};

#[derive(Parser)]
#[command(name = "clap_todo")]
#[command(about = "简单的任务管理器", long_about = None)]
struct 命令行 {
    #[command(subcommand)]
    命令: 命令,
}

#[derive(Subcommand)]
enum 命令 {
    /// 添加一个任务 (例: td 添加 "写报告" "2025-06-01")
    添加 { 内容: String, 时间: String },
    /// 列出所有任务
    列出,
    /// 修改任务内容 (例: td 修改 1 "新内容")
    修改 { 序号: u32, 内容: String },
    /// 标记任务完成状态 (例: td 完成 1 true)
    完成 { 序号: u32, 状态: bool },
    /// 删除任务 (例: td 删除 1)
    删除 { 序号: u32 },
}

fn main() {
    let 参数 = 命令行::parse();

    match 参数.命令 {
        命令::添加 { 内容, 时间 } => {
            let 新序号 = 添加任务(内容, 时间);
            println!("✅ 任务添加成功，ID: {}", 新序号);
        }
        命令::列出 => {
            let 所有任务 = 列出任务();
            if 所有任务.is_empty() {
                println!("📭 暂无任务。");
                return;
            }
            for 任务 in 所有任务 {
                let 状态标志 = if 任务.完成 { "✔️" } else { "⏳" };
                println!(
                    "[{}] #{:2} | {} | 截止: {}",
                    状态标志, 任务.序号, 任务.内容, 任务.时间
                );
            }
        }
        命令::修改 { 序号, 内容 } => {
            if 修改任务内容(序号, 内容) {
                println!("🔄 任务 #{} 内容已更新。", 序号);
            } else {
                println!("❌ 未找到 ID 为 {} 的任务。", 序号);
            }
        }
        命令::完成 { 序号, 状态 } => {
            if 标记完成状态(序号, 状态) {
                let 状态描述 = if 状态 { "完成" } else { "未完成" };
                println!("✅ 任务 #{} 已标记为 {}", 序号, 状态描述);
            } else {
                println!("❌ 未找到 ID 为 {} 的任务。", 序号);
            }
        }
        命令::删除 { 序号 } => {
            if 删除任务(序号) {
                println!("🗑️ 任务 #{} 已删除。", 序号);
            } else {
                println!("❌ 未找到 ID 为 {} 的任务。", 序号);
            }
        }
    }
}
