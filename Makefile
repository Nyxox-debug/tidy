reset:
	rm -rf testfolder && mkdir testfolder
	touch testfolder/photo.jpg testfolder/resume.pdf testfolder/notes.txt

run: reset
	cargo run -- ./testfolder

.PHONY: reset run
