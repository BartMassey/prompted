- Feature Name: Line Input and Prompting Macros
- Start Date: 2024-05-23
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

This RFC proposes adding standard library API for reading a
line of text from an input source, with optional
prompting. The primary audience for the API is those who
want an ergonomic way to simply read a line; a secondary
audience is those hoping to use the provided functionality
in production work.

# Motivation
[motivation]: #motivation

A common task in CLI programs is to read a line of user
text.  This interaction is extremely common in tutorials,
demos and examples. However, it also is quite common in
production tools.

In most programming languages, simple and ergonomic support
is provided for reading a line of standard input from the
user. Support is also usually provided for printing a prompt
and allowing the user to input on the same line. Rust
currently does not provide this convenience — an important
one for teaching, for quick trials and hacks, and for
general user interaction.

## Prior Art: Python

A motivating example of good ergonomics is provided by
Python. Python provides the `input()` function, which prints
an optional prompt argument and returns the user's line of
input as a string with the line ending removed.

```text
>>> input("prompt: ")
prompt:   testing  
'  testing  '
```

## Current Rust: The "Guessing Game"

Reading a line with prompting in Rust is currently
relatively awkward, even after recent ergonomic
improvements. The "Guessing Game" example in *The Rust
Programming Language* provides a motivating example here:
the code for reading the user's guess currently looks like
this:

```rust
use std::io;
println!("Please input your guess.");
let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

* Because `stdin()` is not part of the Rust Prelude
  the module system gets introduced here. This is unlike
  `println!()`, which is provided by the Prelude.

* The prompt is printed on a line before the user input
  principally to ensure that the prompt is flushed to Rust's
  line-oriented standard output before the user input is
  read.

* The necessity of mutating a `String` to read the line
  is an efficiency win, but that is irrelevant in this case.

* This API introduces a footgun in that the user might
  expect to be able to immediately reuse the `guess` buffer
  for subsequent input. This will fail on the next input:
  the buffer string is not cleared by `read_line()`, so the
  next input will be appended rather than freshly read.

* The line ending is preserved by `read_line()`. This is
  almost certainly an undesired behavior in this sort of
  situation. Because there is no standard method in Rust for
  removing *just* a line-ending, the normal strategy is to
  use `trim_end()` or `trim()` to clear all leading and
  trailing whitespace. While a `trim()` is desirable for the
  subsequent integer `parse()` in the Guessing Game, in
  general the user may want non-line-terminating whitespace
  to be preserved.
  
* The `expect()` here is not particularly useful. "Failed to
  read line" is both quite unlikely to happen in practice
  and not informative beyond what is provided by the panic
  message, especially the IO error contained therein.

## Current Rust: An Alternate Approach

An alternate approach in current Rust is to use the
`lines()` iterator:

```rust
use std::io;
println!("Please input your guess.");
let guess = io::stdin()
    .lines()
    .next()
    .expect("Input ended without a line")
    .expect("Failed to read a line");
```

This approach solves a few of the problems:

* The line ending will be removed.

* The result `String` will be allocated by the iterator.

However, this approach introduces its own issues:

* EOF will cause a panic here rather than being treated as a
  line terminator, since the iterator will return `None`.

* The verbosity and complexity of this solution is
  non-trivial.
  
These are likely reasons that the first approach was adopted
in the Guessing Game example.

## A Better Solution: `inputln!()`

This RFC proposes providing an `inputln!()` macro available
in the Rust Prelude for reading a line from input. The
design of `inputln!()` is strongly inspired by the design of
`println!()`. The `inputln!()` macro returns a `String`
containing the user's line of input with the line ending
removed.

An optional "prompt" argument to `inputln!()`
allows formatting a prompt that will be printed without a
line ending and then flushed, so that the user can be
prompted on the line where input will be given.

`inputln!()` panics on IO errors: IO errors can rarely be
usefully handled anyhow, and not returning any makes
`inputln!()` usable without understanding the details of
Rust error handling.

With `inputln!()` the Guessing Game example looks like:

```rust
let guess = inputln!("Please input your guess: ");
```

## Functionality For The More General Case: prompting and `readln()`

In Rust `println()` prints only to standard output and
panics on errors. `eprintln()` prints only to standard error
and panics on errors. There are obviously situations where
one would like to print to arbitrary writers, and to handle
errors in some non-panicking way. For this purpose, Rust
provides `writeln!()`, which cleanly solves both of these
problems.

Similarly, this crate proposes to provide a `readln()`
function which takes a writer and returns a result.  Because
`readln()` takes a reader and prompting needs a writer which
may well be independent of the reader, prompting is split
out into separate macros `prompt!()`, `eprompt!()`, and
`write_prompt!()`. This machinery allows the convenience and
correctness of `inputln!()` while allowing for more involved
programming situations.

HERE

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.
- Discuss how this impacts the ability to read, understand, and maintain Rust code. Code is read and modified far more often than written; will the proposed feature make code easier to maintain?

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- If this is a language proposal, could this be done in a library or macro instead? Does the proposed change make Rust code easier or harder to read, understand, and maintain?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

# Future possibilities
[future-possibilities]: #future-possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
