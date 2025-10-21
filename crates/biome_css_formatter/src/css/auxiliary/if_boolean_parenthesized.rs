use crate::prelude::*;
use biome_css_syntax::CssIfBooleanParenthesized;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfBooleanParenthesized;
impl FormatNodeRule<CssIfBooleanParenthesized> for FormatCssIfBooleanParenthesized {
    fn fmt_fields(
        &self,
        node: &CssIfBooleanParenthesized,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
