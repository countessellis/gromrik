# gromrik

Gromrik is a displeased Dwarf who will grumpily tell you why you're wrong and not worth his time. He might give helpful ideas, if you don't mind him insulting you.

There is also a second character, Lyranis, a dreamy elven illusionist. She might give you helpful inspiration, if you can follow the threads.


## Requirements

* An Ollama compatable API to hit. Confirmed to work with Ollama itself and Lemonade
* Appropriate models set in the config and existing on the LLM server. Default and recommended is qwen2.5:7b-instruct-q4_K_M.


## Reference

[Licensing Information](LICENSING.md)


## Usage

Usage: gromrik [(OPTIONS)]

  A Displeased Dwarf Chat Bot

Options:

  --config <CONFIG FILE PATH>          Path to the config file (default: config/gromrik.cfg)
  --persona <PERSONA>                  Persona to interact with (default: gromrik, choices: gromrik, lyranis, commoner
  --llm-server-url <LLM SERVER URL>    URL for LLM chat endpoint, must be compatible with Ollama's /api/chat (default: http://localhost:11434/api/chat)
  --model <LLM MODEL NAME>             LLM model to use for interacting with the persona, must be already loaded into the server (default: qwen2.5:7b-instruct-q4_K_M)
  --history-file <HISTORY FILE PATH>   Path to the default file to use for saving and loading history (default: history/history.json)
  --mode <MODE>                        Mode to run in, (default: gui, choices: cli, tui, gui, web, help
  --cli                                Run in command line mode
  --tui                                Run in TUI mode
  --gui                                Run in GUI mode
  --web                                Run in web server mode
  --help                               Display this help message

©2026 Bethany Davis <ellis@despoena.com>.


## Config File and Settings

The config file by default is config/gromrik.cfg irelative to the run location. You can specify a different file with:

    --config <file path>

The config file should look something like:


    mode: cli
    llm_server_url: http://localhost:11434/api/chat
    model: qwen2.5:7b-instruct-q4_K_M
    history_file: history/history.json
    persona: gromrik

### Mode

Mode can be:

    cli: Command line output
    tui: Terminal user interface (psuedo GUI)
    gui: Graphical user interface (graphical application, default)
    web: Web server (for interacting through a browser, not yet implemented)

Mode can be set from command line with these options:

    --mode <mode>
    --cli
    --tui
    --gui
    --web

Mode can also be defaulted to a specific one by using the binaries:

    gromrik
    gromrik-cli
    gromrik-tui
    gromrik-gui
    gromrik-web

Note that config file and command line options change the mode as normal, using the other binaries only changes the default if not set with one or the other.

### LLM Server URL

The server to hit for LLM output can be set with llm_server_url. This should be an ollama compatible chat API. Default:

    http://localhost:11434/api/chat

It can also be set with:

    --llm-server-url <URL>

### Model

This is the LLM that is being used. It must be already loaded into the server. For ollama, add the default, qwen2.5:7b-instruct-q4_K_M, like this:

    ollama pull qwen2.5:7b-instruct-q4_K_M

The default model works very well, but you can change it either in the config file or from the command line:

    --model <model>

### History File

The path used for saving and loading conversation history (supported in TUI and GUI).

Default is relative to where the program is ran:

    history/history.json

This can be set in the config file with history_file, or from the command line:

    --history-file <file path>

### Persona

The compiled in persona to interact with.

Options:

    gromrik  | Interact with Gromriki, the default.
    lyranis  | Interact with Lyranis.
    commoner | Interact with a random commoner.

Thsi can be set in the config file with persona, or from the command line:

    --persona <persona>


## Building Gromrik

### Init

    curl https://sh.rustup.rs -sSf | sh                        | Install rustup.
    rustup toolchain install nightly && rustup default nightly | Install the tool chains.
    cargo install cross                                        | Install Cross (cross compiling only).
    rustup update                                              | Update (do this regularly).

### Debug

Just build it:

    cargo build --bins | Build it.
    cargo run          | Run it.

Build with features used to make smaller binaries:

    make all RELEASE_FLAG=""

For all build targets (replace "all" above with the listed target):

    make help

### Release:

Compiling:

    cargo build --bins --release | Build it.
    cargo run --release          | Run it.

Build with features used to make smaller binaries:

    make all

For all build targets (replace "all" above with the listed target):

    make help
