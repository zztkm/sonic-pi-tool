Sonic Pi Tools
==============

```sh
# Install go and the glide package manager.
brew install go glide

# Install the deps
glide install

make test       # Run the tests
make build      # Build the binaries
make clean      # Clean the build dir
make clean-deps # Clean the vendor dir
```
