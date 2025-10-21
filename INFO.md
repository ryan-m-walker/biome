
3.2. Boolean Expression Multiplier <boolean-expr[]>
Several contexts (such as @media, @supports, if(), ...) specify conditions, and allow combining those conditions with boolean logic (and/or/not/grouping). Because they use the same non-trivial recursive syntax structure, the special <boolean-expr> production represents this pattern generically.

The <boolean-expr[]> notation wraps another value type in the square brackets within it, e.g. <boolean[ <test> ]>, and represents that value type alone as well as boolean combinations using the not, and, and or keywords and grouping parenthesis. It is formally equivalent to:

<boolean-expr[ <test> ]> = not <boolean-expr-group> | <boolean-expr-group>
                                            [ [ and <boolean-expr-group> ]*
                                            | [ or <boolean-expr-group> ]* ]

<boolean-expr-group> = <test> | ( <boolean-expr[ <test> ]> ) | <general-enclosed>
The <boolean-expr[]> production represents a true, false, or unknown value. Its value is resolved using 3-value Kleene logic, with top-level unknown values (those not directly nested inside the grammar of another <boolean-expr[]>) resolving to false unless otherwise specified; see Appendix B: Boolean Logic for details.

For example, the @container rule allows a wide variety of tests: including size queries, style queries, and scroll-state queries. All of these are arbitrarily combinable with boolean logic. Using <boolean-expr[]>, the grammar for an @container query could be written as:
<container-query> = <boolean-expr[ <cq-test> ]>
<cq-test> = (<size-query>) | style( <style-query> ) | scroll-state( <scroll-state-query> )
<size-query> = <boolean-expr[ ( <size-feature> ) ]> | <size-feature>
<style-query> = <boolean-expr[ ( <style-feature> ) ]> | <style-feature>
<scroll-state-query> = <boolean-expr[ ( <scroll-state-feature> ) ]> | <scroll-state-feature>
The <general-enclosed> branch of the logic allows for future compatibility—​unless otherwise specified new expressions in an older UA will be parsed and considered “unknown”, rather than invalidating the production. For consistency with that allowance, the <test> term in a <boolean-expr[]> should be defined to match <general-enclosed>.


8.3. Conditional Value Selection: the if() notation
The if() function is an arbitrary substitution function that represents conditional values. Its argument consists of an ordered semi-colon–separated list of statements, each consisting of a condition followed by a colon followed by a value. An if() function represents the value corresponding to the first condition in its argument list to be true; if no condition matches, then the if() function represents an empty token stream.

The if() function’s syntax is defined as follows:

<if()> = if( [ <if-branch> ; ]* <if-branch> ;? )
<if-branch> = <if-condition> : <declaration-value>?
<if-condition> = <boolean-expr[ <if-test> ]> | else
<if-test> =
  supports( [ <ident> : <declaration-value> ] | <supports-condition> ) |
  media( <media-feature> | <media-condition> ) |
  style( <style-query> )
The else keyword represents a condition that is always true.

The if() function’s argument grammar is:

<if-args> = if( [ <if-args-branch> ; ]* <if-args-branch> ;? )
<if-args-branch> = <declaration-value> : <declaration-value>?
To replace an if() function, given a list of arguments:
For each <if-args-branch> branch in arguments:

Substitute arbitrary substitution functions in the first <declaration-value> of branch, then parse the result as an <if-condition>. If parsing returns failure, continue; otherwise, let the result be condition.

Evaluate condition.

If a <style-query> in condition tests the value of a property, and guarding a substitution context «"property", referenced-property-name» would mark it as a cyclic substitution context, that query evaluates to false.

For example, in --foo: if(style(--foo: bar): baz); the style() query is automatically false, since property replacement has already established a «"property", "--foo"» substitution context.
If the result of condition is false, continue.

Substitute arbitrary substitution functions in the second <declaration-value> of branch, and return the result.

Return nothing (an empty sequence of component values).

Note: Unlike using @media/@supports/@container rules, which just ignore their contents when they’re false and let the cascade determine what values otherwise apply, declarations with if() do not roll back the cascade if the conditions are false; any fallback values must be provided inline. However, see the revert-rule CSS-wide keyword.





