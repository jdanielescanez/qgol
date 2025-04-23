
# Probabilistic Game of Life

This repo contains a simulator for a probabilistic Game of Life version. Most of the code is in Rust, but there is also a script that converts the JSON output of the Rust simulator into GIFs.

## Quick Start

### Simulation execution

With [Rust and Cargo installed](https://www.rust-lang.org/tools/install) you can execute the simulation with a command like the following example. 


```sh
#              INITIAL_TURN_FILE      OUTPUT FILE        NUMBER_OF_TURNS
cargo run -- examples/720x1080.init examples/720x1080_30.out 30
```

It's also possible to use `--help` to get information about the different options.

```sh
cargo run -- --help
```

### Gif generation

A python script is also provided to convert the simulation output into a gif. This allows for a quick visualization of the results of the simulation.

With [uv installed](https://docs.astral.sh/uv/getting-started/installation/) you can run a command like the following example to generate a gif. 

```sh
#                  SIMULATION_OUTPUT       OUTPUT_GIF_FILE
uv run main.py examples/720x1080_30.out examples/720x1080_30.gif
```

> [!NOTE]  
> You can just run it with any virtual environment as long as you install the requirements in the `pyproject.toml`


## Input format

The input format consists on space separated floating point numbers. Check out any of the examples.

## Output format

The output format is a JSON array in which each element is a generation of the game of life.
Each generation is represented as a matrix of floating point numbers.

## Example GIF

![Example GIF of the simulation running](/examples/100x100_50.gif)
