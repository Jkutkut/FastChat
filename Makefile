.PHONY: server

server:
	docker build -t fastchat server

run-server:
	docker run --name fastchat -it -p 4242:4242 --rm fastchat
	@#docker run --name fastchat -it --rm fastchat

stop-server:
	docker kill fastchat

# ********* Server triggers *********

UUID ?= $(shell bash -c 'read -p "uuid: " uuid; echo $$uuid')
uwsc:
	@#read -p "uuid: " r
	@#echo "uwsc ws://127.0.0.1:4242/ws/${r}"
	uwsc ws://127.0.0.1:4242/ws/${UUID}

addPepe:
	curl -X "POST" 'http://localhost:4242/add_usr' -H 'Content-Type: application/json' -d '{"name": "pepe"}'

addPaco:
	curl -X "POST" 'http://localhost:4242/add_usr' -H 'Content-Type: application/json' -d '{"name": "paco"}'
