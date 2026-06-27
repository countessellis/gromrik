# gromrik

Gromrik is a displeased Dwarf who will grumpily tell you why you're wrong and not worth his time. He might give helpful ideas, if you don't mind him insulting you.

## Requirements

* An Ollama compatable API to hit. Confirmed to work with Ollama itself and Lemonade
* Appropriate models set in the config and existing on the LLM server. Default and recommended is qwen2.5:7b.

## Config File and Settings

The config file by default is config/gromrik.cfg irelative to the run location. You can specify a different file with:

    --config <file path>

The config file should look something like:


    mode: cli
    llm_server_url: http://localhost:11434/api/chat
    model: qwen2.5:7b

### Mode

Mode can be:

    cli: Command line output (default)
    tui: Terminal user interface (psuedo GUI, not yet implemented)
    gui: Graphical user interface (graphical application, not yet implemented)
    web: Web server (for interacting through a browser, not yet implemented)

Mode can be set from command line with these options:

    --mode <mode>
    --cli
    --tui
    --gui
    --web

### LLM Server URL

The server to hit for LLM output can be set with llm_server_url. This should be an ollama compatible chat API. Default:

    http://localhost:11434/api/chat

It can also be set with:

    --llm-server-url <URL>

### Model

This is the LLM that is being used. It must be already loaded into the server. For ollama, add the default, qwen2.5:7b, like this:

    ollama pull qwen2.5:7b

The default model works very well, but you can change it either in the config file or from the command line:

    --model <model>

## Building Gromrik

### Init

    curl https://sh.rustup.rs -sSf | sh                        | Install rustup.
    rustup toolchain install nightly && rustup default nightly | Install the tool chains.
    cargo install cross                                        | Install Cross (cross compiling only).
    rustup update                                              | Update (do this regularly).

### Debug

Just build it:

    cargo build | Build it.
    cargo run   | Run it.

### Release:

Compiling local:

    cargo build --release | Build it.
    cargo run --release   | Run it.

If you want it portable but aren't cross compiling, use the appropriate for ARM and x86_64 (RISCV :

    cargo build --release --target=aarch64-unknown-linux-musl  | ARM
    cargo build --release --target=x86_64-unknown-linux-musl   | x86_64

Cross compiling:

    cross build --release --target=aarch64-unknown-linux-musl  | ARM
    cross build --release --target=x86_64-unknown-linux-musl   | x86_64
    cross build --release --target=riscv64gc-unknown-linux-gnu | RISC-V

