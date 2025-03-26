#![allow(dead_code)]
pub mod horizontal {
    use crate::Set;
    /// `+===+===+`
    /// inspired by Spectre.Console's ASCII border
    pub const ASCII: Set = Set {
        start: '+',
        left_symbol: '=',
        center: '=',
        right_symbol: '=',
        end: '+',
    };
    /// `⠺⠿⠿⠿⠿⠿⠿⠿⠗`
    pub const BRAILLE_3_POINTED_FILLED: Set = Set {
        start: '⠺',
        left_symbol: '⠿',
        center: '⠿',
        right_symbol: '⠿',
        end: '⠗',
    };
    /// `⠪⠭⠭⠭⠭⠭⠭⠭⠕`
    pub const BRAILLE_3_POINTED_HOLLOW: Set = Set {
        start: '⠪',
        left_symbol: '⠭',
        center: '⠭',
        right_symbol: '⠭',
        end: '⠕',
    };
    /// `⠮⠭⠭⠭⠭⠭⠭⠭⠵`
    pub const BRAILLE_3_DOWN_HOLLOW: Set = Set {
        start: '⠮',
        left_symbol: '⠭',
        center: '⠭',
        right_symbol: '⠭',
        end: '⠵',
    };
    /// `⠾⠿⠿⠿⠿⠿⠿⠿⠷`
    pub const BRAILLE_3_DOWN_FILLED: Set = Set {
        start: '⠾',
        left_symbol: '⠿',
        center: '⠿',
        right_symbol: '⠿',
        end: '⠷',
    };
    /// `⠻⠿⠿⠿⠿⠿⠿⠿⠟`
    pub const BRAILLE_3_UP_FILLED: Set = Set {
        start: '⠻',
        left_symbol: '⠿',
        center: '⠿',
        right_symbol: '⠿',
        end: '⠟',
    };
    /// `⠫⠭⠭⠭⠭⠭⠭⠭⠝`
    pub const BRAILLE_3_UP_HOLLOW: Set = Set {
        start: '⠫',
        left_symbol: '⠭',
        center: '⠭',
        right_symbol: '⠭',
        end: '⠝',
    };
    /// `⠻⠿⠿⠿⠿⠿⠿⠿⠷`
    pub const BRAILLE_3_PARALLELOGRAM_UP_FILLED: Set =
        Set {
            start: '⠻',
            left_symbol: '⠿',
            center: '⠿',
            right_symbol: '⠿',
            end: '⠷',
        };
    /// `⠫⠭⠭⠭⠭⠭⠭⠭⠵`
    pub const BRAILLE_3_PARALLELOGRAM_UP_HOLLOW: Set =
        Set {
            start: '⠫',
            left_symbol: '⠭',
            center: '⠭',
            right_symbol: '⠭',
            end: '⠵',
        };
    /// `⠾⠿⠿⠿⠿⠿⠿⠿⠟`
    pub const BRAILLE_3_PARALLELOGRAM_DOWN_FILLED: Set =
        Set {
            start: '⠾',
            left_symbol: '⠿',
            center: '⠿',
            right_symbol: '⠿',
            end: '⠟',
        };
    /// `⠮⠭⠭⠭⠭⠭⠭⠭⠝`
    pub const BRAILLE_3_PARALLELOGRAM_DOWN_HOLLOW: Set =
        Set {
            start: '⠮',
            left_symbol: '⠭',
            center: '⠭',
            right_symbol: '⠭',
            end: '⠝',
        };
}
pub mod vertical {
    pub use crate::Set;
    pub const ASCII: Set = Set {
        start: '+',
        left_symbol: '|',
        center: '+',
        right_symbol: '|',
        end: '+',
    };
}
pub mod test_sets {
    pub use crate::Set;
    pub const VERTICAL: Set = Set {
        start: '+',
        left_symbol: '│',
        center: '+',
        right_symbol: '│',
        end: '+',
    };
    pub const HORIZONTAL: Set = Set {
        start: '+',
        left_symbol: '─',
        center: '+',
        right_symbol: '─',
        end: '+',
    };
}
