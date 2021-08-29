watch_src() {
	watch -g ls -lRa src >/dev/null
	clear
	echo ------ File change detected, running \`cargo run\` ------
	echo
	cargo run
	exitCode=$?
	echo
	echo ------ Exited with code $exitCode ------
	watch_src
}

echo Starting...
echo

cargo run

echo "
------ Exited with code $? ------"
watch_src
