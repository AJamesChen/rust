#!/bin/sh
set -eu

program=$1
shift

exec probe-rs run \
    --chip STM32F303VC \
    --connect-under-reset \
    --log-format=oneline \
    "$program" "$@"
