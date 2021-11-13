.ONESHELL: 
.PHONY: debpackage
debpackage:
	cd ./dfinity-socks-01
	dpkg-buildpackage -us -uc

