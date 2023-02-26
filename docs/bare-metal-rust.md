# Bare Metal Rust (on ESP32C3 via ESP32-C3-DevKit-RUST-1)

## Project / Module Structure
Need to study up and develop a plan.  See [Project Architecture](bare-metal-rust-architecture.md)

## Dan's ESP32-C3-DevKit-RUST-1 boards  
|Board|Setup|MAC Addr|Static_IP|Wifi Name|Battery|Status|
|:---:|---|---|---|:---:|:---:|:---:|
| A |privateA|34:85:18:01:f2:e8|192.168.121.3|esp_rs_A||😃|
| B |privateB|34:85:18:00:9e:10|192.168.121.4|esp_rs_B|350mAh|😃|
| C |privateC|34:85:18:02:3c:ec|192.168.121.5|esp_rs_C|350mAh|😃|

## Wondering how to STOP these apps running on the hardware -- I think you don't. So how do I make them sleep?  
So... If I'm on battery, how do I "shut off"?  How do I preserve the battery? Hmm...
I could simply **not** use the radio.  I'll do that and see how much longer than 4.5 hours it's going to last.  Using the color/orientation app. Done. We'll see how that goes.

## esp-wifi  
esp-wifi examples seem to work really well. ~~Can't test esp-now until 2nd, 3rd board arrive.~~  
Board are here!  They work with DHCP (unique/solo) and with esp-now (together, two at a time for now.)  See Jug02 for that functionality.  

## Batteries! (and esp-now peer-to-peer comms)  
[bare-metal-rust-batteries](bare-metal-rust-batteries.md)
* 350 mAh Batteries charge to 90% in a few hours.  Discharge with embassy-esp-now app running in 4.5 hours.  
* espmonitor has some glitch (or my PC has a COM glitch) that generates an error every hour or two.  Monitor only, not esp-rs board.  

## Original Walkthrough
From the [esp-rs Organization on GitHub](https://github.com/esp-rs) we see that bare metal rust requires requires an _extremely experimental_ [esp-rs/esp-hal](https://github.com/esp-rs/esp-hal), specifically the [esp32c3](https://github.com/esp-rs/esp-hal/tree/main/esp32c3) and an appropriate [Peripheral Access Crate](https://github.com/esp-rs/esp-pacs).  Perhaps [esp32c3](https://github.com/esp-rs/esp-pacs/tree/main/esp32c3) will be enough?  Or maybe I can get all I need by just using Cargo generate with the [esp-rs/esp-template](https://github.com/esp-rs/esp-template)?  Let's try that.

As the repo suggests, I'll give this a try:
```
cargo generate https://github.com/esp-rs/esp-template
```
then  
```
cargo espflash --release --monitor COM3 [COM4, COM5]
```
Works!  But does nothing. Println!() is part of std, so won't compile in our no-std app.  Hmm... How to output to the serial port?  I could try dbg!()?  Nope.  Also std.  How about the bare metal log crate... what's that called? (Not "log", that's a facade for all logging.  [defmt](https://ferrous-systems.com/blog/defmt/) from Ferrous-Systems looks interesting, but might be a distraction at this point -- there's a template repo and [a whole book](https://defmt.ferrous-systems.com/)!

Oh!  I found esp-println in [esp-rs/esp-hal](https://github.com/esp-rs/esp-hal)'s *Ancillary Crates* section.  Works!

# ESP32-C3-DevKit-RUST-1
What about a board support crate? A simple local crate, 'esp32-c3-dkc02-bsc', was used for the espressif training, even though their custom board was a [ESP32-C3-DevKit-RUST-1 Repo](https://github.com/esp-rs/esp-rust-board 'Open source HW'). 

Oh! I had previously found/cloned the board support package and it includes a nice [README](esp-rust-board-bsp\README.md).  Refactored to use this package as a local crate.  (But I could point to it on my fork on GitHub...?)

Examples in esp-rust-board-bsp all want more stuff including pac crate.  I think this gets somehow built from the svd file and svd2rust.  I read that on one of the repos...


This blog series by [Scott Mabez](https://mabez.dev/) goes into some helpful history:
[Rust on Espressif chips - 10-01-2022](https://mabez.dev/blog/posts/esp-rust-10-01-2022/)
[Rust on Espressif chips](https://mabez.dev/blog/posts/esp-rust-espressif/)
[Rust on the ESP32 - SVD's, PAC's and USB flashing](https://mabez.dev/blog/posts/esp32-rust-svd-pac/)

See Scott on [this youtube video](https://www.youtube.com/watch?v=qeEmJ_-6fPg&list=RDCMUCDBWNF7CJ2U5eLGT7o3rKog&start_radio=1&rv=qeEmJ_-6fPg&t=4) from Espressif DevCon22 in October.
