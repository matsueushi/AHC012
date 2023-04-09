
FROM rust:latest

RUN apt-get update && apt-get install -y \ 
    python3-pip \
    awscli \
 && pip3 install pandas \
    numpy \ 
    cargo-lambda \
 && rustup component add rustfmt \
 && git config --global gpg.program "$(which gpg)" \
 && rm -rf /var/lib/apt/lists/*