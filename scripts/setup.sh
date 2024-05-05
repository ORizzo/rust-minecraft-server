#!/bin/bash

# Switch directory to base directory
cd ../../../

# Update system packages
sudo apt update -y
sudo apt upgrade -y

sleep 20

# Install docker
sudo snap install docker

sleep 20

# Pull my minecraft server modded docker image
sudo docker pull orizzo/minecraft-server-new:latest
# Pull minecraft server backup image
sudo docker pull itzg/mc-backup

# Switch to root path
cd ~

# Create minecraft server folder
mkdir ./minecraft-server

# Change to minecraft server 
cd ./minecraft-server

# Get docker-compose.yml file
wget https://4m-discord-bot.b-cdn.net/docker-compose.yml

sleep 20

# Compose minecraft server containers
sudo docker compose up -d --wait

# OLHAR EM QUAL VOLUMES OS CONTAINERS ESTÃO MONTANDO OS DIRETÓRIOS