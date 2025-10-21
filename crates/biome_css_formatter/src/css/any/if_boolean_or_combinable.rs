//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfBooleanOrCombinable;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfBooleanOrCombinable;
impl FormatRule<AnyCssIfBooleanOrCombinable> for FormatAnyCssIfBooleanOrCombinable {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfBooleanOrCombinable, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfBooleanOrCombinable::CssIfBooleanExprGroup(node) => node.format().fmt(f),
            AnyCssIfBooleanOrCombinable::CssIfBooleanOr(node) => node.format().fmt(f),
        }
    }
}
