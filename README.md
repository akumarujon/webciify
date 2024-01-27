<p align="center">
<img src="./assets/logo.png" width=380>
</p>
<h1 align="center">Webciify</h2>
<p align="center">Web version of Tapciify.</p>

## Requirements
1. CC linker (Windows - Microsoft Visual Studio with C++ Support) (Linux - gcc)
2. [Rust](https://rust-lang.org)

## Install
For now Webciify can be installed using Git + Rust.

Clone this repository:
```bash
git clone https://github.com/akumarujon/webciify
```

Change dir and Compile:
```bash
cd webciify
cargo build --release
```
Run:
```bash
target/release/webciify
```
or
```bash
cargo run -r
```

* `-r` is for release option 


## Usage
The program will run on port 3000. To get your images in ASCII, you have to give image path and width as a parameter.

```bash
http://localhost:3000?image={image_link}&width={width}
```

For example:
```bash
http://localhost:3000/?image=https://github.com/tapnisu/tapciify/blob/main/assets/original.png?raw=true&width=32
```

### Colors are not supported yet. Don't forget to leave a ⭐️