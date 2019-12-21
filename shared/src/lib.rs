//! A module to type styles.

mod codegen;
mod color;
mod parse;

use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

pub use color::Color;

/*
#[macro_export]
macro_rules! styles {
    ($($key:expr => $val:expr),*) => {{
        let mut styles = crate::style::Styles::new();
        $(
            styles.add($key($val));
        )*
        styles
    }};
    ($($key:expr => $val:expr ,)*) => {crate::styles!($($key => $val),*)}
}
*/

// TODO make container generic over heap (e.g. support bumpalo)
pub struct Styles<'a>(Vec<Style<'a>>);

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        Styles(Vec::new())
    }

    pub fn merge(&mut self, other: Styles<'a>) {
        self.0.extend(other.0.into_iter())
    }
}

impl<'a> Deref for Styles<'a> {
    type Target = Vec<Style<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for Styles<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Vec<Style<'a>>> for Styles<'a> {
    fn from(v: Vec<Style<'a>>) -> Self {
        Self(v)
    }
}

impl<'a> From<Styles<'a>> for Vec<Style<'a>> {
    fn from(v: Styles<'a>) -> Self {
        v.0
    }
}

impl fmt::Display for Styles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for style in self.0.iter() {
            write!(f, "{};", style)?;
        }
        Ok(())
    }
}

