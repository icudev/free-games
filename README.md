## Free Games Bot
An automated bot for X/Twitter that posts free games on [Steam](https://store.steampowered.com/) and 
[Epic Games](https://store.epicgames.com/de/).<br>
It currently checks the stores in an interval of 10 minutes and posts them if they're not already posted.

## Setup using [Docker](https://www.docker.com/)
1. Create a X/Twitter Developer Account at [developer.x.com](https://developer.x.com/)
1. Create a new app in [Projects and Apps](https://developer.x.com/en/portal/projects-and-apps)
1. Generate your API Key+Secret and Access Token+Secret, make sure the access token has `Read and Write` permissions
1. Copy `.env.example` to `.env` and insert all needed variables
1. Build the project using 
    ```bash
    docker compose build
    ```
1. Run the project using
    ```bash
    docker compose up -d
    ```
