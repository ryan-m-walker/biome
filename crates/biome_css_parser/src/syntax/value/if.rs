use crate::parser::CssParser;
use crate::syntax::CssComponentValueList;
use crate::syntax::is_nth_at_identifier;
use crate::syntax::parse_error::expected_declaration_item;
use crate::syntax::value::function::is_nth_at_function;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, token_set};

/// Checks if the current position in the CSS parser is at the start of an if() function.
#[inline]
pub(crate) fn is_at_if_function(p: &mut CssParser) -> bool {
    is_nth_at_function(p, 0) && p.cur_text() == "if"
}

/// Parses a CSS if() function
///
/// Syntax: if( [ <if-branch> ; ]* <if-branch> ;? )
#[inline]
pub(crate) fn parse_if_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_function(p) {
        return Absent;
    }

    let m = p.start();

    // Parse 'if' keyword
    p.bump(T![if]);
    p.bump(T!['(']);

    // Parse if-branch list
    IfBranchList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, CSS_IF_FUNCTION))
}

struct IfBranchListParseRecovery;

impl ParseRecovery for IfBranchListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_PARAMETER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set!(T![;], T![')'])) || is_at_if_branch(p)
    }
}

struct IfBranchList;

impl ParseSeparatedList for IfBranchList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_IF_BRANCH_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_if_branch(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &IfBranchListParseRecovery, expected_declaration_item)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// Checks if the parser is at the start of an if-branch
#[inline]
fn is_at_if_branch(p: &mut CssParser) -> bool {
    is_at_if_condition(p)
}

/// Parses an if-branch: <if-condition> : <declaration-value>?
#[inline]
fn parse_if_branch(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_branch(p) {
        return Absent;
    }

    let m = p.start();

    // Parse condition
    parse_if_condition(p).ok();

    // Parse colon
    p.expect(T![:]);

    // Parse value (list of component values)
    CssComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_IF_BRANCH))
}

/// Checks if the parser is at the start of an if-condition
#[inline]
fn is_at_if_condition(p: &mut CssParser) -> bool {
    p.cur_text() == "else" || is_at_if_boolean_expr(p)
}

/// Parses an if-condition: <boolean-expr> | else
#[inline]
fn parse_if_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_condition(p) {
        return Absent;
    }

    let m = p.start();

    if p.cur_text() == "else" {
        p.bump(T![else]);
    } else {
        parse_if_boolean_expr(p).ok();
    }

    Present(m.complete(p, CSS_IF_CONDITION))
}

/// Checks if the parser is at the start of a boolean expression
#[inline]
fn is_at_if_boolean_expr(p: &mut CssParser) -> bool {
    p.cur_text() == "not" || is_at_if_test(p) || p.at(T!['('])
}

/// Parses a boolean expression: not <expr-group> | <expr-group> [ and <expr-group> | or <expr-group> ]*
#[inline]
fn parse_if_boolean_expr(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_boolean_expr(p) {
        return Absent;
    }

    // Handle 'not' prefix
    if p.cur_text() == "not" {
        let m = p.start();
        p.bump(T![not]);
        parse_if_boolean_expr_group(p).ok();
        return Present(m.complete(p, CSS_IF_BOOLEAN_NOT));
    }

    // Parse first expression group
    let left = parse_if_boolean_expr_group(p);

    // Check for 'and' or 'or' combinators
    if p.cur_text() == "and" {
        let m = left.precede(p);
        p.bump(T![and]);
        parse_if_boolean_and_combinable(p).ok();
        Present(m.complete(p, CSS_IF_BOOLEAN_AND))
    } else if p.cur_text() == "or" {
        let m = left.precede(p);
        p.bump(T![or]);
        parse_if_boolean_or_combinable(p).ok();
        Present(m.complete(p, CSS_IF_BOOLEAN_OR))
    } else {
        left
    }
}

