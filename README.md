# Counter - web application
## Table of contents
* [Technology stack](#technology-stack)
* [Build and run](#build-and-run)
## Technology stack
* Containers: Docker
* Container Tools: Docker Compose
* Api: Rocket (Rust)
* Database: Redis
* Reverse proxy: Nginx
* Frontend: React.js Bootstrap
## Build and run
### Prerequisites
Before you start install docker and docker-compose
### Pull images
If you don't want build the whole project you can pull all images
```docker compose pull```
### Build images
After making any changes build project
```docker-compose build```
### Run
Application will start on port 80 by default
* not run in the background\
```docker-comose up```
* run in the backgroung\
```docker-compose up -d```
