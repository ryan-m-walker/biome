//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfBooleanExpr;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfBooleanExpr;
impl FormatRule<AnyCssIfBooleanExpr> for FormatAnyCssIfBooleanExpr {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfBooleanExpr, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfBooleanExpr::CssIfBooleanAnd(node) => node.format().fmt(f),
            AnyCssIfBooleanExpr::CssIfBooleanExprGroup(node) => node.format().fmt(f),
            AnyCssIfBooleanExpr::CssIfBooleanNot(node) => node.format().fmt(f),
            AnyCssIfBooleanExpr::CssIfBooleanOr(node) => node.format().fmt(f),
        }
    }
}
