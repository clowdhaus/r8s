# r8s

Notes:

- If the `kubectl.kubernetes.io/last-applied-configuration` annotation contains an old API version, it suggests the resource was created using the old API version (i.e. - you will run into trouble if you try to perform the same operation again after the cluster has been upgraded).

- The API version of the returned object will be governed by the client and not representative of what will or will not fail if POSTed after a cluster upgrade.

## Reference [sig-architecture/api_changes.md](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api_changes.md)

The Kubernetes API has two major components - the internal structures and the versioned APIs. The versioned APIs are intended to be stable,  while the internal structures are implemented to best reflect the needs of the Kubernetes code itself. Every versioned API can be converted to the internal form (and vice-versa), but versioned APIs do not convert to other versioned APIs directly. While all of the Kubernetes code operates on the internal structures, they are always converted to a versioned form before being written to storage (disk or etcd) or being sent over a wire. Clients should consume and operate on the versioned APIs exclusively.

To demonstrate the general process, here is a (hypothetical) example:

   1. A user POSTs a `Pod` object to `/api/v7beta1/...`
   2. The JSON is unmarshalled into a `v7beta1.Pod` structure
   3. Default values are applied to the `v7beta1.Pod`
   4. The `v7beta1.Pod` is converted to an `api.Pod` structure
   5. The `api.Pod` is validated, and any errors are returned to the user
   6. The `api.Pod` is converted to a `v6.Pod` (because v6 is the latest stable version)
   7. The `v6.Pod` is marshalled into JSON and written to etcd

Now that we have the `Pod` object stored, a user can GET that object in any supported api version. For example:

   1. A user GETs the `Pod` from `/api/v5/...`
   2. The JSON is read from etcd and unmarshalled into a `v6.Pod` structure
   3. Default values are applied to the `v6.Pod`
   4. The `v6.Pod` is converted to an `api.Pod` structure
   5. The `api.Pod` is converted to a `v5.Pod` structure
   6. The `v5.Pod` is marshalled into JSON and sent to the user
