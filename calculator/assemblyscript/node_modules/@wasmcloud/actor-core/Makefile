.PHONY: all deps codegen clean doc test

all: deps codegen

deps:
	npm install

codegen:
	wapc generate codegen.yaml

clean:
	rm -Rf build

doc:

test: build
	npm run test
