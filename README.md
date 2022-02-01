# Disaff - Injectable DLL
This is a pretty simple library for Windows that upon injection, constantly looks for a process' windows and calls on them winapi function `SetWindowDisplayAffinity` to make them "invisible for screen capture software" (in simple words). 

## Build
If you want to receive an info box upon first scan to check which windows were altered, enable `info_box` feature. <br>
<img src="https://image.prntscr.com/image/fj_olKAWRJOBTXzFwW6scQ.png" width=45%>

for 32 bit systems: <br>
`cargo build --release --target i686-pc-windows-msvc`

but remember to add the target to toolchain by: <br>
`rustup target add i686-pc-windows-msvc`

## Usage
Simply use any DLL injector. I personally use [GH Injector](https://github.com/guided-hacking/GuidedHacking-Injector)  

## Example
| Before                               | After                                |
|--------------------------------------|--------------------------------------|
| ![](https://i.imgur.com/Zs1gJkT.png) | ![](https://i.imgur.com/tip3Ea8.png) |

_Captured with Snipping Toolbar [WIN]+[SHIFT]+[S]_
