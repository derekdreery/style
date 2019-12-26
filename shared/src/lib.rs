//! A module to type styles.

mod codegen;
mod color;
mod parse;

use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

pub use color::{Color, DynamicColor};

// todo possibly support dynamically generated lists of styles in proc macro
pub struct DynamicStyles<'a>(pub Vec<DynamicStyle<'a>>);

impl<'a> From<Vec<DynamicStyle<'a>>> for DynamicStyles<'a> {
    fn from(v: Vec<DynamicStyle<'a>>) -> Self {
        Self(v)
    }
}

impl fmt::Display for DynamicStyles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for style in self
            .0
            .iter()
            .filter(|style| !(style.is_dynamic() || style.is_dummy()))
        {
            write!(f, "{};", style)?;
        }
        Ok(())
    }
}

// TODO make container generic over heap (e.g. support bumpalo)
#[derive(Debug, Clone, PartialEq)]
pub struct Styles<'a>(pub Vec<Style<'a>>);

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        Styles(Vec::new())
    }

    pub fn add(&mut self, style: Style<'a>) {
        self.0.push(style);
    }

    pub fn merge(&mut self, other: Styles<'a>) {
        self.0.extend(other.0.into_iter())
    }
}

impl<'a> From<DynamicStyles<'a>> for Styles<'a> {
    fn from(dy: DynamicStyles<'a>) -> Self {
        Styles(
            dy.0.into_iter()
                .filter_map(|dy_sty| match dy_sty {
                    DynamicStyle::Dynamic(_) => None,
                    DynamicStyle::Literal(l) => Some(l),
                })
                .collect(),
        )
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

#[derive(Debug, Clone, PartialEq)]
pub enum DynamicStyle<'a> {
    /// A literal style.
    Literal(Style<'a>),
    /// Tokens to pass through directly to codegen.
    Dynamic(syn::Block),
}

impl DynamicStyle<'_> {
    pub fn is_dynamic(&self) -> bool {
        match self {
            DynamicStyle::Literal(style) => style.is_dynamic(),
            DynamicStyle::Dynamic(_) => true,
        }
    }
    pub fn is_dummy(&self) -> bool {
        match self {
            DynamicStyle::Literal(style) => style.is_dummy(),
            DynamicStyle::Dynamic(_) => false,
        }
    }
}

impl<'a> From<Style<'a>> for DynamicStyle<'a> {
    fn from(style: Style<'a>) -> Self {
        DynamicStyle::Literal(style)
    }
}

impl fmt::Display for DynamicStyle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynamicStyle::Literal(style) => style.fmt(f),
            DynamicStyle::Dynamic(_) => Ok(()),
        }
    }
}

