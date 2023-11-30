# math-engine - Writing a modular math solver to make collaborative, easy to understand solutions for math problems

## Goal of the project
  - make a debugger for math
  - make template solutions for most math problems
  - make collaborative math solving for schools/univerity
  - make a sandbox where math is easy to learn, where there is not fear to tryout/break stuff
  - make practicing math easy/trivial

## Todos
- [ ] write basic parser for latex-math
    - [x] basic parsing
    - [ ] make it async for nested math parsing
    - [ ] write error handling proprely

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
    - maybe? - math-plotter  - a ploting engine to plot math functions written in bevy/threejs
