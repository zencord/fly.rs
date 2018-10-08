FROM node:10 as v8env

COPY --from=neomantra/flatbuffers /usr/local/bin/flatc /usr/local/bin/flatc

WORKDIR /v8env
COPY package.json package.json
RUN yarn install

ADD msg.fbs .

RUN flatc --ts -o v8env/src --no-fb-import --gen-mutable msg.fb

ADD v8env/ .

RUN ./node_modules/.bin/rollup -c

RUN ls -lah dist

FROM flyio/v8:7.1 as v8

FROM rust:1.29

WORKDIR /usr/src/myapp

COPY --from=neomantra/flatbuffers /usr/local/bin/flatc /usr/local/bin/flatc

ADD libfly libfly

COPY --from=v8 /v8/lib libfly/third_party/v8/out.gn/obj
# COPY --from=v8 /v8/include $GO_V8_DIR/include/

COPY . .
RUN cargo build --release --bin create_snapshot

RUN ls -lah target/release

COPY --from=v8env v8env/dist/* v8env/dist/

RUN target/release/create_snapshot v8env/dist/v8env.js v8env.bin

RUN cargo build --release

RUN ls -lah target/release