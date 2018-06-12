www:
	cd web
	cargo +nightly web deploy --release
	cp -r target/deploy ../deploy
	cd ..
	git checkout www
	rm * 2> /dev/null
	mv deploy/* .
	rm -rf deploy
	git add *.html *.js *.css *.wasm
	git commit --no-verify
