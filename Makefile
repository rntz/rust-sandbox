RUSTCFLAGS:=-g -L .

%: %.rs
	rustc $(RUSTCFLAGS) $<

.PHONY: rutil
rutil:
	rustc $(RUSTCFLAGS) rutil.rc

.PHONY: clean
clean:
	rm -f $(basename $(wildcard *.rs))
	rm -f lib*.so
