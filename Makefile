UUID ?= $(shell bash -c 'read -p "uuid: " uuid; echo $$uuid')
uwsc:
	@#read -p "uuid: " r
	@#echo "uwsc ws://127.0.0.1:4242/ws/${r}"
	uwsc ws://127.0.0.1:4242/ws/${UUID}

addPepe:
	curl -X "POST" 'http://localhost:4242/add_usr' -H 'Content-Type: application/json' -d '{"name": "pepe"}'

addPaco:
	curl -X "POST" 'http://localhost:4242/add_usr' -H 'Content-Type: application/json' -d '{"name": "paco"}'

rmPaco:
	curl -X "DELETE" 'http://localhost:4242/rm_usr' -H 'Content-Type: application/json' -d '{"name": "paco"}'
