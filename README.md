# git-contrib-aggregator

**git-contrib-aggregator** is a lightweight tool designed to aggregate all your Git contributions across different Git instances (e.g., GitHub, GitLab) into a single source of truth. The aggregated data can then be used on multiple websites, dashboards, or personal tools. While it is tailored for personal use, it is also suitable for others who want to unify their contribution activity in one place.


## Features

- Aggregates contributions from multiple Git instances.
- Supports **GitHub** (GraphQL API) and **GitLab** (REST API v4 compatible).
- Exposes contributions per day and total per source for the current year.
- Tracks total commits, issues, and merge requests/pull requests.
- Lightweight and minimalistic; no unnecessary bloat.

> **Note:** GitLab contributions are slightly different than GitHub. For GitLab, the total commits are counted per push, rather than by individual commit events, to match the way the contribution chart is displayed in GitLab. Make sure your GitLab instance supports **API v4**.


## Getting Started

### Requirements

- Docker
- Docker Compose
- A `config.toml` file with your GitHub and GitLab PATs (Personal Access Tokens)


### Configuration

Create a `config.toml` file with your tokens and instance URLs. Example:

```toml
github_pat = "ghp_your_personal_access_token"
cache_ttl = 24

[[gitlab_pats]]
name = "Personal Gitlab instance"
uri = "https://gitlab.example.com/api/v4"
token = "glpat_your_personal_access_token"
```

> **NOTE:** Ensure that your PATs do have enough permissions to access the required resources. GitHub needs at least access to your repositories and organisations if you want to track contributions from multiple organisations. GitLab needs api read permissions.

### Starting the application

Use the given `docker-compose.yml` file to start the application:

```bash
docker-compose up -d
```

### API Access

`GET /api/activity` Gets you all the aggregated activity data.


## Contributing

This project welcomes contributions!

- Open issues for bugs or feature requests.
- Pull requests are welcome for improvements.
- Please keep the project lightweight and general-purpose; avoid adding niche or overly complex features.

## Philosophy

git-contrib-aggregator is intentionally minimalistic. Its goal is to provide a reliable aggregation of Git contributions with just enough features to be useful without becoming bloated. It’s a tool for general-purpose use, adaptable to personal dashboards or websites.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
