+++
Target = "Terrain.PasteRegion"
Type = { (region: TerrainRegion, corner: Vector3int16, pasteEmptyCells: boolean): void; (region: TerrainRegion, corner: Vector3int16, pasteEmptyCells: boolean): void; }
+++

Applies a chunk of terrain to the Terrain object. Note: `TerrainRegion` data does not replicate between server and client.