//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfBooleanAndCombinable;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfBooleanAndCombinable;
impl FormatRule<AnyCssIfBooleanAndCombinable> for FormatAnyCssIfBooleanAndCombinable {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfBooleanAndCombinable, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfBooleanAndCombinable::CssIfBooleanAnd(node) => node.format().fmt(f),
            AnyCssIfBooleanAndCombinable::CssIfBooleanExprGroup(node) => node.format().fmt(f),
        }
    }
}
