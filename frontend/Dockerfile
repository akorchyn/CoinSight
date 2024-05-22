FROM --platform=$BUILDPLATFORM node:18 as frontend
WORKDIR /app 
COPY ./package.json ./yarn.lock /app 
RUN yarn install --production=true 
COPY . /app 
CMD yarn run start --prod
EXPOSE 8000
