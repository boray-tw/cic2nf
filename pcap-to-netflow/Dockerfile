FROM alpine:3.23
# config
ARG USERNAME=user
ARG GID=1000
ARG UID=1000

ENV CWD=/pcap-to-nf

# RUN apk add --no-cache bash nfdump softflowd wireshark-common
RUN apk add --no-cache \
"bash=5.3.3-r1" \
# nfdump: Used to retransmit PCAP as NetFlow flow
"nfdump=1.7.6-r0" \
# softflowd: NetFlow exporter used to receive NetFlow flow and store as files ([Matt Hayes, 2018](https://mattjhayes.com/2018/08/19/collecting-netflow-with-nfcapd-and-nfdump/)))
"softflowd=1.1.0-r0" \
"wireshark-common=4.6.1-r0"

COPY entry-inside-container.sh "$CWD/run.sh"

RUN adduser -D -u $UID $USERNAME && \
  addgroup $USERNAME wireshark && \
  chown -R $USERNAME:$USERNAME "$CWD" && \
  chmod +x "$CWD/run.sh" && \
  install -d -D -o $USERNAME -g $USERNAME "$CWD/input" "$CWD/output"

WORKDIR "$CWD/"
USER $USERNAME
CMD ["./run.sh"]
