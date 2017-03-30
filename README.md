This repository is to help solve a [StackOverflow question](http://stackoverflow.com/questions/43016684/tokio-tls-self-signed-certificate) about using Rust to make HTTP requests to sites with self-signed certificates.
It sets up a basic server with HTTPS,
and includes a Rust client program to talk to it.
To set up the server and client machines:

    make
    vagrant up

You should be able to see that the server is listening:

    curl https://192.168.48.10/      # error: the cert is self signed
    curl -k https://192.168.48.10/   # ok

Those should give the same results whether you run them from the client VM or the host.

Now use `vagrant ssh client` to get on the client machine,
and do this:

    cd /vagrant/client
    cargo run

You should get an error like this:

    thread 'main' panicked at 'Sending request failed: Ssl(Failure(MidHandshakeSslStream { stream: SslStream { stream: HttpStream(_), ssl: Ssl { state: "SSLv3 read server certificate B", verify_result: X509VerifyError { code: 18, error: "self signed certificate" } } }, error: Ssl(ErrorStack([Error { code: 336134278, library: "SSL routines", function: "SSL3_GET_SERVER_CERTIFICATE", reason: "certificate verify failed", file: "s3_clnt.c", line: 1186 }])) }))', /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libcore/result.rs:868

But then do this to trust the certificate:

    sudo cp /vagrant/localhost.crt /usr/local/share/ca-certificates/
    sudo /usr/sbin/update-ca-certificates

Now run the rust client again:

    cargo run

And it should work!

Note that the cert's Common Name (CN) must be the hostname the client uses to make the request.
Here we use `rustsslserver` and put that into the client's `/etc/hosts`,
but if you have a domain name you control you could use that instead.

