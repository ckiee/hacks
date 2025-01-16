#!/bin/sh
#nodemon -x ./deploy.sh
xargs -a index.js -0 -I{} echo "update users set custom_js='{}' where id=1;" | ssh flowe "sudo -u postgres psql -d miniflux"
