SRC_FILES := $(shell find . -name '*.go' | grep -v vendor)
GOOS := $(shell go env | grep GOOS | sed 's/GOOS=//')
GOARCH := $(shell go env | grep GOARCH | sed 's/GOARCH=//')
INSTALL_DIR := /usr/local/bin

build: clean cli

clean:
	rm -rf ./dist ./build

lint:
	gometalinter $(SRC_FILES)

test:
	go test -v ./...

dist:
	mkdir -p dist

projector: build/projector
cli: build/projector
build/projector: build
	go build -o ./build/projector .

install: package install-artifacts

install-snapshot: snapshot install-artifacts

install-artifacts:
	cp dist/${GOOS}_${GOARCH}/projector /usr/local/bin/

snapshot: clean
	goreleaser release --skip-publish --snapshot

package: clean
	goreleaser release --skip-publish

tag-%:
	git tag v$*
	git push --tags

publish:
	goreleaser

release-%: clean tag-% publish
