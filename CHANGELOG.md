# CHANGELOG

## 1.1.4

* Added chooser for location and persona.
* Standardized dimensions of persona full images (the three added in 1.1.3 were narrower).
* Miscellaneous bug fixes and tweaks.

## 1.1.3

* Changed text splash from Gromrik to all the Ghosts.
* Added text splash for TUI before displaying the UI.
* Added image splash for GUI with Gromrik's Ghosts shown before showing the UI.
* Added auto-focus for input in GUI.
* Added setting of scene, for customizing the conversation, using /scene or proving it in a bundle, in the config file, or from commandline.
* Added persistence when switching personas then returning the previous.
* Added Melisande persona (satyr minstrella) as a bundle.
* Added Thorvag Ulgun persona (troll scholar) as a bundle.
* Added Sir Alaric de Winton (human nobleman) as a bundle.
* Miscellaneous bug fixes and tweaks.

## 1.1.2

* Added tab completion for / commands.
* Added history of inputs separate from chat history, and ability to use arrows for previous and next.
* Page Up and Page Down now scroll the chat history.
* Removed normal window controls (close and maximize) and replaced it with a bronze bevel.
* Added Bramlink Brightheart persona (gnome palidin) as a bundle.
* Added Zahirik persona (goblin merchant) as a bundle.
* Miscellaneous bug fixes and tweaks.

## 1.1.1

* Added support for persona bundles, bundling during compile, and auto-include of bundles bundled at compile.
* Added Velissa persona (faerie assassin) as a bundle.
* Miscellaneous bug fixes and tweaks.

## 1.1.0

* Fleshed out / commands beyond /exit and /clear.
* Added canned dismissal for timeout and on exit.
* Added Lyranis persona (elven illusionist).
* Added generic commoner persona generated at run time, as a fallback if no persona is compiled in.
* Added Makefile for feature gating and github release.
* Miscellaneous bug fixes and tweaks.

## 1.0.0

* Initial minimum viable dwarf, including:
  - Gromrik persona (surly dwarf).
  - CLI, TUI, and GUI interfaces (web version stubbed in but not implemented).
  - /exit and /clear commands in TUI and GUI.
  - GUI mode as default.
