# HPGO
Hybrid Parallelism Global Orchestration

## Build
#### Prerequisite
- a C++17 compiler (GCC >= 7, Clang >= 5, MSVC >= 2017)
- CMake > 2.8
- Boost::System (tested on 1.71)
- Boost::Python
- Python-Dev (3)
- GraphViz
- Catch2 (bundled)

#### Compile
```bash
mkdir build
cd build
cmake ..
make -j
```

## Test
```bash
# run all the unit tests
./test_main -s
```

## Run

