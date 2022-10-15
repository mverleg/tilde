
# Tilde parsing

## Multiple formats

Tilde has 16 characters, so that two of them fit into a byte, in order to make maximum use of bytes. These can be stored in two ways, which map one-to-one:

* Actual bytes. There really is 1 byte per token and size is minimized, but it is unreadable in most programs.

* A selection of 256 unicode (utf8) code points. The actual storage size is larger than 1 byte, but it is readable to humans.

In unicode mode, any other code points, or non-unicode bytes, cause the whole input to be rejected.

In addition, there is a textual representation. This is just for display, and not currently parseable. It is a superset of the tokens, that maps more closely to the abstract syntax tree than the tokens.

## Grouping

Tokens come in three versions:

* Variable length openers. The largest group, here a single token has an effect, but it can optionally be followed by a modifier to change the meaning.
* Fixed length openers. These tokens must be followed by one other token, which can be any token (not just modifiers).
* Modifiers. These cannot appear by themselves, but change the meaning of the previous opener (how it changes depends on the opener).

#TODO @mverleg: what about separators (between e.g. 3 and 5), and end-of-block tokens?

## Context-dependent

Symbols have different meanings in different contexts. This also affects parsing, since grouping of bytes into operations is different.

* General code
* Function definitions, which can only appear at the start
* Numbers
* Strings
* Regular expressions

More contexts might be added.

Note that the meaning of symbols also depends on the type of operands, but types are not known at time of parsing.

