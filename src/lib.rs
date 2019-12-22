use proc_macro_hack::proc_macro_hack;

pub use style_shared::*;

/// Parse a list of css properties.
///
/// # Examples
///
/// ```
/// # use style::*;
/// // the dummy property will be ignored
/// let styles = styles! {
///     dummy;
///     height: 10px;
///     display: flex;
///     justify-content: space-around;
///     font-weight: 200;
///     padding: 0 10px;
/// };
///
/// // The types are quite verbose - it's much easier to construct them using the `styles!`
/// // macro.
/// assert_eq!(styles, Styles(vec![
///     Style::Height(WidthHeight::LengthPercentage(LengthPercentage::Length(Length::Px(10.0)))),
///     Style::Display(Display::Flex),
///     Style::JustifyContent(JustifyContent::SpaceAround),
///     Style::FontWeight(FontWeight::Number(200.0)),
///     Style::Padding(Padding::VerticalHorizontal(
///         LengthPercentage::Length(Length::Zero),
///         LengthPercentage::Length(Length::Px(10.0)),
///     )),
/// ]));
/// assert_eq!(
///     styles.to_string(),
///     "height:10px;display:flex;justify-content:space-around;font-weight:200;padding:0 10px;".to_string()
/// );
/// ```
#[proc_macro_hack]
pub use style_proc::styles;

/// Parse a css property.
///
/// # Examples
///
/// ```
/// # use style::property;
/// let prop = property!(display: flex);
/// assert_eq!(prop.to_string(), "display:flex".to_string());
///
/// let prop = property!(dummy); // special property name that means ignore
/// assert_eq!(prop.to_string(), "".to_string());
/// ```
#[proc_macro_hack]
pub use style_proc::property;
