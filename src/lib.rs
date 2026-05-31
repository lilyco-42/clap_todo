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
    let 数据 = fs::read_to_string(路径).expect("无法读取任务文件");
    serde_json::from_str(&数据).unwrap_or_else(|_| Vec::new())
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
