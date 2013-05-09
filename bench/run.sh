#!/bin/bash

cd bench
for f in *.rs; do
    echo '*** '$f' ***'
    echo '** Rust **'
	rustc -L .. $f -O
	time ${f%.rs}

    if [[ -f ext/$f.c ]]; then
        echo '** C **'
        gcc -O3 ext/$f.c -o ext/${f%.rs}
        time ext/${f%.rs}
    fi
    echo
done
