ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: tt
tt: 
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 18384526 -t +10 

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run map_block -e eth.substreams.pinax.network:9000 -s -1000 -o jsonl

.PHONY: gui
gui:
	substreams gui map_block -e eth.substreams.pinax.network:9000 -s 447766

.PHONY: deploy
deploy:
	graph deploy --studio erc-20
