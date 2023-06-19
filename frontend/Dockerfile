FROM node:18
WORKDIR /app 
COPY package.json /app 
COPY yarn.lock /app
RUN yarn install --production=true 
COPY . /app 
CMD yarn run start --prod
EXPOSE 8000
