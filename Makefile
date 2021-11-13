.ONESHELL: 
.PHONY: debpackage
debpackage:
	cd ./python-socks-proxy
	dpkg-buildpackage -us -uc
	cd ..
	mkdir -p ./release
	mv  python-socks-proxy_0.0.1_amd64.buildinfo  python-socks-proxy_0.0.1.dsc  python-socks-proxy_0.0.1_amd64.changes python-socks-proxy_0.0.1.tar.xz python-socks-proxy_0.0.1_amd64.deb release/

clean:
	rm -rf ./release
	cd ./python-socks-proxy
	dpkg-buildpackage -rfakeroot -Tclean

buildenv:
	sudo apt-get -y install dh-systemd
