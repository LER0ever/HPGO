# HPGO 
Hybrid Parallelism Global Orchestration

## Features

## How To

### Build
```bash
cargo build --release
```

### Use
- build using cargo
- rename the built binary to `HPGO.so`
- copy `HPGO.so` to your working directory

```python
# Import HPGO Python API from HPGO.so
import HPGO
```

## Project Structure
Output of `tree -L 3 -I "target|*.txt|*.dot|*.log|*.png"`:

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── contrib
│   ├── graph
│   │   ├── __init__.py
│   │   ├── __pycache__
│   │   ├── graph.py
│   │   ├── test.py
│   │   └── test_graphs
│   ├── torch_graph.py
│   └── utils
│       ├── __init__.py
│       └── prepare.py
├── examples
│   ├── amoebanet_speedup.rs
│   ├── bert_speedup.rs
│   ├── xlnet_scalability.rs
│   └── xlnet_speedup.rs
├── logs
├── profiles
│   ├── alexnet
│   ├── amb_issue_minimal
│   ├── amoebanet
│   ├── bert_48
│   ├── bert_large
│   ├── densenet121
│   ├── ggv
│   ├── gnmt
│   ├── gnmt_large
│   ├── inception_v3
│   ├── nasnetalarge
│   ├── nasnetamobile
│   ├── resnet101
│   ├── resnet18
│   ├── resnet50
│   ├── resnext101
│   ├── resnext50
│   ├── squeezenet1_0
│   ├── vgg16
│   ├── vgg19
│   └── xlnet
├── scripts
│   ├── FPL
│   │   ├── Arrangement.py
│   │   └── FPL.py
│   ├── google-cloud-build
│   │   ├── Dockerfile
│   │   ├── build-release.sh
│   │   ├── build-test.sh
│   │   ├── cloudbuild-example.yaml
│   │   ├── cloudbuild-release.yaml
│   │   └── cloudbuild-test.yaml
│   └── graph-visualizer
│       └── visualize.py
├── src
│   ├── analysis
│   │   ├── cc_overlap.rs
│   │   ├── gpu_memory.rs
│   │   └── mod.rs
│   ├── api
│   │   ├── capi.rs
│   │   ├── mod.rs
│   │   └── pylib.rs
│   ├── conductor
│   │   └── mod.rs
│   ├── environment
│   │   ├── device.rs
│   │   ├── ethernet.rs
│   │   ├── mod.rs
│   │   ├── network.rs
│   │   └── nvlink.rs
│   ├── input
│   │   ├── mod.rs
│   │   ├── tensorflow_timeline.rs
│   │   ├── torch_graph.rs
│   │   └── torch_graph_py.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── model
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── model_perf.rs
│   ├── orchestration
│   │   ├── mod.rs
│   │   ├── orchestrate_async.rs
│   │   ├── orchestrate_hierarchical.rs
│   │   └── orchestrate_sync.rs
│   └── parallelism
│       ├── data_parallel.rs
│       ├── gpipe.rs
│       ├── gradient_accumulation.rs
│       ├── mod.rs
│       ├── split_concat.rs
│       └── sync_pipeline.rs
└── tests
    ├── data_parallel_test.rs
    ├── device_test.rs
    ├── ga_test.rs
    ├── gpu_memory_test.rs
    ├── orchestrate_hierarchical_test.rs
    ├── orchestrate_test.rs
    ├── p3_test.rs
    ├── speedup_test.rs
    ├── split_concat_test.rs
    └── torch_graph_test.rs

43 directories, 63 files
```

## License
