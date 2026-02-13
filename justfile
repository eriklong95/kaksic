build:
    cargo b -r

run:
    just build
    cd target/release && ./runner

watch:
    @tail -f target/release/engine.log

test arg="":
    cargo nextest run -r {{ arg }}

run-raw:
    cargo run --release --bin $(awk '/\[package\]/{flag=1;next}/\[/{flag=0}flag && /^name *=/ {gsub(/"/,""); print $3}' Cargo.toml )

baseline benchname="":
    @if [ -n "{{ benchname }}" ]; then \
        echo "\033[1mSetting baseline for: {{ benchname }}\033[0m"; \
        cargo bench --bench "{{ benchname }}" -- --save-baseline baseline; \
    else \
        for f in benches/*.rs; do \
            name=$(basename "$f" .rs); \
            echo "\033[1mSetting baseline for: $name\033[0m"; \
            cargo bench --bench "$name" -- --save-baseline baseline; \
        done; \
    fi

bench benchname="":
    @if [ -n "{{ benchname }}" ]; then \
    echo "\033[1mBenchmarking: {{ benchname }}\033[0m"; \
        cargo bench --bench "{{ benchname }}" -- --baseline baseline; \
    else \
        for f in benches/*.rs; do \
            name=$(basename "$f" .rs); \
            echo "\033[1mBenchmarking: $name\033[0m"; \
            cargo bench --bench "$name" -- --baseline baseline; \
        done; \
    fi
