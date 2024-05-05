#!/bin/bash

# Switch directory to base directory
cd ../../../

# Update system packages
sudo apt update
sudo apt upgrade -y

# Prepare setup to docker installation
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update

# Install docker
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# Pull my minecraft server modded docker image
sudo docker pull orizzo/minecraft-server-new:latest
# Pull minecraft server backup image
sudo docker pull itzg/mc-backup

# Install my rust program
wget https://github.com/ORizzo/rust-minecraft-server/releases/download/test2/hello-rust

# Turn rust program into executable
chmod +x ./hello-rust

# Run rust program 
./hello-rust

# Change to minecraft server 
cd ./minecraft-server

# Compose minecraft server containers
sudo docker compose up -d

# OLHAR EM QUAL VOLUMES OS CONTAINERS ESTÃO MONTANDO OS DIRETÓRIOS