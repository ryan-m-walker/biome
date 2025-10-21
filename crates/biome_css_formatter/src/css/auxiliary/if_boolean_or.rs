use crate::prelude::*;
use biome_css_syntax::CssIfBooleanOr;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBooleanOr;
impl FormatNodeRule<CssIfBooleanOr> for FormatCssIfBooleanOr {
    fn fmt_fields(&self, node: &CssIfBooleanOr, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
