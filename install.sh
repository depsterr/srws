#!/bin/sh
sudo cargo install --path . --root /
sudo cp srws.service /etc/systemd/system/
