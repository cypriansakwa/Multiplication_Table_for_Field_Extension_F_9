## Overview

This program computes the multiplication table for the finite field extension $\mathbb{F}\_{3^2}$. The field $\mathbb{F}\_{3^2}$ is constructed using the relation $x^2 = 2$ over $\mathbb{F}_3$. Elements in this field are of the form $a + bx$, where $a, b \in$ {0, 1, 2}, and $x$ is the indeterminate.

## Field Elements in $\mathbb{F}\_{3^2}$

The nine elements of the field $\mathbb{F}_{3^2}$ are:
\[
\{ 0, 1, 2, x, 1 + x, 2 + x, 2x, 1 + 2x, 2 + 2x \}
\]
These elements are used to construct the multiplication table.

## Program Description

The program defines a struct `F3x2` to represent elements in $\mathbb{F}\_{3^2}$ with two fields:
  - `a`: Coefficient of the constant term.
  - `b`: Coefficient of the $x$ term.

Multiplication of two elements is implemented in the `multiply` function, taking into account the relation $x^2 = 2$ in the field.


\section*{Usage}

To run the program, execute the following command:
\begin{verbatim}
cargo run
\end{verbatim}

The program will display the elements of the field and output the multiplication table in the console.
## Multiplication Table

The multiplication table for $\mathbb{F}_{3^2}$ is computed and printed, with each element in the rows and columns being multiplied according to the rules of field multiplication.
