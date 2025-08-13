# rust project complete human
<!-- change for test -->
## project path

## start codium
<!-- keep the format -->
- installed local system path=> /usr/bin/codium
- with permanently assigned **--extensions-dir /home/trapapa/codium_one/extensions_one**
- with Permanently assigned **--user-data-dir /home/trapapa/codium_one**
- with Permanently assigned **--profile my_one**
- set log level to **--log debug**
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
$codium --user-data-dir /home/trapapa/codium_one/ --extensions-dir /home/trapapa/codium_one/extensions_one --log debug --profile my_one
```

>[TIP!]
> How to tail all the log files inside a folder and sub folders
><!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->

```
<!-- keep the format -->
## Start Date of project
<!-- keep the format -->
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
## Create for your own project a new project folder in the console(terminal ,bash shell), e.g. in your own home folder, and open it as a new project inside your program used - in my case MS VSCode / MS VSCodium
<!-- To comply with the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="new_project"
echo ${project_name} 
# cd && mkdir <project_name> folder> && cd $_
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
&& rustup show \
&& rustup check \
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
```
<!-- keep the format -->
## Access to performance monitoring and observability across the operating system
<!-- keep the format -->
- on debian 12.09 **cat /etc/debian_version**
- More information can be found at 'Perf events and tool security' document: [![alt text][1]](https://www.kernel.org/doc/html/latest/admin-guide/perf-security.html)
  perf_event_paranoid setting is 3:
- -1: Allow use of (almost) all events by all users
      Ignore mlock limit after perf_event_mlock_kb without CAP_IPC_LOCK
<!-- -->
- 0: Disallow raw and ftrace function tracepoint access
- 1: Disallow CPU event access
- 2: Disallow kernel profiling
- 3: perf_event_paranoid setting is 3
<!-- -->
- Command to set the prof setting
sudo sysctl kernel.perf_event_paranoid=0
<!-- keep the format -->
>[TIP!]
>How can I set the grep after context to be "until the next blank line"? [![alt text][1]](https://stackoverflow.com/questions/13534306/how-can-i-set-the-grep-after-context-to-be-until-the-next-blank-line)
>rustup show |sed -n '/active toolchain/,/^$/p'
<!-- -->
>```bash <!-- markdownlint-disable-line code-block-style -->
>rustup show |sed -n '/active toolchain/,/^$/p'
>```
><!-- keep the format -->
<!-- keep the format -->
## Make sure the stable toolchain is activated
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->
## Project clean - remove all file inside the target folder
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
## Project build --release
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo build --release
```
<!-- keep the format -->
## Project run
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo run
```
<!-- keep the format -->
## Project run flamegraph
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin <project_folder>
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin rust_project_complete_human
```
<!-- keep thr format-->
## Memory Profiling with DHAT
<!-- kep the format -->
### Install valgrind with DHAT [![alt text][1]](https://markaicode.com/profiling-applications-2025/)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# Install valgrind with DHAT
sudo apt-get install valgrind
# Install DHAT viewer
## doesn't work on this os - as specified in the tutorial
## cargo install dhat-rs
cargo add  dhat
```
<!-- keep the format -->
>[TIP!]
>Install valgrind on debian 12 [![alt text][1]](https://valgrind.org/downloads/repository.html)
><!-- keep the format-->
>- Install
><!-- keep the format-->
>```bash <!-- markdownlint-disable-line code-block-style -->
>git clone https://sourceware.org/git/valgrind.git
>cd valgrind
>./autogen.sh
>./configure
>  make
>```
<!-- keep the format-->
```bash <!-- markdownlint-disable-line code-block-style -->
# test folder is NOT exits 
create_folder_file="/home/trapapa/bin_valgrind" && if [[ (-f ${create_folder_file} ) || ( -d ${create_folder_file} ) || (-L ${create_folder_file}) ]]; \
then echo "folder/file/link ${create_folder_file} exists"; \
else echo "folder/file/link ${create_folder_file} NOT exists";\
fi 
cd /tmp
git clone https://sourceware.org/git/valgrind.git && \
cd valgrind && \
./autogen.sh && \
#./configure --prefix=<installation-directory>
./configure --prefix=${create_folder_file} && \
make && \
make install
```
><!-- keep the format -->
<!-- keep the format -->
### Run program whit DHAT -follow this tut  [![alt text][1]](https://www.justanotherdot.com/posts/profiling-with-perf-and-dhat-on-rust-code-in-linux.html)
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
cargo build --release
```
<!--keep the format -->
### Add/replace  cargo.toml
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
[profile.release]
incremental = true
debug = true
lto = "fat"
```
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
DHAT=yes ./target/release/rust_project_complete_human
```
<!--keep the format -->
>[!TIP]
>How do I recursively grep all directories and subdirectories? [![alt text][1]](https://stackoverflow.com/questions/1987926/how-do-i-recursively-grep-all-directories-and-subdirectories)
><!-- keep the format-->
>```bash <!-- markdownlint-disable-line code-block-style -->
>grep -r --include "*.md" if .
>```
><!-- keep the format -->
>- with line number
>-- flag -n for line number
><!-- keep the format -->
>```bash <!-- markdownlint-disable-line code-block-style -->
>grep -rn --include "*.md" if .
>```
><!-- keep the format -->
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
## rust toolchain - stable to nightly
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup override set nightly
#or
rustup override set stable
```
<!-- keep the format -->
>[NOTE:]
> 
FIXME: rustup -v  2>&1 | grep show better command description
<!-- keep the format -->
## rust toolchain - which is active
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
# use show command to show which is active and installed toolchains or profiles
# see the description of option rustup show
rustup -v  2>&1 | grep show
# detect active toolchain
rustup show
# show active toolchain
rustup show |sed -n '/active toolchain/,/^$/p'

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