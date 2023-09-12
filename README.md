# Multiplicative Persistence Calculator

## Introduction

- The Persistence of a number is the number of times we can apply a function f to it.
- The Multiplicative Persistence is the Persistence on applying the function "multiply each digit of the number together":

![Multiplcative function formula](/assets/CodeCogsEqn.gif "Multiplicative function formula")

With `f` the Multiplicative function, `n` an integer, `b` a specific base.

## The project

The program will attempt to find the smallest number for each Multiplicative Persistence, given a specific base and a maximum value to stop at. The main focus of the project will be:

- Optimisation: The program should not naively search every possible number. It should be able to work search all values of uint64 in reasonable amount of time.
- Learning: The program is a way for the author to learn writing Rust, thus it will have to contain multiple different modules at the minimum (not just a script).
