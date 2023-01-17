# Get a python 3.10 image
FROM python:3.10.9
LABEL org.opencontainers.image.source=https://github.com/gbdev/pandocs
SHELL ["bash", "-lc"]
RUN apt update
RUN apt install curl -y

# Install rust and mdbook
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN apt install gcc -y
RUN source "$HOME/.cargo/env"
RUN cargo install mdbook

COPY . /code
WORKDIR /code

# Init python3 env
RUN python -m venv env
RUN source env/bin/activate
RUN pip install -r requirements.txt

# Serve pandocs at localhost:8000
RUN mdbook build
CMD mdbook watch & (cd /code/docs/pandocs/ && python3 -m http.server)
