PROJECT = kafka
NAME = "Kafka Messaging"
VENDOR = "wasmCloud"
CAPABILITY_ID = wasmcloud:messaging
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
oci_url  = localhost:5000/v2/$(PROJECT):$(VERSION)

include provider.mk

create-topic: ## Creates a topic for testing
	kafka-topics --create  --replication-factor 1 --partitions 1 --topic my-topic --bootstrap-server localhost:9092
