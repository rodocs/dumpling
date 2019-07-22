+++
Target = "DataStoreService.GetOrderedDataStore"
Type = { (name: string, scope?: string | undefined): OrderedDataStore; (name: string, scope?: string | undefined): OrderedDataStore; }
+++

This method returns an `OrderedDataStore`, similar to the way [GetDataStore()](https://developer.roblox.com/api-reference/function/DataStoreService/GetDataStore) does with [GlobalDataStores](https://developer.roblox.com/api-reference/class/GlobalDataStore). Subsequent calls to this method with the same name/scope will return the same object.