/// a `Style` is one of the css key/value pairs.
///
/// Styles borrow any heap data making them cheap to copy etc. Use `into_owned` to get something
/// `'static`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Style<'a> {
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
    BackgroundColor(DynamicColor),
    // background-image
    // background-origin
    // background-position
    // background-repeat
    // background-size
    /// border
    Border(Border),
    // border-bottom
    /// border-bottom-color
    BorderBottomColor(Color),
    // border-bottom-left-radius
    // border-bottom-right-radius
    /// border-bottom-style
    BorderBottomStyle(LineStyle),
    /// border-bottom-width
    BorderBottomWidth(LineWidth),
    // border-collapse
    /// border-color
    BorderColor(BorderColor),
    // border-image
    // border-image-outset
    // border-image-repeat
    // border-image-slice
    // border-image-source
    // border-image-width
    // border-left
    /// border-left-color
    BorderLeftColor(Color),
    /// border-left-style
    BorderLeftStyle(LineStyle),
    /// border-left-width
    BorderLeftWidth(LineWidth),
    // border-radius
    // border-right
    /// border-right-color
    BorderRightColor(Color),
    /// border-right-style
    BorderRightStyle(LineStyle),
    /// border-right-width
    BorderRightWidth(LineWidth),
    // border-spacing
    /// border-style
    BorderStyle(BorderStyle),
    // border-top
    /// border-top-color
    BorderTopColor(Color),
    // border-top-left-radius
    // border-top-right-radius
    /// border-top-style
    BorderTopStyle(LineStyle),
    /// border-top-width
    BorderTopWidth(LineWidth),
    /// border-width
    BorderWidth(BorderWidth),
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
    Color(DynamicColor),
    // contain
    // content
    // counter-increment
    // counter-reset
    // cue
    // cue-after
    // cue-before
    /// cursor
    Cursor(Cursor),
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

    fn is_dynamic(&self) -> bool {
        match self {
            Style::BackgroundColor(value) => value.is_dynamic(),
            Style::Color(value) => value.is_dynamic(),
            _ => false,
        }
    }

    pub fn into_owned(self) -> Style<'static> {
        match self {
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
            Style::Border(value) => Style::Border(value),
            // border-bottom
            Style::BorderBottomColor(value) => Style::BorderBottomColor(value),
            // border-bottom-left-radius
            // border-bottom-right-radius
            Style::BorderBottomStyle(value) => Style::BorderBottomStyle(value),
            Style::BorderBottomWidth(value) => Style::BorderBottomWidth(value),
            // border-collapse
            // border-color
            Style::BorderColor(value) => Style::BorderColor(value),
            // border-image
            // border-image-outset
            // border-image-repeat
            // border-image-slice
            // border-image-source
            // border-image-width
            // border-left
            Style::BorderLeftColor(value) => Style::BorderLeftColor(value),
            Style::BorderLeftStyle(value) => Style::BorderLeftStyle(value),
            Style::BorderLeftWidth(value) => Style::BorderLeftWidth(value),
            // border-radius
            // border-right
            Style::BorderRightColor(value) => Style::BorderRightColor(value),
            Style::BorderRightStyle(value) => Style::BorderRightStyle(value),
            Style::BorderRightWidth(value) => Style::BorderRightWidth(value),
            // border-spacing
            Style::BorderStyle(value) => Style::BorderStyle(value),
            // border-top
            Style::BorderTopColor(value) => Style::BorderTopColor(value),
            // border-top-left-radius
            // border-top-right-radius
            Style::BorderTopStyle(value) => Style::BorderTopStyle(value),
            Style::BorderTopWidth(value) => Style::BorderTopWidth(value),
            Style::BorderWidth(value) => Style::BorderWidth(value),
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
            Style::Cursor(value) => Style::Cursor(value),
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
            Style::Border(v) => write!(f, "border:{}", v),
            // border-bottom
            Style::BorderBottomColor(v) => write!(f, "border-bottom-color:{}", v),
            // border-bottom-left-radius
            // border-bottom-right-radius
            Style::BorderBottomStyle(v) => write!(f, "border-bottom-style:{}", v),
            Style::BorderBottomWidth(v) => write!(f, "border-bottom-width:{}", v),
            // border-collapse
            Style::BorderColor(v) => write!(f, "border-color:{}", v),
            // border-image
            // border-image-outset
            // border-image-repeat
            // border-image-slice
            // border-image-source
            // border-image-width
            // border-left
            Style::BorderLeftColor(v) => write!(f, "border-left-color:{}", v),
            Style::BorderLeftStyle(v) => write!(f, "border-left-style:{}", v),
            Style::BorderLeftWidth(v) => write!(f, "border-left-width:{}", v),
            // border-radius
            // border-right
            Style::BorderRightColor(v) => write!(f, "border-right-color:{}", v),
            Style::BorderRightStyle(v) => write!(f, "border-right-style:{}", v),
            Style::BorderRightWidth(v) => write!(f, "border-right-width:{}", v),
            // border-spacing
            Style::BorderStyle(v) => write!(f, "border-style:{}", v),
            // border-top
            Style::BorderTopColor(v) => write!(f, "border-top-color:{}", v),
            // border-top-left-radius
            // border-top-right-radius
            Style::BorderTopStyle(v) => write!(f, "border-top-style:{}", v),
            Style::BorderTopWidth(v) => write!(f, "border-top-width:{}", v),
            Style::BorderWidth(v) => write!(f, "border-width:{}", v),
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
            Style::Cursor(v) => write!(f, "cursor:{}", v),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub line_width: Option<LineWidth>,
    pub line_style: Option<LineStyle>,
    pub color: Option<Color>,
}

impl Border {
    fn new() -> Self {
        Border {
            line_width: None,
            line_style: None,
            color: None,
        }
    }

    fn is_full(&self) -> bool {
        match (&self.line_width, &self.line_style, &self.color) {
            (Some(_), Some(_), Some(_)) => true,
            _ => false,
        }
    }

