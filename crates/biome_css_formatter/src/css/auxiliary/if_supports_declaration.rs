use crate::prelude::*;
use biome_css_syntax::CssIfSupportsDeclaration;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSupportsDeclaration;
impl FormatNodeRule<CssIfSupportsDeclaration> for FormatCssIfSupportsDeclaration {
    fn fmt_fields(
        &self,
        node: &CssIfSupportsDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
