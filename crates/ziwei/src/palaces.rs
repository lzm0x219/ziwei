//! 十二宫集合（内部存储，按下标 = [`Branch::index`]）。

use super::{branch::Branch, palace::Palace};

/// 十二宫集合，数组下标 = [`Branch::index`]（子=0）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Palaces(pub(crate) [Palace; 12]);

impl Palaces {
    /// 按宫支取宫（按值拷贝；`Palace` 为 `Copy`）。
    pub(crate) const fn get(self, branch: Branch) -> Palace {
        self.0[branch.index()]
    }

    /// 由构造管线填满十二支后装配；支互异是调用方不变量。
    pub(crate) const fn from_filled(palaces: [Palace; 12]) -> Self {
        Self(palaces)
    }
}
