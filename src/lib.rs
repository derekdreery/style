use proc_macro_hack::proc_macro_hack;

pub use style_shared::*;

/// Parse a list of css properties.
///
/// # Examples
///
/// ```
/// # use style::styles;
/// let styles = styles! {
///     height: 10px;
///     display: flex;
/// };
///
/// assert_eq!(styles.to_string(), "height:10px;display:flex;".to_string());
/// ```
#[proc_macro_hack]
pub use style_proc::styles;

/// Parse a css property.
///
/// # Examples
///
/// ```
/// # use style::property;
/// let styles = property!(display: flex);
///
/// assert_eq!(styles.to_string(), "display:flex".to_string());
/// ```
#[proc_macro_hack]
pub use style_proc::property;
