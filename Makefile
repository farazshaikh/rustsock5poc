.ONESHELL: 
.PHONY: debpackage
debpackage:
	cd ./dfinity-socks-01
	dpkg-buildpackage -us -uc

clean:
	rm -rf dfinity-socks-01_0.0.1_amd64.changes dfinity-socks-01_0.0.1_amd64.buildinfo  dfinity-socks-01_0.0.1.dsc  dfinity-socks-01_0.0.1_amd64.changes dfinity-socks-01_0.0.1.tar.xz python-socks5-server_0.0.1_amd64.deb
