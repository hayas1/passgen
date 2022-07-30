# passgen
simple password generator implemented by (almost) rust only
![logo](/static/passgen.png)

## deploy
https://hayas1.github.io/passgen/

This page can generate random password such as `Z914f&P@Fupdj5RLMsmy`.

### PWA
This application support Progressive Web Apps (PWA).
So, if you use iOS, you can add this application to home screen with `Add to Home Screen` button.
Or, in windows, you may use this application as desktop application, with browser.
The same is probably true for Android and MacOS.
### available
- [x] password length from 8 to 128
- [x] use lower case
- [x] use upper case
- [x] use numeric
- [x] use some mark symbols
- [x] use custom characters

### feature
Since this is implemented by [Rust](https://github.com/rust-lang/rust),
it can be compiled into [WebAssembly](https://webassembly.org/) and served as a web application.

[Yew](https://yew.rs/docs/) is used for the framework.

## cli tool
This crate also can be compiled as CLI tools.

### available
- [x] password length from 8 to 128
- [x] use lower case
- [x] use upper case
- [x] use numeric
- [x] use some mark symbols
- [x] use custom characters

### feature
[Clap](https://github.com/clap-rs/clap) is used for command line arguments parser.

### usage
#### simple generate
`$ passgen`
`odJezbxZF^siSQ0TxAvM`
#### change length
`$ passgen 128`
`7V0sPmPU5hEtaE10mHA#F&YA#k#9uOmuSoASpEHPUu8TnEPW!r1INluR7k6eIc9iR^3dPzC0@b!#U8SsYsuPTXcnbfg@iLpCP@s733W0LZJmLrBH#3wvEEkh&5un2NHQ`
#### not use lower case alphabet
`$ passgen -l`
`C&IECAT!EYLQWS9F9CC5`
#### try empty available symbols
`$ passgen -lunm`
`because no available symbol, cannot generate a password`
#### any other help
`$ passgen -h` or `$ passgen -help`
