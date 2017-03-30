# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  # All Vagrant configuration is done here. The most common configuration
  # options are documented and commented below. For a complete reference,
  # please see the online documentation at vagrantup.com.

  # Every Vagrant virtual environment requires a box to build off of.
  config.vm.box = "ubuntu/trusty64"
  # config.vm.box_version = '0.1.0'

  config.ssh.forward_agent = true

  config.vm.define "client" do |host|
    host.vm.network :private_network, ip: "192.168.48.11"
    host.vm.provider :virtualbox do |vb|
      vb.name = "rust-ssl-client"
      vb.customize ["modifyvm", :id, "--memory", "1024"]
    end

    host.vm.provision :shell, privileged: false do |host|
      host.inline = <<-EOS
        sudo apt-get install -y pkg-config libssl-dev
        echo "192.168.48.10 rustsslserver" | sudo tee -a /etc/hosts
        curl https://sh.rustup.rs -sSf | sh -s -- -y
      EOS
    end
  end

  config.vm.define "server" do |host|
    host.vm.network :private_network, ip: "192.168.48.10"
    host.vm.provider :virtualbox do |vb|
      vb.name = "rust-ssl-server"
      vb.customize ["modifyvm", :id, "--memory", "1024"]
    end

    host.vm.provision :shell, privileged: true do |host|
      host.inline = <<-EOS
        apt-get install -y nginx

        rm -f /etc/nginx/sites-available/default
        cp /vagrant/nginx-site /etc/nginx/sites-available/default

        mkdir -p /etc/nginx/html
        echo "hi there" > /etc/nginx/html/index.html

        service nginx restart
      EOS
    end
  end
end
