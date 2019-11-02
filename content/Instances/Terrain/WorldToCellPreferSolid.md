+++
Target = "Terrain.WorldToCellPreferSolid"
Type = (position: Vector3) => Vector3
+++

Returns the grid cell location that contains the point position, preferring non-empty grid cells when position is on a grid edge.