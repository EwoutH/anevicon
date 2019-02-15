<div align="center">
  <h1>anevicon</h1>
  
  <a href="https://gitter.im/Gymmasssorla/anevicon">
    <img src="https://img.shields.io/badge/chat-on%20gitter-pink.svg">
  </a>
  <a href="https://travis-ci.com/Gymmasssorla/anevicon">
    <img src="https://travis-ci.com/Gymmasssorla/anevicon.svg?branch=master">
  </a>
  <a href="https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-GPLv3-blue.svg">
  </a>
  <a href="https://crates.io/crates/anevicon">
    <img src="https://img.shields.io/badge/crates.io-v0.1.0-orange.svg">
  </a>
  
  The fastest UDP-based server stress-testing tool, written in Rust
</div>

## Introduction
Anevicon is a high-performance traffic generator, designed to be as convenient and reliable as it is possible. It sends numerous heavy UDP-packets to a victim web server, thereby simulating an activity that can be produced by your end users or a group of hackers.

## Cautions
 - The goal of Anevicon is to produce the maximum possible (for the attacking system) load on the specified victim address. Thereby, this does not mean that Anevicon will break absolutely any server while running on your computer.
 
 - Anevicon was developed as a means of testing the stress resistance of servers, and not for hacking, that is, the author of the project **IS NOT RESPONSIBLE** for any damage caused by your use of my program. See [the license notice](https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE#L600).
 
 - Despite the fact that Anevicon is heavily tested both automatically and manually, does not mean that the author is responsible for any bug in his work. The program comes with **ABSOLUTELY NO WARRANTY**, see [the license disclaimer](https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE#L589).

## Installation
```bash
$ cargo install anevicon
```
