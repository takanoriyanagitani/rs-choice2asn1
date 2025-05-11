#!/bin/sh

node sample.mjs |
	xxd -ps |
	tr -d '\n' |
	python3 \
		-m asn1tools \
		convert \
			-i der \
			-o jer \
			./flat.asn \
			FlatValue \
			-
