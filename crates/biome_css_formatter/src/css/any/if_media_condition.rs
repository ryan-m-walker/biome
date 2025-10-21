//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfMediaCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfMediaCondition;
impl FormatRule<AnyCssIfMediaCondition> for FormatAnyCssIfMediaCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfMediaCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfMediaCondition::AnyCssMediaCondition(node) => node.format().fmt(f),
            AnyCssIfMediaCondition::AnyCssQueryFeature(node) => node.format().fmt(f),
        }
    }
}
