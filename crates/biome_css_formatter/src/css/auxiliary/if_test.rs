use crate::prelude::*;
use biome_css_syntax::AnyCssIfTest;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTest;
impl FormatNodeRule<AnyCssIfTest> for FormatAnyCssIfTest {
    fn fmt_fields(&self, node: &AnyCssIfTest, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
