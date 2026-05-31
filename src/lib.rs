use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const 存储文件: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct 任务条目 {
    pub 序号: u32,
    pub 内容: String,
    pub 时间: String,
    pub 完成: bool,
}

/// 加载所有任务
pub fn 加载任务() -> Vec<任务条目> {
    let 路径 = PathBuf::from(存储文件);
    if !路径.exists() {
        return Vec::new();
    }
    let 数据 = match fs::read_to_string(&路径) {
        Ok(内容) => 内容,
        Err(e) => {
            eprintln!("警告: 无法读取任务文件 {}: {}", 路径.display(), e);
            return Vec::new();
        }
    };
    serde_json::from_str(&数据).unwrap_or_else(|_| {
        eprintln!("警告: 任务文件格式损坏，将使用空列表");
        Vec::new()
    })
}
/// 保存任务列表
pub fn 保存任务(任务列表: &[任务条目]) {
    let 数据 = serde_json::to_string_pretty(任务列表).expect("序列化失败");
    fs::write(存储文件, 数据).expect("写入失败");
}

/// 生成新序号
fn 下一个序号(任务列表: &[任务条目]) -> u32 {
    任务列表.iter().map(|t| t.序号).max().unwrap_or(0) + 1
}

// ----- 业务逻辑函数（增删改查）-----

pub fn 添加任务(内容: String, 时间: String) -> u32 {
    let mut 所有任务 = 加载任务();
    let 新序号 = 下一个序号(&所有任务);
    所有任务.push(任务条目 {
        序号: 新序号,
        内容,
        时间,
        完成: false,
    });
    保存任务(&所有任务);
    新序号
}

pub fn 列出任务() -> Vec<任务条目> {
    加载任务()
}

pub fn 修改任务内容(序号: u32, 新内容: String) -> bool {
    let mut 所有任务 = 加载任务();
    if let Some(任务) = 所有任务.iter_mut().find(|t| t.序号 == 序号) {
        任务.内容 = 新内容;
        保存任务(&所有任务);
        true
    } else {
        false
    }
}

pub fn 标记完成状态(序号: u32, 状态: bool) -> bool {
    let mut 所有任务 = 加载任务();
    if let Some(任务) = 所有任务.iter_mut().find(|t| t.序号 == 序号) {
        任务.完成 = 状态;
        保存任务(&所有任务);
        true
    } else {
        false
    }
}

pub fn 删除任务(序号: u32) -> bool {
    let mut 所有任务 = 加载任务();
    let 原长度 = 所有任务.len();
    所有任务.retain(|t| t.序号 != 序号);
    if 所有任务.len() < 原长度 {
        保存任务(&所有任务);
        true
    } else {
        false
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    /// 创建一个临时文件路径，并临时替换全局存储文件的常量（通过修改 const 不行，这里我们动态替换）
    /// 实际上因为 存储文件 是 const，无法在运行时更改。所以我们改为：每个测试函数自己创建临时路径，
    /// 并且手动调用加载/保存函数时传入路径？但我们的函数目前硬编码了全局常量。
    ///
    /// 因此更好的做法是重构 lib.rs，让函数接收一个可选的路径参数或使用内部可变静态变量。
    /// 为了快速演示测试，我们可以利用 std::env::set_current_dir 切换到临时目录。
    /// 但最简单且不影响原有接口的方式：在测试中改变当前工作目录到临时目录。
    fn with_temp_dir<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let temp_dir = tempfile::tempdir().expect("创建临时目录失败");
        let original_dir = std::env::current_dir().unwrap();
        // 切换工作目录到临时目录
        std::env::set_current_dir(&temp_dir).unwrap();
        let result = f();
        // 恢复原工作目录
        std::env::set_current_dir(original_dir).unwrap();
        result
    }

    #[test]
    fn test_添加任务() {
        with_temp_dir(|| {
            let 内容 = "测试任务".to_string();
            let 时间 = "2025-06-01".to_string();
            let id = 添加任务(内容.clone(), 时间.clone());
            assert_eq!(id, 1);
            let tasks = 列出任务();
            assert_eq!(tasks.len(), 1);
            assert_eq!(tasks[0].序号, 1);
            assert_eq!(tasks[0].内容, 内容);
            assert_eq!(tasks[0].时间, 时间);
            assert!(!tasks[0].完成);
        });
    }

    #[test]
    fn test_列出任务_空() {
        with_temp_dir(|| {
            let tasks = 列出任务();
            assert!(tasks.is_empty());
        });
    }

    #[test]
    fn test_修改任务内容() {
        with_temp_dir(|| {
            let id = 添加任务("原内容".to_string(), "2025-06-01".to_string());
            let success = 修改任务内容(id, "新内容".to_string());
            assert!(success);
            let tasks = 列出任务();
            assert_eq!(tasks[0].内容, "新内容");

            // 修改不存在的任务
            let fail = 修改任务内容(999, "不存在".to_string());
            assert!(!fail);
        });
    }

    #[test]
    fn test_标记完成状态() {
        with_temp_dir(|| {
            let id = 添加任务("任务".to_string(), "2025-06-01".to_string());
            assert!(标记完成状态(id, true));
            let tasks = 列出任务();
            assert!(tasks[0].完成);

            assert!(标记完成状态(id, false));
            let tasks = 列出任务();
            assert!(!tasks[0].完成);

            // 标记不存在的任务
            assert!(!标记完成状态(999, true));
        });
    }

    #[test]
    fn test_删除任务() {
        with_temp_dir(|| {
            let id1 = 添加任务("任务1".to_string(), "2025-06-01".to_string());
            let id2 = 添加任务("任务2".to_string(), "2025-06-02".to_string());
            assert_eq!(列出任务().len(), 2);

            assert!(删除任务(id1));
            assert_eq!(列出任务().len(), 1);
            assert_eq!(列出任务()[0].序号, id2);

            // 删除不存在的任务
            assert!(!删除任务(999));
        });
    }

    #[test]
    fn test_加载保存_文件损坏处理() {
        with_temp_dir(|| {
            // 写入无效 JSON
            fs::write(存储文件, "这不是json").unwrap();
            // 加载任务应该返回空 Vec，不会 panic
            let tasks = 加载任务();
            assert!(tasks.is_empty());
        });
    }
}