/// a `Style` is one of the css key/value pairs.
///
/// Styles borrow any heap data making them cheap to copy etc. Use `into_owned` to get something
/// `'static`.
#[derive(Debug, Clone)]
pub enum Style<'a> {
    // align-content
    /// align-items
    AlignItems(AlignItems),
    // align-self
    // all
    // azimuth
    // background
    // background-attachment
    // background-blend-mode
    // background-clip
    /// background-color
    BackgroundColor(Color),
    // background-image
    // background-origin
    // background-position
    // background-repeat
    // background-size
    // border
    // border-bottom
    // border-bottom-color
    // border-bottom-left-radius
    // border-bottom-right-radius
    // border-bottom-style
    // border-bottom-width
    // border-collapse
    // border-color
    // border-image
    // border-image-outset
    // border-image-repeat
    // border-image-slice
    // border-image-source
    // border-image-width
    // border-left
    // border-left-color
    // border-left-style
    // border-left-width
    // border-radius
    // border-right
    // border-right-color
    // border-right-style
    // border-right-width
    // border-spacing
    // border-style
    // border-top
    // border-top-color
    // border-top-left-radius
    // border-top-right-radius
    // border-top-style
    // border-top-width
    // border-width
    // bottom
    // box-decoration-break
    // box-shadow
    /// box-sizing
    BoxSizing(BoxSizing),
    // break-after
    // break-before
    // break-inside
    // caption-side
    // caret-color
    // clear
    // clip
    // clip-path
    // clip-rule
    /// color
    Color(Color),
    // contain
    // content
    // counter-increment
    // counter-reset
    // cue
    // cue-after
    // cue-before
    // cursor
    // direction
    /// display https://www.w3.org/TR/css-display-3/#typedef-display-outside
    Display(Display),
    // elevation
    // empty-cells
    // flex
    /// flex-basis
    FlexBasis(Length),
    /// flex-direction
    FlexDirection(FlexDirection),
    // flex-flow
    /// flex-grow
    FlexGrow(f64),
    /// flex-shrink
    FlexShrink(f64),
    /// flex-wrap
    FlexWrap(FlexWrap),
    // float
    // font
    /// font-family
    FontFamily(Cow<'a, str>),
    // font-feature-settings
    // font-kerning
    // font-size
    // font-size-adjust
    // font-stretch
    /// font-style
    FontStyle(FontStyle),
    // font-synthesis
    // font-variant
    // font-variant-caps
    // font-variant-east-asian
    // font-variant-ligatures
    // font-variant-numeric
    // font-variant-position
    /// font-weight
    FontWeight(FontWeight),
    // glyph-orientation-vertical
    // grid
    // grid-area
    // grid-auto-columns
    // grid-auto-flow
    // grid-auto-rows
    // grid-column
    // grid-column-end
    // grid-column-start
    // grid-row
    // grid-row-end
    // grid-row-start
    // grid-template
    // grid-template-areas
    // grid-template-columns
    // grid-template-rows
    /// height
    Height(Length),
    // image-orientation
    // image-rendering
    // isolation
    /// justify-content
    JustifyContent(JustifyContent),
    // left
    // letter-spacing
    // line-height
    // list-style
    // list-style-image
    // list-style-position
    /// list-style-type
    ListStyleType(ListStyleType),
    /// margin
    Margin(Margin),
    /// margin-bottom
    MarginBottom(Length),
    /// margin-left
    MarginLeft(Length),
    /// margin-right
    MarginRight(Length),
    /// margin-top
    MarginTop(Length),
    // mask
    // mask-border
    // mask-border-mode
    // mask-border-outset
    // mask-border-repeat
    // mask-border-slice
    // mask-border-source
    // mask-border-width
    // mask-clip
    // mask-composite
    // mask-image
    // mask-mode
    // mask-origin
    // mask-position
    // mask-repeat
    // mask-size
    // mask-type
    // max-height
    // max-width
    /// min-height
    MinHeight(Length),
    /// min-width
    MinWidth(Length),
    // mix-blend-mode
    // object-fit
    // object-position
    // opacity
    // order
    // orphans
    // outline
    // outline-color
    // outline-offset
    // outline-style
    // outline-width
    // overflow
    /// padding
    Padding(Padding),
    // padding-bottom
    // padding-left
    // padding-right
    // padding-top
    // page-break-after
    // page-break-before
    // page-break-inside
    // pause
    // pause-after
    // pause-before
    // pitch
    // pitch-range
    // play-during
    // position
    // quotes
    /// resize
    Resize(Resize),
    // richness
    // right
    // scroll-margin
    // scroll-margin-block
    // scroll-margin-block-end
    // scroll-margin-block-start
    // scroll-margin-bottom
    // scroll-margin-inline
    // scroll-margin-inline-end
    // scroll-margin-inline-start
    // scroll-margin-left
    // scroll-margin-right
    // scroll-margin-top
    // scroll-padding
    // scroll-padding-block
    // scroll-padding-block-end
    // scroll-padding-block-start
    // scroll-padding-bottom
    // scroll-padding-inline
    // scroll-padding-inline-end
    // scroll-padding-inline-start
    // scroll-padding-left
    // scroll-padding-right
    // scroll-padding-top
    // scroll-snap-align
    // scroll-snap-stop
    // scroll-snap-type
    // shape-image-threshold
    // shape-margin
    // shape-outside
    // speak
    // speak-header
    // speak-numeral
    // speak-punctuation
    // speech-rate
    // stress
    // table-layout
    // text-align
    // text-combine-upright
    // text-decoration
    // text-decoration-color
    // text-decoration-line
    // text-decoration-style
    // text-emphasis
    // text-emphasis-color
    // text-emphasis-position
    // text-emphasis-style
    // text-indent
    // text-orientation
    // text-overflow
    // text-shadow
    // text-transform
    // text-underline-position
    // top
    // transform
    // transform-box
    // transform-origin
    // unicode-bidi
    // vertical-align
    // visibility
    // voice-family
    // volume
    // white-space
    // widows
    /// width
    Width(Width),
    // will-change
    // word-spacing
    // writing-mode
    // z-index
}

