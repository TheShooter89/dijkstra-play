run:
	dx serve

watch:
	dx serve

test:
	watchexec --notify -r -- cargo test

doc:
	cargo doc && python3 -m http.server 1989 --directory target/doc

bootstrap:
	# 
	mkdir assets/daisyui && cd assets/daisyui && curl -sL daisyui.com/fast | bash && cd - \
		&& ./assets/daisyui/tailwindcss -i tailwind.css -o assets/tailwind.css \
		&& dx serve

# for this command refer to daisyUI installation docs https://daisyui.com/docs/install/standalone/#build-css
css:
	./assets/daisyui/tailwindcss -i tailwind.css -o assets/tailwind.css --watch

css-build:
	./assets/daisyui/tailwindcss -i tailwind.css -o assets/tailwind.css

# for this command refer to daisyUI installation docs https://daisyui.com/docs/install/standalone/
# section 'Fast install'
daisyui-install:
	mkdir assets/daisyui && cd assets/daisyui && curl -sL daisyui.com/fast | bash && cd -

example:
	watchexec --notify -r -- cargo run --example simple
