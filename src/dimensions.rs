#[cfg(
    any(
        all(
            feature = "2D",
            any(
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "3D",
            any(
                feature = "2D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "4D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "5D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "6D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "7D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "8D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "9D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "9D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "10D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "10D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "11D",
                feature = "12D"
            )
        ),
        all(
            feature = "11D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "12D"
            )
        ),
        all(
            feature = "12D",
            any(
                feature = "2D",
                feature = "3D",
                feature = "4D",
                feature = "5D",
                feature = "6D",
                feature = "7D",
                feature = "8D",
                feature = "9D",
                feature = "10D",
                feature = "11D"
            )
        )
    )
)]
compile_error!(
    "More than one number of dimensions was specified, but this crate currently only supports \
     compiling for a single dimensionality at a time. Please choose a singular dimensionality and \
     try again."
);

/// The number of dimensions in which all sudoku methods will operate.
///
/// To change the number of dimensions, compile with the appropriate feature name.
/// Feature names are *n*D, where *n* is the number of dimensions (2–12).
///
/// # Notes
/// Some features are missing for higher-dimension sudokus.
#[cfg(feature = "2D")]
pub const DIMENSIONS: usize = 2;
#[cfg(feature = "3D")]
pub const DIMENSIONS: usize = 3;
#[cfg(feature = "4D")]
pub const DIMENSIONS: usize = 4;
#[cfg(feature = "5D")]
pub const DIMENSIONS: usize = 5;
#[cfg(feature = "6D")]
pub const DIMENSIONS: usize = 6;
#[cfg(feature = "7D")]
pub const DIMENSIONS: usize = 7;
#[cfg(feature = "8D")]
pub const DIMENSIONS: usize = 8;
#[cfg(feature = "9D")]
pub const DIMENSIONS: usize = 9;
#[cfg(feature = "10D")]
pub const DIMENSIONS: usize = 10;
#[cfg(feature = "11D")]
pub const DIMENSIONS: usize = 11;
#[cfg(feature = "12D")]
pub const DIMENSIONS: usize = 12;
