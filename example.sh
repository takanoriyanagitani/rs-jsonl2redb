#!/bin/bash

set -u


oredb='example.redb'
tabname='kvpairs'

wsm='./jsonl2redb.wasm'
hostd="${PWD}"
guestd='/guest.d'
redb4guest="${guestd}/${oredb}"

run_wasi(){
  wasmtime \
    run \
    --dir "${hostd}::${guestd}" \
    --env ENV_TABLE_NAME="${tabname}" \
    --env ENV_OUTPUT_REDB_PATH="${redb4guest}" \
    "${wsm}"
}

input1(){
  echo '["helo", "wrld"]'
  echo '["hello", "world"]'
}

input1 | run_wasi

ls -l "${oredb}"
