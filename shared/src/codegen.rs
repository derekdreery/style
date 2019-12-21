use crate::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for Style<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            // align-content
            Style::AlignItems(v) => quote!(style::Style::AlignItems(#v)),
            // align-self
            // all
            // azimuth
            // background
            // background-attachment
            // background-blend-mode
            // background-clip
            Style::BackgroundColor(v) => quote!(style::Style::BackgroundColor(#v)),
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
            Style::BoxSizing(v) => quote!(style::Style::BoxSizing(#v)),
            // break-after
            // break-before
            // break-inside
            // caption-side
            // caret-color
            // clear
            // clip
            // clip-path
            // clip-rule
            Style::Color(v) => quote!(style::Style::Color(#v)),
            // contain
            // content
            // counter-increment
            // counter-reset
            // cue
            // cue-after
            // cue-before
            // cursor
            // direction
            Style::Display(v) => quote!(style::Style::Display(#v)),
            // elevation
            // empty-cells
            // flex
            Style::FlexBasis(v) => quote!(style::Style::FlexBasis(#v)),
            Style::FlexDirection(v) => quote!(style::Style::FlexDirection(#v)),
            // flex-flow
            Style::FlexGrow(v) => quote!(style::Style::FlexDirection(#v)),
            Style::FlexShrink(v) => quote!(style::Style::FlexShrink(#v)),
            Style::FlexWrap(v) => quote!(style::Style::FlexWrap(#v)),
            // float
            // font
            Style::FontFamily(v) => quote!(style::Style::FontFamily(#v)),
            // font-feature-settings
            // font-kerning
            // font-size
            // font-size-adjust
            // font-stretch
            Style::FontStyle(v) => quote!(style::Style::FontStyle(#v)),
            // font-synthesis
            // font-variant
            // font-variant-caps
            // font-variant-east-asian
            // font-variant-ligatures
            // font-variant-numeric
            // font-variant-position
            Style::FontWeight(v) => quote!(style::Style::FontWeight(#v)),
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
            Style::Height(v) => quote!(style::Style::Height(#v)),
            // image-orientation
            // image-rendering
            // isolation
            Style::JustifyContent(v) => quote!(style::Style::JustifyContent(#v)),
            // left
            // letter-spacing
            // line-height
            // list-style
            // list-style-image
            // list-style-position
            Style::ListStyleType(v) => quote!(style::Style::ListStyleType(#v)),
            Style::Margin(v) => quote!(style::Style::Margin(#v)),
            Style::MarginBottom(v) => quote!(style::Style::MarginBottom(#v)),
            Style::MarginLeft(v) => quote!(style::Style::MarginLeft(#v)),
            Style::MarginRight(v) => quote!(style::Style::MarginRight(#v)),
            Style::MarginTop(v) => quote!(style::Style::MarginTop(#v)),
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
            Style::MinHeight(v) => quote!(style::Style::MinHeight(#v)),
            Style::MinWidth(v) => quote!(style::Style::MinWidth(#v)),
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
            Style::Padding(v) => quote!(style::Style::Padding(#v)),
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
            Style::Resize(v) => quote!(style::Style::Resize(#v)),
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
            Style::Width(v) => quote!(style::Style::Width(#v)),
            // will-change
            // word-spacing
            // writing-mode
            // z-index
        });
    }
}

impl ToTokens for Display {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Display::Block => quote!(style::Display::Block),
            Display::Flex => quote!(style::Display::Flex),
            Display::Inline => quote!(style::Display::Inline),
        });
    }
}

impl ToTokens for FlexDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FlexDirection::Row => quote!(style::FlexDirection::Row),
            FlexDirection::Column => quote!(style::FlexDirection::Column),
        });
    }
}

impl ToTokens for FlexWrap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FlexWrap::Wrap => quote!(style::FlexWrap::Wrap),
            FlexWrap::Nowrap => quote!(style::FlexWrap::Nowrap),
        });
    }
}

impl ToTokens for AlignItems {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            AlignItems::Normal => quote!(style::AlignItems::Normal),
            AlignItems::Stretch => quote!(style::AlignItems::Stretch),
            AlignItems::Center => quote!(style::AlignItems::Center),
            AlignItems::Start => quote!(style::AlignItems::Start),
            AlignItems::End => quote!(style::AlignItems::End),
        });
    }
}

impl ToTokens for JustifyContent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            JustifyContent::Center => quote!(style::JustifyContent::Center),
            JustifyContent::End => quote!(style::JustifyContent::End),
            JustifyContent::SpaceAround => quote!(style::JustifyContent::SpaceAround),
            JustifyContent::SpaceBetween => quote!(style::JustifyContent::SpaceBetween),
            JustifyContent::SpaceEvenly => quote!(style::JustifyContent::SpaceEvenly),
            JustifyContent::Start => quote!(style::JustifyContent::Start),
        });
    }
}

impl ToTokens for FontWeight {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontWeight::_900 => quote!(style::FontWeight::_900),
            FontWeight::Normal => quote!(style::FontWeight::Normal),
            FontWeight::Bold => quote!(style::FontWeight::Bold),
        });
    }
}

impl ToTokens for FontStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontStyle::Normal => quote!(style::FontStyle::Normal),
        });
    }
}

impl ToTokens for BoxSizing {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BoxSizing::BorderBox => quote!(style::BoxSizing::BorderBox),
        });
    }
}
pub enum BoxSizing {
    BorderBox,