impl<'a> Style<'a> {
    pub fn into_owned(self) -> Style<'static> {
        match self {
            // align-content
            Style::AlignItems(value) => Style::AlignItems(value),
            // align-self
            // all
            // azimuth
            // background
            // background-attachment
            // background-blend-mode
            // background-clip
            Style::BackgroundColor(value) => Style::BackgroundColor(value),
            // background-image
            // background-origin
            // background-position
            // background-repeat
            // background-size
            // border
            // border-bottom
            // border-bottom-color
            // border-bottom-left-radius
            // border-bottom-right-radius
            // border-bottom-style
            // border-bottom-width
            // border-collapse
            // border-color
            // border-image
            // border-image-outset
            // border-image-repeat
            // border-image-slice
            // border-image-source
            // border-image-width
            // border-left
            // border-left-color
            // border-left-style
            // border-left-width
            // border-radius
            // border-right
            // border-right-color
            // border-right-style
            // border-right-width
            // border-spacing
            // border-style
            // border-top
            // border-top-color
            // border-top-left-radius
            // border-top-right-radius
            // border-top-style
            // border-top-width
            // border-width
            // bottom
            // box-decoration-break
            // box-shadow
            Style::BoxSizing(value) => Style::BoxSizing(value),
            // break-after
            // break-before
            // break-inside
            // caption-side
            // caret-color
            // clear
            // clip
            // clip-path
            // clip-rule
            Style::Color(value) => Style::Color(value),
            // contain
            // content
            // counter-increment
            // counter-reset
            // cue
            // cue-after
            // cue-before
            // cursor
            // direction
            Style::Display(value) => Style::Display(value),
            // elevation
            // empty-cells
            // flex
            Style::FlexBasis(value) => Style::FlexBasis(value),
            Style::FlexDirection(value) => Style::FlexDirection(value),
            // flex-flow
            Style::FlexGrow(value) => Style::FlexGrow(value),
            Style::FlexShrink(value) => Style::FlexShrink(value),
            Style::FlexWrap(value) => Style::FlexWrap(value),
            // float
            // font
            Style::FontFamily(value) => Style::FontFamily(Cow::Owned(value.into_owned())),
            // font-feature-settings
            // font-kerning
            // font-size
            // font-size-adjust
            // font-stretch
            Style::FontStyle(value) => Style::FontStyle(value),
            // font-synthesis
            // font-variant
            // font-variant-caps
            // font-variant-east-asian
            // font-variant-ligatures
            // font-variant-numeric
            // font-variant-position
            Style::FontWeight(value) => Style::FontWeight(value),
            // glyph-orientation-vertical
            // grid
            // grid-area
            // grid-auto-columns
            // grid-auto-flow
            // grid-auto-rows
            // grid-column
            // grid-column-end
            // grid-column-start
            // grid-row
            // grid-row-end
            // grid-row-start
            // grid-template
            // grid-template-areas
            // grid-template-columns
            // grid-template-rows
            Style::Height(value) => Style::Height(value),
            // image-orientation
            // image-rendering
            // isolation
            Style::JustifyContent(value) => Style::JustifyContent(value),
            // left
            // letter-spacing
            // line-height
            // list-style
            // list-style-image
            // list-style-position
            Style::ListStyleType(value) => Style::ListStyleType(value),
            Style::Margin(value) => Style::Margin(value),
            Style::MarginBottom(value) => Style::MarginBottom(value),
            Style::MarginLeft(value) => Style::MarginLeft(value),
            Style::MarginRight(value) => Style::MarginRight(value),
            Style::MarginTop(value) => Style::MarginTop(value),
            // mask
            // mask-border
            // mask-border-mode
            // mask-border-outset
            // mask-border-repeat
            // mask-border-slice
            // mask-border-source
            // mask-border-width
            // mask-clip
            // mask-composite
            // mask-image
            // mask-mode
            // mask-origin
            // mask-position
            // mask-repeat
            // mask-size
            // mask-type
            // max-height
            // max-width
            Style::MinHeight(value) => Style::MinHeight(value),
            Style::MinWidth(value) => Style::MinWidth(value),
            // mix-blend-mode
            // object-fit
            // object-position
            // opacity
            // order
            // orphans
            // outline
            // outline-color
            // outline-offset
            // outline-style
            // outline-width
            // overflow
            Style::Padding(value) => Style::Padding(value),
            // padding-bottom
            // padding-left
            // padding-right
            // padding-top
            // page-break-after
            // page-break-before
            // page-break-inside
            // pause
            // pause-after
            // pause-before
            // pitch
            // pitch-range
            // play-during
            // position
            // quotes
            Style::Resize(value) => Style::Resize(value),
            // richness
            // right
            // scroll-margin
            // scroll-margin-block
            // scroll-margin-block-end
            // scroll-margin-block-start
            // scroll-margin-bottom
            // scroll-margin-inline
            // scroll-margin-inline-end
            // scroll-margin-inline-start
            // scroll-margin-left
            // scroll-margin-right
            // scroll-margin-top
            // scroll-padding
            // scroll-padding-block
            // scroll-padding-block-end
            // scroll-padding-block-start
            // scroll-padding-bottom
            // scroll-padding-inline
            // scroll-padding-inline-end
            // scroll-padding-inline-start
            // scroll-padding-left
            // scroll-padding-right
            // scroll-padding-top
            // scroll-snap-align
            // scroll-snap-stop
            // scroll-snap-type
            // shape-image-threshold
            // shape-margin
            // shape-outside
            // speak
            // speak-header
            // speak-numeral
            // speak-punctuation
            // speech-rate
            // stress
            // table-layout
            // text-align
            // text-combine-upright
            // text-decoration
            // text-decoration-color
            // text-decoration-line
            // text-decoration-style
            // text-emphasis
            // text-emphasis-color
            // text-emphasis-position
            // text-emphasis-style
            // text-indent
            // text-orientation
            // text-overflow
            // text-shadow
            // text-transform
            // text-underline-position
            // top
            // transform
            // transform-box
            // transform-origin
            // unicode-bidi
            // vertical-align
            // visibility
            // voice-family
            // volume
            // white-space
            // widows
            Style::Width(value) => Style::Width(value),
            // will-change
            // word-spacing
            // writing-mode
            // z-index
        }
    }
}

