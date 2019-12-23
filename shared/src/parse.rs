//! Implementation of `syn::parse::Parse` for styles, and associated helper data/functions.
// TODO make all parsers use HyphenWord where appropriate
// TODO make all error messages nice
// TODO 100% test coverage
// TODO see if I can get https://github.com/rust-lang/rust/issues/67544 accepted. then change m to
// em and x to ex.
use crate::*;
use proc_macro2::Span;
use std::{cell::RefCell, collections::BTreeSet, fmt::Write};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Ident, Token,
};

impl Parse for Styles<'static> {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let punc = s.parse_terminated::<_, Token![;]>(<Style as Parse>::parse)?;
        Ok(Styles::from(punc.into_iter().collect::<Vec<_>>()))
    }
}

impl Parse for Style<'static> {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        // Pass through brackets
        if s.peek(syn::token::Brace) {
            let literal;
            syn::braced!(literal in s);
            let wrapper = TokenWrapper(literal.cursor().token_stream());
            //panic!("{:#?}", wrapper);
            return Ok(Style::Tokens(wrapper));
        }

        let name: HyphenWord = s.parse()?;
        if name.try_match("dummy") {
            return Ok(Style::Dummy);
        }

        s.parse::<Token![:]>()?;

        // align-content
        if name.try_match("align-items") {
            Ok(Style::AlignItems(s.parse()?))
        // align-self
        // all
        // azimuth
        // background
        // background-attachment
        // background-blend-mode
        // background-clip
        } else if name.try_match("background-color") {
            Ok(Style::BackgroundColor(s.parse()?))
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
        } else if name.try_match("box-sizing") {
            Ok(Style::BoxSizing(s.parse()?))
        // break-after
        // break-before
        // break-inside
        // caption-side
        // caret-color
        // clear
        // clip
        // clip-path
        // clip-rule
        } else if name.try_match("color") {
            Ok(Style::Color(s.parse()?))
        // contain
        // content
        // counter-increment
        // counter-reset
        // cue
        // cue-after
        // cue-before
        // cursor
        // direction
        } else if name.try_match("display") {
            Ok(Style::Display(s.parse()?))
        // elevation
        // empty-cells
        // flex
        } else if name.try_match("flex-basis") {
            Ok(Style::FlexBasis(s.parse()?))
        } else if name.try_match("flex-direction") {
            Ok(Style::FlexDirection(s.parse()?))
        // flex-flow
        } else if name.try_match("flex-grow") {
            let (val, span, suffix) = parse_number(s)?;
            if !suffix.is_empty() {
                return Err(syn::Error::new(span, "expected number"));
            }
            Ok(Style::FlexGrow(val))
        } else if name.try_match("flex-shrink") {
            let (val, span, suffix) = parse_number(s)?;
            if !suffix.is_empty() {
                return Err(syn::Error::new(span, "expected number"));
            }
            Ok(Style::FlexShrink(val))
        } else if name.try_match("flex-wrap") {
            Ok(Style::FlexWrap(s.parse()?))
        // float
        // font
        } else if name.try_match("font-family") {
            let lit: syn::LitStr = s.parse()?;
            Ok(Style::FontFamily(lit.value().into()))
        // font-feature-settings
        // font-kerning
        // font-size
        // font-size-adjust
        // font-stretch
        } else if name.try_match("font-style") {
            Ok(Style::FontStyle(s.parse()?))
        // font-synthesis
        // font-variant
        // font-variant-caps
        // font-variant-east-asian
        // font-variant-ligatures
        // font-variant-numeric
        // font-variant-position
        } else if name.try_match("font-weight") {
            Ok(Style::FontWeight(s.parse()?))
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
        } else if name.try_match("height") {
            Ok(Style::Height(s.parse()?))
        // image-orientation
        // image-rendering
        // isolation
        } else if name.try_match("justify-content") {
            Ok(Style::JustifyContent(s.parse()?))
        // left
        // letter-spacing
        // line-height
        // list-style
        // list-style-image
        // list-style-position
        } else if name.try_match("list-style-type") {
            Ok(Style::ListStyleType(s.parse()?))
        } else if name.try_match("margin") {
            Ok(Style::Margin(s.parse()?))
        } else if name.try_match("margin-bottom") {
            Ok(Style::MarginBottom(s.parse()?))
        } else if name.try_match("margin-left") {
            Ok(Style::MarginLeft(s.parse()?))
        } else if name.try_match("margin-right") {
            Ok(Style::MarginRight(s.parse()?))
        } else if name.try_match("margin-top") {
            Ok(Style::MarginTop(s.parse()?))
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
        } else if name.try_match("min-height") {
            Ok(Style::MinHeight(s.parse()?))
        } else if name.try_match("min-width") {
            Ok(Style::MinWidth(s.parse()?))
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
        } else if name.try_match("padding") {
            Ok(Style::Padding(s.parse()?))
        } else if name.try_match("padding-bottom") {
            Ok(Style::PaddingBottom(s.parse()?))
        } else if name.try_match("padding-left") {
            Ok(Style::PaddingLeft(s.parse()?))
        } else if name.try_match("padding-right") {
            Ok(Style::PaddingRight(s.parse()?))
        } else if name.try_match("padding-top") {
            Ok(Style::PaddingTop(s.parse()?))
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
        } else if name.try_match("resize") {
            Ok(Style::Resize(s.parse()?))
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
        } else if name.try_match("width") {
            Ok(Style::Width(s.parse()?))
        // will-change
        // word-spacing
        // writing-mode
        // z-index
        } else {
            Err(name.error())
        }
    }
}

