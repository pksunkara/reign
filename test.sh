set -e

for dir in $(find examples -maxdepth 2 -mindepth 2 -type d); do
	cd $dir
	cargo test
	cd ../../..
done
