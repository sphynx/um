SHELL = /bin/bash

.PHONY: run run2

UM = target/release/um

all: score

extract: $(UM)
	cat key extract.script | $(UM) codex.umz > out

umix: $(UM)
	$(UM) prog.um

hack:
	cat hack1.script hack2.bas hack2.script - | $(UM) prog.um

adventure:
	cat adventure.script - | $(UM) prog.um

ohmega:
	cat ohmega.script - | $(UM) prog.um

ftd:
	cat ftd.script - | $(UM) prog.um

hmonk:
	cat hmonk.script - | $(UM) prog.um

yang:
	cat yang.script - | $(UM) prog.um

score:
	cat score.script publications.txt <(printf "\n") | $(UM) prog.um
