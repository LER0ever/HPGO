#!/bin/bash
# Run from project root
gcloud builds submit . --config=./scripts/google-cloud-build/cloudbuild.yaml --substitutions=_NOW="$(date +%y%m%d_%H%M%S)"