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
# cd && mkdir <project_name folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir rust-example-cov && cd $_
```
<!-- keep the format -->
>[!TIP]
>Bash Special Variables (\$0,\$?,\$#, \$@, \$\$, \$*, \$-) [![alt text][1]](https://tecadmin.net/bash-special-variables/)
>Another desc [1](https://stackoverflow.com/questions/5163144/what-are-the-special-dollar-sign-shell-variables)
<!-- -->
```bash <!-- markdownlint-disable-line code-block-style -->

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
>```bash
># First make sure that the target directory exists
>mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"
>```
<!-- keep the format -->
>[!TIP]
>Add link to files - README.md [![alt text][1]](https://github.com/MathiasStadler/rust_project_complete_human/blob/d45e5b8abe947ca525ce2d29437056dd4775e408/README.md#L1)and project_path.md [![alt text][1]](https://github.com/MathiasStadler/rust_project_complete_human/blob/d45e5b8abe947ca525ce2d29437056dd4775e408/project_path.md#L1)
><!-- -->
>```bash
> bash -c echo "\n\n<-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email --> \n\n[1]: ./img/link_symbol.svg"  >> ./project_path.md
>```
<!-- -->
>[!TIP]
>Marker
TODO: Test todo
FIXME:
BUG:
NOTE:
HACK:
[ ]:
[x]:
HACK:
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->