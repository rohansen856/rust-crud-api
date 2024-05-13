.DEFAULT_GOAL := watch
run:
	cargo run
watch:
	npx nodemon --watch src -e rs --exec cargo run