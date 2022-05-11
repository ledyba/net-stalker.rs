########################################################################################################################
## build
########################################################################################################################

.PHONY: FORCE
FORCE: ;

.PHONY: build
build: FORCE
	UID=$(shell id -u) GID=$(shell id -g) \
		docker-compose build

########################################################################################################################
## DB
########################################################################################################################

.PHONY: up
up:
	UID=$(shell id -u) GID=$(shell id -g) docker-compose up -d

.PHONY: run
run:
	UID=$(shell id -u) GID=$(shell id -g) docker-compose run --rm --entrypoint sh web

.PHONY: down
down:
	UID=$(shell id -u) GID=$(shell id -g) docker-compose down

.PHONY: log
log:
	docker-compose logs -f --tail 0

.PNONY: reload
reload:
	$(MAKE) down
	$(MAKE) up
