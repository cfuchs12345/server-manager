ARG UBUNTU_VERSION=focal
ARG VERSION=1
ARG NAME=server-manager-rust

FROM ubuntu:${UBUNTU_VERSION}
# need to repeat the variables after from since from consumes all args and they are not available afterwards
ARG HTTP_PORT

LABEL NAME ${NAME}
LABEL version ${VERSION}

RUN apt update \
&& apt upgrade -y \
&& apt install -y openssh-server \
&& ssh-keygen -A

ADD inet.dll ./
ADD inet.so ./
ADD server-manager-rust ./
ADD .env.example ./.env.example
ADD shipped_plugins ./shipped_plugins
ADD server ./
ADD server-manager-frontend ./server/static

RUN mkdir -p ./external_files
RUN touch ./server-manager.db

COPY docker/entrypoint.sh ./

RUN chmod +x /server-manager-rust \
&& chmod +x /entrypoint.sh

RUN ls -altr

ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 0