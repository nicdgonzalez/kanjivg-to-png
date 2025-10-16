./data/kanji_output: ./target/release/kanjivg-to-png ./data/kanji
	mkdir --parents logs
	python3 ./scripts/main.py --input ./data/kanji --output ./data/kanji_output 2>> ./logs/main.log

./target/release/kanjivg-to-png:
	cargo build --release

./data/kanji:
	bash ./scripts/get-kanjivg-data.sh ./data

clean:
	rm -r ./data ./target
