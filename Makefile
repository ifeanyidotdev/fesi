run-test-endpoint:
	cargo run -- run -m GET -e https://dev-api-nukord.nuxalle.com/health/ 

file:
	cargo run -- file ./act.yaml
