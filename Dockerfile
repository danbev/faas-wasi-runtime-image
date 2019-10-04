FROM nodeshift/ubi8-s2i-wasi:0.1

ENV PORT=8080
EXPOSE $PORT

USER 0

COPY . /opt/app-root/src
COPY s2i /usr/libexec/s2i
COPY ./contrib/ /opt/app-root

RUN yum -y install gcc && \
    /opt/app-root/etc/install.sh && \
    chmod -R 777 /opt/app-root/src
ENV PATH="/opt/app-root/src/.cargo/bin/:${PATH}"

USER 1001

CMD ["cargo", "run"]
