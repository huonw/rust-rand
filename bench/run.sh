#!/bin/bash

cd bench
for f in *.rs; do
    if [[ $f = run.rs ]]; then continue; fi
    echo '*** '$f' ***'
    echo '** Rust **'
	rustc --opt-level=3 -L .. $f
	time ${f%.rs}

    if [[ -f ext/$f.c ]]; then
        echo '** C **'
        gcc -O3 ext/$f.c -o ext/${f%.rs}
        time ext/${f%.rs}
    fi
    echo
done
