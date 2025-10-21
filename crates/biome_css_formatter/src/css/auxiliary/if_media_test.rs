use crate::prelude::*;
use biome_css_syntax::CssIfMediaTest;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfMediaTest;
impl FormatNodeRule<CssIfMediaTest> for FormatCssIfMediaTest {
    fn fmt_fields(&self, node: &CssIfMediaTest, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
