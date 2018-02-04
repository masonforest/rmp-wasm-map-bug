FROM rustlang/rust:nightly
RUN rustup update
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN curl --silent --location https://deb.nodesource.com/setup_4.x | bash -
RUN apt-get install --yes nodejs

