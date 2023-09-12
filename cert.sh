#!/bin/bash

# Load environment variables
[ ! -f .env ] || export $(grep -v '^#' .env | xargs)

# Set the server name
if [ -z "$1" ]; then
    SERVER="localhost"
else
    SERVER="$1"
fi

# Generate the certificate authority (CA)
# If not found cert.pem
if [[ -f ca.pem ]]; them
else
openssl req -x509 -nodes  \
  -newkey rsa:4096  \
  -days 365  \
  -subj "/C=CA/ST=None/L=Earth/O=${DB_NAMESPACE}/OU=${DB_USERNAME}/CN=$SERVER"  \
  -out ca.pem  \
  -keyout ca.key  \
  -sha256
fi

# Generate the client certificate
openssl req -nodes   \
  -newkey rsa:2048   \
  -keyout key.pem \
  -out "$SERVER".csr    \
  -subj "/C=CA/ST=None/L=Earth/O=${DB_NAMESPACE}/OU=${DB_USERNAME}/CN=$SERVER:${PORT}"
openssl x509 -req \
  -CA ca.pem  \
  -CAkey ca.key  \
  -in "$SERVER".csr \
  -out cert.pem  \
  -days 365 \
  -CAcreateserial \
  -extfile <(cat <<END
basicConstraints = CA:FALSE
nsCertType = server
nsComment = "OpenSSL Generated Server Certificate"
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid,issuer
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names
[alt_names]
DNS.1 = $SERVER:${PORT}
DNS.2 = *.$SERVER:${PORT}
END
    )

# Clean up
rm -f "$SERVER".csr