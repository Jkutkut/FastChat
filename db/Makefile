include .env
include .env_controller

SECRETS_DB_SCRIPT	=	./.scripts/setsecrets_db.sh

SECRETS_DB_C_SCRIPT	=	./.scripts/setsecrets_db_controller.sh

usage:
	@echo "Usage:"
	@echo "make install        -> Creates DB container"
	@echo "make db_controller  -> Creates DB controller container"
	@echo "make run            -> Starts the already created DB"
	@echo "make runController  -> Starts the controller"
	@echo "make runAll         -> Starts the DB and the controller"
	@echo "make stop           -> Stops DB container"
	@echo "make stopController -> Stops DB controller container"
	@echo "make stopAll        -> Stops both containers"
	@echo "make uninstall      -> Removes the DB container"
	@echo "make dockerls       -> Check current state of dockers"

# Config files

.env: ${SECRETS_DB_SCRIPT}
	@bash ${SECRETS_DB_SCRIPT}

.env_controller: ${SECRETS_DB_C_SCRIPT}
	@bash ${SECRETS_DB_C_SCRIPT}

# Commands

install:
	docker run -d --name ${DB_NAME} -p ${DB_PORT}:5432 -e POSTGRES_PASSWORD=${DB_USR_PASSWD} -e POSTGRES_USER=${DB_USR} -e POSTGRES_DB=postgres postgres:latest

db_controller:
	docker run -d --name ${WEB_CONTROLLER_NAME} -p ${WEB_CONTROLLER_PORT}:80 -e PGADMIN_DEFAULT_EMAIL="${WEB_CONTROLLER_EMAIL}" -e PGADMIN_DEFAULT_PASSWORD="${WEB_CONTROLLER_PASSWD}" dpage/pgadmin4

run:
	docker start ${DB_NAME}

runController:
	docker start ${WEB_CONTROLLER_NAME}

runAll: run runController

stop:
	docker stop ${DB_NAME}

stopController:
	docker stop ${WEB_CONTROLLER_NAME}

stopAll: stop stopController

uninstall: stop
	docker rm ${DB_NAME}

dockerls:
	docker ps -a
	@echo
	@echo
	@echo "docker images"
	docker images
	@echo
	@echo
	docker volume ls
