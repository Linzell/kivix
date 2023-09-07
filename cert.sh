#!/bin/sh

# Load environment variables
[ ! -f .env ] || export $(grep -v '^#' .env | xargs)

# Set the server name
if [ -z "$1" ]; then
    SERVER="localhost"
else
    SERVER="$1"
fi

# Generate the certificate
openssl req -x509 -nodes  \
  -newkey rsa:4096  \
  -days 365  \
  -subj "/CN=$SERVER"  \
  -out cert.pem  \
  -keyout key.pem  \
  -sha256
openssl rsa -in key.pem -out nopass.pem
rm key.pem
mv nopass.pem key.pem