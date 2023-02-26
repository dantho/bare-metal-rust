# Plan  
(a [markmap](https://markmap.js.org/))

## Next steps
* Refactor esp-now/DHCP code into a library  
* esp-now and dhcp become separate binaries
* Experiments become binaries (not examples)  
* Compile all binaries as regression?

## Learning / Experiements  

### IMU data capture and transfer   
Need a means to transfer datasets to host for debug/display/offline use.  BAUD rate on serial port is 9600 bps, so 960 Bytes per second.  We have 3 channels of two-byte data, so that's 180 samples per channel per second max.
    * What would be required for our app?
        * 100 samples/sec?
        * 20 samples/sec?
        * Slower rates yield less timing precision
        * Slower rates may miss events
        * Slower rates use less power
    * Combine channels, report magnitude only?

### Host-based plotting routine for esp-monitor  
Found [this post](https://dev.to/apollolabsbin/embedded-rust-embassy-uart-serial-communication-4fd3) that led me to, OMG, [Serial Studio](https://serial-studio.github.io/)!

### Sleep mode for low power
Put the MCU and the IMU into the lowest sleep level possible, with all lights off.  Button press should wake up both and the MCU should turn on the LED to report its state.  Going to sleep should be timer-based and should fade-to-black the LED.

### Wake on Shake
Let the IMU wake the MCU. Check wire for a physical connection for the interupt, a poor man's version could be that the MCU periodically wakes and polls the IMU for wake state.  The IMU does have wake on shake, right?

## Library APIs
* Comms
    * Peer_to_Peer()
        * Ping()
        * Hello()
        * GroupLead()
    * HostDebug
        * Accel data xfer
* LED
    * LED.fade_to_black()
    * LED.dim_to_brightness(20%)
    * LED.set_color(color)
* IMU
    * Sleep()
    * WakeOnShake_config()
    * WoS()
* MCU
    * Sleep()
    * WakeOnShakeInterrupt(??)
