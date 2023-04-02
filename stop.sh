#!/bin/bash

session="pi_webserver_personal"

tmux kill-session -t $session
echo "Killed HTTP server"
