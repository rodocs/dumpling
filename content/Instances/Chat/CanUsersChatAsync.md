+++
Target = "Chat.CanUsersChatAsync"
Type = (userIdFrom: number, userIdTo: number) => boolean
+++

Will return false if the two users cannot communicate because their account settings do not allow it.