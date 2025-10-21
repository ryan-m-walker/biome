use crate::prelude::*;
use biome_css_syntax::CssIfBooleanAnd;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBooleanAnd;
impl FormatNodeRule<CssIfBooleanAnd> for FormatCssIfBooleanAnd {
    fn fmt_fields(&self, node: &CssIfBooleanAnd, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
