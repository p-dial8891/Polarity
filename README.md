# Polarity
A terminal interface to the Polaris music streaming server made for a raspberry pi zero device

## Introduction
The Polarity project aims to capture track enqueueing features that were present in popular players like Winamp and combines that with music downloading through the [Polaris](https://github.com/agersant/polaris) music server. It has been developed on a raspberry pi zero 2 W with a 1.3in TFT bonnet from Adafruit.

## Components
The software is divided into two executables - one that acts as the terminal and the other that plays the audio. The terminal has access to the Polaris server and downloads track info from it. The player also has Polaris access, using it to download songs and play them locally. The two components communicate with each other through a TCP connection. While both components can be placed in the same host and the command line options set to use the same host address, they can also be placed on separate hosts on a network. Although, placing them on different hosts is still an unstable feature.

## Usage
The two executables need to be specified with an options file which contains the polaris hostname, a token to connect to the polaris server, the hostname / ip address of the host running the terminal (TUI) application and the hostname / ip address of the host running the player. Two certificates (.der files) were added in this repository and are required to be in the current working directory when running the programs. 
```
# File name : options.file
--host=www.mypolarisserver.com
--token=abcdef1234567890
--player=192.168.1.1
--tui=192.168.1.2
```
From the root of the repository, launching the programs is in the following manner:
```
target/release/tui @options.file
```
```
player/target/release/player @options.file
```
Note that the binaries on the release page are compiled for the Raspberry Pi OS running on a Raspberry Pi Zero 2W.


## Setup
### Parts List
With a few parts and the release binaries, a simple remote-controlled music player for Polaris can be made. The setup on which the releases were tested consisted of:  
1. [Raspberry Pi Zero 2W](https://www.amazon.co.uk/TurboBit-Raspberry-Soldered-Headers-Color-Coded/dp/B0DJ7TCVKF/ref?th=1)
2. [Pimoroni Audio DAC Shim (LIne out)](https://wholesale.pimoroni.com/products/audio-dac-shim-line-out)
3. [IR Receiver VS/1838B](https://www.amazon.co.uk/dp/B016DA2FIK)
4. [Geekworm X306 UPS](https://geekworm.com/products/x306)

### System Overview
The raspberry pi runs both the player and terminal binaries. The line out from the audio DAC is connected to a speaker. The raspberrypi OS can be setup to receive IR commands from any standard remote control handset. The scan codes from the remote control are converted to key codes and the terminal app responds to the keys and controls the playlist and the playback.

Instead of running on the raspberry pi, the two binaries can be compiled for windows or linux and run on a PC. The application will respond to key presses of the keyboard. The keys to control the applcation are:
1. Up - Scroll up
2. Down - Scroll down
3. Left - Dequeue a track
4. Right - Enqueue a track
5. Enter - OK / Autoplay tracks
6. Tab - Move to next screen
   
## Screenshots
Main Screen  
![Main screen](https://github.com/user-attachments/assets/2b339683-0b5f-4a65-ad3e-5bb56167c118)  

Playback Screen  
![Playback screen](https://github.com/user-attachments/assets/2eae641e-a636-4c6e-9f1b-4360167e8e24)    

Shutdown Screen  
![Shutdown screen](https://github.com/user-attachments/assets/26e21f44-4ceb-4dbc-b056-6657b5a4e573)  


## Compatibility
This software is known to work with 0.14.0 version of Polaris but seems to be incompatible with one of the later versions.

## Resources
[Video](https://www.youtube.com/watch?v=HmmpZukn4Zg)  
[Hackster.io](https://www.hackster.io/hardcoder/polarity-a-music-player-for-polaris-8cd4eb)
