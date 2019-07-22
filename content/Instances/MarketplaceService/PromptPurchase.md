+++
Target = "MarketplaceService.PromptPurchase"
Type = { (player: Player, assetId: number, equipIfPurchased?: boolean | undefined, currencyType?: 0 | 1 | 2 | Enum.CurrencyType.Default | Enum.CurrencyType.Robux | Enum.CurrencyType.Tix | "Default" | "Robux" | "Tix" | undefined): void; (player: Player, assetId: number, equipIfPurchased?: boolean | undefined, currencyType?: 0 | 1 | 2 | Enum.CurrencyType.Default | Enum.CurrencyType.Robux | Enum.CurrencyType.Tix | "Default" | "Robux" | "Tix" | undefined): void; }
+++

Used to prompt a user to purchase an item with the given assetId.For game passes, use [MarketplaceService.PromptGamePassPurchase](https://developer.roblox.com/api-reference/function/MarketplaceService/PromptGamePassPurchase).