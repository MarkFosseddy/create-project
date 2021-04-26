# Create Project

Shell command to create project

```console
$ create-project <programming_language> <project_name>
$ create-project javascript my-project
```

* creates github repo
* clones it into project directory

Needs config file $HOME/.config/create-project.config
```console
githubAPIKey = api_key
githubUsername = username
projectsDir = /path/to/dir
allowedLanguages = lang1,lang2,lang3
```
