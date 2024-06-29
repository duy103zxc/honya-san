## Buukuru

A command-line tool for downloading web novels from Syosetu, Hameln and Kakuyomu (Currently being implemented).

## Installation

Visit `Release` section to download the application. Currently, only Linux binary is available. If you wanna try building the application from source, please check [Build](#build)

## How to use

- `buukuru [source] [ID]` - Download all the available chapters. 

- `syosetu` If the webnovel's URL is `https://ncode.syosetu.com/n8584jc`, the command should be `buukuru syosetu n8584jc`
- `hameln` If the webnovel's URL is `https://syosetu.org/novel/346076/`, the command should be `buukuru hameln 346076`  
- `kakuyomu` - Currently working on it.


## Build

You need `cargo`.

1. `git clone https://github.com/duy103zxc/buukuru.git`
2. `cd buukuru`
3. `./build.sh`