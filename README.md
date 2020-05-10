# subtractesizer
Things that goes beep but written in Rust this time.
## Prerequisistes
Most of this automated/packaged anyway but here's everything...
* **Rust compiler including rustup and Cargo.** This needs to be the nightly build due to dependency on the python bindings (PyOt2).
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    curl https://sh.rustup.rs -sSf | sh
    rustup override set nightly
    ```
    Check https://www.rust-lang.org/tools/install if all else fails.

* **python3 with PyQt5:** using standard cpython for now. TODO: virtual-env.
    ```
    apt install python3
    apt install python3-pip
    pip install PyQt5
    ```
## Usage
* **To build the audio engine without launching:** 
```
cargo build
```
* **To build the engine and launch:** 
```
bash ./scripts/run-all.sh
```