impl fmt::Display for Style<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // align-content
            Style::AlignItems(v) => write!(f, "align-items:{}", v),
            // align-self
            // all
            // azimuth
            // background
            // background-attachment
            // background-blend-mode
            // background-clip
            Style::BackgroundColor(v) => write!(f, "background-color:{}", v),
            // background-image
            // background-origin
            // background-position
            // background-repeat
            // background-size
            // border
            // border-bottom
            // border-bottom-color
            // border-bottom-left-radius
            // border-bottom-right-radius
            // border-bottom-style
            // border-bottom-width
            // border-collapse
            // border-color
            // border-image
            // border-image-outset
            // border-image-repeat
            // border-image-slice
            // border-image-source
            // border-image-width
            // border-left
            // border-left-color
            // border-left-style
            // border-left-width
            // border-radius
            // border-right
            // border-right-color
            // border-right-style
            // border-right-width
            // border-spacing
            // border-style
            // border-top
            // border-top-color
            // border-top-left-radius
            // border-top-right-radius
            // border-top-style
            // border-top-width
            // border-width
            // bottom
            // box-decoration-break
            // box-shadow
            Style::BoxSizing(v) => write!(f, "box-sizing:{}", v),
            // break-after
            // break-before
            // break-inside
            // caption-side
            // caret-color
            // clear
            // clip
            // clip-path
            // clip-rule
            Style::Color(v) => write!(f, "color:{}", v),
            // contain
            // content
            // counter-increment
            // counter-reset
            // cue
            // cue-after
            // cue-before
            // cursor
            // direction
            Style::Display(v) => write!(f, "display:{}", v),
            // elevation
            // empty-cells
            // flex
            Style::FlexBasis(v) => write!(f, "flex-basis:{}", v),
            Style::FlexDirection(v) => write!(f, "flex-direction:{}", v),
            // flex-flow
            Style::FlexGrow(v) => write!(f, "flex-grow:{}", v),
            Style::FlexShrink(v) => write!(f, "flex-shrink:{}", v),
            Style::FlexWrap(v) => write!(f, "flex-wrap:{}", v),
            // float
            // font
            Style::FontFamily(v) => write!(f, "font-family:{}", v),
            // font-feature-settings
            // font-kerning
            // font-size
            // font-size-adjust
            // font-stretch
            Style::FontStyle(v) => write!(f, "font-style:{}", v),
            // font-synthesis
            // font-variant
            // font-variant-caps
            // font-variant-east-asian
            // font-variant-ligatures
            // font-variant-numeric
            // font-variant-position
            Style::FontWeight(v) => write!(f, "font-weight:{}", v),
            // glyph-orientation-vertical
            // grid
            // grid-area
            // grid-auto-columns
            // grid-auto-flow
            // grid-auto-rows
            // grid-column
            // grid-column-end
            // grid-column-start
            // grid-row
            // grid-row-end
            // grid-row-start
            // grid-template
            // grid-template-areas
            // grid-template-columns
            // grid-template-rows
            Style::Height(v) => write!(f, "height:{}", v),
            // image-orientation
            // image-rendering
            // isolation
            Style::JustifyContent(v) => write!(f, "justify-content:{}", v),
            // left
            // letter-spacing
            // line-height
            // list-style
            // list-style-image
            // list-style-position
            Style::ListStyleType(v) => write!(f, "list-style-type:{}", v),
            Style::Margin(v) => write!(f, "margin:{}", v),
            Style::MarginBottom(v) => write!(f, "margin-bottom:{}", v),
            Style::MarginLeft(v) => write!(f, "margin-left:{}", v),
            Style::MarginRight(v) => write!(f, "margin-right:{}", v),
            Style::MarginTop(v) => write!(f, "margin-top:{}", v),
            // mask
            // mask-border
            // mask-border-mode
            // mask-border-outset
            // mask-border-repeat
            // mask-border-slice
            // mask-border-source
            // mask-border-width
            // mask-clip
            // mask-composite
            // mask-image
            // mask-mode
            // mask-origin
            // mask-position
            // mask-repeat
            // mask-size
            // mask-type
            // max-height
            // max-width
            Style::MinHeight(v) => write!(f, "min-height:{}", v),
            Style::MinWidth(v) => write!(f, "min-width:{}", v),
            // mix-blend-mode
            // object-fit
            // object-position
            // opacity
            // order
            // orphans
            // outline
            // outline-color
            // outline-offset
            // outline-style
            // outline-width
            // overflow
            Style::Padding(v) => write!(f, "padding:{}", v),
            // padding-bottom
            // padding-left
            // padding-right
            // padding-top
            // page-break-after
            // page-break-before
            // page-break-inside
            // pause
            // pause-after
            // pause-before
            // pitch
            // pitch-range
            // play-during
            // position
            // quotes
            Style::Resize(v) => write!(f, "resize:{}", v),
            // richness
            // right
            // scroll-margin
            // scroll-margin-block
            // scroll-margin-block-end
            // scroll-margin-block-start
            // scroll-margin-bottom
            // scroll-margin-inline
            // scroll-margin-inline-end
            // scroll-margin-inline-start
            // scroll-margin-left
            // scroll-margin-right
            // scroll-margin-top
            // scroll-padding
            // scroll-padding-block
            // scroll-padding-block-end
            // scroll-padding-block-start
            // scroll-padding-bottom
            // scroll-padding-inline
            // scroll-padding-inline-end
            // scroll-padding-inline-start
            // scroll-padding-left
            // scroll-padding-right
            // scroll-padding-top
            // scroll-snap-align
            // scroll-snap-stop
            // scroll-snap-type
            // shape-image-threshold
            // shape-margin
            // shape-outside
            // speak
            // speak-header
            // speak-numeral
            // speak-punctuation
            // speech-rate
            // stress
            // table-layout
            // text-align
            // text-combine-upright
            // text-decoration
            // text-decoration-color
            // text-decoration-line
            // text-decoration-style
            // text-emphasis
            // text-emphasis-color
            // text-emphasis-position
            // text-emphasis-style
            // text-indent
            // text-orientation
            // text-overflow
            // text-shadow
            // text-transform
            // text-underline-position
            // top
            // transform
            // transform-box
            // transform-origin
            // unicode-bidi
            // vertical-align
            // visibility
            // voice-family
            // volume
            // white-space
            // widows
            Style::Width(v) => write!(f, "width:{}", v),
            // will-change
            // word-spacing
            // writing-mode
            // z-index
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Display {
    Block,
    Flex,
    Inline,
    // todo incomplete
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Display::Block => f.write_str("block"),
            Display::Flex => f.write_str("flex"),
            Display::Inline => f.write_str("inline"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlexDirection::Row => f.write_str("row"),
            FlexDirection::Column => f.write_str("column"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FlexWrap {
    Wrap,
    Nowrap,
}

impl fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlexWrap::Wrap => write!(f, "wrap"),
            FlexWrap::Nowrap => write!(f, "nowrap"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
#[derive(Debug, Clone, Copy)]
pub enum AlignItems {
    Normal,
    Stretch,
    Center,
    Start,
    End,
    //todo
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlignItems::Normal => write!(f, "normal"),
            AlignItems::Stretch => write!(f, "stretch"),
            AlignItems::Center => write!(f, "center"),
            AlignItems::Start => write!(f, "start"),
            AlignItems::End => write!(f, "end"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content
#[derive(Debug, Clone, Copy)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    //todo-
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JustifyContent::Start => write!(f, "start"),
            JustifyContent::Center => write!(f, "center"),
            JustifyContent::End => write!(f, "end"),
            JustifyContent::SpaceAround => write!(f, "space-around"),
            JustifyContent::SpaceBetween => write!(f, "space-between"),
            JustifyContent::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    _900,
    Normal,
    Bold,
    // todo
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontWeight::_900 => f.write_str("900"),
            FontWeight::Normal => f.write_str("normal"),
            FontWeight::Bold => f.write_str("bold"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FontStyle {
    Normal,
    //todo
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontStyle::Normal => f.write_str("normal"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoxSizing {
    BorderBox,
    //todo
}

impl fmt::Display for BoxSizing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxSizing::BorderBox => f.write_str("border-box"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Padding {
    All(Length),
    VerticalHorizontal(Length, Length),
    TopHorizontalBottom(Length, Length, Length),
    LeftTopRightBottom(Length, Length, Length, Length),
    Inherit,
}

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Padding::All(len) => write!(f, "{}", len),
            Padding::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            Padding::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            Padding::LeftTopRightBottom(l, t, r, b) => write!(f, "{} {} {} {}", l, t, r, b),
            Padding::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Margin {
    All(Length),
    VerticalHorizontal(Length, Length),
    TopHorizontalBottom(Length, Length, Length),
    LeftTopRightBottom(Length, Length, Length, Length),
    Inherit,
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Margin::All(len) => write!(f, "{}", len),
            Margin::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            Margin::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            Margin::LeftTopRightBottom(l, t, r, b) => write!(f, "{} {} {} {}", l, t, r, b),
            Margin::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ListStyleType {
    Circle,
    None,
    // todo
}

impl fmt::Display for ListStyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListStyleType::Circle => write!(f, "circle"),
            ListStyleType::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
    Initial,
    Inherit,
}

impl fmt::Display for Resize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resize::None => write!(f, "none"),
            Resize::Both => write!(f, "both"),
            Resize::Horizontal => write!(f, "horizontal"),
            Resize::Vertical => write!(f, "vertical"),
            Resize::Initial => write!(f, "initial"),
            Resize::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Length(Length),
    Percentage(Percentage),
    Auto,
    Inherit,
}

impl Default for Width {
    fn default() -> Self {
        Width::Auto
    }
}

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Width::Length(v) => write!(f, "{}", v),
            Width::Percentage(v) => write!(f, "{}", v),
            Width::Auto => write!(f, "auto"),
            Width::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Em(f64),
    Ex(f64),
    In(f64),
    Cm(f64),
    Mm(f64),
    Pt(f64),
    Pc(f64),
    Px(f64),
    Zero,
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Length::Em(val) => write!(f, "{}em", val),
            Length::Ex(val) => write!(f, "{}ex", val),
            Length::In(val) => write!(f, "{}in", val),
            Length::Cm(val) => write!(f, "{}cm", val),
            Length::Mm(val) => write!(f, "{}mm", val),
            Length::Pt(val) => write!(f, "{}pt", val),
            Length::Pc(val) => write!(f, "{}pc", val),
            Length::Px(val) => write!(f, "{}px", val),
            Length::Zero => write!(f, "zero"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Percentage(pub f64);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}
