# Projector, a CLI for managing multiple projects

I work on a lot of FOSS projects, it's literally part of my job. Unfortunately
this means that moving around my project directories (especially my GOPATH) can
get insanely burdensome. So I wrote this CLI to help me find and manage my
enormous git repos folder. Projector can be viewed as a "no setup" required
version of [mr (myrepos)](https://myrepos.branchable.com/). But a bit more
flexible than that.

- [Installation](#installation)
- [Commands](#commands)
- [Configuration](#configuration)

## Installation

You can install projector from crates.io via cargo:

```
cargo install projector
```

Or you can build it locally with:

```
git clone https://github.com/chasinglogic/projector $PATH_TO_PROJECTOR
cd $PATH_TO_PROJECTOR
cargo install --path .
```

## Usage

```
A code repository manager.

Usage: projector [OPTIONS] [COMMAND]

Commands:
  list  List all projects that projector would operate on
  find  Find projects by matching their paths
  run   Run a command on all matching projects
  help  Print this message or the help of the given subcommand(s)

Options:
  -e, --excludes <EXCLUDES>  
  -i, --includes <INCLUDES>  
  -v, --verbose              
  -c, --code-dir <CODE_DIR>  
  -h, --help                 Print help
  -V, --version              Print version
```

### Definitions

Projector considers any git repo as a "project". In future versions I will
expand this definition but it works for me now so that's the way it is.

## Commands

### List

List will literally print a list of the projects under your code directories. 

List is useful for scripting purposes and with the `--dirty` flag for clocking
out for the day. I use `projector list --dirty` at the end of my coding sessions
so I can find any git repositories I've touched by not committed my work on.

If you have another tool I'm fond of called
[FZF](https://github.com/junegunn/fzf) you can take this a step further and make
a fuzzy selector for your projects in scripts:

```
function sp() {
  cd $(projector list | fzf)
}
```

Now you get something like this:

```
  /Users/chasinglogic/Code/mongodb/mongo-tools
  /Users/chasinglogic/Code/mongodb/kernel-tools
  /Users/chasinglogic/Code/mongodb/evergreen-packer
  /Users/chasinglogic/Code/mongodb/chef-repo
  /Users/chasinglogic/Code/mongodb/mongo-release-tools
  /Users/chasinglogic/Code/website
  /Users/chasinglogic/Code/python/praelatus
  /Users/chasinglogic/Code/python/praelatus/.venv/src/elasticutils
  /Users/chasinglogic/Code/python/projector
  /Users/chasinglogic/Code/archive/projector
  /Users/chasinglogic/Code/projector
  /Users/chasinglogic/Code/go/src/github.com/pkg/errors
  /Users/chasinglogic/Code/go/src/github.com/praelatus/praelatus
  /Users/chasinglogic/Code/go/src/github.com/praelatus/praelatus-poc
  /Users/chasinglogic/Code/go/src/github.com/chasinglogic/dfm
  /Users/chasinglogic/Code/go/src/github.com/chasinglogic/fireplace
  /Users/chasinglogic/Code/go/src/github.com/mattn/go-zglob
  /Users/chasinglogic/Code/go/src/github.com/josharian/impl
  /Users/chasinglogic/Code/go/src/github.com/jstemmer/gotags
  /Users/chasinglogic/Code/go/src/github.com/dgrijalva/jwt-go
  /Users/chasinglogic/Code/go/src/github.com/fatih/motion
  /Users/chasinglogic/Code/go/src/github.com/fatih/gomodifytags
  /Users/chasinglogic/Code/go/src/github.com/dominikh/go-tools
  /Users/chasinglogic/Code/go/src/github.com/goreleaser/goreleaser
  /Users/chasinglogic/Code/go/src/github.com/goreleaser/archive
  /Users/chasinglogic/Code/go/src/github.com/nsf/gocode
  /Users/chasinglogic/Code/go/src/github.com/davidrjenni/reftools
  /Users/chasinglogic/Code/go/src/github.com/alecthomas/gometalinter
  /Users/chasinglogic/Code/go/src/github.com/kisielk/errcheck
  /Users/chasinglogic/Code/go/src/github.com/kisielk/gotool
  /Users/chasinglogic/Code/go/src/github.com/zmb3/gogetdoc
  /Users/chasinglogic/Code/go/src/github.com/google/go-github
  /Users/chasinglogic/Code/go/src/github.com/google/go-querystring
  /Users/chasinglogic/Code/go/src/github.com/golang/lint
  /Users/chasinglogic/Code/go/src/github.com/rogpeppe/godef
  /Users/chasinglogic/Code/go/src/github.com/apex/log
  /Users/chasinglogic/Code/go/src/github.com/urfave/cli
  /Users/chasinglogic/Code/go/src/github.com/klauspost/asmfmt
  /Users/chasinglogic/Code/go/src/github.com/gorilla/mux
  /Users/chasinglogic/Code/go/src/golang.org/x/sync
  /Users/chasinglogic/Code/go/src/golang.org/x/oauth2
  /Users/chasinglogic/Code/go/src/golang.org/x/net
  /Users/chasinglogic/Code/go/src/golang.org/x/tools
  /Users/chasinglogic/Code/go/src/golang.org/x/crypto
  /Users/chasinglogic/Code/go/src/honnef.co/go/tools
  /Users/chasinglogic/Code/go/src/gopkg.in/mgo.v2
> /Users/chasinglogic/Code/go/src/gopkg.in/yaml.v2
  56/56
>
```

and you can search and select using FZF's awesome interface:

```
> /Users/chasinglogic/Code/archive/projector
  /Users/chasinglogic/Code/python/projector
  /Users/chasinglogic/Code/projector
  3/56
> projector
```

Enter and voila!:

```
Users/chasinglogic λ sp
Code/projector master λ pwd
/Users/chasinglogic/Code/projector
Code/projector master λ
```

### Find

Find searches your project list for the given regex and returns the project who
has the rightmost match of that search.

For example let's say you have 4 projects:

```
/Users/mathewrobinson/Code/cdb
/Users/mathewrobinson/Code/homelab
/Users/mathewrobinson/Code/taskforge
/Users/mathewrobinson/Code/taskforge.old
```

If we run `projector find taskforge` here we will get
`/Users/mathewrobinson/Code/taskforge` because in that case the match of
`taskforge` is literally at the end (i.e. rightmost). If we instead wanted to
find `/Users/mathewrobinson/Code/taskforge.old` we could run `projector find
--reverse taskforge` which will find the leftmost match instead.

This can lead to really handy project switching shell functions such as the one
I have:

```bash
function sp() {
    cd $(projector find "$1")
}
```

Because find fully supports regex as provided by the regex crate you can also
make this case insensitive:

```bash
function sp() {
    cd $(projector find "(?i)$1")
}
```

Now if you source your bashrc and type `sp $name_of_a_project` you will
instantly be transported to your project directory. For example:

```
Users/chasinglogic λ . .bashrc
Users/chasinglogic λ sp projector
Code/projector master λ sp dfm
chasinglogic/dfm master λ pwd
/Users/chasinglogic/Code/go/src/github.com/chasinglogic/dfm
chasinglogic/dfm master λ
```
### Run

Run allows you to run shell commands in all of your projects. For example if you
wanted to run git status on every project:

```
Code/projector master λ projector run git status
nothing to commit, working tree clean
On branch emacs-26
Your branch is up-to-date with 'origin/emacs-26'.

Untracked files:
  (use "git add <file>..." to include in what will be committed)

	update.sh

nothing added to commit but untracked files present (use "git add" to track)
On branch master
Your branch is up-to-date with 'origin/master'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

	modified:   Cargo.lock

no changes added to commit (use "git add" and/or "git commit -a")
On branch master
Your branch is up-to-date with 'origin/master'.

nothing to commit, working tree clean
On branch master
Your branch is up-to-date with 'origin/master'.

nothing to commit, working tree clean
Code/projector master λ
```

Any flags you pass after the program will get passed to the program so you can
type the command just like you would normally, no weird shell quoting! (ok maybe
just the regular amount of weird shell quoting.)


## Configuration

Projector does not require any setup to use provided that you either have your
code in `~/Code` or set the `$CODE_DIR` environment variable. However every
good CLI does provide some configuration for power users, so of course
projector does as well. 

The config file should be located in `$HOME/.projector.yml` and can contain
3 keys: `code_dirs`, `excludes`, and `includes`. Here is my config file as an
example:

```yaml
code_dirs:
  - ~/Code
  - ~/Work
  - ~/kde/src
  - ~/.config/dfm/profiles

excludes:
  - go/pkg
  - go/src

includes:
  - go/src/github\.com/chasinglogic
```

`code_dirs` allows you to specify more than one root directory for projector to
search.

`excludes` is a list of regex patterns that if matched will cause a directory to
be excluded from the projects list and not searched further.

`includes` is a list of regex patterns that overrides an exclude. So if an
exclude pattern and an include pattern match a directory it will be searched or
included in the project list.

All keys besides `code_dirs` are optional so you can omit any which you do not want to
configure.

### Configuration Example

Primarily the configuration centers around inclusion and exclusion of project
directories. For example I have a huge amount of go repos in my `$GOPATH`:

```
/Users/mathewrobinson/Code/go/src/github.com/goreleaser/nfpm
/Users/mathewrobinson/Code/go/src/github.com/goreleaser/archive
/Users/mathewrobinson/Code/go/src/github.com/goreleaser/goreleaser
/Users/mathewrobinson/Code/go/src/github.com/dominikh/go-tools
/Users/mathewrobinson/Code/go/src/github.com/ramya-rao-a/go-outline
/Users/mathewrobinson/Code/go/src/github.com/fatih/gomodifytags
/Users/mathewrobinson/Code/go/src/github.com/fatih/color
/Users/mathewrobinson/Code/go/src/github.com/fatih/motion
/Users/mathewrobinson/Code/go/src/github.com/evergreen-ci/evergreen
/Users/mathewrobinson/Code/go/src/github.com/derekparker/delve
/Users/mathewrobinson/Code/go/src/github.com/mdempsky/unconvert
/Users/mathewrobinson/Code/go/src/github.com/jstemmer/gotags
/Users/mathewrobinson/Code/go/src/github.com/josharian/impl
/Users/mathewrobinson/Code/go/src/github.com/MichaelTJones/walk
/Users/mathewrobinson/Code/go/src/github.com/mattn/go-isatty
/Users/mathewrobinson/Code/go/src/github.com/mattn/go-zglob
/Users/mathewrobinson/Code/go/src/github.com/mattn/go-colorable
/Users/mathewrobinson/Code/go/src/github.com/uudashr/gopkgs
/Users/mathewrobinson/Code/go/src/github.com/chasinglogic/licensure
... (list truncated for brevity)
```

Of which most are not mine so I don't want them to show up in my projector
output or be used when I run `projector run`. I could use the `--exclude`
flag which supports a regular expression as accepted by the
[regex](https://docs.rs/regex/1.0.1/regex/#syntax) crate to exclude the
go directories. Something like:

```
projector --exclude '.*go.*' list
```

But that would also exclude my go source directories. Using the `--include`
flag I can add a regex which will include a directory if it matches the include
regex and the exclude regex. This feature exists because the regex crate
does not support look-ahead/behind. So the new command is:

```
projector --exclude '.*go.*' --include '.*github.com/chasinglogic.*' list
```

This is a pretty tiresome command to type every time so you can make an alias, or
create a config file at `~/.projector.yml` that looks like this:

```yaml
---
code_dirs: 
    - ~/Code
includes:
    - .*github.com/chasinglogic.*
    - .*github.com/mongodb.*
excludes:
    - .*go.*
```

This does the same thing. In this config file includes and excludes are lists
of regexes which will be or'd together. Anything that matches an exclude
pattern will be excluded unless it also matches an include pattern.

code\_dir is required inside of the config file.



## License

```
Copyright 2018 Mathew Robinson <chasinglogic@gmail.com>. All rights reserved.
Use of this source code is governed by the GPLv3 license that can be found in
the LICENSE file.
```
