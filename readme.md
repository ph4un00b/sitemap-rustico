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

## ✨ Agregando un nuevo libro:

- obtener los path que van ser indexados, por ejemplo:
  - para el libro _rust para dotnet devs_:
    - clonar el repositorio `https://github.com/RustLangES/rust-para-dotnet-devs.git`
    - asignarle un nombre de una sola palabra `git clone https://github.com/RustLangES/rust-para-dotnet-devs.git dotnet`
    - obtener los path de las nuevas páginas para ser indexadas.
      - trae las archivos de forma recursiva: `git ls-tree -r --name-only HEAD --full-tree src/es`
      - filtramos y formato: `grep "\.md$" | awk -F "." "{print $1}"`
      - podes ver un ejemplo para bash o batch en `/scripts`
    - agregar al `ignore_paths.rs` los path que no tienen sentido como:
      - `dotnet/src/es/SUMMARY`,
      - `dotnet/src/es/license`

## 🧪 testing workflow

- deps:
  - docker
  - gh
  - act
- act.exe -j test -s GITHUB_TOKEN="$(gh auth token)"

## 🍗 hard todo

- [ ] remove xmllint
