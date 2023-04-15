#!/bin/bash

session="pi_webserver_personal"
tmux new-session -d -s $session

window=0
tmux rename-window -t $session:$window 'run'
tmux send-keys -t $session:$window "cargo run --release" C-m

echo "HTTPS server running on port 443"

