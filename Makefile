www:
	cd web && cargo +nightly web deploy --release
	cp -r web/target/deploy/ deploy/ && \
	git checkout www && \
	find . -type f -maxdepth 1 -exec rm {} \; && \
	mv deploy/* . && \
	rm -rf deploy && \
	git add *.html *.js *.css *.wasm && \
	git commit --no-verify -m "Automated deploy of $$(git describe --tags master)."