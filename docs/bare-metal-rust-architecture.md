# Architecture ("What did you do here?!")

## See [Plan](plan-options-incremental.md), especially "Next Steps"

## Components needed for our embedded app project
* esp32c3 -- our embedded app MCU
    * Sleep mode (to conserve battery power)
    * Embedded Hal -- to talk to the following components on our board
* esp-wifi / esp-now communication (between devices)
* esp-wifi / esp-dhcp for over-the-air debug and over-the-air updates
* esp-wifi / esp-ble for app-to-host communication
* IMU 
    * Wake on shake
    * Sleep mode (to conserve battery power)
* Smart-LED
    * With timeout and transition to off (to conserve battery power)
* Library of app-specific algos
    * Freefall detect / measure
    * timestamp syncronization / reporting / translation to absolute time
* App-specific data structures
    * App-relative timestamp
    * Acceleration raw data
    * Acceleration event history (calculated from raw data)
    * Group handshaking -- "Meshing up"


## Links
Learned about Rust mod's, bin's, and lib's by walking through through [rust-split-example](https://github.com/robertorojasr/rust-split-example).  See my refactoring dreams here: [crate-evolution](crate-evolution.md).   Note that a binary crate can have modules inside of it, and a module crate (a library) can have binaries inside of it.

## Original experiments
... were based off refactored [esp-rs/esp-wifi](..\..\esp-wifi\README.md) examples, as well as the  [esp-rs/esp-template](https://github.com/esp-rs/esp-template) for bare metal.  I refactored the two esp-wifi examples into [my own crate](../../jug02_edhcp/), but the two are clumsily merged.