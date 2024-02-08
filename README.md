# Traveling Salesman Genetic Algorithm (Rust)

The Traveling Salesman Genetic Algorithm project is designed to delve into the realm of genetic algorithms within the context of solving the classic Traveling Salesman Problem (TSP). This project specifically utilizes Rust programming language to implement the genetic algorithm approach.

## Project Description

The project aims to tackle the Traveling Salesman Problem, a well-known NP-hard problem in combinatorial optimization. The problem involves finding the shortest possible route that visits every city exactly once and returns to the origin city. By employing a genetic algorithm, this project seeks to find an approximate solution to this computationally challenging problem.

## Features

- **Graph Read from File**: The project is capable of reading graph data from a file, allowing for flexibility in input formats and facilitating experimentation with different problem instances.

- **Permutation-based Representation**: Utilizes permutation-based representation for candidate solutions, where each solution represents a permutation of city indices, ensuring that every city is visited exactly once.

- **Implemented Order Crossover for Reproduction**: Implements order crossover as a genetic operator for reproduction, facilitating the creation of new offspring by combining the genetic material of parent solutions.

- **Rust Parallelism**: Leverages Rust's built-in support for parallelism to exploit multi-core processors and accelerate the execution of genetic algorithm operations, such as fitness evaluation and reproduction.


<!-- ## Table of Contents -->
<!-- - [Installation](#installation) -->
<!-- - [Usage](#usage) -->
<!-- - [Features](#features) -->
<!-- - [Contributing](#contributing) -->
<!-- - [License](#license) -->

