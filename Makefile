JAVA_SRC_FILES=$(wildcard tests/*.java)

.PHONY: test clean

test:
	javac $(JAVA_SRC_FILES)
	cargo test

clean:
	rm -f tests/*.class
