ARG UBUNTU_VERSION=focal
ARG VERSION=1
ARG NAME=server-manager-rust

FROM ubuntu:${UBUNTU_VERSION}
# need to repeat the variables after from since from consumes all args and they are not available afterwards
ARG HTTP_PORT

LABEL NAME ${NAME}
LABEL version ${VERSION}

RUN apt-get update \
&& apt-get upgrade -y \
&& apt-get install wget gnupg2 -y \
&& apt-get clean
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys D8FF8E1F7DF8B07E
# && apt install -y openssh-server \
# && ssh-keygen -A

ADD inet.dll ./
ADD inet.so ./
ADD server-manager-rust ./
ADD .env.example ./.env.example
ADD shipped_plugins ./shipped_plugins
ADD server ./
ADD server-manager-frontend ./server/static

RUN mkdir -p ./external_files

COPY docker/entrypoint.sh ./

RUN chmod +x /server-manager-rust \
&& chmod +x /entrypoint.sh

# install influx DB
RUN wget -q https://repos.influxdata.com/influxdata-archive_compat.key \
&& echo '393e8779c89ac8d958f81f942f9ad7fb82a25e133faddaf92e15b16e6ac9ce4c influxdata-archive_compat.key' | sha256sum -c && cat influxdata-archive_compat.key | gpg --dearmor | tee /etc/apt/trusted.gpg.d/influxdata-archive_compat.gpg > /dev/null \
&& echo 'deb [signed-by=/etc/apt/trusted.gpg.d/influxdata-archive_compat.gpg] https://repos.influxdata.com/debian stable main' | tee /etc/apt/sources.list.d/influxdata.list \
&& apt-get update -y \
&& apt-get install influxdb -y \
&& apt-get clean \
&& systemctl unmask influxdb.service


ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 0