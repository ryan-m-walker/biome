//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfSupportsCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfSupportsCondition;
impl FormatRule<AnyCssIfSupportsCondition> for FormatAnyCssIfSupportsCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfSupportsCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfSupportsCondition::AnyCssSupportsCondition(node) => node.format().fmt(f),
            AnyCssIfSupportsCondition::CssIfSupportsDeclaration(node) => node.format().fmt(f),
        }
    }
}
