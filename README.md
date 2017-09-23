Instructions for installing Emscripten (based on Windows), assuming you are following directions here: http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/, to get Rustup and the right target installed.

#### Emscripten setup
- Navigate here, http://kripken.github.io/emscripten-site/docs/getting_started/downloads.html, and download 64-bit Emscripten SDK for Windows.
- Unzip and place files wherever makes most sense on your system, it doesn't matter
- Make sure Microsoft Visual Studio 2015 is installed
- Navigate to the directory where you placed unzipped files
- Execute `emcmdprompt.bat` to start a command prompt
- Run `emsdk update`
- Run `emsdk install sdk-incoming-64bit --vs2015`
- Run `emsdk activate sdk-incoming-64bit`
- In the same command prompt you should be able to run `emcc -v` and see the version output

#### Compiling Rust and building site
In this directory where you've cloned this repo:
- In `.cargo/config`, most like located at `C:\Users\<username>` place the following in the file:
```
[target.wasm32-unknown-emscripten]
linker = "C:\\Users\\<username>\\path\\to\\emscripten\\<emscripten version number>\\emcc.bat"
```
- Run `cargo build --target=wasm32-unknown-emscripten --release`
- Run `npm install`
- Run `npm run gulp`
- Navigate to `http://localhost:3006/` in web browser
- If using SAFE browser, for now, you'll observe an error in console that Fetch API failed due to localhost URL scheme not being supported.
