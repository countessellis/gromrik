# Define the binary name base
CARGO = cargo
RELEASE_FLAG = --release

# Phony targets prevent conflicts with files named 'all', 'clean', etc.
.PHONY: all full clean help
.DEFAULT_GOAL := help

help:

	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Default task: Builds everything with all features
all: full gromrik lyranis commoner ## Build all targets

gromrik: gromrik-full gromrik-cli gromrik-tui gromrik-gui gromrik-web ## Build just Gromrik targets

lyranis: lyranis-full lyranis-cli lyranis-tui lyranis-gui lyranis-web ## Build just Lyranis targets

clean: ## Clean cargo target directory
	$(CARGO) clean

full: ## Build full build with all features
	$(CARGO) build $(RELEASE_FLAG) --bin chat

commoner: ## Build bare bone build with just commoner persona
	$(CARGO) build $(RELEASE_FLAG) --bin commoner --no-default-features --features cli,tui,gui,web

gromrik-full: ## Build full Gromrik build
	$(CARGO) build $(RELEASE_FLAG) --bin gromrik --no-default-features --features gromrik,cli,tui,gui,web

gromrik-cli: ## Build CLI only Gromrik build
	$(CARGO) build $(RELEASE_FLAG) --bin gromrik-cli --no-default-features --features gromrik,cli

gromrik-tui: ## Build TUI only Gromrik build
	$(CARGO) build $(RELEASE_FLAG) --bin gromrik-tui --no-default-features --features gromrik,tui

gromrik-gui: ## Build GUI only Gromrik build
	$(CARGO) build $(RELEASE_FLAG) --bin gromrik-gui --no-default-features --features gromrik,gui

gromrik-web: ## Build Web only Gromrik build
	$(CARGO) build $(RELEASE_FLAG) --bin gromrik-web --no-default-features --features gromrik,web

lyranis-full: ## Build full Lyranis build
	$(CARGO) build $(RELEASE_FLAG) --bin lyranis --no-default-features --features lyranis,cli,tui,gui,web

lyranis-cli: ## Build CLI only Lyranis build
	$(CARGO) build $(RELEASE_FLAG) --bin lyranis-cli --no-default-features --features lyranis,cli

lyranis-tui: ## Build TUI only Lyranis build
	$(CARGO) build $(RELEASE_FLAG) --bin lyranis-tui --no-default-features --features lyranis,tui

lyranis-gui: ## Build GUI only Lyranis build
	$(CARGO) build $(RELEASE_FLAG) --bin lyranis-gui --no-default-features --features lyranis,gui

lyranis-web: ## Build Web only Lyranis build
	$(CARGO) build $(RELEASE_FLAG) --bin lyranis-web --no-default-features --features lyranis,web


