## diagrama
- [https://excalidraw.com/#json=3WjyGq26navNFA_6ST0Fd,MtXXo_sCj0VeW6OfTswl-g](https://excalidraw.com/#json=3WjyGq26navNFA_6ST0Fd,MtXXo_sCj0VeW6OfTswl-g)


## generate
- ```sh
  git clone https://github.com/RustLangES/blog.git
  git clone https://github.com/RustLangES/RustLangES.github.io.git home
  git clone https://github.com/RustLangES/rust-book-es.git book
  ```
- windows: `generate.bat`
- unix: `generate.sh`

## testing workflow
- deps:
  - docker
  - gh
  - act
- act.exe -j test -s GITHUB_TOKEN="$(gh auth token)"

## external deps for debug
- cat
- awk
- grep
- xargs
- windows: you can get plenty of unix utils from busybox
    - `scoop install busybox`

## external deps for scrapper
- wget
- grep
- awk
- xmllint from libxml2

## tested on
- [x] windows 10
    - scoop
- [ ] macos
    - brew
- [ ] linux

## ez todo
- [ ] remove duplicates
- [ ] remove grep
- [ ] remove awk

## hard todo
- [ ] remove wget
- [ ] remove xmllint
