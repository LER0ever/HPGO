# HPGO
Hybrid Parallelism Global Orchestration

[![](https://img.shields.io/badge/PyPI-HPGO%200.92-blue?logo=python&logoColor=yellow)](https://pypi.org/project/HPGO/) [![Build Status](https://dev.azure.com/rongyi-io/HPGO/_apis/build/status/LER0ever.HPGO?branchName=master)](https://dev.azure.com/rongyi-io/HPGO/_build/latest?definitionId=1&branchName=master) [![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

(_Project Name TBD_)

## Get Started

### Install

#### From Rust Cargo, as a Rust crate
```bash
cargo install HPGO
```

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
res = c.py_orchestrate()
print(res)
```

## License

This project is open sourced under the terms of BSD-3-Clause, details of which can be found in the [`LICENSE`](LICENSE) file

If you apply this library to any project and research, please cite this code:

```
@misc{HPGO,
  author       = {Yi Rong},
  title        = {HPGO, [{Hybrid Parallelism} Global Orchestration]},
  howpublished = {\url{https://github.com/LER0ever/HPGO}},
  year         = {2020}
}
```

The project contains source code from [PipeDream](https://github.com/msr-fiddle/pipedream), a Microsoft Research project licensed under the MIT License. Code taken from Pipedream includes a Rust file `src/input/torch_graph_py.rs`, several profiling data files under the `profiles` directory, and the `contrib` directory.

This project also contains profiling data provided by the [AlibabaPAI/DAPPLE](https://github.com/AlibabaPAI/DAPPLE) project. The development of the DAPPLE planner continues here.