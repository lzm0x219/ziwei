//! Ziwei Rust SDK 的统一门面。
//!
//! 选择性重导出 `ziwei_core` 的不可变事实与 `ziwei_query` 的只读查询公开面。
//!
//! # Examples
//!
//! ```
//! use ziwei::{Gender, PalaceName, ZiweiBirth, create_from_birth, query};
//!
//! let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4)?;
//! let natal = create_from_birth(birth);
//! let ming = query(&natal).natal().palace(PalaceName::Ming);
//!
//! assert_eq!(ming.relative_name(), PalaceName::Ming);
//! # Ok::<(), ziwei::ZiweiInputError>(())
//! ```

pub use ziwei_core::{
    Branch, Decade, DecadeDirection, DecadeIndex, DecadeIndexError, DecadeYear, FiveElementBureau,
    Gender, Natal, NatalContext, Palace, PalaceName, PalaceTransformation, Star, StarCategory,
    StarGalaxy, StarName, StarSelfTransformations, Stem, Transformation, ZiweiBirth, ZiweiInput,
    ZiweiInputError, Zodiac, create_from_birth, create_from_input,
};
pub use ziwei_query::{
    DecadeAgeError, DecadeLunarYearError, DecadeScope, DecadeYearOrdinal, DecadeYearOrdinalError,
    DecadeYearSelection, PalaceLine, Query, ReframeScope, ScopedBirthTransformationOpposition,
    ScopedPalace, ScopedPalaceLine, ScopedPalaceTransformation, ScopedStar, query,
};
