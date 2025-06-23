#!/bin/bash

cargo build --release

session="pi_webserver_personal"
tmux new-session -d -s $session

window=0
tmux rename-window -t $session:$window 'run'
# tmux send-keys -t $session:$window "authbind --deep target/release/mndco11age" C-m
tmux send-keys -t $session:$window "sudo target/release/mndco11age" C-m


echo "HTTPS server running on port 443"

