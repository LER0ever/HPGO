#![allow(non_snake_case)]
extern crate ndarray;
extern crate docopt;
#[macro_use]
extern crate serde;
extern crate itertools;
extern crate strsim;

extern crate HPGO;

use std::fs::File;
use docopt::Docopt;

const USAGE: &str = "
Estimate k-NN error and convergence.

Usage: fbleau <estimate> [--knn-strategy=<strategy>] [options] <train> <eval>
       fbleau (--help | --version)

Arguments:
    estimate:   nn              Nearest Neighbor. Converges only if the
                                observation space is finite.
                knn             k-NN rule. Converges for finite/continuous
                                observation spaces.
                frequentist     Frequentist estimator. Converges only if the
                                observation space is finite.
    knn-strategy: ln            k-NN with k = ln(n).
                  log10         k-NN with k = log10(n).
    train                       Training data (.csv file).
    eval                        Evaluation data (.csv file).

Options:
    --logfile=<fname>           Log estimates at each step.
    --logerrors=<fname>         Log the individual error for each test object
                                for the smallest error estimate.
    --delta=<d>                 Delta for delta covergence.
    --qstop=<q>                 Number of examples to declare
                                delta-convergence. Default is 10% of
                                training data.
    --absolute                  Use absolute convergence instead of relative
                                convergence.
    --scale                     Scale features before running k-NN
                                (only makes sense for objects of 2 or more
                                dimensions).
    --distance=<name>           Distance metric in (\"euclidean\",
                                \"levenshtein\").
    -h, --help                  Show help.
    --version                   Show the version.
";


fn main() {

}
