# Tutorlolv2 product UI

Preserve the established dark Yew/Tailwind interface: zinc borders, existing `std` background tokens, sky links, compact account status above the main content, and shared history/replay controls. This is a League of Legends analysis tool for desktop and website users; match data and readable charts remain the visual focus.

Account state belongs to `src/account.rs`; owner transports and wire contracts belong to `src/history/source.rs`. Native credentials never enter UI or IPC. The overlay does not mount the account provider. Public Watch stays independent of owner identity. Identity changes remount owner history/replay and invalidate transport registries.

Use the same account action labels throughout: Sign in with Google, Sign out, Retry. Display loading and pending status with `role=status`, errors with `role=alert`, disabled actions while pending, visible keyboard focus, and wrapping text at narrow widths. Website hide affects only the current browser session; desktop repair and upload behavior must only appear in the desktop app. Browser verification covers signed-out guidance, signed-in history/replay, logout, expiry, and narrow viewport overflow.
