use crate::prelude::*;
use biome_css_syntax::CssIfStyleTest;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfStyleTest;
impl FormatNodeRule<CssIfStyleTest> for FormatCssIfStyleTest {
    fn fmt_fields(&self, node: &CssIfStyleTest, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
