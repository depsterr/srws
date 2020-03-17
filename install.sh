#!/bin/sh
sudo cargo install --path . --root / --force
sudo cp srws.service /etc/systemd/system/
sudo touch /etc/srws.conf
