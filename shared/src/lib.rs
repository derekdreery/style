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

// TODO make container generic over heap (e.g. support bumpalo)
#[derive(Debug, Clone, PartialEq)]
pub struct Styles<'a>(pub Vec<Style<'a>>);

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
        for style in self.0.iter().filter(|style| !style.is_dummy()) {
            write!(f, "{};", style)?;
        }
        Ok(())
    }
}

/// a `Style` is one of the css key/value pairs.
///
/// Styles borrow any heap data making them cheap to copy etc. Use `into_owned` to get something
/// `'static`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Style<'a> {
    /// Tokens to pass through directly to codegen.
    #[doc(hidden)]
    Tokens(TokenWrapper),

    /// For when you don't want to include any style at all (useful in expressions like `if`)
    Dummy,

    // *From w3 spec:*
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
    FlexBasis(FlexBasis),
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
    Height(WidthHeight),
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
    MarginBottom(MarginWidth),
    /// margin-left
    MarginLeft(MarginWidth),
    /// margin-right
    MarginRight(MarginWidth),
    /// margin-top
    MarginTop(MarginWidth),
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
    MinHeight(LengthPercentage), // todo inherit
    /// min-width
    MinWidth(LengthPercentage), // todo none, inherit
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
    /// padding-bottom
    PaddingBottom(PaddingWidth),
    /// padding-left
    PaddingLeft(PaddingWidth),
    /// padding-right
    PaddingRight(PaddingWidth),
    /// padding-top
    PaddingTop(PaddingWidth),
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
    Width(WidthHeight),
    // will-change
    // word-spacing
    // writing-mode
    // z-index
}

impl<'a> Style<'a> {
    fn is_dummy(&self) -> bool {
        match self {
            Style::Dummy => true,
            _ => false,
        }
    }

