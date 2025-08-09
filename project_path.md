# rust project complete human
<!-- change for test -->
## project path
<!-- keep the format -->
## Start Date of project

```bash <!-- markdownlint-disable-line code-block-style -->
$ date
Fri Jul 25 09:07:19 AM CEST 2025
```

## Environment

### Os
<!-- keep the format -->
```bash
uname -a
Linux debian 6.1.0-37-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.140-1 (2025-05-22) x86_64 GNU/Linux
```

## Point release
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cat /etc/debian_version
12.11
```
<!-- keep the format -->
## BASH version used
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
echo $BASH_VERSION
5.2.15(1)-release
```
<!-- keep the format -->
## Create plain rust format inside terminal
<!-- keep the format -->
- ## Create for your own project a new project folder in the console(terminal ,bash shell), e.g. in your own home folder, and open it as a new project inside your program used - in my case MS VSCode / MS VSCodium
<!-- To comply with the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="new_project"
echo ${project_name} 
# cd && mkdir <project_name folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir ${project_name} && cd $_
```
<!-- keep the format -->
>[!TIP]
><!-- keep the format -->
>- Bash Special Variables (\$0,\$?,\$#, \$@, \$\$, \$*, \$-) [![alt text][1]](https://tecadmin.net/bash-special-variables/)
>- Another desc [1](https://stackoverflow.com/questions/5163144/what-are-the-special-dollar-sign-shell-variables)
<!-- -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
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
&& mkdir tests
```
<!-- keep the format -->
>[!TIP]
>Show all installed component with version
>
<!-- keep the format -->
>[!TIP]
>Show and delete all installed package [![alt text][1]]( https://stackoverflow.com/questions/12137431/test-if-a-command-outputs-an-empty-string)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# list all installed crates and write to file with complete command fo uninstall
if [[ $(cargo install --list| head | wc -c) -gt 0 ]]; then
cargo install --list |grep "^\s\s\s\s*" |xargs -n 1 echo "cargo uninstall " |tee /tmp/uninstall.txt
else
echo "NO rust package SYSTEM WIDE installed"
fi
# cargo install --list  |cut -d " " -f1 | grep -v "^$" |xargs -n 1 echo "cargo uninstall "
# cargo install --list
```
<!-- keep the format -->
## Install necessary package - Followed this tutorial from web page [![alt text][1]](https://markaicode.com/profiling-applications-2025/)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# Install perf (on Ubuntu/Debian)
sudo apt-get install linux-tools-common linux-tools-generic
<!-- -->
# [x]: 
sudo apt install usbip
# [x]:
sudo apt install hwdata
# [x]: 
sudo apt install usbutils
# [x]:
cargo install flamegraph
```
<!-- keep the format -->
## Add more packages depending on the operating system used here [![alt text][1]](https://pkgs.org/download/linux-perf)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
sudo apt install linux-perf
<!-- keep the format -->
## Make sure the stable toolchain is activated
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
```
<!-- keep the format -->
## Project clean
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo clean # remove everything of target folder of project
```
<!-- keep the format -->
## Project build
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo build
```
<!-- keep the format -->
## Project run
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo run
```
<!-- keep the format -->
>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- -->
>[!TIP]
>Get the link symbol with the curl command using the console/terminal
>
>>-m, --mode=MODE [![alt text][1]](https://www.man7.org/linux/man-pages/man1/mkdir.1.html) \
    set file mode (as in chmod), not a=rwx - umask
>><!-- -->
>>-p, --parents [![alt text][1]](https://www.man7.org/linux/man-pages/man1/mkdir.1.html) \
    no error if existing, make parent directories as needed,
    with their file modes unaffected by any -m option
><!-- -->
>```bash <!-- markdownlint-disable-line code-block-style -->
># First make sure that the target directory exists
>mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"
>```
<!-- keep the format -->
>[!TIP]
>Add link to files - README.md [![alt text][1]](https://github.com/MathiasStadler/rust_project_complete_human/blob/d45e5b8abe947ca525ce2d29437056dd4775e408/README.md#L1)and project_path.md [![alt text][1]](https://github.com/MathiasStadler/rust_project_complete_human/blob/d45e5b8abe947ca525ce2d29437056dd4775e408/project_path.md#L1)
><!-- -->
>```bash <!-- markdownlint-disable-line code-block-style -->
> bash -c echo "\n\n<-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email --> \n\n[1]: ./img/link_symbol.svg"  >> ./project_path.md
>```
<!-- keep the format -->
## rust toolchain [![alt text][1]](https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# rust toolchain help - long output
rustup help toolchain
# rust toolchain help - the save
rustup toolchain help
# Install, uninstall, or list toolchains
# Usage: rustup[EXE] toolchain <COMMAND>
# rust toolchain already installed on system
```
<!-- keep the format -->
## rust toolchain - switch
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup override set nightly
#or
rustup override set stable
```
<!-- keep the format -->
## rust toolchain - which is active
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# use show command to show which is active and installed toolchains or profiles
# see the description
rustup -v  2>&1 | grep show
# detect active toolchain
rustup show

```

>[!TIP]
>Marker
<!-- -->
TODO: Test todo
<!-- -->
FIXME:
<!-- -->
BUG:
<!-- -->
NOTE:
<!-- -->
HACK:
<!-- -->
[ ]:<!-- -->
<!-- -->
[x]:
<!-- -->
HACK:
<!-- -->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->