    fn has_line_width(&self) -> bool {
        self.line_width.is_some()
    }
    fn has_line_style(&self) -> bool {
        self.line_style.is_some()
    }
    fn has_color(&self) -> bool {
        self.color.is_some()
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn space(yes: bool) -> &'static str {
            if yes {
                " "
            } else {
                ""
            }
        }
        let mut cont = false;
        if let Some(line_width) = self.line_width {
            write!(f, "{}", line_width)?;
            cont = true;
        }
        if let Some(line_style) = self.line_style {
            write!(f, "{}{}", space(cont), line_style)?;
            cont = true;
        }
        if let Some(color) = self.color {
            write!(f, "{}{}", space(cont), color)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderColor {
    All(Color),
    VerticalHorizontal(Color, Color),
    TopHorizontalBottom(Color, Color, Color),
    TopRightBottomLeft(Color, Color, Color, Color),
}

impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BorderColor::All(a) => write!(f, "{}", a),
            BorderColor::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            BorderColor::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            BorderColor::TopRightBottomLeft(t, r, b, l) => write!(f, "{} {} {} {}", t, r, b, l),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    All(LineStyle),
    VerticalHorizontal(LineStyle, LineStyle),
    TopHorizontalBottom(LineStyle, LineStyle, LineStyle),
    TopRightBottomLeft(LineStyle, LineStyle, LineStyle, LineStyle),
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BorderStyle::All(a) => write!(f, "{}", a),
            BorderStyle::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            BorderStyle::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            BorderStyle::TopRightBottomLeft(t, r, b, l) => write!(f, "{} {} {} {}", t, r, b, l),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderWidth {
    All(LineWidth),
    VerticalHorizontal(LineWidth, LineWidth),
    TopHorizontalBottom(LineWidth, LineWidth, LineWidth),
    TopRightBottomLeft(LineWidth, LineWidth, LineWidth, LineWidth),
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BorderWidth::All(a) => write!(f, "{}", a),
            BorderWidth::VerticalHorizontal(v, h) => write!(f, "{} {}", v, h),
            BorderWidth::TopHorizontalBottom(t, h, b) => write!(f, "{} {} {}", t, h, b),
            BorderWidth::TopRightBottomLeft(t, r, b, l) => write!(f, "{} {} {} {}", t, r, b, l),
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
pub enum Cursor {
    // todo url
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NEResize,
    NWResize,
    SResize,
    SEResize,
    SWResize,
    WResize,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cursor::Auto => f.write_str("auto"),
            Cursor::Default => f.write_str("default"),
            Cursor::None => f.write_str("none"),
            Cursor::ContextMenu => f.write_str("context-menu"),
            Cursor::Help => f.write_str("help"),
            Cursor::Pointer => f.write_str("pointer"),
            Cursor::Progress => f.write_str("progress"),
            Cursor::Wait => f.write_str("wait"),
            Cursor::Cell => f.write_str("cell"),
            Cursor::Crosshair => f.write_str("crosshair"),
            Cursor::Text => f.write_str("text"),
            Cursor::VerticalText => f.write_str("vertical-text"),
            Cursor::Alias => f.write_str("alias"),
            Cursor::Copy => f.write_str("copy"),
            Cursor::Move => f.write_str("move"),
            Cursor::NoDrop => f.write_str("no-drop"),
            Cursor::NotAllowed => f.write_str("not-allowed"),
            Cursor::Grab => f.write_str("grab"),
            Cursor::Grabbing => f.write_str("grabbing"),
            Cursor::EResize => f.write_str("e-resize"),
            Cursor::NResize => f.write_str("n-resize"),
            Cursor::NEResize => f.write_str("ne-resize"),
            Cursor::NWResize => f.write_str("nw-resize"),
            Cursor::SResize => f.write_str("s-resize"),
            Cursor::SEResize => f.write_str("se-resize"),
            Cursor::SWResize => f.write_str("sw-resize"),
            Cursor::WResize => f.write_str("w-resize"),
            Cursor::EWResize => f.write_str("ew-resize"),
            Cursor::NSResize => f.write_str("ns-resize"),
            Cursor::NESWResize => f.write_str("nesw-resize"),
            Cursor::NWSEResize => f.write_str("nwse-resize"),
            Cursor::ColResize => f.write_str("col-resize"),
            Cursor::RowResize => f.write_str("row-resize"),
            Cursor::AllScroll => f.write_str("all-scroll"),
            Cursor::ZoomIn => f.write_str("zoom-in"),
            Cursor::ZoomOut => f.write_str("zoom-out"),
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl fmt::Display for LineStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineStyle::None => write!(f, "none"),
            LineStyle::Hidden => write!(f, "hidden"),
            LineStyle::Dotted => write!(f, "dotted"),
            LineStyle::Dashed => write!(f, "dashed"),
            LineStyle::Solid => write!(f, "solid"),
            LineStyle::Double => write!(f, "double"),
            LineStyle::Groove => write!(f, "groove"),
            LineStyle::Ridge => write!(f, "ridge"),
            LineStyle::Inset => write!(f, "inset"),
            LineStyle::Outset => write!(f, "outset"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineWidth {
    Length(Length),
    Thin,
    Medium,
    Thick,
}

impl fmt::Display for LineWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineWidth::Length(v) => fmt::Display::fmt(v, f),
            LineWidth::Thin => write!(f, "thin"),
            LineWidth::Medium => write!(f, "medium"),
            LineWidth::Thick => write!(f, "thick"),
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
pub enum Margin {
    All(MarginWidth),
    VerticalHorizontal(MarginWidth, MarginWidth),
    TopHorizontalBottom(MarginWidth, MarginWidth, MarginWidth),
    // todo order might be wrong (doesn't affect output becacuse its only the name that it changes).
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
pub struct Percentage(pub f64);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Url<'a> {
    // todo modifiers
    url: Cow<'a, str>,
}

impl fmt::Display for Url<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "url(\"{}\")", self.url)
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
