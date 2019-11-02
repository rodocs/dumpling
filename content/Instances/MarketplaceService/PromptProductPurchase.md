+++
Target = "MarketplaceService.PromptProductPurchase"
Type = { (player: Player, productId: number, equipIfPurchased?: boolean | undefined, currencyType?: 0 | 1 | 2 | Enum.CurrencyType.Default | Enum.CurrencyType.Robux | Enum.CurrencyType.Tix | "Default" | "Robux" | "Tix" | undefined): void; (player: Player, productId: number, equipIfPurchased?: boolean | undefined, currencyType?: 0 | 1 | 2 | Enum.CurrencyType.Default | Enum.CurrencyType.Robux | Enum.CurrencyType.Tix | "Default" | "Robux" | "Tix" | undefined): void; }
+++

Used to prompt a user to purchase a product with the given product id.