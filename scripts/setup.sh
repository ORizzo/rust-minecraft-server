#!/bin/bash

# Switch directory to base directory
cd ../../../

# Update system packages
sudo apt update
sudo apt upgrade

# Prepare setup to docker installation
sudo apt-get install  curl apt-transport-https ca-certificates software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt update
apt-cache policy docker-ce

# Install docker 
sudo apt install docker-ce

# Install wine (used to execute .exe on linux distros)
sudo apt-get install wine

# Install my rust program
wget https://github.com/ORizzo/rust-minecraft-server/releases/download/test2/hello-rust.exe
wine hello-rust.exe