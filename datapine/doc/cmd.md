```
insert v1 0.0 1.0 2.0
get v1
remove v1
insert v2 0.0 1.5 2.0
insert v3 1.0 1.5 2.0
insert v4 0.0 1.5 3.0
insert v5 1.0 2.0 3.0
knn v2 3
cosine v2 v3
cosine v2 v5
dump store.json
```