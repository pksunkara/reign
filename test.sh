set -e

# TODO: Make workflow
for dir in $(find examples -maxdepth 2 -mindepth 2 -type d); do
	cargo test --target-dir target --manifest-path $dir/Cargo.toml
done
