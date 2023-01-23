// --------------------
//
// iizna.
//
// iizna_uti/src/lib.rs
// (C) 2023 IIZNA COMMUNITY.
//
//! iiznaで他の内部クレートに非依存な機能を提供します。

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
