.ONESHELL: 
.PHONY: debpackage
debpackage:
	cd ./python-socks-proxy
	dpkg-buildpackage -us -uc
	cd ..
	mkdir -p ./release
	mv  dfinity-socks-01_0.0.1_amd64.buildinfo  dfinity-socks-01_0.0.1.dsc  dfinity-socks-01_0.0.1_amd64.changes dfinity-socks-01_0.0.1.tar.xz python-socks5-server_0.0.1_amd64.deb release/

clean:
	rm -rf ./release 

buildenv:
	sudo apt-get -y install dh-systemd
