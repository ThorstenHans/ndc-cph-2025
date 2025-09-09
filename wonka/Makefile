.PHONY build:
build:
	source ./.venv/bin/activate; \
	pip install -r requirements.txt; \
	rm -rf wit_world; \
	componentize-py -d ./../wit -w consumer bindings . ;\
	componentize-py -d ./../wit -w consumer componentize -m spin_sdk=spin-http app -o app.wasm ;\
	deactivate ;\
	wac plug --plug ./../signing/target/wasm32-wasip2/release/signing.wasm ./app.wasm -o ./composed.wasm 
