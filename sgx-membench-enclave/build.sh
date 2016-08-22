#!/bin/bash

set -e
set -x

cargo build-enclave -H 0x8000000 -S 0x20000

pushd target/debug

openssl genrsa -3 3072 > private.pem

sgxs-sign --key private.pem -d sgx_membench_enclave.sgxs sgx_membench_enclave.sig

popd

