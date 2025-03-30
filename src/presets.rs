#![allow(dead_code)]
pub mod horizontal {
    use crate::Set;
    /// `+---+---+`
    /// inspired by Spectre.Console's ASCII table border
    pub const ASCII: Set = Set {
        start: '+',
        rep_1: '-',
        center: '+',
        rep_2: '-',
        end: '+',
    };
    /// `⠺⠿⠿⠿⠿⠿⠿⠿⠗`
    pub const BRAILLE_3_POINTED_FILLED: Set = Set {
        start: '⠺',
        rep_1: '⠿',
        center: '⠿',
        rep_2: '⠿',
        end: '⠗',
    };
    /// `⠪⠭⠭⠭⠭⠭⠭⠭⠕`
    pub const BRAILLE_3_POINTED_HOLLOW: Set = Set {
        start: '⠪',
        rep_1: '⠭',
        center: '⠭',
        rep_2: '⠭',
        end: '⠕',
    };
    /// `⠮⠭⠭⠭⠭⠭⠭⠭⠵`
    pub const BRAILLE_3_UP_HOLLOW: Set = Set {
        start: '⠮',
        rep_1: '⠭',
        center: '⠭',
        rep_2: '⠭',
        end: '⠵',
    };
    /// `⠾⠿⠿⠿⠿⠿⠿⠿⠷`
    pub const BRAILLE_3_UP_FILLED: Set = Set {
        start: '⠾',
        rep_1: '⠿',
        center: '⠿',
        rep_2: '⠿',
        end: '⠷',
    };
    /// `⠻⠿⠿⠿⠿⠿⠿⠿⠟`
    pub const BRAILLE_3_DOWN_FILLED: Set = Set {
        start: '⠻',
        rep_1: '⠿',
        center: '⠿',
        rep_2: '⠿',
        end: '⠟',
    };
    /// `⠫⠭⠭⠭⠭⠭⠭⠭⠝`
    pub const BRAILLE_3_DOWN_HOLLOW: Set = Set {
        start: '⠫',
        rep_1: '⠭',
        center: '⠭',
        rep_2: '⠭',
        end: '⠝',
    };
    /// `⠻⠿⠿⠿⠿⠿⠿⠿⠷`
    pub const BRAILLE_3_PARALLELOGRAM_LEFT_FILLED: Set =
        Set {
            start: '⠻',
            rep_1: '⠿',
            center: '⠿',
            rep_2: '⠿',
            end: '⠷',
        };
    /// `⠫⠭⠭⠭⠭⠭⠭⠭⠵`
    pub const BRAILLE_3_PARALLELOGRAM_LEFT_HOLLOW: Set =
        Set {
            start: '⠫',
            rep_1: '⠭',
            center: '⠭',
            rep_2: '⠭',
            end: '⠵',
        };
    /// `⠾⠿⠿⠿⠿⠿⠿⠿⠟`
    pub const BRAILLE_3_PARALLELOGRAM_RIGHT_FILLED: Set =
        Set {
            start: '⠾',
            rep_1: '⠿',
            center: '⠿',
            rep_2: '⠿',
            end: '⠟',
        };
    /// `⠮⠭⠭⠭⠭⠭⠭⠭⠝`
    pub const BRAILLE_3_PARALLELOGRAM_RIGHT_HOLLOW: Set =
        Set {
            start: '⠮',
            rep_1: '⠭',
            center: '⠭',
            rep_2: '⠭',
            end: '⠝',
        };
}
pub mod vertical {
    pub use crate::Set;
    pub const ASCII: Set = Set {
        start: '+',
        rep_1: '|',
        center: '+',
        rep_2: '|',
        end: '+',
    };
}
pub mod test_sets {
    pub use crate::Set;
    pub const VERTICAL: Set = Set {
        start: '+',
        rep_1: '│',
        center: '+',
        rep_2: '│',
        end: '+',
    };
    pub const HORIZONTAL: Set = Set {
        start: '+',
        rep_1: '─',
        center: '+',
        rep_2: '─',
        end: '+',
    };
}