/// Parses the combinable part of an 'and' expression
#[inline]
fn parse_if_boolean_and_combinable(p: &mut CssParser) -> ParsedSyntax {
    let expr = parse_if_boolean_expr_group(p);

    if p.cur_text() == "and" {
        let m = expr.precede(p);
        p.bump(T![and]);
        parse_if_boolean_and_combinable(p).ok();
        Present(m.complete(p, CSS_IF_BOOLEAN_AND))
    } else {
        expr
    }
}

/// Parses the combinable part of an 'or' expression
#[inline]
fn parse_if_boolean_or_combinable(p: &mut CssParser) -> ParsedSyntax {
    let expr = parse_if_boolean_expr_group(p);

    if p.cur_text() == "or" {
        let m = expr.precede(p);
        p.bump(T![or]);
        parse_if_boolean_or_combinable(p).ok();
        Present(m.complete(p, CSS_IF_BOOLEAN_OR))
    } else {
        expr
    }
}

/// Parses a boolean expression group: <if-test> | ( <boolean-expr> )
#[inline]
fn parse_if_boolean_expr_group(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T!['(']) {
        // Parenthesized boolean expression
        let m = p.start();
        p.bump(T!['(']);
        parse_if_boolean_expr(p).ok();
        p.expect(T![')']);
        Present(m.complete(p, CSS_IF_BOOLEAN_PARENTHESIZED))
    } else if is_at_if_test(p) {
        parse_if_test(p)
    } else {
        let m = p.start();
        Present(m.complete(p, CSS_IF_BOOLEAN_EXPR_GROUP))
    }
}

/// Checks if the parser is at the start of an if-test
#[inline]
fn is_at_if_test(p: &mut CssParser) -> bool {
    let text = p.cur_text();
    text == "supports" || text == "media" || text == "style"
}

/// Parses an if-test: supports(...) | media(...) | style(...)
#[inline]
fn parse_if_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_test(p) {
        return Absent;
    }

    let text = p.cur_text();

    if text == "supports" {
        parse_if_supports_test(p)
    } else if text == "media" {
        parse_if_media_test(p)
    } else if text == "style" {
        parse_if_style_test(p)
    } else {
        Absent
    }
}

/// Parses supports() test
#[inline]
fn parse_if_supports_test(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![supports]);
    p.expect(T!['(']);

    // Try to parse as a declaration (ident : value) or a supports condition
    if is_nth_at_identifier(p, 0) && p.nth_at(1, T![:]) {
        // Parse as declaration: ident : value
        parse_if_supports_declaration(p).ok();
    } else {
        // Parse as supports condition - for now just consume tokens until ')'
        // TODO: Parse proper supports condition
        let _ = p.start();
        while !p.at(T![')']) && !p.at(T![EOF]) {
            p.bump_any();
        }
    }

    p.expect(T![')']);
    Present(m.complete(p, CSS_IF_SUPPORTS_TEST))
}

/// Parses supports declaration: ident : value
#[inline]
fn parse_if_supports_declaration(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_any(); // identifier
    p.bump(T![:]);
    CssComponentValueList.parse_list(p);
    Present(m.complete(p, CSS_IF_SUPPORTS_DECLARATION))
}

/// Parses media() test
#[inline]
fn parse_if_media_test(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![media]);
    p.expect(T!['(']);

    // TODO: Parse proper media condition
    // For now just consume tokens until ')'
    while !p.at(T![')']) && !p.at(T![EOF]) {
        p.bump_any();
    }

    p.expect(T![')']);
    Present(m.complete(p, CSS_IF_MEDIA_TEST))
}

/// Parses style() test
#[inline]
fn parse_if_style_test(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![style]);
    p.expect(T!['(']);

    // TODO: Parse proper style query
    // For now just consume tokens until ')'
    while !p.at(T![')']) && !p.at(T![EOF]) {
        p.bump_any();
    }

    p.expect(T![')']);
    Present(m.complete(p, CSS_IF_STYLE_TEST))
}
