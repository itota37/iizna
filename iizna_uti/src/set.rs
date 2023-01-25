// -------------------------
//
// iizna.
//
// iizna_uti/src/set.rs
// (C) 2023 IIZNA COMMUNITY.
//
//! 高速なハッシュを使用した集合を提供します。
// =========================

use super::hash::BuildFxHasher;
use std::collections::HashSet;

/// FxHasherを使用した集合型です。
pub type FxHashSet<T> = HashSet<T, BuildFxHasher>;
