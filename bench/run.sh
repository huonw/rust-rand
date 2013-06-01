#!/bin/bash

cd bench
for f in *.rs; do
    if [[ $f = run.rs ]]; then continue; fi
    echo '*** '$f' ***'
    echo '** Rust **'
	rustc --opt-level=3 --passes='strip-dead-prototypes targetlibinfo basicaa scev-aa early-cse scalarrepl globalopt ipsccp deadargelim instcombine simplifycfg prune-eh mergefunc inline functionattrs argpromotion scalarrepl early-cse simplify-libcalls jump-threading correlated-propagation simplifycfg instcombine tailcallelim simplifycfg reassociate loop-rotate licm loop-unswitch instcombine indvars loop-idiom loop-deletion loop-vectorize loop-unroll gvn memcpyopt sccp instcombine jump-threading correlated-propagation dse adce simplifycfg instsimplify globaldce constmerge' -L .. $f
	time ${f%.rs}

    if [[ -f ext/$f.c ]]; then
        echo '** C **'
        clang -O3 ext/$f.c -o ext/${f%.rs}
        time ext/${f%.rs}
    fi
    echo
done
