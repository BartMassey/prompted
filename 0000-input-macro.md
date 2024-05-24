- Feature Name: Input and Prompting Macros
- Start Date: 2017-12-05
- RFC PR: (leave this empty)
- Rust Issue: (leave this empty)

# Summary
[summary]: #summary

Standard macros gets a macro for reading a line from
standard input with optional prompt; also macros and
functions for flushing non-terminated lines to standard
output and standard error.

# Motivation
[motivation]: #motivation

In most programming languages, it is straightforward to
command-line programs to prompt the user for a line of input
and get back a string containing what was typed.

In current Rust, prompting for a line of text involves
importing and understanding a chunk of the standard IO
library and doing a complicated flush of the prompt. Thus,
simple Rust programs needing this functionality tend either
to behave in inferior ways (the Guess A Number example in
the standard documentation does not prompt, for example) or
to contain sometimes-dubious code copied from the Internet
with little understanding of its function.

A number of solutions to this problem have been discussed in
the Rust community, notably in Github issue
[#23818](https://github.com/rust-lang/rust/issues/23818).
The Rust library authors have resisted changing the flushing
behavior of the standard libraries. However, they are
amenable to other solutions. I believe the proposed macro
will meet most of the needs.

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

The package `std::io::prompted` is part of the default Rust
environment. The package provides three macros for
interacting with the user at the command line: `input!()`,
`prompt!()` and `eprompt!()`. The package also
provides three convenience functions for user interaction:
`flush()`, `eflush()` and `read_line()`.

The `input!()` macro accepts optional `format!` arguments
describing a prompt to be printed. If given, the prompt is
printed to `stdout()`: it is not newline terminated
unless the format string requests it. `stdout()` is then
flushed to make the prompt visible using `flush()`,
and a line of user input is read from `stdin()` using
`read_line()`. This line has its line terminator removed
and is then returned to the caller as a `String`.

An example of the use of `input!()` is inspired by the
Guessing Game example from the Rust book.

    let guess = input!("Please input your guess (1-{}): ", n);
    let guess: u32 = match guess.trim().parse() {
        ...

This code behaves as expected. The first line prompts the
user and gets a `String` representing the user's guess. The
second line turns the guess string into a number and then
proceeds based on the success or failure of this conversion.

The `prompt!()` macro takes a `format!` argument and
displays it to the user on standard output. It then flushes
standard output to guarantee visibility.

`prompt!()` can be used to display a prompt in cases where
more complicated user input processing is to be done by the
program. Another example of the use of `prompt!()` shows the
standard practice of overwriting the output line using a
carriage return to display the status of ongoing
long-running operations:

    let phases = &[
        (1, "pre", &f_pre as &'static Fn ()),
        (2, "op", &f_op),
        (3, "post", &f_post)];
    let mut last_len = 0;
    for &(n, name, f) in phases {
        for _ in 0..last_len {
            print!(" ")
        }
        let message = format!("{}: {}", n, name);
        prompt!("\r{}", message);
        f();
        last_len = message.len();
        print!("\r")
    }
    println!()

Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.

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
[alternatives]: #alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

# Unresolved questions
[unresolved]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?
