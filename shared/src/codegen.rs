use crate::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for DynamicStyles<'_> {
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

impl ToTokens for DynamicStyle<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            DynamicStyle::Dynamic(block) => quote!(#block),
            DynamicStyle::Literal(lit) => quote!(#lit),
        })
    }
}

impl ToTokens for Style<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
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
            Style::Border(v) => quote!(style::Style::Border(#v)),
            // border-bottom
            Style::BorderBottomColor(v) => quote!(style::Style::BorderBottomColor(#v)),
            // border-bottom-left-radius
            // border-bottom-right-radius
            Style::BorderBottomStyle(v) => quote!(style::Style::BorderBottomStyle(#v)),
            Style::BorderBottomWidth(v) => quote!(style::Style::BorderBottomWidth(#v)),
            // border-collapse
            // border-color
            Style::BorderColor(v) => quote!(style::Style::BorderColor(#v)),
            // border-image
            // border-image-outset
            // border-image-repeat
            // border-image-slice
            // border-image-source
            // border-image-width
            // border-left
            Style::BorderLeftColor(v) => quote!(style::Style::BorderLeftColor(#v)),
            Style::BorderLeftStyle(v) => quote!(style::Style::BorderLeftStyle(#v)),
            Style::BorderLeftWidth(v) => quote!(style::Style::BorderLeftWidth(#v)),
            // border-radius
            // border-right
            Style::BorderRightColor(v) => quote!(style::Style::BorderRightColor(#v)),
            Style::BorderRightStyle(v) => quote!(style::Style::BorderRightStyle(#v)),
            Style::BorderRightWidth(v) => quote!(style::Style::BorderRightWidth(#v)),
            // border-spacing
            Style::BorderStyle(v) => quote!(style::Style::BorderStyle(#v)),
            // border-top
            Style::BorderTopColor(v) => quote!(style::Style::BorderTopColor(#v)),
            // border-top-left-radius
            // border-top-right-radius
            Style::BorderTopStyle(v) => quote!(style::Style::BorderTopStyle(#v)),
            Style::BorderTopWidth(v) => quote!(style::Style::BorderTopWidth(#v)),
            Style::BorderWidth(v) => quote!(style::Style::BorderWidth(#v)),
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
            Style::Cursor(v) => quote!(style::Style::Cursor(#v)),
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
            Style::FontSize(v) => quote!(style::Style::FontSize(#v)),
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

impl ToTokens for Cursor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Cursor::Auto => quote!(Cursor::Auto),
            Cursor::Default => quote!(Cursor::Default),
            Cursor::None => quote!(Cursor::None),
            Cursor::ContextMenu => quote!(Cursor::ContextMenu),
            Cursor::Help => quote!(Cursor::Help),
            Cursor::Pointer => quote!(Cursor::Pointer),
            Cursor::Progress => quote!(Cursor::Progress),
            Cursor::Wait => quote!(Cursor::Wait),
            Cursor::Cell => quote!(Cursor::Cell),
            Cursor::Crosshair => quote!(Cursor::Crosshair),
            Cursor::Text => quote!(Cursor::Text),
            Cursor::VerticalText => quote!(Cursor::VerticalText),
            Cursor::Alias => quote!(Cursor::Alias),
            Cursor::Copy => quote!(Cursor::Copy),
            Cursor::Move => quote!(Cursor::Move),
            Cursor::NoDrop => quote!(Cursor::NoDrop),
            Cursor::NotAllowed => quote!(Cursor::NotAllowed),
            Cursor::Grab => quote!(Cursor::Grab),
            Cursor::Grabbing => quote!(Cursor::Grabbing),
            Cursor::EResize => quote!(Cursor::EResize),
            Cursor::NResize => quote!(Cursor::NResize),
            Cursor::NEResize => quote!(Cursor::NEResize),
            Cursor::NWResize => quote!(Cursor::NWResize),
            Cursor::SResize => quote!(Cursor::SResize),
            Cursor::SEResize => quote!(Cursor::SEResize),
            Cursor::SWResize => quote!(Cursor::SWResize),
            Cursor::WResize => quote!(Cursor::WResize),
            Cursor::EWResize => quote!(Cursor::EWResize),
            Cursor::NSResize => quote!(Cursor::NSResize),
            Cursor::NESWResize => quote!(Cursor::NESWResize),
            Cursor::NWSEResize => quote!(Cursor::NWSEResize),
            Cursor::ColResize => quote!(Cursor::ColResize),
            Cursor::RowResize => quote!(Cursor::RowResize),
            Cursor::AllScroll => quote!(Cursor::AllScroll),
            Cursor::ZoomIn => quote!(Cursor::ZoomIn),
            Cursor::ZoomOut => quote!(Cursor::ZoomOut),
        })
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
impl ToTokens for FontSize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontSize::XXSmall => quote!(style::FontSize::XXSmall),
            FontSize::XSmall => quote!(style::FontSize::XSmall),
            FontSize::Small => quote!(style::FontSize::Small),
            FontSize::Medium => quote!(style::FontSize::Medium),
            FontSize::Large => quote!(style::FontSize::Large),
            FontSize::XLarge => quote!(style::FontSize::XLarge),
            FontSize::XXLarge => quote!(style::FontSize::XXLarge),
            FontSize::XXXLarge => quote!(style::FontSize::XXXLarge),
            FontSize::Larger => quote!(style::FontSize::Larger),
            FontSize::Smaller => quote!(style::FontSize::Smaller),
            FontSize::LengthPercentage(v) => quote!(style::FontSize::LengthPercentage(#v)),
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

impl ToTokens for Border {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let line_width = match self.line_width {
            Some(line_width) => quote!(Some(#line_width)),
            None => quote!(None),
        };
        let line_style = match self.line_style {
            Some(line_style) => quote!(Some(#line_style)),
            None => quote!(None),
        };
        let color = match self.color {
            Some(color) => quote!(Some(#color)),
            None => quote!(None),
        };
        tokens.extend(quote!(
            style::Border {
                line_width: #line_width,
                line_style: #line_style,
                color: #color,
            }
        ))
    }
}

impl ToTokens for BorderColor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BorderColor::All(v) => quote!(style::BorderColor::All(#v)),
            BorderColor::VerticalHorizontal(v, h) => {
                quote!(style::BorderColor::VerticalHorizontal(#v, #h))
            }
            BorderColor::TopHorizontalBottom(t, h, b) => {
                quote!(style::BorderColor::TopHorizontalBottom(#t, #h, #b))
            }
            BorderColor::TopRightBottomLeft(t, r, b, l) => {
                quote!(style::BorderColor::TopRightBottomLeft(#t, #r, #b, #l))
            }
        })
    }
}

impl ToTokens for BorderStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BorderStyle::All(v) => quote!(style::BorderStyle::All(#v)),
            BorderStyle::VerticalHorizontal(v, h) => {
                quote!(style::BorderStyle::VerticalHorizontal(#v, #h))
            }
            BorderStyle::TopHorizontalBottom(t, h, b) => {
                quote!(style::BorderStyle::TopHorizontalBottom(#t, #h, #b))
            }
            BorderStyle::TopRightBottomLeft(t, r, b, l) => {
                quote!(style::BorderStyle::TopRightBottomLeft(#t, #r, #b, #l))
            }
        })
    }
}

impl ToTokens for BorderWidth {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BorderWidth::All(v) => quote!(style::BorderWidth::All(#v)),
            BorderWidth::VerticalHorizontal(v, h) => {
                quote!(style::BorderWidth::VerticalHorizontal(#v, #h))
            }
            BorderWidth::TopHorizontalBottom(t, h, b) => {
                quote!(style::BorderWidth::TopHorizontalBottom(#t, #h, #b))
            }
            BorderWidth::TopRightBottomLeft(t, r, b, l) => {
                quote!(style::BorderWidth::TopRightBottomLeft(#t, #r, #b, #l))
            }
        })
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

impl ToTokens for LineStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            LineStyle::None => quote!(style::LineStyle::None),
            LineStyle::Hidden => quote!(style::LineStyle::Hidden),
            LineStyle::Dotted => quote!(style::LineStyle::Dotted),
            LineStyle::Dashed => quote!(style::LineStyle::Dashed),
            LineStyle::Solid => quote!(style::LineStyle::Solid),
            LineStyle::Double => quote!(style::LineStyle::Double),
            LineStyle::Groove => quote!(style::LineStyle::Groove),
            LineStyle::Ridge => quote!(style::LineStyle::Ridge),
            LineStyle::Inset => quote!(style::LineStyle::Inset),
            LineStyle::Outset => quote!(style::LineStyle::Outset),
        })
    }
}

impl ToTokens for LineWidth {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            LineWidth::Length(length) => quote!(style::LineWidth::Length(#length)),
            LineWidth::Thin => quote!(style::LineWidth::Thin),
            LineWidth::Medium => quote!(style::LineWidth::Medium),
            LineWidth::Thick => quote!(style::LineWidth::Thick),
        })
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

impl ToTokens for DynamicColor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            DynamicColor::Dynamic(block) => quote!(style::DynamicColor::Literal(#block)),
            DynamicColor::Literal(color) => quote!(style::DynamicColor::Literal(#color)),
        })
    }
}

impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Color::HexRGB(r, g, b) => quote!(style::Color::HexRGB(#r, #g, #b)),
            Color::HexRGBA(r, g, b, a) => quote!(style::Color::HexRGB(#r, #g, #b, #a)),
            Color::HSL(h, s, l) => quote!(style::Color::HSL(#h, #s, #l)),
            Color::HSLA(h, s, l, a) => quote!(style::Color::HSLA(#h, #s, #l, #a)),
            Color::IndianRed => quote!(style::Color::IndianRed),
            Color::LightCoral => quote!(style::Color::LightCoral),
            Color::Salmon => quote!(style::Color::Salmon),
            Color::DarkSalmon => quote!(style::Color::DarkSalmon),
            Color::LightSalmon => quote!(style::Color::LightSalmon),
            Color::Crimson => quote!(style::Color::Crimson),
            Color::Red => quote!(style::Color::Red),
            Color::FireBrick => quote!(style::Color::FireBrick),
            Color::DarkRed => quote!(style::Color::DarkRed),
            Color::Pink => quote!(style::Color::Pink),
            Color::LightPink => quote!(style::Color::LightPink),
            Color::HotPink => quote!(style::Color::HotPink),
            Color::DeepPink => quote!(style::Color::DeepPink),
            Color::MediumVioletRed => quote!(style::Color::MediumVioletRed),
            Color::PaleVioletRed => quote!(style::Color::PaleVioletRed),
            Color::Coral => quote!(style::Color::Coral),
            Color::Tomato => quote!(style::Color::Tomato),
            Color::OrangeRed => quote!(style::Color::OrangeRed),
            Color::DarkOrange => quote!(style::Color::DarkOrange),
            Color::Orange => quote!(style::Color::Orange),
            Color::Gold => quote!(style::Color::Gold),
            Color::Yellow => quote!(style::Color::Yellow),
            Color::LightYellow => quote!(style::Color::LightYellow),
            Color::LemonChiffon => quote!(style::Color::LemonChiffon),
            Color::LightGoldenrodYellow => quote!(style::Color::LightGoldenrodYellow),
            Color::PapayaWhip => quote!(style::Color::PapayaWhip),
            Color::Moccasin => quote!(style::Color::Moccasin),
            Color::PeachPuff => quote!(style::Color::PeachPuff),
            Color::PaleGoldenrod => quote!(style::Color::PaleGoldenrod),
            Color::Khaki => quote!(style::Color::Khaki),
            Color::DarkKhaki => quote!(style::Color::DarkKhaki),
            Color::Lavender => quote!(style::Color::Lavender),
            Color::Thistle => quote!(style::Color::Thistle),
            Color::Plum => quote!(style::Color::Plum),
            Color::Violet => quote!(style::Color::Violet),
            Color::Orchid => quote!(style::Color::Orchid),
            Color::Fuchsia => quote!(style::Color::Fuchsia),
            Color::Magenta => quote!(style::Color::Magenta),
            Color::MediumOrchid => quote!(style::Color::MediumOrchid),
            Color::MediumPurple => quote!(style::Color::MediumPurple),
            Color::RebeccaPurple => quote!(style::Color::RebeccaPurple),
            Color::BlueViolet => quote!(style::Color::BlueViolet),
            Color::DarkViolet => quote!(style::Color::DarkViolet),
            Color::DarkOrchid => quote!(style::Color::DarkOrchid),
            Color::DarkMagenta => quote!(style::Color::DarkMagenta),
            Color::Purple => quote!(style::Color::Purple),
            Color::Indigo => quote!(style::Color::Indigo),
            Color::SlateBlue => quote!(style::Color::SlateBlue),
            Color::DarkSlateBlue => quote!(style::Color::DarkSlateBlue),
            Color::MediumSlateBlue => quote!(style::Color::MediumSlateBlue),
            Color::GreenYellow => quote!(style::Color::GreenYellow),
            Color::Chartreuse => quote!(style::Color::Chartreuse),
            Color::LawnGreen => quote!(style::Color::LawnGreen),
            Color::Lime => quote!(style::Color::Lime),
            Color::LimeGreen => quote!(style::Color::LimeGreen),
            Color::PaleGreen => quote!(style::Color::PaleGreen),
            Color::LightGreen => quote!(style::Color::LightGreen),
            Color::MediumSpringGreen => quote!(style::Color::MediumSpringGreen),
            Color::SpringGreen => quote!(style::Color::SpringGreen),
            Color::MediumSeaGreen => quote!(style::Color::MediumSeaGreen),
            Color::SeaGreen => quote!(style::Color::SeaGreen),
            Color::ForestGreen => quote!(style::Color::ForestGreen),
            Color::Green => quote!(style::Color::Green),
            Color::DarkGreen => quote!(style::Color::DarkGreen),
            Color::YellowGreen => quote!(style::Color::YellowGreen),
            Color::OliveDrab => quote!(style::Color::OliveDrab),
            Color::Olive => quote!(style::Color::Olive),
            Color::DarkOliveGreen => quote!(style::Color::DarkOliveGreen),
            Color::MediumAquamarine => quote!(style::Color::MediumAquamarine),
            Color::DarkSeaGreen => quote!(style::Color::DarkSeaGreen),
            Color::LightSeaGreen => quote!(style::Color::LightSeaGreen),
            Color::DarkCyan => quote!(style::Color::DarkCyan),
            Color::Teal => quote!(style::Color::Teal),
            Color::Aqua => quote!(style::Color::Aqua),
            Color::Cyan => quote!(style::Color::Cyan),
            Color::LightCyan => quote!(style::Color::LightCyan),
            Color::PaleTurquoise => quote!(style::Color::PaleTurquoise),
            Color::Aquamarine => quote!(style::Color::Aquamarine),
            Color::Turquoise => quote!(style::Color::Turquoise),
            Color::MediumTurquoise => quote!(style::Color::MediumTurquoise),
            Color::DarkTurquoise => quote!(style::Color::DarkTurquoise),
            Color::CadetBlue => quote!(style::Color::CadetBlue),
            Color::SteelBlue => quote!(style::Color::SteelBlue),
            Color::LightSteelBlue => quote!(style::Color::LightSteelBlue),
            Color::PowderBlue => quote!(style::Color::PowderBlue),
            Color::LightBlue => quote!(style::Color::LightBlue),
            Color::SkyBlue => quote!(style::Color::SkyBlue),
            Color::LightSkyBlue => quote!(style::Color::LightSkyBlue),
            Color::DeepSkyBlue => quote!(style::Color::DeepSkyBlue),
            Color::DodgerBlue => quote!(style::Color::DodgerBlue),
            Color::CornflowerBlue => quote!(style::Color::CornflowerBlue),
            Color::RoyalBlue => quote!(style::Color::RoyalBlue),
            Color::Blue => quote!(style::Color::Blue),
            Color::MediumBlue => quote!(style::Color::MediumBlue),
            Color::DarkBlue => quote!(style::Color::DarkBlue),
            Color::Navy => quote!(style::Color::Navy),
            Color::MidnightBlue => quote!(style::Color::MidnightBlue),
            Color::Cornsilk => quote!(style::Color::Cornsilk),
            Color::BlanchedAlmond => quote!(style::Color::BlanchedAlmond),
            Color::Bisque => quote!(style::Color::Bisque),
            Color::NavajoWhite => quote!(style::Color::NavajoWhite),
            Color::Wheat => quote!(style::Color::Wheat),
            Color::BurlyWood => quote!(style::Color::BurlyWood),
            Color::Tan => quote!(style::Color::Tan),
            Color::RosyBrown => quote!(style::Color::RosyBrown),
            Color::SandyBrown => quote!(style::Color::SandyBrown),
            Color::Goldenrod => quote!(style::Color::Goldenrod),
            Color::DarkGoldenrod => quote!(style::Color::DarkGoldenrod),
            Color::Peru => quote!(style::Color::Peru),
            Color::Chocolate => quote!(style::Color::Chocolate),
            Color::SaddleBrown => quote!(style::Color::SaddleBrown),
            Color::Sienna => quote!(style::Color::Sienna),
            Color::Brown => quote!(style::Color::Brown),
            Color::Maroon => quote!(style::Color::Maroon),
            Color::White => quote!(style::Color::White),
            Color::Snow => quote!(style::Color::Snow),
            Color::HoneyDew => quote!(style::Color::HoneyDew),
            Color::MintCream => quote!(style::Color::MintCream),
            Color::Azure => quote!(style::Color::Azure),
            Color::AliceBlue => quote!(style::Color::AliceBlue),
            Color::GhostWhite => quote!(style::Color::GhostWhite),
            Color::WhiteSmoke => quote!(style::Color::WhiteSmoke),
            Color::SeaShell => quote!(style::Color::SeaShell),
            Color::Beige => quote!(style::Color::Beige),
            Color::OldLace => quote!(style::Color::OldLace),
            Color::FloralWhite => quote!(style::Color::FloralWhite),
            Color::Ivory => quote!(style::Color::Ivory),
            Color::AntiqueWhite => quote!(style::Color::AntiqueWhite),
            Color::Linen => quote!(style::Color::Linen),
            Color::LavenderBlush => quote!(style::Color::LavenderBlush),
            Color::MistyRose => quote!(style::Color::MistyRose),
            Color::Gainsboro => quote!(style::Color::Gainsboro),
            Color::LightGray => quote!(style::Color::LightGray),
            Color::Silver => quote!(style::Color::Silver),
            Color::DarkGray => quote!(style::Color::DarkGray),
            Color::Gray => quote!(style::Color::Gray),
            Color::DimGray => quote!(style::Color::DimGray),
            Color::LightSlateGray => quote!(style::Color::LightSlateGray),
            Color::SlateGray => quote!(style::Color::SlateGray),
            Color::DarkSlateGray => quote!(style::Color::DarkSlateGray),
            Color::Black => quote!(style::Color::Black),
        })
    }
}
