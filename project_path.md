# The detailed project path  

## Project name => prompt_rust_ibkr_search_put_ratio_credit_spread

## Project target

- search/scan for profitable put ratio credit spread

## Project start date

```bash <!-- markdownlint-disable-line code-block-style -->
$ date
Wed May 21 09:17:17 AM CEST 2025
```

## Hardware

### Install How do install lshw  on debian and usw it [![alt text][1]](https://www.tecmint.com/commands-to-collect-system-and-hardware-information-in-linux/)
<!--- THis empty line inside the block is necessary for correct format -->
     ```bash<!-- markdownlint-disable-line code-block-style -->
     sudo apt update
     sudo apt install lshw
     ```
    <!--- THis empty line inside the block is necessary for correct format -->
    ### used
    <!--- THis empty line inside the block is necessary for correct format -->
    ```bash<!-- markdownlint-disable-line code-block-style -->
     sudo lshw -class cpu -class memory
    [sudo] password for trapapa:
    *-cpu
        description: CPU
        product: Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
        vendor: Intel Corp.
        physical id: 42
        bus info: cpu@0
        version: 6.58.9
        slot: SOCKET 0
        size: 2836MHz
        capacity: 3800MHz
        width: 64 bits
        #truncated
     *-memory
       description: System Memory
       physical id: 41
       slot: System board or motherboard
       size: 16GiB
       #truncated
    >```
><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;
<!--- THis empty line is necessary for correct format -->
> [!NOTE]
> How do I list all of the RUST packages
> are installed **globally** with the
> command **cargo install**?[![alt text][1]](https://stackoverflow.com/questions/60857222/how-do-i-list-all-of-the-packages-ive-installed-globally-with-cargo-install)
>
> ```bash<!-- markdownlint-disable-line code-block-style -->
> cargo install --list
> ```
><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

### OS-Version

```bash
cat /etc/os-release 
PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"
NAME="Debian GNU/Linux"
VERSION_ID="12"
VERSION="12 (bookworm)"
VERSION_CODENAME=bookworm
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"
```

## OS-Version - uname

```bash <!-- markdownlint-disable-line code-block-style -->
$ uname -a
Linux debian 6.1.0-28-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.119-1 (2024-11-22) x86_64 GNU/Linux
```

## IDE - MS Visual Studio Code

```text
Version: 1.100.2
Commit: 848b80aeb52026648a8ff9f7c45a9b0a80641e2e
Date: 2025-05-14T21:47:40.416Z
Electron: 34.5.1
ElectronBuildId: 11369351
Chromium: 132.0.6834.210
Node.js: 20.19.0
V8: 13.2.152.41-electron.0
OS: Linux x64 6.1.0-34-amd64
```

## Create new folder for these project

```bash <!-- markdownlint-disable-line code-block-style -->
# cd && mkdir <project_name folder> && cd $_
# command 'cd' change to home folder from logged in user
cd && mkdir prompt_rust_ibkr_search_put_ratio_credit_spread && cd $_ 
```
<!-- -->
## Oen the project folder in MS studio code

```bash <!-- markdownlint-disable-line code-block-style -->
# code <folder path>
code prompt_rust_ibkr_search_put_ratio_credit_spread
```

>[!TIP]
> Don't forget to save your project on GitHub - saves you serious headaches
<!-- -->
## Init a new rust based project inside the previously generated folder
<!-- -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& cargo build \
&& cargo run \
&& cargo build --examples \
&& cargo run --example example \
&& mkdir tests  
```

## No test existing

- command

```bash
cargo test
```

- output

```txt
 Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/main.rs (target/debug/deps/prompt_rust_ibkr_search_put_ratio_credit_spread-aad63c388e7d2ce9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Empty testfile without any testcase

<!-- -->
>[!TIP]
> How to cat ```<<EOF>>``` a file containing code
> Test link image for a Markdown link text [![alt text][1]](https://stackoverflow.com/questions/22697688/how-to-cat-eof-a-file-containing-code)
>
><!--- THis empty line inside the block is necessary for correct format -->
<!-- -->
>[!TIP]
>cargo edit [![alt text][1]](https://crates.io/crates/cargo-edit)
>
>This tool extends Cargo to allow you to add, remove, and >upgrade dependencies by modifying your Cargo.toml file from >the command line.
>
>Currently available subcommands:
><!-- -->
>- cargo upgrade
>- cargo set-version
><!-- -->
>Installation
><!-- -->
>```bash
>cargo install cargo-edit
>```

## Create first simple testcase

<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
