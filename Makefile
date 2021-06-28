cargo=cargo.exe
release_path=target/release

clippy:
	find -path ./target -prune -o -iname "*.rs" -exec touch "{}" \; && $(cargo) fmt --all && $(cargo) clippy

release:
	$(cargo) test
	$(cargo) build --release
	rm -r $(release_path)/databases && cp -r databases $(release_path)
	cd $(release_path) && mkdir trilogy-save-editor && cp -r databases trilogy_save_editor.exe trilogy-save-editor
	cd $(release_path) && 7z a ../../trilogy-save-editor.zip trilogy-save-editor -mx=7
	rm -r $(release_path)/trilogy-save-editor/