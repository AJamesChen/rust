#!/bin/sh
set -eu

program=$1
shift

if [ "${program##*/}" = "panic-index" ]; then
    exec probe-rs run \
        --chip STM32F303VC \
        --log-format=oneline \
        "$program" "$@"
fi

exec probe-rs run \
    --chip STM32F303VC \
    --connect-under-reset \
    --log-format=oneline \
    "$program" "$@"
