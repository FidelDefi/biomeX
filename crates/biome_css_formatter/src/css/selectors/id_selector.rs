use crate::prelude::*;
use biome_css_syntax::CssIdSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdSelector;
impl FormatNodeRule<CssIdSelector> for FormatCssIdSelector {
    fn fmt_fields(&self, node: &CssIdSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
