#!/bin/bash

# Switch directory to base directory
cd ../../../

# Update system packages
sudo apt update
sudo apt upgrade -y

# Install docker 
sudo apt install docker.io -y
sudo apt install docker-compose -y

# Pull my minecraft server modded docker image
sudo docker pull orizzo/minecraft-server-new:latest
# Pull minecraft server backup image
sudo docker pull itzg/mc-backup

# Install my rust program
wget https://github.com/ORizzo/rust-minecraft-server/releases/download/test2/hello-rust

# Run rust program 
hello-rust

# Change to minecraft server 
cd ./minecraft-server

# Compose minecraft server containers
docker-compose up -d

# OLHAR EM QUAL VOLUMES OS CONTAINERS ESTÃO MONTANDO OS DIRETÓRIOS