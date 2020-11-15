# Tag Simulator
#### An agent-based simulation engine for the game of tag.

## Getting Started
* **Note**: since the addition of performance benchmarking this requires nightly Rust.

To set up and run (first ensure Vulkan is installed if on Linux),
```
$ git clone https://github.com/derekam/tag-simulator.git
$ cd tag-simulator
$ cargo +nightly run -- -t
```

You may want to run with logging enabled at a higher level than the 'error' default, e.g. ```$ RUST_LOG=info RUST_BACKTRACE=1 cargo +nightly run```


## Command-Line Options
* **Speed** (-s, --speed, f64): The maximum cartesian distance a player can travel in a given move.
* **Proximity** (-p, --proximity, f64): The maximum cartesian distance a player can be from another player and tag them.
* **Height** (-h, --height, u64): The height of the playing field.
* **Width** (-w, --width, u64): The width of the playing field.
* **Num Players** (-n, --num_players, usize): The number of players in the game.
* **Directional Agent** (-d, --directional_agent, flag): Have players run towards/away from others instead of moving randomly.
* **Text Numbers** (-t, --text_numbers, flag): Show the text number id of each player (has a performance cost).

Example usage of the command line:
```
$ RUST_LOG=info RUST_BACKTRACE=1 cargo +nightly run -- -s 5.0 -p 15.0 -w 1000 -h 600 -n 50 -t -d
```

## Troubleshooting

The most likely problems are compatibility issues between Iced and your machine. Encountered issues:

* **Error: GraphicsAdapterNotFound**, OR thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /[...].../.cargo/registry/src/github.com-[...]/wgpu-native-0.4.3/src/instance.rs:474:72.

Iced requires Vulkan to run; please install it. On Ubuntu:
```
$ sudo add-apt-repository ppa:oibaf/graphics-drivers
$ sudo apt update
$ sudo apt upgrade
$ apt install libvulkan1 mesa-vulkan-drivers vulkan-utils
```

## Testing

To run, ```cargo +nightly test```.

## Performance

To run benchmarks of the simulator without the GUI, ```cargo +nightly bench```.