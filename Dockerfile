FROM ubuntu

PREPARE ./examples/packages.md

INSTALL ./examples/packages.md

RUN vim
