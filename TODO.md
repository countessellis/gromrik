# TODO

## TODO for 1.1.X

* Create chooser for TUI.
* Fix duplicate scene when selecting scene before persona in GUI chooser.
* Fix issue where escape doesn't allow focus returning from chooser.

## TODO for 1.2.X+

* Map to navigate in TUI and GUI, and Web when it's implement.
  - Add displaying of map on ESC or /map.
  - Define locations.
  - Add locations to bundles (load optional in case of old bundles) with weights.
  - Allow clicking on locations in map (GUI version).
  - Build TUI map, with arrow navigation and enter to go to the location.
  - Location determines scene.
* Web version using tiny_http, replicating GUI as much as possible.
  - Add listen to config.
  - Add tiny_http to WEB.
  - Add sharing of fonts to web.
  - Add sharing of full image to web.
  - Build API to receive from web client and respond.
  - Build web client.
  - Implement persona and map in web.
* API for connecting personas (a sender and a reciever), maybe using same API as web client.
