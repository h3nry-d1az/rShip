
<h1 align="center">hShip <i>(a.k.a rShip)</i></h1>
<p align="center">A tiny ships game made of ASCII characters</p>

<hr>

**hShip** is a small ships game made from simple **ASCII** characters, for **Windows** devices.
It is worth mentioning that this repository contains the code of the **rShip** project, which is nothing more than an *improved* version and *rewritten from scratch* of the original project *(a.k.a a remake)*.
Of course, the *source code* of the original project and its *binaries* will be available in the [releases section](https://github.com/h3nry-d1az/rShip/releases) so that a comparison can be made between the two, but I must warn you that the original version is only in **Spanish**.
Also, this game is *(like most of my titles)* published on the [itch.io](https://itch.io/) platform, so you can find *this same information* as well as *download the game binaries* more easily on [that page](https://h3nry-d1az.itch.io/hShip).

This is, however, a small project that I carried out **quite some time ago**, so don't expect *high quality* as most of the code has remained the same *(except for the language change from being written in C /C++ to being in Rust)*.

<hr>

### Install instructions
#### Get the already compiled binary
1. **Click on the** *hShip.Win64.zip* **file** and then **unzip it**.
2. Then **execute the file with extension .exe** contained in the newly extracted folder *(hShip.exe)*.
3. *(Optional)* **Move the newly extracted directory to a safe location** *(such as* `C:\`*)* and **add the folder to your** `PATH` **variable**.

#### Build the project from source code
1. **Click on the** *hShip.Source.zip* **file** and then **unzip it**, or **clone the repository via git** *(run* `git clone https://github.com/h3nry-d1az/rShip.git` *in* **CMD** *or* **PowerShell** *)*.
2. **Then compile it using cargo** *(if you don't have* **Rust** *installed go* [here](https://www.rust-lang.org/)*)* **using the command** `cargo build --release` if you want to build **just the binary** or **run** `cargo install --path .` to **install it**.

#### Easiest method *(requires* [Rust installed](https://www.rust-lang.org/) *)*
1. Just **run the command** `cargo install --git https://github.com/h3nry-d1az/rShip.git` and **wait for it to finish**.
