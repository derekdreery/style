//! Javascript string escaping

pub fn escape<'a>(&'a str) -> impl Display + 'a {

}

struct CssEscape<'a>(&'a str);

impl<'a> fmt::Display for CssEscape<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ch in self.0.chars() {
            todo!()
        }
    }
}
