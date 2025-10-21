//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::CssIfBooleanExprGroup;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBooleanExprGroup;
impl FormatRule<CssIfBooleanExprGroup> for FormatCssIfBooleanExprGroup {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssIfBooleanExprGroup, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            CssIfBooleanExprGroup::AnyCssIfTest(node) => node.format().fmt(f),
            CssIfBooleanExprGroup::CssIfBooleanParenthesized(node) => node.format().fmt(f),
        }
    }
}
