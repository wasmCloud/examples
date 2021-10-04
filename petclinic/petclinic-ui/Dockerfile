FROM node:14-buster

ENV HOME /home/node

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
     bash \
     curl \
     git \
     less \
     sed \
     vim \
    && npm install -g serve

COPY --chown=node:node \
    .env \
    craco.config.js \
    package.json \
    package-lock.json \
    tailwind.config.js          /home/node/
COPY --chown=node:node public/  /home/node/public/
COPY --chown=node:node src/     /home/node/src/
WORKDIR /home/node
RUN npm install \
    && npm run build \
    && chown -R node:node /home/node

USER node
CMD [ "serve", "-s", "build", "-p", "3000"]
