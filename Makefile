CARGO ?= cargo
GO ?= go
GOMD2MAN ?= $(shell command -v go-md2man || echo '$(GOPATH)/bin/go-md2man')
SELINUXOPT ?= $(shell test -x /usr/sbin/selinuxenabled && selinuxenabled && echo -Z)

define go-get
	GO111MODULE=off go get -u $1
endef

.gopath:
ifeq ($(GOPATH),)
export GOPATH := $(shell $(GO) env GOPATH)
endif

.PHONY: build
build:
	cargo build --release

.PHONY: docs
docdir:
	mkdir -p docs/build/man

.PHONY: install.tools
install.tools: .install.md2man

.PHONY: .install.md2man
.install.md2man: .gopath
	if [ ! -x "$(GOMD2MAN)" ]; then \
	    $(call go-get,github.com/cpuguy83/go-md2man); \
	fi

MANPAGES_MD ?= $(wildcard docs/*.md)
MANPAGES ?= $(MANPAGES_MD:%.md=%)

$(MANPAGES): %: %.md .install.md2man docdir
	@sed -e 's/\((grpc-health-check.*\.md)\)//' -e 's/\[\(grpc-health-check.*\)\]/\1/' $< | $(GOMD2MAN) -in /dev/stdin -out $(subst docs,docs/build/man,$@)

.PHONY: docs
docs: $(MANPAGES)

.PHONY: install
install: install.bin install.man

.PHONY: install.bin
install.bin:
	install ${SELINUXOPT} -d -m 755 $(DESTDIR)${PREFIX}/bin
	install ${SELINUXOPT} -m 755 target/release/grpc-health-check $(DESTDIR)${PREFIX}/bin/grpc-health-check
	test -z "${SELINUXOPT}" || chcon --verbose --reference=$(DESTDIR)${PREFIX}/bin/grpc-health-check target/release/grpc-health-check

.PHONY: install.man
install.man:
	install ${SELINUXOPT} -d -m 755 $(DESTDIR)${PREFIX}/share/man/man1
	install ${SELINUXOPT} -m 644 docs/build/man/*.1 -t $(DESTDIR)${PREFIX}/share/man/man1

.PHONY: clean
clean:
	rm -rf \
		docs/build \
		target