    pub fn into_owned(self) -> Style<'static> {
        match self {
            Style::Tokens(tok) => Style::Tokens(tok),
            Style::Dummy => Style::Dummy,

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
            Style::PaddingBottom(value) => Style::PaddingBottom(value),
            Style::PaddingLeft(value) => Style::PaddingLeft(value),
            Style::PaddingRight(value) => Style::PaddingRight(value),
            Style::PaddingTop(value) => Style::PaddingTop(value),
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
            Style::Tokens(_) => Ok(()),
            Style::Dummy => Ok(()),

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
            Style::PaddingBottom(v) => write!(f, "padding-bottom:{}", v),
            Style::PaddingLeft(v) => write!(f, "padding-left:{}", v),
            Style::PaddingRight(v) => write!(f, "padding-right:{}", v),
            Style::PaddingTop(v) => write!(f, "padding-top:{}", v),
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexBasis {
    Width(Width21),
    Content,
}

impl fmt::Display for FlexBasis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlexBasis::Width(v) => fmt::Display::fmt(v, f),
            FlexBasis::Content => f.write_str("content"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// https://www.w3.org/TR/css-flexbox-1/#propdef-justify-content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JustifyContent::FlexStart => write!(f, "flex-start"),
            JustifyContent::Center => write!(f, "center"),
            JustifyContent::FlexEnd => write!(f, "flex-end"),
            JustifyContent::SpaceAround => write!(f, "space-around"),
            JustifyContent::SpaceBetween => write!(f, "space-between"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Lighter,
    Bolder,
    /// Between 1 and 1000
    Number(f64),
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontWeight::Normal => f.write_str("normal"),
            FontWeight::Bold => f.write_str("bold"),
            FontWeight::Lighter => f.write_str("lighter"),
            FontWeight::Bolder => f.write_str("bolder"),
            FontWeight::Number(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FontStyle::Normal => f.write_str("normal"),
            FontStyle::Italic => f.write_str("italic"),
            FontStyle::Oblique => f.write_str("oblique"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxSizing {
    BorderBox,
    ContentBox,
}

impl fmt::Display for BoxSizing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxSizing::BorderBox => f.write_str("border-box"),
            BoxSizing::ContentBox => f.write_str("content-box"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Padding {
    All(PaddingWidth),
    VerticalHorizontal(PaddingWidth, PaddingWidth),
    TopHorizontalBottom(PaddingWidth, PaddingWidth, PaddingWidth),
    LeftTopRightBottom(PaddingWidth, PaddingWidth, PaddingWidth, PaddingWidth),
}

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Padding::All(len) => write!(f, "{}", len),
            Padding::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            Padding::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            Padding::LeftTopRightBottom(l, t, r, b) => write!(f, "{} {} {} {}", l, t, r, b),
        }
    }
}

/// for e.g. `padding-top`
pub type PaddingWidth = LengthPercentage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Margin {
    All(MarginWidth),
    VerticalHorizontal(MarginWidth, MarginWidth),
    TopHorizontalBottom(MarginWidth, MarginWidth, MarginWidth),
    LeftTopRightBottom(MarginWidth, MarginWidth, MarginWidth, MarginWidth),
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Margin::All(len) => write!(f, "{}", len),
            Margin::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            Margin::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            Margin::LeftTopRightBottom(l, t, r, b) => write!(f, "{} {} {} {}", l, t, r, b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarginWidth {
    LengthPercentage(LengthPercentage),
    Auto,
}

impl fmt::Display for MarginWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MarginWidth::LengthPercentage(v) => fmt::Display::fmt(v, f),
            MarginWidth::Auto => write!(f, "auto"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListStyleType {
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    UpperGreek,
    LowerLatin,
    UpperLatin,
    Armenian,
    Georgian,
    LowerAlpha,
    UpperAlpha,
    None,
}

impl fmt::Display for ListStyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListStyleType::Disc => write!(f, "disc"),
            ListStyleType::Circle => write!(f, "circle"),
            ListStyleType::Square => write!(f, "square"),
            ListStyleType::Decimal => write!(f, "decimal"),
            ListStyleType::DecimalLeadingZero => write!(f, "decimal-leading-zero"),
            ListStyleType::LowerRoman => write!(f, "lower-roman"),
            ListStyleType::UpperRoman => write!(f, "upper-roman"),
            ListStyleType::LowerGreek => write!(f, "lower-greek"),
            ListStyleType::UpperGreek => write!(f, "upper-greek"),
            ListStyleType::LowerLatin => write!(f, "lower-latin"),
            ListStyleType::UpperLatin => write!(f, "upper-latin"),
            ListStyleType::Armenian => write!(f, "armenian"),
            ListStyleType::Georgian => write!(f, "georgian"),
            ListStyleType::LowerAlpha => write!(f, "lower-alpha"),
            ListStyleType::UpperAlpha => write!(f, "upper-alpha"),
            ListStyleType::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resize {
    None,
    Both,
    Horizontal,
    Vertical,
}

impl fmt::Display for Resize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resize::None => write!(f, "none"),
            Resize::Both => write!(f, "both"),
            Resize::Horizontal => write!(f, "horizontal"),
            Resize::Vertical => write!(f, "vertical"),
        }
    }
}

/// for max-width and max-height
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaxWidthHeight {
    None,
    LengthPercentage(LengthPercentage),
    MinContent,
    MaxContent,
    FitContent(LengthPercentage),
}

impl fmt::Display for MaxWidthHeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaxWidthHeight::None => write!(f, "none"),
            MaxWidthHeight::LengthPercentage(v) => write!(f, "{}", v),
            MaxWidthHeight::MinContent => write!(f, "min-content"),
            MaxWidthHeight::MaxContent => write!(f, "max-content"),
            MaxWidthHeight::FitContent(v) => write!(f, "fit-content({})", v),
        }
    }
}

/// values of `width` and `height`, `min-width`, `min-height`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WidthHeight {
    Auto,
    LengthPercentage(LengthPercentage),
    MinContent,
    MaxContent,
    FitContent(LengthPercentage),
}

impl fmt::Display for WidthHeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WidthHeight::Auto => write!(f, "auto"),
            WidthHeight::LengthPercentage(v) => write!(f, "{}", v),
            WidthHeight::MinContent => write!(f, "min-content"),
            WidthHeight::MaxContent => write!(f, "max-content"),
            WidthHeight::FitContent(v) => write!(f, "fit-content({})", v),
        }
    }
}

/// CSS2.1 width, for use with flexbox.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Width21 {
    Auto,
    LengthPercentage(LengthPercentage),
}

impl fmt::Display for Width21 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Width21::Auto => write!(f, "auto"),
            Width21::LengthPercentage(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            Length::Zero => write!(f, "0"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(pub f64);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl fmt::Display for LengthPercentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LengthPercentage::Length(v) => fmt::Display::fmt(v, f),
            LengthPercentage::Percentage(v) => fmt::Display::fmt(v, f),
        }
    }
}

// util
// ====

/// wrapper for TokenStream so I can impl PartialEq
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct TokenWrapper(proc_macro2::TokenStream);

impl PartialEq for TokenWrapper {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
