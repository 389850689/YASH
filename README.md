# YASH
***Y***et ***A***nother ***S***imple [VT-x] ***H***ypervisor
<sub><br><i>Compiler Information:</i><br>
nightly-x86_64-pc-windows-msvc (default)<br>
rustc 1.57.0-nightly (aa8f2d432 2021-09-18)<br></sub>
***

### Build Instructions
<sub>Make sure you've installed the Windows 10 WDK & Visual Studio 2019 before building.</sub><br>
<sub>(It might be important to note that the post-build signing process is not robust, and out of the box it only supports visual studio 2019 community edition. If you use a different version it should be trivial to fix.)
```sh
$ cargo install --force cargo-make
$ git clone https://github.com/389850689/YASH/
$ cd YASH
$ cargo make sign
```
***
### Running Tests
Yeah, I don't know about this one.
***
### Load Instructions
1) Install OSR Driver Loader.
2) Profit.
