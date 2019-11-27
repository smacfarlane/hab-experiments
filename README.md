# Habitat Experiments

## Hab dev shell
An experiment to provide a local develement experience based on Habitat packages 

#### Running the dev shell
In its current form, the dev shell takes a list of Habitat packages parameters. There is currently no protection against having multiple versions of the same package in the environment, like you would get from `plan-build`. 

Based on that input, it will then set the same environment variables that plan-build does, hopefully in the same order!, and spawn a Bash shell with your environment configured like it is during a `hab pkg build`

Example:
> hab-dev-shell core/rust core/protobuf core/glibc

#### Dragons
Running this via `cargo run` causes interesting behavior, especially if you're trying to build Rust packages.
This is because `cargo run` will add some environment variables, such as `LD_LIBRARY_PATH`, to the environment it creates.

It's recommended you run `cargo build` or `cargo build --release`, and then invoke `target/{debug,release}/hab-dev-shell`

![recording](images/dev-shell-example.gif)


