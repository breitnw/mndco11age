sudo gunicorn 'app:create_app()' -w 2 --certfile '/etc/letsencrypt/live/mndco11age.xyz/fullchain.pem' --keyfile '/etc/letsencrypt/live/mndco11age.xyz/privkey.pem' -b 0.0.0.0:443