impl Parse for AlignItems {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(normal);
        syn::custom_keyword!(stretch);
        syn::custom_keyword!(center);
        syn::custom_keyword!(start);
        syn::custom_keyword!(end);

        let lookahead = s.lookahead1();
        if lookahead.peek(normal) {
            s.parse::<normal>()?;
            Ok(AlignItems::Normal)
        } else if lookahead.peek(stretch) {
            s.parse::<stretch>()?;
            Ok(AlignItems::Stretch)
        } else if lookahead.peek(center) {
            s.parse::<center>()?;
            Ok(AlignItems::Center)
        } else if lookahead.peek(start) {
            s.parse::<start>()?;
            Ok(AlignItems::Start)
        } else if lookahead.peek(end) {
            s.parse::<end>()?;
            Ok(AlignItems::End)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for BoxSizing {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(border);
        syn::custom_keyword!(content);

        let lookahead = s.lookahead1();
        if lookahead.peek(border) {
            s.parse::<border>()?;
            s.parse::<Token![-]>()?; // todo error should span all 3 tokens
            s.parse::<Token![box]>()?;
            Ok(BoxSizing::BorderBox)
        } else if lookahead.peek(content) {
            s.parse::<content>()?;
            s.parse::<Token![-]>()?;
            s.parse::<Token![box]>()?;
            Ok(BoxSizing::ContentBox)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Display {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(block);
        syn::custom_keyword!(flex);
        syn::custom_keyword!(inline);

        let lookahead = s.lookahead1();
        if lookahead.peek(block) {
            s.parse::<block>()?;
            Ok(Display::Block)
        } else if lookahead.peek(flex) {
            s.parse::<flex>()?;
            Ok(Display::Flex)
        } else if lookahead.peek(inline) {
            s.parse::<inline>()?;
            Ok(Display::Inline)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for FlexBasis {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(content);

        if s.peek(content) {
            s.parse::<content>()?;
            Ok(FlexBasis::Content)
        } else {
            let w: Width21 = s.parse()?;
            Ok(FlexBasis::Width(w))
        }
    }
}

impl Parse for FlexDirection {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(column);
        syn::custom_keyword!(row);

        let lookahead = s.lookahead1();
        if lookahead.peek(column) {
            s.parse::<column>()?;
            Ok(FlexDirection::Column)
        } else if lookahead.peek(row) {
            s.parse::<row>()?;
            Ok(FlexDirection::Row)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for FlexWrap {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(wrap);
        syn::custom_keyword!(nowrap);

        let lookahead = s.lookahead1();
        if lookahead.peek(wrap) {
            s.parse::<wrap>()?;
            Ok(FlexWrap::Wrap)
        } else if lookahead.peek(nowrap) {
            s.parse::<nowrap>()?;
            Ok(FlexWrap::Nowrap)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for FontStyle {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(normal);
        syn::custom_keyword!(italic);
        syn::custom_keyword!(oblique);

        let lookahead = s.lookahead1();
        if lookahead.peek(normal) {
            s.parse::<normal>()?;
            Ok(FontStyle::Normal)
        } else if lookahead.peek(italic) {
            s.parse::<italic>()?;
            Ok(FontStyle::Italic)
        } else if lookahead.peek(oblique) {
            s.parse::<oblique>()?;
            Ok(FontStyle::Oblique)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for FontWeight {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        const ERR_MSG: &'static str = "expected one of `normal`, `bold`, `lighter`, `bolder`, \
                                       number where `1 <= number <= 1000`";
        syn::custom_keyword!(normal);
        syn::custom_keyword!(bold);
        syn::custom_keyword!(lighter);
        syn::custom_keyword!(bolder);

        if s.peek(normal) {
            s.parse::<normal>()?;
            Ok(FontWeight::Normal)
        } else if s.peek(bold) {
            s.parse::<bold>()?;
            Ok(FontWeight::Bold)
        } else if s.peek(lighter) {
            s.parse::<lighter>()?;
            Ok(FontWeight::Lighter)
        } else if s.peek(bolder) {
            s.parse::<bolder>()?;
            Ok(FontWeight::Bolder)
        } else {
            let (val, span, suffix) =
                parse_number(s).map_err(|e| syn::Error::new(e.span(), ERR_MSG))?;
            if !suffix.is_empty() || val < 1.0 || val > 1000.0 {
                Err(syn::Error::new(span, ERR_MSG))
            } else {
                Ok(FontWeight::Number(val))
            }
        }
    }
}

impl Parse for JustifyContent {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let name: HyphenWord = s.parse()?;

        if name.try_match("flex-start") {
            Ok(JustifyContent::FlexStart)
        } else if name.try_match("flex-end") {
            Ok(JustifyContent::FlexEnd)
        } else if name.try_match("center") {
            Ok(JustifyContent::Center)
        } else if name.try_match("space-between") {
            Ok(JustifyContent::SpaceBetween)
        } else if name.try_match("space-around") {
            Ok(JustifyContent::SpaceAround)
        } else if name.try_match("start") {
            // - not in level 1 spec
            Ok(JustifyContent::FlexStart)
        } else if name.try_match("end") {
            // - not in level 1 spec
            Ok(JustifyContent::FlexEnd)
        } else {
            Err(name.error())
        }
    }
}

impl Parse for Length {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let (val, span, suffix) = parse_number(s)?;
        parse_length(val, span, &suffix, false)
    }
}

impl Parse for ListStyleType {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let name: HyphenWord = s.parse()?;

        if name.try_match("disc") {
            Ok(ListStyleType::Disc)
        } else if name.try_match("circle") {
            Ok(ListStyleType::Circle)
        } else if name.try_match("square") {
            Ok(ListStyleType::Square)
        } else if name.try_match("decimal") {
            Ok(ListStyleType::Decimal)
        } else if name.try_match("decimal-leading-zero") {
            Ok(ListStyleType::DecimalLeadingZero)
        } else if name.try_match("lower-roman") {
            Ok(ListStyleType::LowerRoman)
        } else if name.try_match("upper-roman") {
            Ok(ListStyleType::UpperRoman)
        } else if name.try_match("lower-greek") {
            Ok(ListStyleType::LowerGreek)
        } else if name.try_match("upper-greek") {
            Ok(ListStyleType::UpperGreek)
        } else if name.try_match("lower-latin") {
            Ok(ListStyleType::LowerLatin)
        } else if name.try_match("upper-latin") {
            Ok(ListStyleType::UpperLatin)
        } else if name.try_match("armenian") {
            Ok(ListStyleType::Armenian)
        } else if name.try_match("georgian") {
            Ok(ListStyleType::Georgian)
        } else if name.try_match("lower-alpha") {
            Ok(ListStyleType::LowerAlpha)
        } else if name.try_match("upper-alpha") {
            Ok(ListStyleType::UpperAlpha)
        } else if name.try_match("none") {
            Ok(ListStyleType::None)
        } else {
            Err(name.error())
        }
    }
}

impl Parse for Margin {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let first = s.parse::<MarginWidth>()?;
        let second = match s.parse::<MarginWidth>() {
            Ok(v) => v,
            Err(_) => return Ok(Margin::All(first)),
        };
        let third = match s.parse::<MarginWidth>() {
            Ok(v) => v,
            Err(_) => return Ok(Margin::VerticalHorizontal(first, second)),
        };
        // todo could make the error message better here by looking for `;` and assuming next token
        // is an incorrect number if not.
        match s.parse::<MarginWidth>() {
            Ok(v) => Ok(Margin::LeftTopRightBottom(first, second, third, v)),
            Err(_) => Ok(Margin::TopHorizontalBottom(first, second, third)),
        }
    }
}

impl Parse for MarginWidth {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(auto);
        if s.peek(auto) {
            s.parse::<auto>()?;
            Ok(MarginWidth::Auto)
        } else {
            Ok(MarginWidth::LengthPercentage(s.parse()?))
        }
    }
}

impl Parse for Padding {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let first = s.parse::<PaddingWidth>()?;
        let second = match s.parse::<PaddingWidth>() {
            Ok(v) => v,
            Err(_) => return Ok(Padding::All(first)),
        };
        let third = match s.parse::<PaddingWidth>() {
            Ok(v) => v,
            Err(_) => return Ok(Padding::VerticalHorizontal(first, second)),
        };
        // todo could make the error message better here by looking for `;` and assuming next token
        // is an incorrect number if not.
        match s.parse::<PaddingWidth>() {
            Ok(v) => Ok(Padding::LeftTopRightBottom(first, second, third, v)),
            Err(_) => Ok(Padding::TopHorizontalBottom(first, second, third)),
        }
    }
}

#[test]
fn test_padding() {
    assert_eq!(
        syn::parse_str::<Style>("padding:1m").unwrap(),
        Style::Padding(Padding::All(LengthPercentage::Length(Length::Em(1.0))))
    );
}

impl Parse for Width21 {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(auto);

        if s.peek(auto) {
            s.parse::<auto>()?;
            Ok(Width21::Auto)
        } else {
            Ok(Width21::LengthPercentage(s.parse()?))
        }
    }
}

impl Parse for WidthHeight {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(auto);
        syn::custom_keyword!(min);
        syn::custom_keyword!(max);
        syn::custom_keyword!(fit);
        syn::custom_keyword!(content);

        if s.peek(auto) {
            s.parse::<auto>()?;
            Ok(WidthHeight::Auto)
        } else if s.peek(min) {
            s.parse::<min>()?;
            s.parse::<Token![-]>()?;
            s.parse::<content>()?;
            Ok(WidthHeight::MinContent)
        } else if s.peek(max) {
            s.parse::<max>()?;
            s.parse::<Token![-]>()?;
            s.parse::<content>()?;
            Ok(WidthHeight::MaxContent)
        } else if s.peek(fit) {
            s.parse::<fit>()?;
            s.parse::<Token![-]>()?;
            s.parse::<content>()?;
            let content;
            syn::parenthesized!(content in s);
            let lp = content.parse::<LengthPercentage>()?;
            if !content.is_empty() {
                Err(content.error("trailing tokens"))
            } else {
                Ok(WidthHeight::FitContent(lp))
            }
        } else {
            Ok(WidthHeight::LengthPercentage(s.parse()?))
        }
    }
}

impl Parse for LengthPercentage {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let (val, span, suffix) = parse_number(s)?;
        if s.peek(Token![%]) {
            s.parse::<Token![%]>()?;
            Ok(LengthPercentage::Percentage(Percentage(val)))
        } else {
            Ok(LengthPercentage::Length(parse_length(
                val, span, &suffix, true,
            )?))
        }
    }
}

impl Parse for Resize {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let name: HyphenWord = s.parse()?;

