# math-engine - Writing a modular math solver to make collaborative, easy to understand solutions for math problems

## Goal of the project
  - make a debugger for math
  - make template solutions for most math problems
  - make collaborative math solving for schools/university
  - make a sandbox where math is easy to learn, where there is not fear to tryout/break stuff
  - make practicing math easy/trivial


## Todos
- [ ] write basic parser for latex-math
    - [x] basic parsing
    - [ ] make it async for nested math parsing
    - [ ] write error handling proprely
    - [ ] make pretty error handling arrows showing the error, make suggestions how to fix it
    - [ ] make ambigous statements correction suggestions

    # Some ideas for fixing ambiguous statements
        - for complex numbers if Complex imported as i then every i is interpreted as complex, otherwise i is just a variable, this would also fix the problem for electro-technicians because they use j for complex numbers, so they would just import it as j
        - strip some latex syntax for example the invisible multiplication operator
            - for example for the input abc there should be a error and a suggestion for the programmer to use the normal syntax a*b*c, so there is no ambiguity between abc(x) and a*b*c*(x)
            - a*b*c would be internally converted to abc tex output in polynomials
            - also units as cm km would be possible without ambiguity
            - because cm would previously be interpreted as c invmul m

            - function syntax should be just sin(x) and should internally be converted to \\mathrm{sin}(x) syntax
        - make definition, constants, formula, identity and pattern prefixes,
            - for example:
                - def f(x) = x^2, the mathlanguage specific prefixes should be optionally shown or hidden in the converted tex output
        - or make prefix for equal:
            - versus equal - vereq/versus
            - check equal - checeq/check
            - def equal - defeq/def
            - formula equal - formeq/form
            - identity equal - identeq/iden
        - the ambiguous statements correction suggestions should be there for helping the user to fix the syntax errors he assumed that are just like in latex for example:
            - if f(x) = x^2 is parsed it would be ambiguous because we don't know how to interpret it
            - therefore, there should be a suggestion to add a prefix to the statement
            - so if there is an eqstatement without prefix, there should be an error thrown from the parser
        - maybe make the versus equal the default (this idea might be not the right solution because there could be accidental errors, maybe add warnings to add an equal prefix)
    - [ ] make shorthand forms of latex tokens:
        - for example \lvert and \rvert should be writable as l| and r|, both forms would be acceptable and the token should be the same for both, internally it would be resolved to the real latex
        - for Greek letters, the alternative without slash should be acceptable:
            - Alpha or \Alpha
            - Pi or \Pi ...
    - why latex as input syntax:
        - it should be easy for mathematicians to put their work or already written papers into the parser and to do just some adjustments to have a nice template or proof of their work in digital proven form
        - a lot of people are already familiar with latex, so the learning curve should not be that high

    - [ ] add comments with // and /* syntax. It would internally be converted to %
        - % and \% will be used for percent, internally every % would be converted to \%
        - % on beginning of lines would give a warning and a suggestion to use the // or /* syntax
        - for now every \text{}/text{} statement will be ignored, maybe could be used as print statement    

    - all mathlanguage text should be convertable to legit latex 

- [ ] write base algebraic math system
    - [x] implement basic math types
    - [ ] implement basic math operations for all basic math types


- [ ] write base calculus math system(derivatives, integrals, etc)
- [ ] write base linear algebra math system(matrix, vector, etc)
- [ ] write base logic math system(logic operators, truth tables, etc)
- [ ] write base discrete math system(mod, prime numbers)
- [ ] write step by step solutions/step tracking
    - [ ] implement step tracking for all math types/operations
    - [ ] implement math tracking/coloring to color some parts of math expressions to track down where the math/numbers came from

- [ ] implement formula and identity conversions
    - [ ] implement pattern matching for all math types
    - [ ] implement contexts to use formulas and identities from context
    - [ ] implement math functions

- [ ] implement math problems and solutions
    - [ ] implement pattern matching for all math types to match problems
        - [ ] implement random generations of specific patterns
            - [ ] generate random math problems
        - [ ] implement module system to cascade math solutions
            - [ ] implement module recomendation for a specific math pattern
            - a module is similar to functions in programming, there are arguments and returns, the arguments have to be of some type of pattern also returns are some type of pattern
            - adapter modules may be needed or for every pattern there may be a module to solve a problem
            - solutions can hold modules or sub-solutions, plots...
    - [ ] write solution templates for text math problems
    - [ ] add socalled "Bemerkung" that the proffesor often mentioned in the class
        - for example if there is something special about a statement there should be an notification about that/ indication about that

- [ ] implement brute force math form conversion, and generate a map from it
    - [ ] save a map to a file
    - [ ] load a map from a file
    - [ ] try generating maps for popular/ofter occurning pattrens to load them as needed to reduce computation time
    - [ ] test for speed(if to slow train a ml model)

- [ ] implement binary search for errors for math form conversion arrays

- after all is done maybe do:
    - [ ] think of a better name than math-engine for the project
    - [ ] write base probability/statistics math system
    - [ ] write base geometry/trigonometry math system

## Some ideas
- [ ] use machine learning to train a model for form conversion(maybe faster)
- [ ] use machine learning to train a model to convert images to latex-math
- [ ] use machine learning to train a model to extract math from text math problems
- [ ] write something like https://oort.rs/ but instead solving it with rust, doing it with math
- [ ] implement a plotting engine to plot math functions and other stuff
    - [ ] make user draw plots by hand, and match them with the real math function

## Components of the project
    - math-engine   - the solver written in rust
    - math-web      - a web frontend written in leptos
    - maybe? - math-plotter  - a ploting engine to plot math functions written in bevy/threejs/wgpu
