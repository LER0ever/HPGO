# HPGO
Hybrid Parallelism Global Orchestration

(_Project Name TBD_)

## Features

## Get Started

### Install

#### From Rust Cargo, as a Rust crate
```bash
cargo install HPGO
```
`TODO: publish to Cargo crates after open source`

#### From Python PyPI, as a Python3 package
```bash
pip3 install HPGO
```

### Build from source
```bash
rustup default nightly
cargo build --release
# library produced under targets/release/libHPGO.so
```

### Use

```python
# Import HPGO Python API from HPGO.so
import HPGO
# Construct the Conductor object
c = HPGO.conductor_from_torch_graph_and_seps("./profiles/xlnet/graph.txt", 64, 512, [8, 16])
c.py_orchestrate()
```

## License

This project is open sourced under the terms of BSD-3-Clause, details of which can be found in the [`LICENSE`](LICENSE) file

The project contains source code from [PipeDream](https://github.com/msr-fiddle/pipedream), a Microsoft Research project, which is licensed under the MIT License. This includes a Rust file `src/input/torch_graph_py.rs`, several profiling data under the `profiles` directory, and the `contrib` directory.

This project also contains profiling data provided by the [AlibabaPAI/DAPPLE](https://github.com/AlibabaPAI/DAPPLE) project. 