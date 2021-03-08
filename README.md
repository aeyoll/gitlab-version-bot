# Gitlab version bot

Post a message to a Rocket.Chat webbook is a Gitlab instance needs to be updated.

Usage
---

```
gitlab-version-bot <gitlab_url> <gitlab_token> <rocket_chat_url> <rocket_chat_token>
```

Example:

```
gitlab-version-bot https://git.yourdomain.com/ <gitlab_token> https://chat.yourdomain.com <rocket_chat_token>
```

Development
---

```
cargo run <gitlab_url> <gitlab_token> <rocket_chat_url> <rocket_chat_token>
cargo build # build for development
cargo build --release # build for production
```
