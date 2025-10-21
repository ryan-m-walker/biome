use crate::prelude::*;
use biome_css_syntax::CssIfCondition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfCondition;
impl FormatNodeRule<CssIfCondition> for FormatCssIfCondition {
    fn fmt_fields(&self, node: &CssIfCondition, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
