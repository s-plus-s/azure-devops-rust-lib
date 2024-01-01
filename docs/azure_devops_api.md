
# Wiql
https://learn.microsoft.com/ja-jp/rest/api/azure/devops/wit/wiql/query-by-wiql?view=azure-devops-rest-7.0&tabs=HTTP

```
POST https://dev.azure.com/{organization}/{project}/{team}/_apis/wit/wiql?api-version=7.0
```

# WorkItems
https://learn.microsoft.com/ja-jp/rest/api/azure/devops/wit/work-items/list?view=azure-devops-rest-7.0&tabs=HTTP

```
GET https://dev.azure.com/{organization}/{project}/_apis/wit/workitems?ids={ids}&api-version=7.0
```

# revisions
https://learn.microsoft.com/ja-jp/rest/api/azure/devops/wit/revisions/list?view=azure-devops-rest-7.0&tabs=HTTP


```
GET https://dev.azure.com/{organization}/{project}/_apis/wit/workItems/{id}/revisions?api-version=7.0
```

# pullrequests
https://learn.microsoft.com/ja-jp/rest/api/azure/devops/git/pull-requests/get-pull-requests?view=azure-devops-rest-7.0&tabs=HTTP

```
GET https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repository_id}/pullrequests?api-version=7.0
```
