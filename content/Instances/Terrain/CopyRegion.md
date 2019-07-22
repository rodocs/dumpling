+++
Target = "Terrain.CopyRegion"
Type = { (region: Region3int16): TerrainRegion; (region: Region3int16): TerrainRegion; }
+++

Stores a chunk of terrain into a `TerrainRegion` object so it can be loaded back later.  Note: `TerrainRegion` data does not replicate between server and client.