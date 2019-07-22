+++
Target = "DataStoreService.GetDataStore"
Type = { (name: string, scope?: string | undefined): GlobalDataStore; (name: string, scope?: string | undefined): GlobalDataStore; }
+++

This method returns a `GlobalDataStore` by name/scope. Subsequent calls to this method with the same name/scope will return the same object.