        if name.try_match("none") {
            Ok(Resize::None)
        } else if name.try_match("both") {
            Ok(Resize::Both)
        } else if name.try_match("horizontal") {
            Ok(Resize::Horizontal)
        } else if name.try_match("vertical") {
            Ok(Resize::Vertical)
        } else {
            Err(name.error())
        }
    }
}

// color
// =====

impl Parse for Color {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        if s.peek(Token![#]) {
            return parse_hex_color(s);
        }
        let fn_name: HyphenWord = s.parse()?;
        if fn_name.try_match("hsl") {
            parse_hsl_color(s, false)
        } else if fn_name.try_match("hsla") {
            parse_hsl_color(s, true)
        } else {
            Err(fn_name.error())
        }
    }
}

fn parse_hex_color(s: ParseStream) -> syn::Result<Color> {
    s.parse::<Token![#]>()?;
    let hex_str: Ident = s.parse()?;
    color::parse_hex(&hex_str.to_string())
        .ok_or(syn::Error::new(hex_str.span(), "hex color is invalid"))
}

fn parse_hsl_color(s: ParseStream, with_alpha: bool) -> syn::Result<Color> {
    let content;
    syn::parenthesized!(content in s);
    let (hue, span, suffix) = parse_number(&content)?;
    empty_suffix(&suffix, span)?;
    if hue < 0.0 || hue >= 360.0 {
        return Err(syn::Error::new(
            span,
            "hue should be in the range `0 <= hue < 360`",
        ));
    }
    content.parse::<Token![,]>()?;
    let (sat, span, suffix) = parse_number(&content)?;
    empty_suffix(&suffix, span)?;
    if sat < 0.0 || sat > 100.0 {
        return Err(syn::Error::new(
            span,
            "saturation should be in the range `0 <= sat < 100`",
        ));
    }
    content.parse::<Token![%]>()?;
    content.parse::<Token![,]>()?;
    let (light, span, suffix) = parse_number(&content)?;
    empty_suffix(&suffix, span)?;
    if light < 0.0 || light > 100.0 {
        return Err(syn::Error::new(
            span,
            "lightness should be in the range `0 <= light < 100`",
        ));
    }
    content.parse::<Token![%]>()?;
    if !with_alpha {
        return if content.is_empty() {
            Ok(Color::HSL(hue, sat, light))
        } else {
            Err(content.error("trailing characters"))
        };
    }
    content.parse::<Token![,]>()?;
    let (alpha, span, suffix) = parse_number(&content)?;
    empty_suffix(&suffix, span)?;
    if alpha < 0.0 || alpha > 1.0 {
        return Err(syn::Error::new(
            span,
            "alpha should be in the range `0 <= alpha < 1`",
        ));
    }
    if content.is_empty() {
        Ok(Color::HSLA(hue, sat, light, alpha))
    } else {
        Err(content.error("unexpected trailing characters"))
    }
}

#[test]
fn test_color() {
    assert_eq!(
        syn::parse_str::<Color>("#ffffff").unwrap(),
        Color::HexRGB(255, 255, 255)
    );
    assert_eq!(
        syn::parse_str::<Color>("#fff").unwrap(),
        Color::HexRGB(255, 255, 255)
    );
    assert_eq!(
        syn::parse_str::<Color>("#ffffffff").unwrap(),
        Color::HexRGBA(255, 255, 255, 255)
    );
    assert_eq!(
        syn::parse_str::<Color>("hsl(60, 0%, 0%)").unwrap(),
        Color::HSL(60.0, 0.0, 0.0)
    );
    assert_eq!(
        syn::parse_str::<Color>("hsla(60, 0%, 0%, 0.2)").unwrap(),
        Color::HSLA(60.0, 0.0, 0.0, 0.2)
    );
}

// Util
// ====

// A float parser that accepts integers
fn parse_number(s: ParseStream) -> syn::Result<(f64, Span, String)> {
    let lookahead = s.lookahead1();
    if lookahead.peek(syn::LitFloat) {
        let tok = s.parse::<syn::LitFloat>()?;
        let num = tok.base10_parse()?;
        Ok((num, tok.span(), tok.suffix().to_string()))
    } else if lookahead.peek(syn::LitInt) {
        let tok = s.parse::<syn::LitInt>()?;
        // we only need up to 360 and u32 can be safely converted into f64
        let num = tok.base10_parse::<u32>()?;
        Ok((num.into(), tok.span(), tok.suffix().to_string()))
    } else {
        Err(lookahead.error())
    }
}

/// A helper so we can parse a length without needing to call `ParseStream::parse`.
///
/// Optionally include `%` in the list of expected suffixes if the caller will accept a percentage.
fn parse_length(val: f64, span: Span, suffix: &str, inc_percent: bool) -> syn::Result<Length> {
    if suffix == "m" {
        Ok(Length::Em(val))
    } else if suffix == "x" {
        Ok(Length::Ex(val))
    } else if suffix == "in" {
        Ok(Length::In(val))
    } else if suffix == "cm" {
        Ok(Length::Cm(val))
    } else if suffix == "mm" {
        Ok(Length::Mm(val))
    } else if suffix == "pt" {
        Ok(Length::Pt(val))
    } else if suffix == "pc" {
        Ok(Length::Pc(val))
    } else if suffix == "px" {
        Ok(Length::Px(val))
    } else if suffix == "" && val == 0.0 {
        Ok(Length::Zero)
    } else {
        let percent_str = if inc_percent { ", `%`" } else { "" };
        Err(syn::Error::new(
            span,
            format!(
                "expected one of `m`, `x`, `in`, `cm`, `mm`, `pt`, `pc`, `px`{} after number",
                percent_str
            ),
        ))
    }
}

fn empty_suffix(suffix: &str, span: Span) -> syn::Result<()> {
    if suffix != "" {
        Err(syn::Error::new(span, "unexpected characters after number"))
    } else {
        Ok(())
    }
}

/// Something like `word-separated-hyphens`
struct HyphenWord {
    pub span: Span,
    pub word: String,
    /// List of tried matches - for building error.
    pub tried: TryList,
}

impl HyphenWord {
    pub fn new(span: Span, word: String) -> Self {
        HyphenWord {
            span,
            word,
            tried: TryList::new(),
        }
    }

    pub fn try_match(&self, other: &str) -> bool {
        if other == self.word {
            true
        } else {
            self.tried.add_literal(other);
            false
        }
    }

    /// Panics if there were no calls to `try_match` before calling this function.
    pub fn error(&self) -> syn::Error {
        self.tried.to_error(self.span)
    }
}

impl Parse for HyphenWord {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let first = s.call(Ident::parse_any)?;
        let mut word = first.to_string();
        let mut span = first.span();
        while s.peek(Token![-]) {
            let hyphen = s.parse::<Token![-]>()?;
            if let Some(joined) = span.join(hyphen.span) {
                span = joined;
            }
            let part = s.call(Ident::parse_any)?;
            write!(word, "-{}", part).unwrap();
            if let Some(joined) = span.join(part.span()) {
                span = joined;
            }
        }
        Ok(HyphenWord::new(span, word))
    }
}

