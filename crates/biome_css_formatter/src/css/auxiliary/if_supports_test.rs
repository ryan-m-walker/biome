use crate::prelude::*;
use biome_css_syntax::CssIfSupportsTest;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSupportsTest;
impl FormatNodeRule<CssIfSupportsTest> for FormatCssIfSupportsTest {
    fn fmt_fields(&self, node: &CssIfSupportsTest, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
