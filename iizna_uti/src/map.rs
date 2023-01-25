// -------------------------
//
// iizna.
//
// iizna_uti/src/map.rs
// (C) 2023 IIZNA COMMUNITY.
//
//! 高速なハッシュを使用した連想配列を提供します。
// =========================

use super::hash::BuildFxHasher;
use std::collections::HashMap;

/// FxHasherを使用した連想配列型です。
pub type FxHashMap<K, V> = HashMap<K, V, BuildFxHasher>;
