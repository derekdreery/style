use crate::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for Styles<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let parts = self
            .0
            .iter()
            .filter(|style| !style.is_dummy())
            .map(|style| style.to_token_stream());
        tokens.extend(quote! {
            {
                let mut styles = style::Styles::new();
                #(styles.push(#parts);)*
                styles
            }
        })
    }
}

impl ToTokens for Style<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Style::Tokens(TokenWrapper(stream)) => quote!(#stream),
            Style::Dummy => quote!(style::Style::Dummy),

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
            Style::FlexGrow(v) => quote!(style::Style::FlexGrow(#v)),
            Style::FlexShrink(v) => quote!(style::Style::FlexShrink(#v)),
            Style::FlexWrap(v) => quote!(style::Style::FlexWrap(#v)),
            // float
            // font
            Style::FontFamily(v) => {
                quote!(style::Style::FontFamily(std::borrow::Cow::Borrowed(#v)))
            }
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
            Style::PaddingBottom(v) => quote!(style::Style::PaddingBottom(#v)),
            Style::PaddingLeft(v) => quote!(style::Style::PaddingLeft(#v)),
            Style::PaddingRight(v) => quote!(style::Style::PaddingRight(#v)),
            Style::PaddingTop(v) => quote!(style::Style::PaddingTop(#v)),
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

impl ToTokens for FlexBasis {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FlexBasis::Width(v) => quote!(style::FlexBasis::Width(#v)),
            FlexBasis::Content => quote!(style::FlexBasis::Content),
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
            JustifyContent::FlexStart => quote!(style::JustifyContent::FlexStart),
            JustifyContent::Center => quote!(style::JustifyContent::Center),
            JustifyContent::FlexEnd => quote!(style::JustifyContent::FlexEnd),
            JustifyContent::SpaceAround => quote!(style::JustifyContent::SpaceAround),
            JustifyContent::SpaceBetween => quote!(style::JustifyContent::SpaceBetween),
        });
    }
}

impl ToTokens for FontWeight {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontWeight::Normal => quote!(style::FontWeight::Normal),
            FontWeight::Bold => quote!(style::FontWeight::Bold),
            FontWeight::Lighter => quote!(style::FontWeight::Lighter),
            FontWeight::Bolder => quote!(style::FontWeight::Bolder),
            FontWeight::Number(v) => quote!(style::FontWeight::Number(#v)),
        });
    }
}

impl ToTokens for FontStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontStyle::Normal => quote!(style::FontStyle::Normal),
            FontStyle::Italic => quote!(style::FontStyle::Italic),
            FontStyle::Oblique => quote!(style::FontStyle::Oblique),
        });
    }
}

impl ToTokens for BoxSizing {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BoxSizing::BorderBox => quote!(style::BoxSizing::BorderBox),
            BoxSizing::ContentBox => quote!(style::BoxSizing::ContentBox),
        });
    }
}

impl ToTokens for Padding {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Padding::All(v) => quote!(style::Padding::All(#v)),
            Padding::VerticalHorizontal(v, h) => quote!(style::Padding::VerticalHorizontal(#v, #h)),
            Padding::TopHorizontalBottom(t, h, b) => {
                quote!(style::Padding::TopHorizontalBottom(#t, #h, #b))
            }
            Padding::LeftTopRightBottom(l, t, r, b) => {
                quote!(style::Padding::LeftTopRightBottom(#l, #t, #r, #b))
            }
        });
    }
}

impl ToTokens for PaddingWidth {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            PaddingWidth::Length(v) => quote!(style::PaddingWidth::Length(#v)),
            PaddingWidth::Percentage(v) => quote!(style::PaddingWidth::Percentage(#v)),
        });
    }
}

impl ToTokens for Margin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Margin::All(v) => quote!(style::Margin::All(#v)),
            Margin::VerticalHorizontal(v, h) => quote!(style::Margin::VerticalHorizontal(#v, #h)),
            Margin::TopHorizontalBottom(t, h, b) => {
                quote!(style::Margin::TopHorizontalBottom(#t, #h, #b))
            }
            Margin::LeftTopRightBottom(l, t, r, b) => {
                quote!(style::Margin::LeftTopRightBottom(#l, #t, #r, #b))
            }
        });
    }
}

impl ToTokens for MarginWidth {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            MarginWidth::LengthPercentage(v) => quote!(style::MarginWidth::LengthPercentage(#v)),
            MarginWidth::Auto => quote!(style::MarginWidth::Auto),
        });
    }
}

impl ToTokens for ListStyleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            ListStyleType::Disc => quote!(style::ListStyleType::Disc),
            ListStyleType::Circle => quote!(style::ListStyleType::Circle),
            ListStyleType::Square => quote!(style::ListStyleType::Square),
            ListStyleType::Decimal => quote!(style::ListStyleType::Decimal),
            ListStyleType::DecimalLeadingZero => quote!(style::ListStyleType::DecimalLeadingZero),
            ListStyleType::LowerRoman => quote!(style::ListStyleType::LowerRoman),
            ListStyleType::UpperRoman => quote!(style::ListStyleType::UpperRoman),
            ListStyleType::LowerGreek => quote!(style::ListStyleType::LowerGreek),
            ListStyleType::UpperGreek => quote!(style::ListStyleType::UpperGreek),
            ListStyleType::LowerLatin => quote!(style::ListStyleType::LowerLatin),
            ListStyleType::UpperLatin => quote!(style::ListStyleType::UpperLatin),
            ListStyleType::Armenian => quote!(style::ListStyleType::Armenian),
            ListStyleType::Georgian => quote!(style::ListStyleType::Georgian),
            ListStyleType::LowerAlpha => quote!(style::ListStyleType::LowerAlpha),
            ListStyleType::UpperAlpha => quote!(style::ListStyleType::UpperAlpha),
            ListStyleType::None => quote!(style::ListStyleType::None),
        })
    }
}

impl ToTokens for Resize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Resize::None => quote!(style::Resize::None),
            Resize::Both => quote!(style::Resize::Both),
            Resize::Horizontal => quote!(style::Resize::Horizontal),
            Resize::Vertical => quote!(style::Resize::Vertical),
        })
    }
}

impl ToTokens for WidthHeight {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            WidthHeight::Auto => quote!(style::WidthHeight::Auto),
            WidthHeight::LengthPercentage(v) => quote!(style::WidthHeight::LengthPercentage(#v)),
            WidthHeight::MinContent => quote!(style::WidthHeight::MinContent),
            WidthHeight::MaxContent => quote!(style::WidthHeight::MaxContent),
            WidthHeight::FitContent(v) => quote!(style::WidthHeight::FitContent(#v)),
        })
    }
}

impl ToTokens for MaxWidthHeight {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            MaxWidthHeight::None => quote!(style::MaxWidthHeight::None),
            MaxWidthHeight::LengthPercentage(v) => {
                quote!(style::MaxWidthHeight::LengthPercentage(#v))
            }
            MaxWidthHeight::MinContent => quote!(style::MaxWidthHeight::MinContent),
            MaxWidthHeight::MaxContent => quote!(style::MaxWidthHeight::MaxContent),
            MaxWidthHeight::FitContent(v) => quote!(style::MaxWidthHeight::FitContent(#v)),
        })
    }
}

impl ToTokens for Width21 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Width21::Auto => quote!(style::Width21::Auto),
            Width21::LengthPercentage(v) => quote!(style::Width21::LengthPercentage(#v)),
        })
    }
}

impl ToTokens for Length {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Length::Em(v) => quote!(style::Length::Em(#v)),
            Length::Ex(v) => quote!(style::Length::Ex(#v)),
            Length::In(v) => quote!(style::Length::In(#v)),
            Length::Cm(v) => quote!(style::Length::Cm(#v)),
            Length::Mm(v) => quote!(style::Length::Mm(#v)),
            Length::Pt(v) => quote!(style::Length::Pt(#v)),
            Length::Pc(v) => quote!(style::Length::Pc(#v)),
            Length::Px(v) => quote!(style::Length::Px(#v)),
            Length::Zero => quote!(style::Length::Zero),
        })
    }
}

impl ToTokens for Percentage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let val = self.0;
        tokens.extend(quote!(style::Percentage(#val)));
    }
}

impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Color::HexRGB(r, g, b) => quote!(style::Color::HexRGB(#r, #g, #b)),
            Color::HexRGBA(r, g, b, a) => quote!(style::Color::HexRGB(#r, #g, #b, #a)),
            Color::HSL(h, s, l) => quote!(style::Color::HSL(#h, #s, #l)),
            Color::HSLA(h, s, l, a) => quote!(style::Color::HSLA(#h, #s, #l, #a)),
        })
    }
}
