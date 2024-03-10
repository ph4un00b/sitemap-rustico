## 🍕 diagrama general

![diagrama](diagrama.png)

- antes de crear algún cambio modificar el diagrama❗
- [https://excalidraw.com/#json=huU3UCnJK3nJJBUWd2EQ8,WGBe0SMz9VYgMuXIIgNZdg](https://excalidraw.com/#json=huU3UCnJK3nJJBUWd2EQ8,WGBe0SMz9VYgMuXIIgNZdg)

## 🍔 Instructions

- fetch below repositories:
  - ```sh
    git clone https://github.com/RustLangES/blog.git
    git clone https://github.com/RustLangES/RustLangES.github.io.git home
    git clone https://github.com/RustLangES/rust-book-es.git book
    ```
- Sólo elegís el script para tu sistema, no tengas miedo de otros scripts de otros sistemas operativos.
  - windows: `generate.bat`
  - unix: `generate.sh`

## 🧪 testing workflow

- deps:
  - docker
  - gh
  - act
- act.exe -j test -s GITHUB_TOKEN="$(gh auth token)"

## 🍗 hard todo

- [ ] remove xmllint