#[test]
fn test_hyphen_word() {
    let word: HyphenWord = syn::parse_str("first-second-third").unwrap();
    assert_eq!(word.word, "first-second-third");
    assert!(syn::parse_str::<HyphenWord>("first-second-").is_err());
    assert!(syn::parse_str::<HyphenWord>("-second").is_err());
    assert!(syn::parse_str::<HyphenWord>("").is_err());
    assert!(syn::parse_str::<HyphenWord>("a a").is_err());
}

/// Keeps track of a list of tokens that have been tried.
pub struct TryList(RefCell<BTreeSet<String>>);

impl TryList {
    pub fn new() -> Self {
        TryList(RefCell::new(BTreeSet::new()))
    }

    /// Same as add, but with quotes
    pub fn add_literal(&self, lit: &str) {
        self.add(format!("`{}`", lit));
    }

    pub fn add(&self, ty: impl Into<String>) {
        self.0.borrow_mut().insert(ty.into());
    }

    fn to_error(&self, span: Span) -> syn::Error {
        let tried = self.0.borrow();
        let mut iter = tried.iter();
        let start = iter.next().unwrap().to_owned();
        let list = iter.fold(start, |mut acc, itm| {
            write!(acc, ", {}", itm).unwrap();
            acc
        });
        let error_msg = format!("expected one of {}", list);
        syn::Error::new(span, error_msg)
    }
}

#[test]
fn downstream_bug1() {
    let s: Styles = syn::parse_str(
        "display: flex;
        flex-direction: column;
        flex-grow: 1;
        flex-shrink: 0;",
    )
    .unwrap();
    assert_eq!(
        s.0,
        vec![
            Style::Display(Display::Flex),
            Style::FlexDirection(FlexDirection::Column),
            Style::FlexGrow(1.0),
            Style::FlexShrink(0.0)
        ]
    )
}

#[test]
#[ignore]
fn inline_logic() {}
