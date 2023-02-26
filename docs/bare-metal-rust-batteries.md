
## Batteries!  (and esp-now peer-to-peer comms)  
To esp-rs-B, I plugged in a 3.7V, 350mAh, lithium ion LP552530 at 7:27.  
The tiny but bright red "BAT" light is lit up!  I wonder it will will turn green? (Or yellow?  Or maybe dim?)  Nope.  It will just turn off as the 3 hour mark approaches. I read the [MCP73831 datasheet](./esp-rust-board/docs/MCP73831-Family-Data-Sheet-DS20001984H.pdf):  
![Graph of Charge Profile for MCP73831 showing 3 hours to full charge for 180uAh battery.](./esp-rust-board/docs/pics/TypicalChargeProfile180mAh.png "MCP73831 Charge Profile")  

Oh!  The red light is out!  Must have _just_ gone out.  It is 9:19, so let's say it went out 2 mins ago, that's 1 hour 50 minutes from store-shelf-empty to about 90% charged (averaging charge under the curve from the 180mAh chart, above.)  

First test -- unplug the usb line while monitoring chatter on the other -- still connected -- board. Success!  

It is 9:30 PM. Now to test whether the radio can run all night... The app running is eesp-now transmitting every 5 seconds, and listening for peers always?  The peer is also transmitting every 5 seconds, so ignoring the phase difference... it's a radio transaction every 2.5 seconds.
Note that NO LED's are on on the unit under battery power.  The two radios are about 4" apart!
Speaking of battery, I'm wondering if bigger is better --> more weight.  Hmm...  This 350uAh is super light.  Barely heavier than the board.  

Nope.  Didn't make it.  Weirdly, and sadly, the plugged in unit died before the battery unit did. Note in the data below, the last data before esp-rs-A died, that dst_address matches esp-rs-A, the plugged in unit that is reporting this data, and src_address matches esp-rs-B, the battery unit sending the data.)

```
Send
Received ReceivedData { data: [48, 65, 6c, 6c, 6f, 20, 50, 65, 65, 72], info: ReceiveInfo { src_address: [34, 85, 18, 0, 9e, 10], dst_address: [34, 85, 18, 1, f2, e8], rx_control: RxControlInfo { rssi: d9, rate: 0, sig_mode: 0, mcs: 0, cwb: 0, smoothing: 0, not_sounding: 0, aggregation: 0, stbc: 0, fec_coding: 0, sgi: 0, ampdu_cnt: 0, channel: 1, secondary_channel: 0, timestamp: eadbf12a, noise_floor: a0, ant: 0, sig_len: 35, rx_state: 0 } } }
Error: The device does not recognize the command. (os error 22)
```

Recharging now, with both radios sending away. It is 7:30AM.

Damn.  Same error on same unit after something less than 1 hour.  (Saving data... Could extract the actual time of failure if it was important.)  Reset and the battery unit (charging) is still chatting away.

Same error, again.  And again on the unit that I was monitoring, A.  Unit B, with the battery, was also plugged in, but I was not monitoring. I have NEVER seen a failure on Unit B.  Might be an esp-monitor error, not an embassy-esp-now error.  Unit B, on battery, wasn't monitored for most of these experiments and I didn't monitor it now during charging out of habit.  (You only need one monitor to see the status of all board communicating.) Gonna add a battery to unit C now -- will give me more flexibility with 2 units on battery.

Time-Check: I have stopped charging unit B, so it was plugged in for many hours and is FULLY charged.  On battery as of 2:50 PM.

3:07 PM. Unit C plugged in and charging AND TRANSMITTING.  Unit A is being monitored and picking up both units B and C.  I must presume units B and C are all receiving each other.  Maybe I should pack this info into the data they are passing back and forth, so that my single monitor strategy will tell me everything I need to know.

4:10 PM Unit A's monitor died again.  :(  Restarted unit A monitor, units B and C still chirping!  (Here unit C is plugged in but not being monitored.)  Unit C is still charging (red light on).  It has been 1 hour so far.  Expecting light to go out in about 50 minutes. (Unit B took 1 hour 50 minutes for first charge.)

5:05 PM Red charge light is out.  Not sure when it went out.  :(  It has been 1:58 since I plugged it in to charge (from warehouse-empty.)

6:25 -- all 3 units still chatting and A is still being monitored.  No error since the 4:10 restart. Unit B has been on battery 3.5 hours.  Unit C is still being charged to a _very full_ state.

7:30 I literally just watched B's data stop. Just now.  So... 2:50 to 7:30 that's 4.5 hours.

7:48 PM Loaded bare_metal_... (orientation color app, no radio) into unit C and unplugged it.  Hoping this simple LED/Accel app lasts all night and then some. Also started charging Unit B.

Unit A now has a battery.  It had some charge on it out of the package.  It is now charging, along with B.  C is on battery power only, started at 7:48 PM

C didn't last all night. 🙁  
