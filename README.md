# molecular-profiler-rs

`molecular-profiler-rs` is a Rust tool for profiling molecular formula datasets before using them in machine-learning workflows.

This project is intended to support careful dataset inspection before training models such as MS/MS-to-atom classifiers.

## Dataset sources

By default, this tool uses the `annotated_ms2` dataset exposed by [`mascot-rs`](https://github.com/earth-metabolome-initiative/mascot-rs).

Local datasets are also supported.

## Usage

### Profile one element

The element symbol is normalized automatically, so these are equivalent:

```bash
cargo run --release -- f
cargo run --release -- F
```

### Profile all observed elements

Profile every observed element in the default dataset:

```bash
cargo run --release -- all
```

This generates one report directory per observed element.

### Profile a local MGF file

Profile all observed elements in a local MGF file:

```bash
cargo run --release -- all path/to/local_file.mgf
```

# Reports

## Generated reports

After running the profiler, open [`REPORTS.md`](REPORTS.md) for links to generated dataset reports.

## Report output

The generated `README.md` inside each report directory is the main human-readable report. It links to the CSV tables and embeds the generated SVG figures.

## Contributing

Contributions are welcome.

## License

MIT
