default: watch

build:
    typst compile main.typ
    zathura main.pdf &

watch: noview-build
    zathura main.pdf &
    typst watch main.typ

clean:
    rm -rf *.pdf

[private]
noview-build:
    typst compile main.typ
