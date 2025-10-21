use crate::prelude::*;
use biome_css_syntax::CssIfBooleanNot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBooleanNot;
impl FormatNodeRule<CssIfBooleanNot> for FormatCssIfBooleanNot {
    fn fmt_fields(&self, node: &CssIfBooleanNot, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
