localhost.pfx: localhost.crt
	openssl pkcs12 -export -nodes -inkey localhost.key -in localhost.crt -out localhost.pfx

localhost.crt: ssl.conf
	openssl req -nodes -x509 -newkey rsa:2048 -config ssl.conf -extensions ext -subj /C=CA/ST=EH/L=Canadia/O=Dis/CN=localhost -keyout localhost.key -out localhost.crt -days 365

clean:
	rm -f localhost.pfx localhost.crt localhost.key

