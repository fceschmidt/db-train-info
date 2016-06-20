# db-train-info

## Why do I need this?

This is an open data project which provides access to on-board data of Deutsche
Bahn ICE trains. It is written in Rust and contains a library which can get
information about the current state of the train (speed, GPS coordinates, all
previous and next stations,...) to whose on-board WiFi network you are currently
connected. It makes the necessary HTTP requests and provides the data in
deserialized Rust structs.

## How do I use this?

Create a Rust project, add this repository as a dependency and generate the
documentation. Inside the Examples section of the TrainInformation struct you
will find a starting point on how to use this library.

## May I even use this for my purpose?

See the MIT license and keep in mind that this project is in no way endorsed by
Deutsche Bahn, so this whole thing may cease to work at any point in time with
no replacement.
