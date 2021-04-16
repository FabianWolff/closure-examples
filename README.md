# Differences in syntax

This is a list of differences wrt. the syntax proposed in our paper.

- Any closure with specifications must be wrapped in a `closure!(...)` macro call. Within the parentheses, zero or more Rust attributes (e.g. `#[requires(...)]`) are expected, followed by the closure itself.
- Closures in `closure!(...)` macros must be annotated with their argument types and return type. This is optional in normal Rust code.
- Any captured variable used within closure specifications must be declared using a `#[view(varname: vartype)]` annotation, where:
  - `varname` is the name of the variable; and
  - `vartype` is the Rust type of the variable.
- Within specifications, views are referred to using `*views.varname`.
- Specification entailments have a syntax of the form: `f |= |args...| [ assertions... ]` (or `f |=! ...`), where:
  - `f` is the closure instance;
  - `args...` is a list of argument variables with type annotations; and
  - `assertions...` is a list of assertions in the form `requires(...)` or `ensures(...)`.
- Call description have a syntax of the form: `f ~> |args...| { prestate } { poststate }`, where:
  - `f` is the closure instance;
  - `args...` is a list of argument variables with type annotations;
  - `prestate` is an assertion that must hold before the call; and
  - `poststate` is an assertion that must hold after the call.
- All arguments in a call description are assumed to be universally quantified (expressed with the colon syntax in the paper); arguments can be constrained to be equal to other variables or constants with appropriate assertions in the prestate.
- Call descriptions in the form `f ~> ...` express that `f` itself is universally quantified (colon syntax in the paper), whereas `f ~>! ...` indicates it is not.
- `outer(...)` is not used in the examples, as it is not necessary to clarify which state we refer to in call descriptions and specification entailments in our examples.
- `cl_result` is available in the poststate of call descriptions and the postconditions of specification entailments to refer to the value returned by the closure, whereas `result` refers to the value returned by the higher-order function itself.
