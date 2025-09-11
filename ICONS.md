# 🎨 Lucide Icons Reference

This document lists all 1636 available icons in the Lucide Rust library, organized by category.

## 📚 Table of Contents

- [Accessibility](#accessibility)- [Account](#account)- [Animals](#animals)- [Arrows](#arrows)- [Brands](#brands)- [Buildings](#buildings)- [Charts](#charts)- [Communication](#communication)- [Connectivity](#connectivity)- [Cursors](#cursors)- [Design](#design)- [Development](#development)- [Devices](#devices)- [Emoji](#emoji)- [Files](#files)- [Finance](#finance)- [Food Beverage](#food-beverage)- [Gaming](#gaming)- [Home](#home)- [Layout](#layout)- [Mail](#mail)- [Math](#math)- [Medical](#medical)- [Multimedia](#multimedia)- [Nature](#nature)- [Navigation](#navigation)- [Notifications](#notifications)- [People](#people)- [Photography](#photography)- [Science](#science)- [Seasons](#seasons)- [Security](#security)- [Shapes](#shapes)- [Shopping](#shopping)- [Social](#social)- [Sports](#sports)- [Sustainability](#sustainability)- [Text](#text)- [Time](#time)- [Tools](#tools)- [Transportation](#transportation)- [Travel](#travel)- [Weather](#weather)
## 🚀 Usage

### Basic Usage

```rust
use lucide::*;

// In your component
<Home class="w-6 h-6" />
```

### With Specific Framework

Add to your `Cargo.toml`:

```toml
# For specific frameworks
lucide = { version = "0.1.0", features = ["leptos"] }
lucide = { version = "0.1.0", features = ["yew"] }
lucide = { version = "0.1.0", features = ["dioxus"] }
lucide = { version = "0.1.0", features = ["sycamore"] }

# With specific icon categories (for smaller bundle size)
lucide = { version = "0.1.0", features = ["leptos", "navigation", "files"] }

# Or include all icons with:
lucide = { version = "0.1.0", features = ["leptos", "all-icons"] }
```

## 🗂️ Icons by Category

### Accessibility

`accessibility` and `all-icons` features - 30 icons

| Icon | Name | Component |
|------|------|-----------|
| [accessibility](https://lucide.dev/icons/accessibility) | `accessibility` | `<Accessibility />` |
| [baby](https://lucide.dev/icons/baby) | `baby` | `<Baby />` |
| [badge-info](https://lucide.dev/icons/badge-info) | `badge-info` | `<BadgeInfo />` |
| [badge-question-mark](https://lucide.dev/icons/badge-question-mark) | `badge-question-mark` | `<BadgeQuestionMark />` |
| [circle-question-mark](https://lucide.dev/icons/circle-question-mark) | `circle-question-mark` | `<CircleQuestionMark />` |
| [closed-caption](https://lucide.dev/icons/closed-caption) | `closed-caption` | `<ClosedCaption />` |
| [contrast](https://lucide.dev/icons/contrast) | `contrast` | `<Contrast />` |
| [ear](https://lucide.dev/icons/ear) | `ear` | `<Ear />` |
| [ear-off](https://lucide.dev/icons/ear-off) | `ear-off` | `<EarOff />` |
| [eclipse](https://lucide.dev/icons/eclipse) | `eclipse` | `<Eclipse />` |
| [eye](https://lucide.dev/icons/eye) | `eye` | `<Eye />` |
| [eye-closed](https://lucide.dev/icons/eye-closed) | `eye-closed` | `<EyeClosed />` |
| [eye-off](https://lucide.dev/icons/eye-off) | `eye-off` | `<EyeOff />` |
| [glasses](https://lucide.dev/icons/glasses) | `glasses` | `<Glasses />` |
| [hand](https://lucide.dev/icons/hand) | `hand` | `<Hand />` |
| [info](https://lucide.dev/icons/info) | `info` | `<Info />` |
| [life-buoy](https://lucide.dev/icons/life-buoy) | `life-buoy` | `<LifeBuoy />` |
| [moon](https://lucide.dev/icons/moon) | `moon` | `<Moon />` |
| [moon-star](https://lucide.dev/icons/moon-star) | `moon-star` | `<MoonStar />` |
| [person-standing](https://lucide.dev/icons/person-standing) | `person-standing` | `<PersonStanding />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-search](https://lucide.dev/icons/scan-search) | `scan-search` | `<ScanSearch />` |
| [speech](https://lucide.dev/icons/speech) | `speech` | `<Speech />` |
| [sun](https://lucide.dev/icons/sun) | `sun` | `<Sun />` |
| [sun-dim](https://lucide.dev/icons/sun-dim) | `sun-dim` | `<SunDim />` |
| [sun-medium](https://lucide.dev/icons/sun-medium) | `sun-medium` | `<SunMedium />` |
| [sun-moon](https://lucide.dev/icons/sun-moon) | `sun-moon` | `<SunMoon />` |
| [transgender](https://lucide.dev/icons/transgender) | `transgender` | `<Transgender />` |
| [zoom-in](https://lucide.dev/icons/zoom-in) | `zoom-in` | `<ZoomIn />` |
| [zoom-out](https://lucide.dev/icons/zoom-out) | `zoom-out` | `<ZoomOut />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "accessibility"] or ["leptos", "all-icons"]
<Accessibility class="w-6 h-6 text-gray-600" />
```

### Account

`account` and `all-icons` features - 133 icons

| Icon | Name | Component |
|------|------|-----------|
| [activity](https://lucide.dev/icons/activity) | `activity` | `<Activity />` |
| [at-sign](https://lucide.dev/icons/at-sign) | `at-sign` | `<AtSign />` |
| [award](https://lucide.dev/icons/award) | `award` | `<Award />` |
| [badge](https://lucide.dev/icons/badge) | `badge` | `<Badge />` |
| [badge-alert](https://lucide.dev/icons/badge-alert) | `badge-alert` | `<BadgeAlert />` |
| [badge-info](https://lucide.dev/icons/badge-info) | `badge-info` | `<BadgeInfo />` |
| [ban](https://lucide.dev/icons/ban) | `ban` | `<Ban />` |
| [bell](https://lucide.dev/icons/bell) | `bell` | `<Bell />` |
| [bell-dot](https://lucide.dev/icons/bell-dot) | `bell-dot` | `<BellDot />` |
| [book-user](https://lucide.dev/icons/book-user) | `book-user` | `<BookUser />` |
| [bookmark](https://lucide.dev/icons/bookmark) | `bookmark` | `<Bookmark />` |
| [bookmark-check](https://lucide.dev/icons/bookmark-check) | `bookmark-check` | `<BookmarkCheck />` |
| [bookmark-minus](https://lucide.dev/icons/bookmark-minus) | `bookmark-minus` | `<BookmarkMinus />` |
| [bookmark-plus](https://lucide.dev/icons/bookmark-plus) | `bookmark-plus` | `<BookmarkPlus />` |
| [bookmark-x](https://lucide.dev/icons/bookmark-x) | `bookmark-x` | `<BookmarkX />` |
| [building](https://lucide.dev/icons/building) | `building` | `<Building />` |
| [building-2](https://lucide.dev/icons/building-2) | `building-2` | `<Building2 />` |
| [cake](https://lucide.dev/icons/cake) | `cake` | `<Cake />` |
| [circle-user](https://lucide.dev/icons/circle-user) | `circle-user` | `<CircleUser />` |
| [circle-user-round](https://lucide.dev/icons/circle-user-round) | `circle-user-round` | `<CircleUserRound />` |
| [cog](https://lucide.dev/icons/cog) | `cog` | `<Cog />` |
| [contact](https://lucide.dev/icons/contact) | `contact` | `<Contact />` |
| [contact-round](https://lucide.dev/icons/contact-round) | `contact-round` | `<ContactRound />` |
| [cookie](https://lucide.dev/icons/cookie) | `cookie` | `<Cookie />` |
| [credit-card](https://lucide.dev/icons/credit-card) | `credit-card` | `<CreditCard />` |
| [file-user](https://lucide.dev/icons/file-user) | `file-user` | `<FileUser />` |
| [fingerprint](https://lucide.dev/icons/fingerprint) | `fingerprint` | `<Fingerprint />` |
| [flag](https://lucide.dev/icons/flag) | `flag` | `<Flag />` |
| [flag-off](https://lucide.dev/icons/flag-off) | `flag-off` | `<FlagOff />` |
| [frown](https://lucide.dev/icons/frown) | `frown` | `<Frown />` |
| [gift](https://lucide.dev/icons/gift) | `gift` | `<Gift />` |
| [hand-coins](https://lucide.dev/icons/hand-coins) | `hand-coins` | `<HandCoins />` |
| [handshake](https://lucide.dev/icons/handshake) | `handshake` | `<Handshake />` |
| [hat-glasses](https://lucide.dev/icons/hat-glasses) | `hat-glasses` | `<HatGlasses />` |
| [heart-handshake](https://lucide.dev/icons/heart-handshake) | `heart-handshake` | `<HeartHandshake />` |
| [heart-minus](https://lucide.dev/icons/heart-minus) | `heart-minus` | `<HeartMinus />` |
| [heart-plus](https://lucide.dev/icons/heart-plus) | `heart-plus` | `<HeartPlus />` |
| [id-card](https://lucide.dev/icons/id-card) | `id-card` | `<IdCard />` |
| [id-card-lanyard](https://lucide.dev/icons/id-card-lanyard) | `id-card-lanyard` | `<IdCardLanyard />` |
| [inbox](https://lucide.dev/icons/inbox) | `inbox` | `<Inbox />` |
| [key](https://lucide.dev/icons/key) | `key` | `<Key />` |
| [key-round](https://lucide.dev/icons/key-round) | `key-round` | `<KeyRound />` |
| [key-square](https://lucide.dev/icons/key-square) | `key-square` | `<KeySquare />` |
| [link](https://lucide.dev/icons/link) | `link` | `<Link />` |
| [link-2](https://lucide.dev/icons/link-2) | `link-2` | `<Link2 />` |
| [log-in](https://lucide.dev/icons/log-in) | `log-in` | `<LogIn />` |
| [log-out](https://lucide.dev/icons/log-out) | `log-out` | `<LogOut />` |
| [mail](https://lucide.dev/icons/mail) | `mail` | `<Mail />` |
| [map-pin](https://lucide.dev/icons/map-pin) | `map-pin` | `<MapPin />` |
| [map-pin-check](https://lucide.dev/icons/map-pin-check) | `map-pin-check` | `<MapPinCheck />` |
| [map-pin-check-inside](https://lucide.dev/icons/map-pin-check-inside) | `map-pin-check-inside` | `<MapPinCheckInside />` |
| [map-pin-house](https://lucide.dev/icons/map-pin-house) | `map-pin-house` | `<MapPinHouse />` |
| [map-pin-minus](https://lucide.dev/icons/map-pin-minus) | `map-pin-minus` | `<MapPinMinus />` |
| [map-pin-minus-inside](https://lucide.dev/icons/map-pin-minus-inside) | `map-pin-minus-inside` | `<MapPinMinusInside />` |
| [map-pin-pen](https://lucide.dev/icons/map-pin-pen) | `map-pin-pen` | `<MapPinPen />` |
| [map-pin-plus](https://lucide.dev/icons/map-pin-plus) | `map-pin-plus` | `<MapPinPlus />` |
| [map-pin-plus-inside](https://lucide.dev/icons/map-pin-plus-inside) | `map-pin-plus-inside` | `<MapPinPlusInside />` |
| [map-pin-x](https://lucide.dev/icons/map-pin-x) | `map-pin-x` | `<MapPinX />` |
| [map-pin-x-inside](https://lucide.dev/icons/map-pin-x-inside) | `map-pin-x-inside` | `<MapPinXInside />` |
| [map-pinned](https://lucide.dev/icons/map-pinned) | `map-pinned` | `<MapPinned />` |
| [menu](https://lucide.dev/icons/menu) | `menu` | `<Menu />` |
| [message-circle-x](https://lucide.dev/icons/message-circle-x) | `message-circle-x` | `<MessageCircleX />` |
| [notebook-tabs](https://lucide.dev/icons/notebook-tabs) | `notebook-tabs` | `<NotebookTabs />` |
| [pin](https://lucide.dev/icons/pin) | `pin` | `<Pin />` |
| [rotate-ccw-key](https://lucide.dev/icons/rotate-ccw-key) | `rotate-ccw-key` | `<RotateCcwKey />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-face](https://lucide.dev/icons/scan-face) | `scan-face` | `<ScanFace />` |
| [scan-qr-code](https://lucide.dev/icons/scan-qr-code) | `scan-qr-code` | `<ScanQrCode />` |
| [settings](https://lucide.dev/icons/settings) | `settings` | `<Settings />` |
| [settings-2](https://lucide.dev/icons/settings-2) | `settings-2` | `<Settings2 />` |
| [share](https://lucide.dev/icons/share) | `share` | `<Share />` |
| [share-2](https://lucide.dev/icons/share-2) | `share-2` | `<Share2 />` |
| [shield](https://lucide.dev/icons/shield) | `shield` | `<Shield />` |
| [shield-alert](https://lucide.dev/icons/shield-alert) | `shield-alert` | `<ShieldAlert />` |
| [shield-ban](https://lucide.dev/icons/shield-ban) | `shield-ban` | `<ShieldBan />` |
| [shield-check](https://lucide.dev/icons/shield-check) | `shield-check` | `<ShieldCheck />` |
| [shield-ellipsis](https://lucide.dev/icons/shield-ellipsis) | `shield-ellipsis` | `<ShieldEllipsis />` |
| [shield-half](https://lucide.dev/icons/shield-half) | `shield-half` | `<ShieldHalf />` |
| [shield-minus](https://lucide.dev/icons/shield-minus) | `shield-minus` | `<ShieldMinus />` |
| [shield-off](https://lucide.dev/icons/shield-off) | `shield-off` | `<ShieldOff />` |
| [shield-plus](https://lucide.dev/icons/shield-plus) | `shield-plus` | `<ShieldPlus />` |
| [shield-question-mark](https://lucide.dev/icons/shield-question-mark) | `shield-question-mark` | `<ShieldQuestionMark />` |
| [shield-user](https://lucide.dev/icons/shield-user) | `shield-user` | `<ShieldUser />` |
| [shield-x](https://lucide.dev/icons/shield-x) | `shield-x` | `<ShieldX />` |
| [slack](https://lucide.dev/icons/slack) | `slack` | `<Slack />` |
| [sliders-horizontal](https://lucide.dev/icons/sliders-horizontal) | `sliders-horizontal` | `<SlidersHorizontal />` |
| [sliders-vertical](https://lucide.dev/icons/sliders-vertical) | `sliders-vertical` | `<SlidersVertical />` |
| [smile](https://lucide.dev/icons/smile) | `smile` | `<Smile />` |
| [square-user](https://lucide.dev/icons/square-user) | `square-user` | `<SquareUser />` |
| [square-user-round](https://lucide.dev/icons/square-user-round) | `square-user-round` | `<SquareUserRound />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [tag](https://lucide.dev/icons/tag) | `tag` | `<Tag />` |
| [tags](https://lucide.dev/icons/tags) | `tags` | `<Tags />` |
| [thumbs-down](https://lucide.dev/icons/thumbs-down) | `thumbs-down` | `<ThumbsDown />` |
| [thumbs-up](https://lucide.dev/icons/thumbs-up) | `thumbs-up` | `<ThumbsUp />` |
| [ticket](https://lucide.dev/icons/ticket) | `ticket` | `<Ticket />` |
| [tickets](https://lucide.dev/icons/tickets) | `tickets` | `<Tickets />` |
| [toggle-left](https://lucide.dev/icons/toggle-left) | `toggle-left` | `<ToggleLeft />` |
| [toggle-right](https://lucide.dev/icons/toggle-right) | `toggle-right` | `<ToggleRight />` |
| [trello](https://lucide.dev/icons/trello) | `trello` | `<Trello />` |
| [twitch](https://lucide.dev/icons/twitch) | `twitch` | `<Twitch />` |
| [twitter](https://lucide.dev/icons/twitter) | `twitter` | `<Twitter />` |
| [user](https://lucide.dev/icons/user) | `user` | `<User />` |
| [user-check](https://lucide.dev/icons/user-check) | `user-check` | `<UserCheck />` |
| [user-cog](https://lucide.dev/icons/user-cog) | `user-cog` | `<UserCog />` |
| [user-lock](https://lucide.dev/icons/user-lock) | `user-lock` | `<UserLock />` |
| [user-minus](https://lucide.dev/icons/user-minus) | `user-minus` | `<UserMinus />` |
| [user-pen](https://lucide.dev/icons/user-pen) | `user-pen` | `<UserPen />` |
| [user-plus](https://lucide.dev/icons/user-plus) | `user-plus` | `<UserPlus />` |
| [user-round](https://lucide.dev/icons/user-round) | `user-round` | `<UserRound />` |
| [user-round-check](https://lucide.dev/icons/user-round-check) | `user-round-check` | `<UserRoundCheck />` |
| [user-round-cog](https://lucide.dev/icons/user-round-cog) | `user-round-cog` | `<UserRoundCog />` |
| [user-round-minus](https://lucide.dev/icons/user-round-minus) | `user-round-minus` | `<UserRoundMinus />` |
| [user-round-pen](https://lucide.dev/icons/user-round-pen) | `user-round-pen` | `<UserRoundPen />` |
| [user-round-plus](https://lucide.dev/icons/user-round-plus) | `user-round-plus` | `<UserRoundPlus />` |
| [user-round-search](https://lucide.dev/icons/user-round-search) | `user-round-search` | `<UserRoundSearch />` |
| [user-round-x](https://lucide.dev/icons/user-round-x) | `user-round-x` | `<UserRoundX />` |
| [user-search](https://lucide.dev/icons/user-search) | `user-search` | `<UserSearch />` |
| [user-star](https://lucide.dev/icons/user-star) | `user-star` | `<UserStar />` |
| [user-x](https://lucide.dev/icons/user-x) | `user-x` | `<UserX />` |
| [users](https://lucide.dev/icons/users) | `users` | `<Users />` |
| [users-round](https://lucide.dev/icons/users-round) | `users-round` | `<UsersRound />` |
| [venetian-mask](https://lucide.dev/icons/venetian-mask) | `venetian-mask` | `<VenetianMask />` |
| [vibrate](https://lucide.dev/icons/vibrate) | `vibrate` | `<Vibrate />` |
| [vibrate-off](https://lucide.dev/icons/vibrate-off) | `vibrate-off` | `<VibrateOff />` |
| [wallet](https://lucide.dev/icons/wallet) | `wallet` | `<Wallet />` |
| [wallet-cards](https://lucide.dev/icons/wallet-cards) | `wallet-cards` | `<WalletCards />` |
| [wallet-minimal](https://lucide.dev/icons/wallet-minimal) | `wallet-minimal` | `<WalletMinimal />` |
| [wallpaper](https://lucide.dev/icons/wallpaper) | `wallpaper` | `<Wallpaper />` |
| [waypoints](https://lucide.dev/icons/waypoints) | `waypoints` | `<Waypoints />` |
| [webhook](https://lucide.dev/icons/webhook) | `webhook` | `<Webhook />` |
| [webhook-off](https://lucide.dev/icons/webhook-off) | `webhook-off` | `<WebhookOff />` |
| [wrench](https://lucide.dev/icons/wrench) | `wrench` | `<Wrench />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "account"] or ["leptos", "all-icons"]
<Activity class="w-6 h-6 text-gray-600" />
```

### Animals

`animals` and `all-icons` features - 22 icons

| Icon | Name | Component |
|------|------|-----------|
| [bird](https://lucide.dev/icons/bird) | `bird` | `<Bird />` |
| [bone](https://lucide.dev/icons/bone) | `bone` | `<Bone />` |
| [bug](https://lucide.dev/icons/bug) | `bug` | `<Bug />` |
| [bug-off](https://lucide.dev/icons/bug-off) | `bug-off` | `<BugOff />` |
| [bug-play](https://lucide.dev/icons/bug-play) | `bug-play` | `<BugPlay />` |
| [cat](https://lucide.dev/icons/cat) | `cat` | `<Cat />` |
| [dog](https://lucide.dev/icons/dog) | `dog` | `<Dog />` |
| [egg](https://lucide.dev/icons/egg) | `egg` | `<Egg />` |
| [fish](https://lucide.dev/icons/fish) | `fish` | `<Fish />` |
| [fish-off](https://lucide.dev/icons/fish-off) | `fish-off` | `<FishOff />` |
| [fish-symbol](https://lucide.dev/icons/fish-symbol) | `fish-symbol` | `<FishSymbol />` |
| [origami](https://lucide.dev/icons/origami) | `origami` | `<Origami />` |
| [panda](https://lucide.dev/icons/panda) | `panda` | `<Panda />` |
| [paw-print](https://lucide.dev/icons/paw-print) | `paw-print` | `<PawPrint />` |
| [rabbit](https://lucide.dev/icons/rabbit) | `rabbit` | `<Rabbit />` |
| [rat](https://lucide.dev/icons/rat) | `rat` | `<Rat />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [shrimp](https://lucide.dev/icons/shrimp) | `shrimp` | `<Shrimp />` |
| [snail](https://lucide.dev/icons/snail) | `snail` | `<Snail />` |
| [squirrel](https://lucide.dev/icons/squirrel) | `squirrel` | `<Squirrel />` |
| [turtle](https://lucide.dev/icons/turtle) | `turtle` | `<Turtle />` |
| [worm](https://lucide.dev/icons/worm) | `worm` | `<Worm />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "animals"] or ["leptos", "all-icons"]
<Bird class="w-6 h-6 text-gray-600" />
```

### Arrows

`arrows` and `all-icons` features - 206 icons

| Icon | Name | Component |
|------|------|-----------|
| [arrow-big-down](https://lucide.dev/icons/arrow-big-down) | `arrow-big-down` | `<ArrowBigDown />` |
| [arrow-big-down-dash](https://lucide.dev/icons/arrow-big-down-dash) | `arrow-big-down-dash` | `<ArrowBigDownDash />` |
| [arrow-big-left](https://lucide.dev/icons/arrow-big-left) | `arrow-big-left` | `<ArrowBigLeft />` |
| [arrow-big-left-dash](https://lucide.dev/icons/arrow-big-left-dash) | `arrow-big-left-dash` | `<ArrowBigLeftDash />` |
| [arrow-big-right](https://lucide.dev/icons/arrow-big-right) | `arrow-big-right` | `<ArrowBigRight />` |
| [arrow-big-right-dash](https://lucide.dev/icons/arrow-big-right-dash) | `arrow-big-right-dash` | `<ArrowBigRightDash />` |
| [arrow-big-up](https://lucide.dev/icons/arrow-big-up) | `arrow-big-up` | `<ArrowBigUp />` |
| [arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) | `arrow-big-up-dash` | `<ArrowBigUpDash />` |
| [arrow-down](https://lucide.dev/icons/arrow-down) | `arrow-down` | `<ArrowDown />` |
| [arrow-down-0-1](https://lucide.dev/icons/arrow-down-0-1) | `arrow-down-0-1` | `<ArrowDown01 />` |
| [arrow-down-1-0](https://lucide.dev/icons/arrow-down-1-0) | `arrow-down-1-0` | `<ArrowDown10 />` |
| [arrow-down-a-z](https://lucide.dev/icons/arrow-down-a-z) | `arrow-down-a-z` | `<ArrowDownAZ />` |
| [arrow-down-from-line](https://lucide.dev/icons/arrow-down-from-line) | `arrow-down-from-line` | `<ArrowDownFromLine />` |
| [arrow-down-left](https://lucide.dev/icons/arrow-down-left) | `arrow-down-left` | `<ArrowDownLeft />` |
| [arrow-down-narrow-wide](https://lucide.dev/icons/arrow-down-narrow-wide) | `arrow-down-narrow-wide` | `<ArrowDownNarrowWide />` |
| [arrow-down-right](https://lucide.dev/icons/arrow-down-right) | `arrow-down-right` | `<ArrowDownRight />` |
| [arrow-down-to-dot](https://lucide.dev/icons/arrow-down-to-dot) | `arrow-down-to-dot` | `<ArrowDownToDot />` |
| [arrow-down-to-line](https://lucide.dev/icons/arrow-down-to-line) | `arrow-down-to-line` | `<ArrowDownToLine />` |
| [arrow-down-up](https://lucide.dev/icons/arrow-down-up) | `arrow-down-up` | `<ArrowDownUp />` |
| [arrow-down-wide-narrow](https://lucide.dev/icons/arrow-down-wide-narrow) | `arrow-down-wide-narrow` | `<ArrowDownWideNarrow />` |
| [arrow-down-z-a](https://lucide.dev/icons/arrow-down-z-a) | `arrow-down-z-a` | `<ArrowDownZA />` |
| [arrow-left](https://lucide.dev/icons/arrow-left) | `arrow-left` | `<ArrowLeft />` |
| [arrow-left-from-line](https://lucide.dev/icons/arrow-left-from-line) | `arrow-left-from-line` | `<ArrowLeftFromLine />` |
| [arrow-left-right](https://lucide.dev/icons/arrow-left-right) | `arrow-left-right` | `<ArrowLeftRight />` |
| [arrow-left-to-line](https://lucide.dev/icons/arrow-left-to-line) | `arrow-left-to-line` | `<ArrowLeftToLine />` |
| [arrow-right](https://lucide.dev/icons/arrow-right) | `arrow-right` | `<ArrowRight />` |
| [arrow-right-from-line](https://lucide.dev/icons/arrow-right-from-line) | `arrow-right-from-line` | `<ArrowRightFromLine />` |
| [arrow-right-left](https://lucide.dev/icons/arrow-right-left) | `arrow-right-left` | `<ArrowRightLeft />` |
| [arrow-right-to-line](https://lucide.dev/icons/arrow-right-to-line) | `arrow-right-to-line` | `<ArrowRightToLine />` |
| [arrow-up](https://lucide.dev/icons/arrow-up) | `arrow-up` | `<ArrowUp />` |
| [arrow-up-0-1](https://lucide.dev/icons/arrow-up-0-1) | `arrow-up-0-1` | `<ArrowUp01 />` |
| [arrow-up-1-0](https://lucide.dev/icons/arrow-up-1-0) | `arrow-up-1-0` | `<ArrowUp10 />` |
| [arrow-up-a-z](https://lucide.dev/icons/arrow-up-a-z) | `arrow-up-a-z` | `<ArrowUpAZ />` |
| [arrow-up-down](https://lucide.dev/icons/arrow-up-down) | `arrow-up-down` | `<ArrowUpDown />` |
| [arrow-up-from-dot](https://lucide.dev/icons/arrow-up-from-dot) | `arrow-up-from-dot` | `<ArrowUpFromDot />` |
| [arrow-up-from-line](https://lucide.dev/icons/arrow-up-from-line) | `arrow-up-from-line` | `<ArrowUpFromLine />` |
| [arrow-up-left](https://lucide.dev/icons/arrow-up-left) | `arrow-up-left` | `<ArrowUpLeft />` |
| [arrow-up-narrow-wide](https://lucide.dev/icons/arrow-up-narrow-wide) | `arrow-up-narrow-wide` | `<ArrowUpNarrowWide />` |
| [arrow-up-right](https://lucide.dev/icons/arrow-up-right) | `arrow-up-right` | `<ArrowUpRight />` |
| [arrow-up-to-line](https://lucide.dev/icons/arrow-up-to-line) | `arrow-up-to-line` | `<ArrowUpToLine />` |
| [arrow-up-wide-narrow](https://lucide.dev/icons/arrow-up-wide-narrow) | `arrow-up-wide-narrow` | `<ArrowUpWideNarrow />` |
| [arrow-up-z-a](https://lucide.dev/icons/arrow-up-z-a) | `arrow-up-z-a` | `<ArrowUpZA />` |
| [arrows-up-from-line](https://lucide.dev/icons/arrows-up-from-line) | `arrows-up-from-line` | `<ArrowsUpFromLine />` |
| [calendar-sync](https://lucide.dev/icons/calendar-sync) | `calendar-sync` | `<CalendarSync />` |
| [chevron-down](https://lucide.dev/icons/chevron-down) | `chevron-down` | `<ChevronDown />` |
| [chevron-first](https://lucide.dev/icons/chevron-first) | `chevron-first` | `<ChevronFirst />` |
| [chevron-last](https://lucide.dev/icons/chevron-last) | `chevron-last` | `<ChevronLast />` |
| [chevron-left](https://lucide.dev/icons/chevron-left) | `chevron-left` | `<ChevronLeft />` |
| [chevron-right](https://lucide.dev/icons/chevron-right) | `chevron-right` | `<ChevronRight />` |
| [chevron-up](https://lucide.dev/icons/chevron-up) | `chevron-up` | `<ChevronUp />` |
| [chevrons-down](https://lucide.dev/icons/chevrons-down) | `chevrons-down` | `<ChevronsDown />` |
| [chevrons-down-up](https://lucide.dev/icons/chevrons-down-up) | `chevrons-down-up` | `<ChevronsDownUp />` |
| [chevrons-left](https://lucide.dev/icons/chevrons-left) | `chevrons-left` | `<ChevronsLeft />` |
| [chevrons-left-right](https://lucide.dev/icons/chevrons-left-right) | `chevrons-left-right` | `<ChevronsLeftRight />` |
| [chevrons-right](https://lucide.dev/icons/chevrons-right) | `chevrons-right` | `<ChevronsRight />` |
| [chevrons-right-left](https://lucide.dev/icons/chevrons-right-left) | `chevrons-right-left` | `<ChevronsRightLeft />` |
| [chevrons-up](https://lucide.dev/icons/chevrons-up) | `chevrons-up` | `<ChevronsUp />` |
| [chevrons-up-down](https://lucide.dev/icons/chevrons-up-down) | `chevrons-up-down` | `<ChevronsUpDown />` |
| [circle-arrow-down](https://lucide.dev/icons/circle-arrow-down) | `circle-arrow-down` | `<CircleArrowDown />` |
| [circle-arrow-left](https://lucide.dev/icons/circle-arrow-left) | `circle-arrow-left` | `<CircleArrowLeft />` |
| [circle-arrow-out-down-left](https://lucide.dev/icons/circle-arrow-out-down-left) | `circle-arrow-out-down-left` | `<CircleArrowOutDownLeft />` |
| [circle-arrow-out-down-right](https://lucide.dev/icons/circle-arrow-out-down-right) | `circle-arrow-out-down-right` | `<CircleArrowOutDownRight />` |
| [circle-arrow-out-up-left](https://lucide.dev/icons/circle-arrow-out-up-left) | `circle-arrow-out-up-left` | `<CircleArrowOutUpLeft />` |
| [circle-arrow-out-up-right](https://lucide.dev/icons/circle-arrow-out-up-right) | `circle-arrow-out-up-right` | `<CircleArrowOutUpRight />` |
| [circle-arrow-right](https://lucide.dev/icons/circle-arrow-right) | `circle-arrow-right` | `<CircleArrowRight />` |
| [circle-arrow-up](https://lucide.dev/icons/circle-arrow-up) | `circle-arrow-up` | `<CircleArrowUp />` |
| [circle-chevron-down](https://lucide.dev/icons/circle-chevron-down) | `circle-chevron-down` | `<CircleChevronDown />` |
| [circle-chevron-left](https://lucide.dev/icons/circle-chevron-left) | `circle-chevron-left` | `<CircleChevronLeft />` |
| [circle-chevron-right](https://lucide.dev/icons/circle-chevron-right) | `circle-chevron-right` | `<CircleChevronRight />` |
| [circle-chevron-up](https://lucide.dev/icons/circle-chevron-up) | `circle-chevron-up` | `<CircleChevronUp />` |
| [circle-fading-arrow-up](https://lucide.dev/icons/circle-fading-arrow-up) | `circle-fading-arrow-up` | `<CircleFadingArrowUp />` |
| [clipboard-copy](https://lucide.dev/icons/clipboard-copy) | `clipboard-copy` | `<ClipboardCopy />` |
| [clipboard-paste](https://lucide.dev/icons/clipboard-paste) | `clipboard-paste` | `<ClipboardPaste />` |
| [cloud-download](https://lucide.dev/icons/cloud-download) | `cloud-download` | `<CloudDownload />` |
| [cloud-upload](https://lucide.dev/icons/cloud-upload) | `cloud-upload` | `<CloudUpload />` |
| [corner-down-left](https://lucide.dev/icons/corner-down-left) | `corner-down-left` | `<CornerDownLeft />` |
| [corner-down-right](https://lucide.dev/icons/corner-down-right) | `corner-down-right` | `<CornerDownRight />` |
| [corner-left-down](https://lucide.dev/icons/corner-left-down) | `corner-left-down` | `<CornerLeftDown />` |
| [corner-left-up](https://lucide.dev/icons/corner-left-up) | `corner-left-up` | `<CornerLeftUp />` |
| [corner-right-down](https://lucide.dev/icons/corner-right-down) | `corner-right-down` | `<CornerRightDown />` |
| [corner-right-up](https://lucide.dev/icons/corner-right-up) | `corner-right-up` | `<CornerRightUp />` |
| [corner-up-left](https://lucide.dev/icons/corner-up-left) | `corner-up-left` | `<CornerUpLeft />` |
| [corner-up-right](https://lucide.dev/icons/corner-up-right) | `corner-up-right` | `<CornerUpRight />` |
| [database-backup](https://lucide.dev/icons/database-backup) | `database-backup` | `<DatabaseBackup />` |
| [decimals-arrow-left](https://lucide.dev/icons/decimals-arrow-left) | `decimals-arrow-left` | `<DecimalsArrowLeft />` |
| [decimals-arrow-right](https://lucide.dev/icons/decimals-arrow-right) | `decimals-arrow-right` | `<DecimalsArrowRight />` |
| [delete](https://lucide.dev/icons/delete) | `delete` | `<Delete />` |
| [download](https://lucide.dev/icons/download) | `download` | `<Download />` |
| [expand](https://lucide.dev/icons/expand) | `expand` | `<Expand />` |
| [external-link](https://lucide.dev/icons/external-link) | `external-link` | `<ExternalLink />` |
| [fast-forward](https://lucide.dev/icons/fast-forward) | `fast-forward` | `<FastForward />` |
| [file-down](https://lucide.dev/icons/file-down) | `file-down` | `<FileDown />` |
| [file-input](https://lucide.dev/icons/file-input) | `file-input` | `<FileInput />` |
| [file-output](https://lucide.dev/icons/file-output) | `file-output` | `<FileOutput />` |
| [file-up](https://lucide.dev/icons/file-up) | `file-up` | `<FileUp />` |
| [fold-horizontal](https://lucide.dev/icons/fold-horizontal) | `fold-horizontal` | `<FoldHorizontal />` |
| [fold-vertical](https://lucide.dev/icons/fold-vertical) | `fold-vertical` | `<FoldVertical />` |
| [folder-down](https://lucide.dev/icons/folder-down) | `folder-down` | `<FolderDown />` |
| [folder-input](https://lucide.dev/icons/folder-input) | `folder-input` | `<FolderInput />` |
| [folder-output](https://lucide.dev/icons/folder-output) | `folder-output` | `<FolderOutput />` |
| [folder-sync](https://lucide.dev/icons/folder-sync) | `folder-sync` | `<FolderSync />` |
| [folder-up](https://lucide.dev/icons/folder-up) | `folder-up` | `<FolderUp />` |
| [git-compare-arrows](https://lucide.dev/icons/git-compare-arrows) | `git-compare-arrows` | `<GitCompareArrows />` |
| [git-pull-request-arrow](https://lucide.dev/icons/git-pull-request-arrow) | `git-pull-request-arrow` | `<GitPullRequestArrow />` |
| [git-pull-request-create-arrow](https://lucide.dev/icons/git-pull-request-create-arrow) | `git-pull-request-create-arrow` | `<GitPullRequestCreateArrow />` |
| [hard-drive-download](https://lucide.dev/icons/hard-drive-download) | `hard-drive-download` | `<HardDriveDownload />` |
| [hard-drive-upload](https://lucide.dev/icons/hard-drive-upload) | `hard-drive-upload` | `<HardDriveUpload />` |
| [history](https://lucide.dev/icons/history) | `history` | `<History />` |
| [import](https://lucide.dev/icons/import) | `import` | `<Import />` |
| [iteration-ccw](https://lucide.dev/icons/iteration-ccw) | `iteration-ccw` | `<IterationCcw />` |
| [iteration-cw](https://lucide.dev/icons/iteration-cw) | `iteration-cw` | `<IterationCw />` |
| [lasso-select](https://lucide.dev/icons/lasso-select) | `lasso-select` | `<LassoSelect />` |
| [list-chevrons-down-up](https://lucide.dev/icons/list-chevrons-down-up) | `list-chevrons-down-up` | `<ListChevronsDownUp />` |
| [list-chevrons-up-down](https://lucide.dev/icons/list-chevrons-up-down) | `list-chevrons-up-down` | `<ListChevronsUpDown />` |
| [log-in](https://lucide.dev/icons/log-in) | `log-in` | `<LogIn />` |
| [log-out](https://lucide.dev/icons/log-out) | `log-out` | `<LogOut />` |
| [maximize-2](https://lucide.dev/icons/maximize-2) | `maximize-2` | `<Maximize2 />` |
| [merge](https://lucide.dev/icons/merge) | `merge` | `<Merge />` |
| [milestone](https://lucide.dev/icons/milestone) | `milestone` | `<Milestone />` |
| [minimize-2](https://lucide.dev/icons/minimize-2) | `minimize-2` | `<Minimize2 />` |
| [mouse-pointer](https://lucide.dev/icons/mouse-pointer) | `mouse-pointer` | `<MousePointer />` |
| [mouse-pointer-2](https://lucide.dev/icons/mouse-pointer-2) | `mouse-pointer-2` | `<MousePointer2 />` |
| [mouse-pointer-ban](https://lucide.dev/icons/mouse-pointer-ban) | `mouse-pointer-ban` | `<MousePointerBan />` |
| [mouse-pointer-click](https://lucide.dev/icons/mouse-pointer-click) | `mouse-pointer-click` | `<MousePointerClick />` |
| [move](https://lucide.dev/icons/move) | `move` | `<Move />` |
| [move-diagonal](https://lucide.dev/icons/move-diagonal) | `move-diagonal` | `<MoveDiagonal />` |
| [move-diagonal-2](https://lucide.dev/icons/move-diagonal-2) | `move-diagonal-2` | `<MoveDiagonal2 />` |
| [move-down](https://lucide.dev/icons/move-down) | `move-down` | `<MoveDown />` |
| [move-down-left](https://lucide.dev/icons/move-down-left) | `move-down-left` | `<MoveDownLeft />` |
| [move-down-right](https://lucide.dev/icons/move-down-right) | `move-down-right` | `<MoveDownRight />` |
| [move-horizontal](https://lucide.dev/icons/move-horizontal) | `move-horizontal` | `<MoveHorizontal />` |
| [move-left](https://lucide.dev/icons/move-left) | `move-left` | `<MoveLeft />` |
| [move-right](https://lucide.dev/icons/move-right) | `move-right` | `<MoveRight />` |
| [move-up](https://lucide.dev/icons/move-up) | `move-up` | `<MoveUp />` |
| [move-up-left](https://lucide.dev/icons/move-up-left) | `move-up-left` | `<MoveUpLeft />` |
| [move-up-right](https://lucide.dev/icons/move-up-right) | `move-up-right` | `<MoveUpRight />` |
| [move-vertical](https://lucide.dev/icons/move-vertical) | `move-vertical` | `<MoveVertical />` |
| [panel-bottom-close](https://lucide.dev/icons/panel-bottom-close) | `panel-bottom-close` | `<PanelBottomClose />` |
| [panel-bottom-open](https://lucide.dev/icons/panel-bottom-open) | `panel-bottom-open` | `<PanelBottomOpen />` |
| [panel-left-close](https://lucide.dev/icons/panel-left-close) | `panel-left-close` | `<PanelLeftClose />` |
| [panel-left-open](https://lucide.dev/icons/panel-left-open) | `panel-left-open` | `<PanelLeftOpen />` |
| [panel-right-close](https://lucide.dev/icons/panel-right-close) | `panel-right-close` | `<PanelRightClose />` |
| [panel-right-open](https://lucide.dev/icons/panel-right-open) | `panel-right-open` | `<PanelRightOpen />` |
| [panel-top-close](https://lucide.dev/icons/panel-top-close) | `panel-top-close` | `<PanelTopClose />` |
| [panel-top-open](https://lucide.dev/icons/panel-top-open) | `panel-top-open` | `<PanelTopOpen />` |
| [phone-forwarded](https://lucide.dev/icons/phone-forwarded) | `phone-forwarded` | `<PhoneForwarded />` |
| [phone-incoming](https://lucide.dev/icons/phone-incoming) | `phone-incoming` | `<PhoneIncoming />` |
| [phone-outgoing](https://lucide.dev/icons/phone-outgoing) | `phone-outgoing` | `<PhoneOutgoing />` |
| [play](https://lucide.dev/icons/play) | `play` | `<Play />` |
| [redo](https://lucide.dev/icons/redo) | `redo` | `<Redo />` |
| [redo-2](https://lucide.dev/icons/redo-2) | `redo-2` | `<Redo2 />` |
| [redo-dot](https://lucide.dev/icons/redo-dot) | `redo-dot` | `<RedoDot />` |
| [refresh-ccw](https://lucide.dev/icons/refresh-ccw) | `refresh-ccw` | `<RefreshCcw />` |
| [refresh-ccw-dot](https://lucide.dev/icons/refresh-ccw-dot) | `refresh-ccw-dot` | `<RefreshCcwDot />` |
| [refresh-cw](https://lucide.dev/icons/refresh-cw) | `refresh-cw` | `<RefreshCw />` |
| [refresh-cw-off](https://lucide.dev/icons/refresh-cw-off) | `refresh-cw-off` | `<RefreshCwOff />` |
| [repeat](https://lucide.dev/icons/repeat) | `repeat` | `<Repeat />` |
| [repeat-2](https://lucide.dev/icons/repeat-2) | `repeat-2` | `<Repeat2 />` |
| [rewind](https://lucide.dev/icons/rewind) | `rewind` | `<Rewind />` |
| [rotate-ccw](https://lucide.dev/icons/rotate-ccw) | `rotate-ccw` | `<RotateCcw />` |
| [rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) | `rotate-ccw-square` | `<RotateCcwSquare />` |
| [rotate-cw](https://lucide.dev/icons/rotate-cw) | `rotate-cw` | `<RotateCw />` |
| [rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) | `rotate-cw-square` | `<RotateCwSquare />` |
| [separator-horizontal](https://lucide.dev/icons/separator-horizontal) | `separator-horizontal` | `<SeparatorHorizontal />` |
| [separator-vertical](https://lucide.dev/icons/separator-vertical) | `separator-vertical` | `<SeparatorVertical />` |
| [shrink](https://lucide.dev/icons/shrink) | `shrink` | `<Shrink />` |
| [shuffle](https://lucide.dev/icons/shuffle) | `shuffle` | `<Shuffle />` |
| [signpost](https://lucide.dev/icons/signpost) | `signpost` | `<Signpost />` |
| [signpost-big](https://lucide.dev/icons/signpost-big) | `signpost-big` | `<SignpostBig />` |
| [skip-back](https://lucide.dev/icons/skip-back) | `skip-back` | `<SkipBack />` |
| [skip-forward](https://lucide.dev/icons/skip-forward) | `skip-forward` | `<SkipForward />` |
| [spline-pointer](https://lucide.dev/icons/spline-pointer) | `spline-pointer` | `<SplinePointer />` |
| [split](https://lucide.dev/icons/split) | `split` | `<Split />` |
| [square-arrow-down](https://lucide.dev/icons/square-arrow-down) | `square-arrow-down` | `<SquareArrowDown />` |
| [square-arrow-down-left](https://lucide.dev/icons/square-arrow-down-left) | `square-arrow-down-left` | `<SquareArrowDownLeft />` |
| [square-arrow-down-right](https://lucide.dev/icons/square-arrow-down-right) | `square-arrow-down-right` | `<SquareArrowDownRight />` |
| [square-arrow-left](https://lucide.dev/icons/square-arrow-left) | `square-arrow-left` | `<SquareArrowLeft />` |
| [square-arrow-out-down-left](https://lucide.dev/icons/square-arrow-out-down-left) | `square-arrow-out-down-left` | `<SquareArrowOutDownLeft />` |
| [square-arrow-out-down-right](https://lucide.dev/icons/square-arrow-out-down-right) | `square-arrow-out-down-right` | `<SquareArrowOutDownRight />` |
| [square-arrow-out-up-left](https://lucide.dev/icons/square-arrow-out-up-left) | `square-arrow-out-up-left` | `<SquareArrowOutUpLeft />` |
| [square-arrow-out-up-right](https://lucide.dev/icons/square-arrow-out-up-right) | `square-arrow-out-up-right` | `<SquareArrowOutUpRight />` |
| [square-arrow-right](https://lucide.dev/icons/square-arrow-right) | `square-arrow-right` | `<SquareArrowRight />` |
| [square-arrow-up](https://lucide.dev/icons/square-arrow-up) | `square-arrow-up` | `<SquareArrowUp />` |
| [square-arrow-up-left](https://lucide.dev/icons/square-arrow-up-left) | `square-arrow-up-left` | `<SquareArrowUpLeft />` |
| [square-arrow-up-right](https://lucide.dev/icons/square-arrow-up-right) | `square-arrow-up-right` | `<SquareArrowUpRight />` |
| [square-chevron-down](https://lucide.dev/icons/square-chevron-down) | `square-chevron-down` | `<SquareChevronDown />` |
| [square-chevron-left](https://lucide.dev/icons/square-chevron-left) | `square-chevron-left` | `<SquareChevronLeft />` |
| [square-chevron-right](https://lucide.dev/icons/square-chevron-right) | `square-chevron-right` | `<SquareChevronRight />` |
| [square-chevron-up](https://lucide.dev/icons/square-chevron-up) | `square-chevron-up` | `<SquareChevronUp />` |
| [square-dashed-mouse-pointer](https://lucide.dev/icons/square-dashed-mouse-pointer) | `square-dashed-mouse-pointer` | `<SquareDashedMousePointer />` |
| [square-mouse-pointer](https://lucide.dev/icons/square-mouse-pointer) | `square-mouse-pointer` | `<SquareMousePointer />` |
| [square-play](https://lucide.dev/icons/square-play) | `square-play` | `<SquarePlay />` |
| [step-back](https://lucide.dev/icons/step-back) | `step-back` | `<StepBack />` |
| [step-forward](https://lucide.dev/icons/step-forward) | `step-forward` | `<StepForward />` |
| [sunrise](https://lucide.dev/icons/sunrise) | `sunrise` | `<Sunrise />` |
| [sunset](https://lucide.dev/icons/sunset) | `sunset` | `<Sunset />` |
| [text-wrap](https://lucide.dev/icons/text-wrap) | `text-wrap` | `<TextWrap />` |
| [trending-down](https://lucide.dev/icons/trending-down) | `trending-down` | `<TrendingDown />` |
| [trending-up](https://lucide.dev/icons/trending-up) | `trending-up` | `<TrendingUp />` |
| [trending-up-down](https://lucide.dev/icons/trending-up-down) | `trending-up-down` | `<TrendingUpDown />` |
| [undo](https://lucide.dev/icons/undo) | `undo` | `<Undo />` |
| [undo-2](https://lucide.dev/icons/undo-2) | `undo-2` | `<Undo2 />` |
| [undo-dot](https://lucide.dev/icons/undo-dot) | `undo-dot` | `<UndoDot />` |
| [unfold-horizontal](https://lucide.dev/icons/unfold-horizontal) | `unfold-horizontal` | `<UnfoldHorizontal />` |
| [unfold-vertical](https://lucide.dev/icons/unfold-vertical) | `unfold-vertical` | `<UnfoldVertical />` |
| [upload](https://lucide.dev/icons/upload) | `upload` | `<Upload />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "arrows"] or ["leptos", "all-icons"]
<ArrowBigDown class="w-6 h-6 text-gray-600" />
```

### Brands

`brands` and `all-icons` features - 21 icons

| Icon | Name | Component |
|------|------|-----------|
| [airplay](https://lucide.dev/icons/airplay) | `airplay` | `<Airplay />` |
| [bitcoin](https://lucide.dev/icons/bitcoin) | `bitcoin` | `<Bitcoin />` |
| [chromium](https://lucide.dev/icons/chromium) | `chromium` | `<Chromium />` |
| [codepen](https://lucide.dev/icons/codepen) | `codepen` | `<Codepen />` |
| [codesandbox](https://lucide.dev/icons/codesandbox) | `codesandbox` | `<Codesandbox />` |
| [dribbble](https://lucide.dev/icons/dribbble) | `dribbble` | `<Dribbble />` |
| [facebook](https://lucide.dev/icons/facebook) | `facebook` | `<Facebook />` |
| [figma](https://lucide.dev/icons/figma) | `figma` | `<Figma />` |
| [framer](https://lucide.dev/icons/framer) | `framer` | `<Framer />` |
| [github](https://lucide.dev/icons/github) | `github` | `<Github />` |
| [gitlab](https://lucide.dev/icons/gitlab) | `gitlab` | `<Gitlab />` |
| [hexagon](https://lucide.dev/icons/hexagon) | `hexagon` | `<Hexagon />` |
| [instagram](https://lucide.dev/icons/instagram) | `instagram` | `<Instagram />` |
| [linkedin](https://lucide.dev/icons/linkedin) | `linkedin` | `<Linkedin />` |
| [pocket](https://lucide.dev/icons/pocket) | `pocket` | `<Pocket />` |
| [slack](https://lucide.dev/icons/slack) | `slack` | `<Slack />` |
| [target](https://lucide.dev/icons/target) | `target` | `<Target />` |
| [trello](https://lucide.dev/icons/trello) | `trello` | `<Trello />` |
| [twitch](https://lucide.dev/icons/twitch) | `twitch` | `<Twitch />` |
| [twitter](https://lucide.dev/icons/twitter) | `twitter` | `<Twitter />` |
| [youtube](https://lucide.dev/icons/youtube) | `youtube` | `<Youtube />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "brands"] or ["leptos", "all-icons"]
<Airplay class="w-6 h-6 text-gray-600" />
```

### Buildings

`buildings` and `all-icons` features - 25 icons

| Icon | Name | Component |
|------|------|-----------|
| [anvil](https://lucide.dev/icons/anvil) | `anvil` | `<Anvil />` |
| [brick-wall](https://lucide.dev/icons/brick-wall) | `brick-wall` | `<BrickWall />` |
| [building](https://lucide.dev/icons/building) | `building` | `<Building />` |
| [building-2](https://lucide.dev/icons/building-2) | `building-2` | `<Building2 />` |
| [castle](https://lucide.dev/icons/castle) | `castle` | `<Castle />` |
| [church](https://lucide.dev/icons/church) | `church` | `<Church />` |
| [cuboid](https://lucide.dev/icons/cuboid) | `cuboid` | `<Cuboid />` |
| [dam](https://lucide.dev/icons/dam) | `dam` | `<Dam />` |
| [factory](https://lucide.dev/icons/factory) | `factory` | `<Factory />` |
| [fence](https://lucide.dev/icons/fence) | `fence` | `<Fence />` |
| [graduation-cap](https://lucide.dev/icons/graduation-cap) | `graduation-cap` | `<GraduationCap />` |
| [hospital](https://lucide.dev/icons/hospital) | `hospital` | `<Hospital />` |
| [hotel](https://lucide.dev/icons/hotel) | `hotel` | `<Hotel />` |
| [house](https://lucide.dev/icons/house) | `house` | `<House />` |
| [house-heart](https://lucide.dev/icons/house-heart) | `house-heart` | `<HouseHeart />` |
| [house-plug](https://lucide.dev/icons/house-plug) | `house-plug` | `<HousePlug />` |
| [house-plus](https://lucide.dev/icons/house-plus) | `house-plus` | `<HousePlus />` |
| [house-wifi](https://lucide.dev/icons/house-wifi) | `house-wifi` | `<HouseWifi />` |
| [landmark](https://lucide.dev/icons/landmark) | `landmark` | `<Landmark />` |
| [school](https://lucide.dev/icons/school) | `school` | `<School />` |
| [store](https://lucide.dev/icons/store) | `store` | `<Store />` |
| [theater](https://lucide.dev/icons/theater) | `theater` | `<Theater />` |
| [university](https://lucide.dev/icons/university) | `university` | `<University />` |
| [utility-pole](https://lucide.dev/icons/utility-pole) | `utility-pole` | `<UtilityPole />` |
| [warehouse](https://lucide.dev/icons/warehouse) | `warehouse` | `<Warehouse />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "buildings"] or ["leptos", "all-icons"]
<Anvil class="w-6 h-6 text-gray-600" />
```

### Charts

`charts` and `all-icons` features - 31 icons

| Icon | Name | Component |
|------|------|-----------|
| [chart-area](https://lucide.dev/icons/chart-area) | `chart-area` | `<ChartArea />` |
| [chart-bar](https://lucide.dev/icons/chart-bar) | `chart-bar` | `<ChartBar />` |
| [chart-bar-big](https://lucide.dev/icons/chart-bar-big) | `chart-bar-big` | `<ChartBarBig />` |
| [chart-bar-decreasing](https://lucide.dev/icons/chart-bar-decreasing) | `chart-bar-decreasing` | `<ChartBarDecreasing />` |
| [chart-bar-increasing](https://lucide.dev/icons/chart-bar-increasing) | `chart-bar-increasing` | `<ChartBarIncreasing />` |
| [chart-bar-stacked](https://lucide.dev/icons/chart-bar-stacked) | `chart-bar-stacked` | `<ChartBarStacked />` |
| [chart-candlestick](https://lucide.dev/icons/chart-candlestick) | `chart-candlestick` | `<ChartCandlestick />` |
| [chart-column](https://lucide.dev/icons/chart-column) | `chart-column` | `<ChartColumn />` |
| [chart-column-big](https://lucide.dev/icons/chart-column-big) | `chart-column-big` | `<ChartColumnBig />` |
| [chart-column-decreasing](https://lucide.dev/icons/chart-column-decreasing) | `chart-column-decreasing` | `<ChartColumnDecreasing />` |
| [chart-column-increasing](https://lucide.dev/icons/chart-column-increasing) | `chart-column-increasing` | `<ChartColumnIncreasing />` |
| [chart-column-stacked](https://lucide.dev/icons/chart-column-stacked) | `chart-column-stacked` | `<ChartColumnStacked />` |
| [chart-gantt](https://lucide.dev/icons/chart-gantt) | `chart-gantt` | `<ChartGantt />` |
| [chart-line](https://lucide.dev/icons/chart-line) | `chart-line` | `<ChartLine />` |
| [chart-network](https://lucide.dev/icons/chart-network) | `chart-network` | `<ChartNetwork />` |
| [chart-no-axes-column](https://lucide.dev/icons/chart-no-axes-column) | `chart-no-axes-column` | `<ChartNoAxesColumn />` |
| [chart-no-axes-column-decreasing](https://lucide.dev/icons/chart-no-axes-column-decreasing) | `chart-no-axes-column-decreasing` | `<ChartNoAxesColumnDecreasing />` |
| [chart-no-axes-column-increasing](https://lucide.dev/icons/chart-no-axes-column-increasing) | `chart-no-axes-column-increasing` | `<ChartNoAxesColumnIncreasing />` |
| [chart-no-axes-combined](https://lucide.dev/icons/chart-no-axes-combined) | `chart-no-axes-combined` | `<ChartNoAxesCombined />` |
| [chart-no-axes-gantt](https://lucide.dev/icons/chart-no-axes-gantt) | `chart-no-axes-gantt` | `<ChartNoAxesGantt />` |
| [chart-pie](https://lucide.dev/icons/chart-pie) | `chart-pie` | `<ChartPie />` |
| [chart-scatter](https://lucide.dev/icons/chart-scatter) | `chart-scatter` | `<ChartScatter />` |
| [chart-spline](https://lucide.dev/icons/chart-spline) | `chart-spline` | `<ChartSpline />` |
| [folder-kanban](https://lucide.dev/icons/folder-kanban) | `folder-kanban` | `<FolderKanban />` |
| [kanban](https://lucide.dev/icons/kanban) | `kanban` | `<Kanban />` |
| [square-chart-gantt](https://lucide.dev/icons/square-chart-gantt) | `square-chart-gantt` | `<SquareChartGantt />` |
| [square-dashed-kanban](https://lucide.dev/icons/square-dashed-kanban) | `square-dashed-kanban` | `<SquareDashedKanban />` |
| [square-kanban](https://lucide.dev/icons/square-kanban) | `square-kanban` | `<SquareKanban />` |
| [trending-down](https://lucide.dev/icons/trending-down) | `trending-down` | `<TrendingDown />` |
| [trending-up](https://lucide.dev/icons/trending-up) | `trending-up` | `<TrendingUp />` |
| [trending-up-down](https://lucide.dev/icons/trending-up-down) | `trending-up-down` | `<TrendingUpDown />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "charts"] or ["leptos", "all-icons"]
<ChartArea class="w-6 h-6 text-gray-600" />
```

### Communication

`communication` and `all-icons` features - 54 icons

| Icon | Name | Component |
|------|------|-----------|
| [antenna](https://lucide.dev/icons/antenna) | `antenna` | `<Antenna />` |
| [audio-lines](https://lucide.dev/icons/audio-lines) | `audio-lines` | `<AudioLines />` |
| [audio-waveform](https://lucide.dev/icons/audio-waveform) | `audio-waveform` | `<AudioWaveform />` |
| [book-user](https://lucide.dev/icons/book-user) | `book-user` | `<BookUser />` |
| [camera](https://lucide.dev/icons/camera) | `camera` | `<Camera />` |
| [camera-off](https://lucide.dev/icons/camera-off) | `camera-off` | `<CameraOff />` |
| [card-sim](https://lucide.dev/icons/card-sim) | `card-sim` | `<CardSim />` |
| [cassette-tape](https://lucide.dev/icons/cassette-tape) | `cassette-tape` | `<CassetteTape />` |
| [cctv](https://lucide.dev/icons/cctv) | `cctv` | `<Cctv />` |
| [chevrons-left-right-ellipsis](https://lucide.dev/icons/chevrons-left-right-ellipsis) | `chevrons-left-right-ellipsis` | `<ChevronsLeftRightEllipsis />` |
| [circle-fading-plus](https://lucide.dev/icons/circle-fading-plus) | `circle-fading-plus` | `<CircleFadingPlus />` |
| [contact](https://lucide.dev/icons/contact) | `contact` | `<Contact />` |
| [contact-round](https://lucide.dev/icons/contact-round) | `contact-round` | `<ContactRound />` |
| [ethernet-port](https://lucide.dev/icons/ethernet-port) | `ethernet-port` | `<EthernetPort />` |
| [hand-fist](https://lucide.dev/icons/hand-fist) | `hand-fist` | `<HandFist />` |
| [handshake](https://lucide.dev/icons/handshake) | `handshake` | `<Handshake />` |
| [headphone-off](https://lucide.dev/icons/headphone-off) | `headphone-off` | `<HeadphoneOff />` |
| [lectern](https://lucide.dev/icons/lectern) | `lectern` | `<Lectern />` |
| [mic](https://lucide.dev/icons/mic) | `mic` | `<Mic />` |
| [mic-off](https://lucide.dev/icons/mic-off) | `mic-off` | `<MicOff />` |
| [newspaper](https://lucide.dev/icons/newspaper) | `newspaper` | `<Newspaper />` |
| [nfc](https://lucide.dev/icons/nfc) | `nfc` | `<Nfc />` |
| [notebook](https://lucide.dev/icons/notebook) | `notebook` | `<Notebook />` |
| [notebook-tabs](https://lucide.dev/icons/notebook-tabs) | `notebook-tabs` | `<NotebookTabs />` |
| [phone](https://lucide.dev/icons/phone) | `phone` | `<Phone />` |
| [phone-call](https://lucide.dev/icons/phone-call) | `phone-call` | `<PhoneCall />` |
| [phone-forwarded](https://lucide.dev/icons/phone-forwarded) | `phone-forwarded` | `<PhoneForwarded />` |
| [phone-incoming](https://lucide.dev/icons/phone-incoming) | `phone-incoming` | `<PhoneIncoming />` |
| [phone-missed](https://lucide.dev/icons/phone-missed) | `phone-missed` | `<PhoneMissed />` |
| [phone-off](https://lucide.dev/icons/phone-off) | `phone-off` | `<PhoneOff />` |
| [phone-outgoing](https://lucide.dev/icons/phone-outgoing) | `phone-outgoing` | `<PhoneOutgoing />` |
| [presentation](https://lucide.dev/icons/presentation) | `presentation` | `<Presentation />` |
| [projector](https://lucide.dev/icons/projector) | `projector` | `<Projector />` |
| [radar](https://lucide.dev/icons/radar) | `radar` | `<Radar />` |
| [screen-share](https://lucide.dev/icons/screen-share) | `screen-share` | `<ScreenShare />` |
| [screen-share-off](https://lucide.dev/icons/screen-share-off) | `screen-share-off` | `<ScreenShareOff />` |
| [send](https://lucide.dev/icons/send) | `send` | `<Send />` |
| [send-horizontal](https://lucide.dev/icons/send-horizontal) | `send-horizontal` | `<SendHorizontal />` |
| [smartphone-nfc](https://lucide.dev/icons/smartphone-nfc) | `smartphone-nfc` | `<SmartphoneNfc />` |
| [smile-plus](https://lucide.dev/icons/smile-plus) | `smile-plus` | `<SmilePlus />` |
| [speech](https://lucide.dev/icons/speech) | `speech` | `<Speech />` |
| [spool](https://lucide.dev/icons/spool) | `spool` | `<Spool />` |
| [spotlight](https://lucide.dev/icons/spotlight) | `spotlight` | `<Spotlight />` |
| [switch-camera](https://lucide.dev/icons/switch-camera) | `switch-camera` | `<SwitchCamera />` |
| [tv](https://lucide.dev/icons/tv) | `tv` | `<Tv />` |
| [video](https://lucide.dev/icons/video) | `video` | `<Video />` |
| [video-off](https://lucide.dev/icons/video-off) | `video-off` | `<VideoOff />` |
| [videotape](https://lucide.dev/icons/videotape) | `videotape` | `<Videotape />` |
| [volume](https://lucide.dev/icons/volume) | `volume` | `<Volume />` |
| [volume-1](https://lucide.dev/icons/volume-1) | `volume-1` | `<Volume1 />` |
| [volume-2](https://lucide.dev/icons/volume-2) | `volume-2` | `<Volume2 />` |
| [volume-off](https://lucide.dev/icons/volume-off) | `volume-off` | `<VolumeOff />` |
| [volume-x](https://lucide.dev/icons/volume-x) | `volume-x` | `<VolumeX />` |
| [webcam](https://lucide.dev/icons/webcam) | `webcam` | `<Webcam />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "communication"] or ["leptos", "all-icons"]
<Antenna class="w-6 h-6 text-gray-600" />
```

### Connectivity

`connectivity` and `all-icons` features - 89 icons

| Icon | Name | Component |
|------|------|-----------|
| [airplay](https://lucide.dev/icons/airplay) | `airplay` | `<Airplay />` |
| [battery](https://lucide.dev/icons/battery) | `battery` | `<Battery />` |
| [battery-charging](https://lucide.dev/icons/battery-charging) | `battery-charging` | `<BatteryCharging />` |
| [battery-full](https://lucide.dev/icons/battery-full) | `battery-full` | `<BatteryFull />` |
| [battery-low](https://lucide.dev/icons/battery-low) | `battery-low` | `<BatteryLow />` |
| [battery-medium](https://lucide.dev/icons/battery-medium) | `battery-medium` | `<BatteryMedium />` |
| [battery-warning](https://lucide.dev/icons/battery-warning) | `battery-warning` | `<BatteryWarning />` |
| [bluetooth](https://lucide.dev/icons/bluetooth) | `bluetooth` | `<Bluetooth />` |
| [bluetooth-connected](https://lucide.dev/icons/bluetooth-connected) | `bluetooth-connected` | `<BluetoothConnected />` |
| [bluetooth-off](https://lucide.dev/icons/bluetooth-off) | `bluetooth-off` | `<BluetoothOff />` |
| [bluetooth-searching](https://lucide.dev/icons/bluetooth-searching) | `bluetooth-searching` | `<BluetoothSearching />` |
| [book-user](https://lucide.dev/icons/book-user) | `book-user` | `<BookUser />` |
| [brick-wall-fire](https://lucide.dev/icons/brick-wall-fire) | `brick-wall-fire` | `<BrickWallFire />` |
| [brick-wall-shield](https://lucide.dev/icons/brick-wall-shield) | `brick-wall-shield` | `<BrickWallShield />` |
| [cable](https://lucide.dev/icons/cable) | `cable` | `<Cable />` |
| [card-sim](https://lucide.dev/icons/card-sim) | `card-sim` | `<CardSim />` |
| [cassette-tape](https://lucide.dev/icons/cassette-tape) | `cassette-tape` | `<CassetteTape />` |
| [cast](https://lucide.dev/icons/cast) | `cast` | `<Cast />` |
| [cctv](https://lucide.dev/icons/cctv) | `cctv` | `<Cctv />` |
| [circle-power](https://lucide.dev/icons/circle-power) | `circle-power` | `<CirclePower />` |
| [cloud-off](https://lucide.dev/icons/cloud-off) | `cloud-off` | `<CloudOff />` |
| [contact](https://lucide.dev/icons/contact) | `contact` | `<Contact />` |
| [contact-round](https://lucide.dev/icons/contact-round) | `contact-round` | `<ContactRound />` |
| [headphone-off](https://lucide.dev/icons/headphone-off) | `headphone-off` | `<HeadphoneOff />` |
| [headphones](https://lucide.dev/icons/headphones) | `headphones` | `<Headphones />` |
| [headset](https://lucide.dev/icons/headset) | `headset` | `<Headset />` |
| [house-wifi](https://lucide.dev/icons/house-wifi) | `house-wifi` | `<HouseWifi />` |
| [mic](https://lucide.dev/icons/mic) | `mic` | `<Mic />` |
| [mic-off](https://lucide.dev/icons/mic-off) | `mic-off` | `<MicOff />` |
| [monitor](https://lucide.dev/icons/monitor) | `monitor` | `<Monitor />` |
| [monitor-check](https://lucide.dev/icons/monitor-check) | `monitor-check` | `<MonitorCheck />` |
| [monitor-cog](https://lucide.dev/icons/monitor-cog) | `monitor-cog` | `<MonitorCog />` |
| [monitor-dot](https://lucide.dev/icons/monitor-dot) | `monitor-dot` | `<MonitorDot />` |
| [monitor-down](https://lucide.dev/icons/monitor-down) | `monitor-down` | `<MonitorDown />` |
| [monitor-off](https://lucide.dev/icons/monitor-off) | `monitor-off` | `<MonitorOff />` |
| [monitor-pause](https://lucide.dev/icons/monitor-pause) | `monitor-pause` | `<MonitorPause />` |
| [monitor-play](https://lucide.dev/icons/monitor-play) | `monitor-play` | `<MonitorPlay />` |
| [monitor-smartphone](https://lucide.dev/icons/monitor-smartphone) | `monitor-smartphone` | `<MonitorSmartphone />` |
| [monitor-speaker](https://lucide.dev/icons/monitor-speaker) | `monitor-speaker` | `<MonitorSpeaker />` |
| [monitor-stop](https://lucide.dev/icons/monitor-stop) | `monitor-stop` | `<MonitorStop />` |
| [monitor-up](https://lucide.dev/icons/monitor-up) | `monitor-up` | `<MonitorUp />` |
| [monitor-x](https://lucide.dev/icons/monitor-x) | `monitor-x` | `<MonitorX />` |
| [phone](https://lucide.dev/icons/phone) | `phone` | `<Phone />` |
| [phone-call](https://lucide.dev/icons/phone-call) | `phone-call` | `<PhoneCall />` |
| [phone-forwarded](https://lucide.dev/icons/phone-forwarded) | `phone-forwarded` | `<PhoneForwarded />` |
| [phone-incoming](https://lucide.dev/icons/phone-incoming) | `phone-incoming` | `<PhoneIncoming />` |
| [phone-missed](https://lucide.dev/icons/phone-missed) | `phone-missed` | `<PhoneMissed />` |
| [phone-off](https://lucide.dev/icons/phone-off) | `phone-off` | `<PhoneOff />` |
| [phone-outgoing](https://lucide.dev/icons/phone-outgoing) | `phone-outgoing` | `<PhoneOutgoing />` |
| [power](https://lucide.dev/icons/power) | `power` | `<Power />` |
| [power-off](https://lucide.dev/icons/power-off) | `power-off` | `<PowerOff />` |
| [rectangle-goggles](https://lucide.dev/icons/rectangle-goggles) | `rectangle-goggles` | `<RectangleGoggles />` |
| [router](https://lucide.dev/icons/router) | `router` | `<Router />` |
| [satellite](https://lucide.dev/icons/satellite) | `satellite` | `<Satellite />` |
| [satellite-dish](https://lucide.dev/icons/satellite-dish) | `satellite-dish` | `<SatelliteDish />` |
| [screen-share](https://lucide.dev/icons/screen-share) | `screen-share` | `<ScreenShare />` |
| [screen-share-off](https://lucide.dev/icons/screen-share-off) | `screen-share-off` | `<ScreenShareOff />` |
| [send](https://lucide.dev/icons/send) | `send` | `<Send />` |
| [send-horizontal](https://lucide.dev/icons/send-horizontal) | `send-horizontal` | `<SendHorizontal />` |
| [signal](https://lucide.dev/icons/signal) | `signal` | `<Signal />` |
| [signal-high](https://lucide.dev/icons/signal-high) | `signal-high` | `<SignalHigh />` |
| [signal-low](https://lucide.dev/icons/signal-low) | `signal-low` | `<SignalLow />` |
| [signal-medium](https://lucide.dev/icons/signal-medium) | `signal-medium` | `<SignalMedium />` |
| [signal-zero](https://lucide.dev/icons/signal-zero) | `signal-zero` | `<SignalZero />` |
| [smartphone](https://lucide.dev/icons/smartphone) | `smartphone` | `<Smartphone />` |
| [smartphone-charging](https://lucide.dev/icons/smartphone-charging) | `smartphone-charging` | `<SmartphoneCharging />` |
| [square-power](https://lucide.dev/icons/square-power) | `square-power` | `<SquarePower />` |
| [vibrate](https://lucide.dev/icons/vibrate) | `vibrate` | `<Vibrate />` |
| [vibrate-off](https://lucide.dev/icons/vibrate-off) | `vibrate-off` | `<VibrateOff />` |
| [video](https://lucide.dev/icons/video) | `video` | `<Video />` |
| [video-off](https://lucide.dev/icons/video-off) | `video-off` | `<VideoOff />` |
| [videotape](https://lucide.dev/icons/videotape) | `videotape` | `<Videotape />` |
| [voicemail](https://lucide.dev/icons/voicemail) | `voicemail` | `<Voicemail />` |
| [volume](https://lucide.dev/icons/volume) | `volume` | `<Volume />` |
| [volume-1](https://lucide.dev/icons/volume-1) | `volume-1` | `<Volume1 />` |
| [volume-2](https://lucide.dev/icons/volume-2) | `volume-2` | `<Volume2 />` |
| [volume-off](https://lucide.dev/icons/volume-off) | `volume-off` | `<VolumeOff />` |
| [volume-x](https://lucide.dev/icons/volume-x) | `volume-x` | `<VolumeX />` |
| [webcam](https://lucide.dev/icons/webcam) | `webcam` | `<Webcam />` |
| [wifi](https://lucide.dev/icons/wifi) | `wifi` | `<Wifi />` |
| [wifi-cog](https://lucide.dev/icons/wifi-cog) | `wifi-cog` | `<WifiCog />` |
| [wifi-high](https://lucide.dev/icons/wifi-high) | `wifi-high` | `<WifiHigh />` |
| [wifi-low](https://lucide.dev/icons/wifi-low) | `wifi-low` | `<WifiLow />` |
| [wifi-off](https://lucide.dev/icons/wifi-off) | `wifi-off` | `<WifiOff />` |
| [wifi-pen](https://lucide.dev/icons/wifi-pen) | `wifi-pen` | `<WifiPen />` |
| [wifi-sync](https://lucide.dev/icons/wifi-sync) | `wifi-sync` | `<WifiSync />` |
| [wifi-zero](https://lucide.dev/icons/wifi-zero) | `wifi-zero` | `<WifiZero />` |
| [zap](https://lucide.dev/icons/zap) | `zap` | `<Zap />` |
| [zap-off](https://lucide.dev/icons/zap-off) | `zap-off` | `<ZapOff />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "connectivity"] or ["leptos", "all-icons"]
<Airplay class="w-6 h-6 text-gray-600" />
```

### Cursors

`cursors` and `all-icons` features - 32 icons

| Icon | Name | Component |
|------|------|-----------|
| [circle-plus](https://lucide.dev/icons/circle-plus) | `circle-plus` | `<CirclePlus />` |
| [hand](https://lucide.dev/icons/hand) | `hand` | `<Hand />` |
| [hand-grab](https://lucide.dev/icons/hand-grab) | `hand-grab` | `<HandGrab />` |
| [lasso](https://lucide.dev/icons/lasso) | `lasso` | `<Lasso />` |
| [lasso-select](https://lucide.dev/icons/lasso-select) | `lasso-select` | `<LassoSelect />` |
| [loader](https://lucide.dev/icons/loader) | `loader` | `<Loader />` |
| [loader-circle](https://lucide.dev/icons/loader-circle) | `loader-circle` | `<LoaderCircle />` |
| [loader-pinwheel](https://lucide.dev/icons/loader-pinwheel) | `loader-pinwheel` | `<LoaderPinwheel />` |
| [mouse-pointer](https://lucide.dev/icons/mouse-pointer) | `mouse-pointer` | `<MousePointer />` |
| [mouse-pointer-2](https://lucide.dev/icons/mouse-pointer-2) | `mouse-pointer-2` | `<MousePointer2 />` |
| [mouse-pointer-ban](https://lucide.dev/icons/mouse-pointer-ban) | `mouse-pointer-ban` | `<MousePointerBan />` |
| [mouse-pointer-click](https://lucide.dev/icons/mouse-pointer-click) | `mouse-pointer-click` | `<MousePointerClick />` |
| [move](https://lucide.dev/icons/move) | `move` | `<Move />` |
| [move-diagonal](https://lucide.dev/icons/move-diagonal) | `move-diagonal` | `<MoveDiagonal />` |
| [move-diagonal-2](https://lucide.dev/icons/move-diagonal-2) | `move-diagonal-2` | `<MoveDiagonal2 />` |
| [move-horizontal](https://lucide.dev/icons/move-horizontal) | `move-horizontal` | `<MoveHorizontal />` |
| [move-vertical](https://lucide.dev/icons/move-vertical) | `move-vertical` | `<MoveVertical />` |
| [pen-tool](https://lucide.dev/icons/pen-tool) | `pen-tool` | `<PenTool />` |
| [pencil](https://lucide.dev/icons/pencil) | `pencil` | `<Pencil />` |
| [pencil-off](https://lucide.dev/icons/pencil-off) | `pencil-off` | `<PencilOff />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [pointer](https://lucide.dev/icons/pointer) | `pointer` | `<Pointer />` |
| [pointer-off](https://lucide.dev/icons/pointer-off) | `pointer-off` | `<PointerOff />` |
| [sparkles](https://lucide.dev/icons/sparkles) | `sparkles` | `<Sparkles />` |
| [spline-pointer](https://lucide.dev/icons/spline-pointer) | `spline-pointer` | `<SplinePointer />` |
| [square-dashed-mouse-pointer](https://lucide.dev/icons/square-dashed-mouse-pointer) | `square-dashed-mouse-pointer` | `<SquareDashedMousePointer />` |
| [square-mouse-pointer](https://lucide.dev/icons/square-mouse-pointer) | `square-mouse-pointer` | `<SquareMousePointer />` |
| [stamp](https://lucide.dev/icons/stamp) | `stamp` | `<Stamp />` |
| [text-cursor](https://lucide.dev/icons/text-cursor) | `text-cursor` | `<TextCursor />` |
| [text-select](https://lucide.dev/icons/text-select) | `text-select` | `<TextSelect />` |
| [wand](https://lucide.dev/icons/wand) | `wand` | `<Wand />` |
| [wand-sparkles](https://lucide.dev/icons/wand-sparkles) | `wand-sparkles` | `<WandSparkles />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "cursors"] or ["leptos", "all-icons"]
<CirclePlus class="w-6 h-6 text-gray-600" />
```

### Design

`design` and `all-icons` features - 144 icons

| Icon | Name | Component |
|------|------|-----------|
| [a-arrow-down](https://lucide.dev/icons/a-arrow-down) | `a-arrow-down` | `<AArrowDown />` |
| [a-arrow-up](https://lucide.dev/icons/a-arrow-up) | `a-arrow-up` | `<AArrowUp />` |
| [a-large-small](https://lucide.dev/icons/a-large-small) | `a-large-small` | `<ALargeSmall />` |
| [app-window](https://lucide.dev/icons/app-window) | `app-window` | `<AppWindow />` |
| [app-window-mac](https://lucide.dev/icons/app-window-mac) | `app-window-mac` | `<AppWindowMac />` |
| [axis-3d](https://lucide.dev/icons/axis-3d) | `axis-3d` | `<Axis3D />` |
| [between-horizontal-end](https://lucide.dev/icons/between-horizontal-end) | `between-horizontal-end` | `<BetweenHorizontalEnd />` |
| [between-horizontal-start](https://lucide.dev/icons/between-horizontal-start) | `between-horizontal-start` | `<BetweenHorizontalStart />` |
| [between-vertical-end](https://lucide.dev/icons/between-vertical-end) | `between-vertical-end` | `<BetweenVerticalEnd />` |
| [between-vertical-start](https://lucide.dev/icons/between-vertical-start) | `between-vertical-start` | `<BetweenVerticalStart />` |
| [blend](https://lucide.dev/icons/blend) | `blend` | `<Blend />` |
| [book-type](https://lucide.dev/icons/book-type) | `book-type` | `<BookType />` |
| [bring-to-front](https://lucide.dev/icons/bring-to-front) | `bring-to-front` | `<BringToFront />` |
| [brush](https://lucide.dev/icons/brush) | `brush` | `<Brush />` |
| [brush-cleaning](https://lucide.dev/icons/brush-cleaning) | `brush-cleaning` | `<BrushCleaning />` |
| [chart-no-axes-gantt](https://lucide.dev/icons/chart-no-axes-gantt) | `chart-no-axes-gantt` | `<ChartNoAxesGantt />` |
| [columns-2](https://lucide.dev/icons/columns-2) | `columns-2` | `<Columns2 />` |
| [columns-3](https://lucide.dev/icons/columns-3) | `columns-3` | `<Columns3 />` |
| [columns-3-cog](https://lucide.dev/icons/columns-3-cog) | `columns-3-cog` | `<Columns3Cog />` |
| [columns-4](https://lucide.dev/icons/columns-4) | `columns-4` | `<Columns4 />` |
| [component](https://lucide.dev/icons/component) | `component` | `<Component />` |
| [contrast](https://lucide.dev/icons/contrast) | `contrast` | `<Contrast />` |
| [crop](https://lucide.dev/icons/crop) | `crop` | `<Crop />` |
| [cylinder](https://lucide.dev/icons/cylinder) | `cylinder` | `<Cylinder />` |
| [database-backup](https://lucide.dev/icons/database-backup) | `database-backup` | `<DatabaseBackup />` |
| [decimals-arrow-left](https://lucide.dev/icons/decimals-arrow-left) | `decimals-arrow-left` | `<DecimalsArrowLeft />` |
| [decimals-arrow-right](https://lucide.dev/icons/decimals-arrow-right) | `decimals-arrow-right` | `<DecimalsArrowRight />` |
| [diameter](https://lucide.dev/icons/diameter) | `diameter` | `<Diameter />` |
| [dock](https://lucide.dev/icons/dock) | `dock` | `<Dock />` |
| [drafting-compass](https://lucide.dev/icons/drafting-compass) | `drafting-compass` | `<DraftingCompass />` |
| [dribbble](https://lucide.dev/icons/dribbble) | `dribbble` | `<Dribbble />` |
| [eclipse](https://lucide.dev/icons/eclipse) | `eclipse` | `<Eclipse />` |
| [eye](https://lucide.dev/icons/eye) | `eye` | `<Eye />` |
| [eye-closed](https://lucide.dev/icons/eye-closed) | `eye-closed` | `<EyeClosed />` |
| [eye-off](https://lucide.dev/icons/eye-off) | `eye-off` | `<EyeOff />` |
| [figma](https://lucide.dev/icons/figma) | `figma` | `<Figma />` |
| [file-axis-3d](https://lucide.dev/icons/file-axis-3d) | `file-axis-3d` | `<FileAxis3D />` |
| [flip-horizontal](https://lucide.dev/icons/flip-horizontal) | `flip-horizontal` | `<FlipHorizontal />` |
| [flip-horizontal-2](https://lucide.dev/icons/flip-horizontal-2) | `flip-horizontal-2` | `<FlipHorizontal2 />` |
| [flip-vertical](https://lucide.dev/icons/flip-vertical) | `flip-vertical` | `<FlipVertical />` |
| [flip-vertical-2](https://lucide.dev/icons/flip-vertical-2) | `flip-vertical-2` | `<FlipVertical2 />` |
| [folder-kanban](https://lucide.dev/icons/folder-kanban) | `folder-kanban` | `<FolderKanban />` |
| [frame](https://lucide.dev/icons/frame) | `frame` | `<Frame />` |
| [framer](https://lucide.dev/icons/framer) | `framer` | `<Framer />` |
| [fullscreen](https://lucide.dev/icons/fullscreen) | `fullscreen` | `<Fullscreen />` |
| [gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) | `gallery-horizontal` | `<GalleryHorizontal />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) | `gallery-thumbnails` | `<GalleryThumbnails />` |
| [gallery-vertical](https://lucide.dev/icons/gallery-vertical) | `gallery-vertical` | `<GalleryVertical />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [grid-2x2](https://lucide.dev/icons/grid-2x2) | `grid-2x2` | `<Grid2X2 />` |
| [grid-3x2](https://lucide.dev/icons/grid-3x2) | `grid-3x2` | `<Grid3X2 />` |
| [grid-3x3](https://lucide.dev/icons/grid-3x3) | `grid-3x3` | `<Grid3X3 />` |
| [hand-grab](https://lucide.dev/icons/hand-grab) | `hand-grab` | `<HandGrab />` |
| [highlighter](https://lucide.dev/icons/highlighter) | `highlighter` | `<Highlighter />` |
| [iteration-ccw](https://lucide.dev/icons/iteration-ccw) | `iteration-ccw` | `<IterationCcw />` |
| [iteration-cw](https://lucide.dev/icons/iteration-cw) | `iteration-cw` | `<IterationCw />` |
| [kanban](https://lucide.dev/icons/kanban) | `kanban` | `<Kanban />` |
| [land-plot](https://lucide.dev/icons/land-plot) | `land-plot` | `<LandPlot />` |
| [lasso](https://lucide.dev/icons/lasso) | `lasso` | `<Lasso />` |
| [lasso-select](https://lucide.dev/icons/lasso-select) | `lasso-select` | `<LassoSelect />` |
| [layers](https://lucide.dev/icons/layers) | `layers` | `<Layers />` |
| [layers-2](https://lucide.dev/icons/layers-2) | `layers-2` | `<Layers2 />` |
| [layout-dashboard](https://lucide.dev/icons/layout-dashboard) | `layout-dashboard` | `<LayoutDashboard />` |
| [layout-grid](https://lucide.dev/icons/layout-grid) | `layout-grid` | `<LayoutGrid />` |
| [layout-list](https://lucide.dev/icons/layout-list) | `layout-list` | `<LayoutList />` |
| [layout-panel-left](https://lucide.dev/icons/layout-panel-left) | `layout-panel-left` | `<LayoutPanelLeft />` |
| [line-squiggle](https://lucide.dev/icons/line-squiggle) | `line-squiggle` | `<LineSquiggle />` |
| [loader](https://lucide.dev/icons/loader) | `loader` | `<Loader />` |
| [loader-pinwheel](https://lucide.dev/icons/loader-pinwheel) | `loader-pinwheel` | `<LoaderPinwheel />` |
| [magnet](https://lucide.dev/icons/magnet) | `magnet` | `<Magnet />` |
| [maximize](https://lucide.dev/icons/maximize) | `maximize` | `<Maximize />` |
| [maximize-2](https://lucide.dev/icons/maximize-2) | `maximize-2` | `<Maximize2 />` |
| [minimize](https://lucide.dev/icons/minimize) | `minimize` | `<Minimize />` |
| [minimize-2](https://lucide.dev/icons/minimize-2) | `minimize-2` | `<Minimize2 />` |
| [move-3d](https://lucide.dev/icons/move-3d) | `move-3d` | `<Move3D />` |
| [notebook](https://lucide.dev/icons/notebook) | `notebook` | `<Notebook />` |
| [origami](https://lucide.dev/icons/origami) | `origami` | `<Origami />` |
| [paint-bucket](https://lucide.dev/icons/paint-bucket) | `paint-bucket` | `<PaintBucket />` |
| [paint-roller](https://lucide.dev/icons/paint-roller) | `paint-roller` | `<PaintRoller />` |
| [paintbrush](https://lucide.dev/icons/paintbrush) | `paintbrush` | `<Paintbrush />` |
| [paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) | `paintbrush-vertical` | `<PaintbrushVertical />` |
| [palette](https://lucide.dev/icons/palette) | `palette` | `<Palette />` |
| [panel-top](https://lucide.dev/icons/panel-top) | `panel-top` | `<PanelTop />` |
| [panels-top-left](https://lucide.dev/icons/panels-top-left) | `panels-top-left` | `<PanelsTopLeft />` |
| [paperclip](https://lucide.dev/icons/paperclip) | `paperclip` | `<Paperclip />` |
| [pen](https://lucide.dev/icons/pen) | `pen` | `<Pen />` |
| [pen-line](https://lucide.dev/icons/pen-line) | `pen-line` | `<PenLine />` |
| [pen-off](https://lucide.dev/icons/pen-off) | `pen-off` | `<PenOff />` |
| [pen-tool](https://lucide.dev/icons/pen-tool) | `pen-tool` | `<PenTool />` |
| [pencil](https://lucide.dev/icons/pencil) | `pencil` | `<Pencil />` |
| [pencil-line](https://lucide.dev/icons/pencil-line) | `pencil-line` | `<PencilLine />` |
| [pencil-off](https://lucide.dev/icons/pencil-off) | `pencil-off` | `<PencilOff />` |
| [pencil-ruler](https://lucide.dev/icons/pencil-ruler) | `pencil-ruler` | `<PencilRuler />` |
| [pipette](https://lucide.dev/icons/pipette) | `pipette` | `<Pipette />` |
| [presentation](https://lucide.dev/icons/presentation) | `presentation` | `<Presentation />` |
| [proportions](https://lucide.dev/icons/proportions) | `proportions` | `<Proportions />` |
| [radius](https://lucide.dev/icons/radius) | `radius` | `<Radius />` |
| [ratio](https://lucide.dev/icons/ratio) | `ratio` | `<Ratio />` |
| [rectangle-horizontal](https://lucide.dev/icons/rectangle-horizontal) | `rectangle-horizontal` | `<RectangleHorizontal />` |
| [rectangle-vertical](https://lucide.dev/icons/rectangle-vertical) | `rectangle-vertical` | `<RectangleVertical />` |
| [rotate-3d](https://lucide.dev/icons/rotate-3d) | `rotate-3d` | `<Rotate3D />` |
| [rotate-ccw](https://lucide.dev/icons/rotate-ccw) | `rotate-ccw` | `<RotateCcw />` |
| [rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) | `rotate-ccw-square` | `<RotateCcwSquare />` |
| [rotate-cw](https://lucide.dev/icons/rotate-cw) | `rotate-cw` | `<RotateCw />` |
| [rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) | `rotate-cw-square` | `<RotateCwSquare />` |
| [rows-2](https://lucide.dev/icons/rows-2) | `rows-2` | `<Rows2 />` |
| [rows-3](https://lucide.dev/icons/rows-3) | `rows-3` | `<Rows3 />` |
| [rows-4](https://lucide.dev/icons/rows-4) | `rows-4` | `<Rows4 />` |
| [ruler](https://lucide.dev/icons/ruler) | `ruler` | `<Ruler />` |
| [ruler-dimension-line](https://lucide.dev/icons/ruler-dimension-line) | `ruler-dimension-line` | `<RulerDimensionLine />` |
| [scale-3d](https://lucide.dev/icons/scale-3d) | `scale-3d` | `<Scale3D />` |
| [scaling](https://lucide.dev/icons/scaling) | `scaling` | `<Scaling />` |
| [scissors](https://lucide.dev/icons/scissors) | `scissors` | `<Scissors />` |
| [scissors-line-dashed](https://lucide.dev/icons/scissors-line-dashed) | `scissors-line-dashed` | `<ScissorsLineDashed />` |
| [send-to-back](https://lucide.dev/icons/send-to-back) | `send-to-back` | `<SendToBack />` |
| [slice](https://lucide.dev/icons/slice) | `slice` | `<Slice />` |
| [spline](https://lucide.dev/icons/spline) | `spline` | `<Spline />` |
| [spline-pointer](https://lucide.dev/icons/spline-pointer) | `spline-pointer` | `<SplinePointer />` |
| [spray-can](https://lucide.dev/icons/spray-can) | `spray-can` | `<SprayCan />` |
| [square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) | `square-bottom-dashed-scissors` | `<SquareBottomDashedScissors />` |
| [square-chart-gantt](https://lucide.dev/icons/square-chart-gantt) | `square-chart-gantt` | `<SquareChartGantt />` |
| [square-dashed](https://lucide.dev/icons/square-dashed) | `square-dashed` | `<SquareDashed />` |
| [square-dashed-kanban](https://lucide.dev/icons/square-dashed-kanban) | `square-dashed-kanban` | `<SquareDashedKanban />` |
| [square-dashed-top-solid](https://lucide.dev/icons/square-dashed-top-solid) | `square-dashed-top-solid` | `<SquareDashedTopSolid />` |
| [square-kanban](https://lucide.dev/icons/square-kanban) | `square-kanban` | `<SquareKanban />` |
| [square-round-corner](https://lucide.dev/icons/square-round-corner) | `square-round-corner` | `<SquareRoundCorner />` |
| [square-scissors](https://lucide.dev/icons/square-scissors) | `square-scissors` | `<SquareScissors />` |
| [squares-exclude](https://lucide.dev/icons/squares-exclude) | `squares-exclude` | `<SquaresExclude />` |
| [squares-intersect](https://lucide.dev/icons/squares-intersect) | `squares-intersect` | `<SquaresIntersect />` |
| [squares-subtract](https://lucide.dev/icons/squares-subtract) | `squares-subtract` | `<SquaresSubtract />` |
| [squares-unite](https://lucide.dev/icons/squares-unite) | `squares-unite` | `<SquaresUnite />` |
| [squircle-dashed](https://lucide.dev/icons/squircle-dashed) | `squircle-dashed` | `<SquircleDashed />` |
| [stamp](https://lucide.dev/icons/stamp) | `stamp` | `<Stamp />` |
| [swatch-book](https://lucide.dev/icons/swatch-book) | `swatch-book` | `<SwatchBook />` |
| [tablet-smartphone](https://lucide.dev/icons/tablet-smartphone) | `tablet-smartphone` | `<TabletSmartphone />` |
| [tangent](https://lucide.dev/icons/tangent) | `tangent` | `<Tangent />` |
| [torus](https://lucide.dev/icons/torus) | `torus` | `<Torus />` |
| [vector-square](https://lucide.dev/icons/vector-square) | `vector-square` | `<VectorSquare />` |
| [view](https://lucide.dev/icons/view) | `view` | `<View />` |
| [wand](https://lucide.dev/icons/wand) | `wand` | `<Wand />` |
| [wand-sparkles](https://lucide.dev/icons/wand-sparkles) | `wand-sparkles` | `<WandSparkles />` |
| [zoom-in](https://lucide.dev/icons/zoom-in) | `zoom-in` | `<ZoomIn />` |
| [zoom-out](https://lucide.dev/icons/zoom-out) | `zoom-out` | `<ZoomOut />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "design"] or ["leptos", "all-icons"]
<AArrowDown class="w-6 h-6 text-gray-600" />
```

### Development

`development` and `all-icons` features - 239 icons

| Icon | Name | Component |
|------|------|-----------|
| [ampersand](https://lucide.dev/icons/ampersand) | `ampersand` | `<Ampersand />` |
| [ampersands](https://lucide.dev/icons/ampersands) | `ampersands` | `<Ampersands />` |
| [app-window](https://lucide.dev/icons/app-window) | `app-window` | `<AppWindow />` |
| [app-window-mac](https://lucide.dev/icons/app-window-mac) | `app-window-mac` | `<AppWindowMac />` |
| [arrow-big-up](https://lucide.dev/icons/arrow-big-up) | `arrow-big-up` | `<ArrowBigUp />` |
| [arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) | `arrow-big-up-dash` | `<ArrowBigUpDash />` |
| [arrow-down-to-line](https://lucide.dev/icons/arrow-down-to-line) | `arrow-down-to-line` | `<ArrowDownToLine />` |
| [arrow-right-to-line](https://lucide.dev/icons/arrow-right-to-line) | `arrow-right-to-line` | `<ArrowRightToLine />` |
| [arrow-up-from-line](https://lucide.dev/icons/arrow-up-from-line) | `arrow-up-from-line` | `<ArrowUpFromLine />` |
| [asterisk](https://lucide.dev/icons/asterisk) | `asterisk` | `<Asterisk />` |
| [binary](https://lucide.dev/icons/binary) | `binary` | `<Binary />` |
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [bitcoin](https://lucide.dev/icons/bitcoin) | `bitcoin` | `<Bitcoin />` |
| [blend](https://lucide.dev/icons/blend) | `blend` | `<Blend />` |
| [blocks](https://lucide.dev/icons/blocks) | `blocks` | `<Blocks />` |
| [book](https://lucide.dev/icons/book) | `book` | `<Book />` |
| [book-alert](https://lucide.dev/icons/book-alert) | `book-alert` | `<BookAlert />` |
| [book-check](https://lucide.dev/icons/book-check) | `book-check` | `<BookCheck />` |
| [book-copy](https://lucide.dev/icons/book-copy) | `book-copy` | `<BookCopy />` |
| [book-dashed](https://lucide.dev/icons/book-dashed) | `book-dashed` | `<BookDashed />` |
| [book-down](https://lucide.dev/icons/book-down) | `book-down` | `<BookDown />` |
| [book-key](https://lucide.dev/icons/book-key) | `book-key` | `<BookKey />` |
| [book-lock](https://lucide.dev/icons/book-lock) | `book-lock` | `<BookLock />` |
| [book-marked](https://lucide.dev/icons/book-marked) | `book-marked` | `<BookMarked />` |
| [book-minus](https://lucide.dev/icons/book-minus) | `book-minus` | `<BookMinus />` |
| [book-open](https://lucide.dev/icons/book-open) | `book-open` | `<BookOpen />` |
| [book-open-check](https://lucide.dev/icons/book-open-check) | `book-open-check` | `<BookOpenCheck />` |
| [book-open-text](https://lucide.dev/icons/book-open-text) | `book-open-text` | `<BookOpenText />` |
| [book-plus](https://lucide.dev/icons/book-plus) | `book-plus` | `<BookPlus />` |
| [book-up](https://lucide.dev/icons/book-up) | `book-up` | `<BookUp />` |
| [book-up-2](https://lucide.dev/icons/book-up-2) | `book-up-2` | `<BookUp2 />` |
| [bot](https://lucide.dev/icons/bot) | `bot` | `<Bot />` |
| [bot-message-square](https://lucide.dev/icons/bot-message-square) | `bot-message-square` | `<BotMessageSquare />` |
| [bot-off](https://lucide.dev/icons/bot-off) | `bot-off` | `<BotOff />` |
| [box](https://lucide.dev/icons/box) | `box` | `<Box />` |
| [boxes](https://lucide.dev/icons/boxes) | `boxes` | `<Boxes />` |
| [braces](https://lucide.dev/icons/braces) | `braces` | `<Braces />` |
| [brackets](https://lucide.dev/icons/brackets) | `brackets` | `<Brackets />` |
| [brain-circuit](https://lucide.dev/icons/brain-circuit) | `brain-circuit` | `<BrainCircuit />` |
| [brain-cog](https://lucide.dev/icons/brain-cog) | `brain-cog` | `<BrainCog />` |
| [bug](https://lucide.dev/icons/bug) | `bug` | `<Bug />` |
| [bug-off](https://lucide.dev/icons/bug-off) | `bug-off` | `<BugOff />` |
| [bug-play](https://lucide.dev/icons/bug-play) | `bug-play` | `<BugPlay />` |
| [case-lower](https://lucide.dev/icons/case-lower) | `case-lower` | `<CaseLower />` |
| [case-upper](https://lucide.dev/icons/case-upper) | `case-upper` | `<CaseUpper />` |
| [chart-no-axes-gantt](https://lucide.dev/icons/chart-no-axes-gantt) | `chart-no-axes-gantt` | `<ChartNoAxesGantt />` |
| [chevron-right](https://lucide.dev/icons/chevron-right) | `chevron-right` | `<ChevronRight />` |
| [circle-arrow-out-up-left](https://lucide.dev/icons/circle-arrow-out-up-left) | `circle-arrow-out-up-left` | `<CircleArrowOutUpLeft />` |
| [circle-dashed](https://lucide.dev/icons/circle-dashed) | `circle-dashed` | `<CircleDashed />` |
| [circle-dot](https://lucide.dev/icons/circle-dot) | `circle-dot` | `<CircleDot />` |
| [circle-dot-dashed](https://lucide.dev/icons/circle-dot-dashed) | `circle-dot-dashed` | `<CircleDotDashed />` |
| [circle-ellipsis](https://lucide.dev/icons/circle-ellipsis) | `circle-ellipsis` | `<CircleEllipsis />` |
| [circle-fading-arrow-up](https://lucide.dev/icons/circle-fading-arrow-up) | `circle-fading-arrow-up` | `<CircleFadingArrowUp />` |
| [circle-plus](https://lucide.dev/icons/circle-plus) | `circle-plus` | `<CirclePlus />` |
| [circle-slash](https://lucide.dev/icons/circle-slash) | `circle-slash` | `<CircleSlash />` |
| [circle-slash-2](https://lucide.dev/icons/circle-slash-2) | `circle-slash-2` | `<CircleSlash2 />` |
| [circle-x](https://lucide.dev/icons/circle-x) | `circle-x` | `<CircleX />` |
| [circuit-board](https://lucide.dev/icons/circuit-board) | `circuit-board` | `<CircuitBoard />` |
| [cloud-alert](https://lucide.dev/icons/cloud-alert) | `cloud-alert` | `<CloudAlert />` |
| [cloud-check](https://lucide.dev/icons/cloud-check) | `cloud-check` | `<CloudCheck />` |
| [cloud-cog](https://lucide.dev/icons/cloud-cog) | `cloud-cog` | `<CloudCog />` |
| [code](https://lucide.dev/icons/code) | `code` | `<Code />` |
| [code-xml](https://lucide.dev/icons/code-xml) | `code-xml` | `<CodeXml />` |
| [codepen](https://lucide.dev/icons/codepen) | `codepen` | `<Codepen />` |
| [codesandbox](https://lucide.dev/icons/codesandbox) | `codesandbox` | `<Codesandbox />` |
| [combine](https://lucide.dev/icons/combine) | `combine` | `<Combine />` |
| [command](https://lucide.dev/icons/command) | `command` | `<Command />` |
| [component](https://lucide.dev/icons/component) | `component` | `<Component />` |
| [computer](https://lucide.dev/icons/computer) | `computer` | `<Computer />` |
| [construction](https://lucide.dev/icons/construction) | `construction` | `<Construction />` |
| [container](https://lucide.dev/icons/container) | `container` | `<Container />` |
| [copy-slash](https://lucide.dev/icons/copy-slash) | `copy-slash` | `<CopySlash />` |
| [corner-down-right](https://lucide.dev/icons/corner-down-right) | `corner-down-right` | `<CornerDownRight />` |
| [database](https://lucide.dev/icons/database) | `database` | `<Database />` |
| [database-backup](https://lucide.dev/icons/database-backup) | `database-backup` | `<DatabaseBackup />` |
| [database-zap](https://lucide.dev/icons/database-zap) | `database-zap` | `<DatabaseZap />` |
| [diff](https://lucide.dev/icons/diff) | `diff` | `<Diff />` |
| [divide](https://lucide.dev/icons/divide) | `divide` | `<Divide />` |
| [dock](https://lucide.dev/icons/dock) | `dock` | `<Dock />` |
| [earth-lock](https://lucide.dev/icons/earth-lock) | `earth-lock` | `<EarthLock />` |
| [eclipse](https://lucide.dev/icons/eclipse) | `eclipse` | `<Eclipse />` |
| [ellipsis](https://lucide.dev/icons/ellipsis) | `ellipsis` | `<Ellipsis />` |
| [equal](https://lucide.dev/icons/equal) | `equal` | `<Equal />` |
| [equal-not](https://lucide.dev/icons/equal-not) | `equal-not` | `<EqualNot />` |
| [file-code](https://lucide.dev/icons/file-code) | `file-code` | `<FileCode />` |
| [file-code-2](https://lucide.dev/icons/file-code-2) | `file-code-2` | `<FileCode2 />` |
| [file-diff](https://lucide.dev/icons/file-diff) | `file-diff` | `<FileDiff />` |
| [file-digit](https://lucide.dev/icons/file-digit) | `file-digit` | `<FileDigit />` |
| [file-json](https://lucide.dev/icons/file-json) | `file-json` | `<FileJson />` |
| [file-json-2](https://lucide.dev/icons/file-json-2) | `file-json-2` | `<FileJson2 />` |
| [file-sliders](https://lucide.dev/icons/file-sliders) | `file-sliders` | `<FileSliders />` |
| [file-stack](https://lucide.dev/icons/file-stack) | `file-stack` | `<FileStack />` |
| [file-terminal](https://lucide.dev/icons/file-terminal) | `file-terminal` | `<FileTerminal />` |
| [flag-triangle-left](https://lucide.dev/icons/flag-triangle-left) | `flag-triangle-left` | `<FlagTriangleLeft />` |
| [flag-triangle-right](https://lucide.dev/icons/flag-triangle-right) | `flag-triangle-right` | `<FlagTriangleRight />` |
| [folder-code](https://lucide.dev/icons/folder-code) | `folder-code` | `<FolderCode />` |
| [folder-dot](https://lucide.dev/icons/folder-dot) | `folder-dot` | `<FolderDot />` |
| [folder-kanban](https://lucide.dev/icons/folder-kanban) | `folder-kanban` | `<FolderKanban />` |
| [folder-open-dot](https://lucide.dev/icons/folder-open-dot) | `folder-open-dot` | `<FolderOpenDot />` |
| [folder-root](https://lucide.dev/icons/folder-root) | `folder-root` | `<FolderRoot />` |
| [gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) | `gallery-horizontal` | `<GalleryHorizontal />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) | `gallery-thumbnails` | `<GalleryThumbnails />` |
| [gallery-vertical](https://lucide.dev/icons/gallery-vertical) | `gallery-vertical` | `<GalleryVertical />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [gem](https://lucide.dev/icons/gem) | `gem` | `<Gem />` |
| [git-branch](https://lucide.dev/icons/git-branch) | `git-branch` | `<GitBranch />` |
| [git-branch-plus](https://lucide.dev/icons/git-branch-plus) | `git-branch-plus` | `<GitBranchPlus />` |
| [git-commit-horizontal](https://lucide.dev/icons/git-commit-horizontal) | `git-commit-horizontal` | `<GitCommitHorizontal />` |
| [git-commit-vertical](https://lucide.dev/icons/git-commit-vertical) | `git-commit-vertical` | `<GitCommitVertical />` |
| [git-compare](https://lucide.dev/icons/git-compare) | `git-compare` | `<GitCompare />` |
| [git-compare-arrows](https://lucide.dev/icons/git-compare-arrows) | `git-compare-arrows` | `<GitCompareArrows />` |
| [git-fork](https://lucide.dev/icons/git-fork) | `git-fork` | `<GitFork />` |
| [git-graph](https://lucide.dev/icons/git-graph) | `git-graph` | `<GitGraph />` |
| [git-merge](https://lucide.dev/icons/git-merge) | `git-merge` | `<GitMerge />` |
| [git-pull-request](https://lucide.dev/icons/git-pull-request) | `git-pull-request` | `<GitPullRequest />` |
| [git-pull-request-arrow](https://lucide.dev/icons/git-pull-request-arrow) | `git-pull-request-arrow` | `<GitPullRequestArrow />` |
| [git-pull-request-closed](https://lucide.dev/icons/git-pull-request-closed) | `git-pull-request-closed` | `<GitPullRequestClosed />` |
| [git-pull-request-create](https://lucide.dev/icons/git-pull-request-create) | `git-pull-request-create` | `<GitPullRequestCreate />` |
| [git-pull-request-create-arrow](https://lucide.dev/icons/git-pull-request-create-arrow) | `git-pull-request-create-arrow` | `<GitPullRequestCreateArrow />` |
| [git-pull-request-draft](https://lucide.dev/icons/git-pull-request-draft) | `git-pull-request-draft` | `<GitPullRequestDraft />` |
| [github](https://lucide.dev/icons/github) | `github` | `<Github />` |
| [gitlab](https://lucide.dev/icons/gitlab) | `gitlab` | `<Gitlab />` |
| [globe-lock](https://lucide.dev/icons/globe-lock) | `globe-lock` | `<GlobeLock />` |
| [hard-drive](https://lucide.dev/icons/hard-drive) | `hard-drive` | `<HardDrive />` |
| [hard-drive-download](https://lucide.dev/icons/hard-drive-download) | `hard-drive-download` | `<HardDriveDownload />` |
| [hard-drive-upload](https://lucide.dev/icons/hard-drive-upload) | `hard-drive-upload` | `<HardDriveUpload />` |
| [hexagon](https://lucide.dev/icons/hexagon) | `hexagon` | `<Hexagon />` |
| [kanban](https://lucide.dev/icons/kanban) | `kanban` | `<Kanban />` |
| [keyboard](https://lucide.dev/icons/keyboard) | `keyboard` | `<Keyboard />` |
| [keyboard-off](https://lucide.dev/icons/keyboard-off) | `keyboard-off` | `<KeyboardOff />` |
| [library](https://lucide.dev/icons/library) | `library` | `<Library />` |
| [library-big](https://lucide.dev/icons/library-big) | `library-big` | `<LibraryBig />` |
| [list-indent-decrease](https://lucide.dev/icons/list-indent-decrease) | `list-indent-decrease` | `<ListIndentDecrease />` |
| [list-indent-increase](https://lucide.dev/icons/list-indent-increase) | `list-indent-increase` | `<ListIndentIncrease />` |
| [merge](https://lucide.dev/icons/merge) | `merge` | `<Merge />` |
| [message-circle-code](https://lucide.dev/icons/message-circle-code) | `message-circle-code` | `<MessageCircleCode />` |
| [message-square-code](https://lucide.dev/icons/message-square-code) | `message-square-code` | `<MessageSquareCode />` |
| [message-square-diff](https://lucide.dev/icons/message-square-diff) | `message-square-diff` | `<MessageSquareDiff />` |
| [milestone](https://lucide.dev/icons/milestone) | `milestone` | `<Milestone />` |
| [minus](https://lucide.dev/icons/minus) | `minus` | `<Minus />` |
| [network](https://lucide.dev/icons/network) | `network` | `<Network />` |
| [omega](https://lucide.dev/icons/omega) | `omega` | `<Omega />` |
| [option](https://lucide.dev/icons/option) | `option` | `<Option />` |
| [package](https://lucide.dev/icons/package) | `package` | `<Package />` |
| [package-2](https://lucide.dev/icons/package-2) | `package-2` | `<Package2 />` |
| [package-check](https://lucide.dev/icons/package-check) | `package-check` | `<PackageCheck />` |
| [package-minus](https://lucide.dev/icons/package-minus) | `package-minus` | `<PackageMinus />` |
| [package-open](https://lucide.dev/icons/package-open) | `package-open` | `<PackageOpen />` |
| [package-plus](https://lucide.dev/icons/package-plus) | `package-plus` | `<PackagePlus />` |
| [package-search](https://lucide.dev/icons/package-search) | `package-search` | `<PackageSearch />` |
| [package-x](https://lucide.dev/icons/package-x) | `package-x` | `<PackageX />` |
| [panel-top](https://lucide.dev/icons/panel-top) | `panel-top` | `<PanelTop />` |
| [panels-top-left](https://lucide.dev/icons/panels-top-left) | `panels-top-left` | `<PanelsTopLeft />` |
| [parentheses](https://lucide.dev/icons/parentheses) | `parentheses` | `<Parentheses />` |
| [percent](https://lucide.dev/icons/percent) | `percent` | `<Percent />` |
| [pi](https://lucide.dev/icons/pi) | `pi` | `<Pi />` |
| [plug](https://lucide.dev/icons/plug) | `plug` | `<Plug />` |
| [plug-2](https://lucide.dev/icons/plug-2) | `plug-2` | `<Plug2 />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [puzzle](https://lucide.dev/icons/puzzle) | `puzzle` | `<Puzzle />` |
| [qr-code](https://lucide.dev/icons/qr-code) | `qr-code` | `<QrCode />` |
| [radical](https://lucide.dev/icons/radical) | `radical` | `<Radical />` |
| [rectangle-circle](https://lucide.dev/icons/rectangle-circle) | `rectangle-circle` | `<RectangleCircle />` |
| [rectangle-ellipsis](https://lucide.dev/icons/rectangle-ellipsis) | `rectangle-ellipsis` | `<RectangleEllipsis />` |
| [refresh-ccw-dot](https://lucide.dev/icons/refresh-ccw-dot) | `refresh-ccw-dot` | `<RefreshCcwDot />` |
| [regex](https://lucide.dev/icons/regex) | `regex` | `<Regex />` |
| [rocket](https://lucide.dev/icons/rocket) | `rocket` | `<Rocket />` |
| [router](https://lucide.dev/icons/router) | `router` | `<Router />` |
| [rss](https://lucide.dev/icons/rss) | `rss` | `<Rss />` |
| [scroll](https://lucide.dev/icons/scroll) | `scroll` | `<Scroll />` |
| [scroll-text](https://lucide.dev/icons/scroll-text) | `scroll-text` | `<ScrollText />` |
| [search-code](https://lucide.dev/icons/search-code) | `search-code` | `<SearchCode />` |
| [server](https://lucide.dev/icons/server) | `server` | `<Server />` |
| [server-cog](https://lucide.dev/icons/server-cog) | `server-cog` | `<ServerCog />` |
| [server-crash](https://lucide.dev/icons/server-crash) | `server-crash` | `<ServerCrash />` |
| [server-off](https://lucide.dev/icons/server-off) | `server-off` | `<ServerOff />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [shield](https://lucide.dev/icons/shield) | `shield` | `<Shield />` |
| [shield-alert](https://lucide.dev/icons/shield-alert) | `shield-alert` | `<ShieldAlert />` |
| [shield-ban](https://lucide.dev/icons/shield-ban) | `shield-ban` | `<ShieldBan />` |
| [shield-check](https://lucide.dev/icons/shield-check) | `shield-check` | `<ShieldCheck />` |
| [shield-ellipsis](https://lucide.dev/icons/shield-ellipsis) | `shield-ellipsis` | `<ShieldEllipsis />` |
| [shield-half](https://lucide.dev/icons/shield-half) | `shield-half` | `<ShieldHalf />` |
| [shield-minus](https://lucide.dev/icons/shield-minus) | `shield-minus` | `<ShieldMinus />` |
| [shield-off](https://lucide.dev/icons/shield-off) | `shield-off` | `<ShieldOff />` |
| [shield-plus](https://lucide.dev/icons/shield-plus) | `shield-plus` | `<ShieldPlus />` |
| [shield-question-mark](https://lucide.dev/icons/shield-question-mark) | `shield-question-mark` | `<ShieldQuestionMark />` |
| [shield-user](https://lucide.dev/icons/shield-user) | `shield-user` | `<ShieldUser />` |
| [shield-x](https://lucide.dev/icons/shield-x) | `shield-x` | `<ShieldX />` |
| [signpost](https://lucide.dev/icons/signpost) | `signpost` | `<Signpost />` |
| [signpost-big](https://lucide.dev/icons/signpost-big) | `signpost-big` | `<SignpostBig />` |
| [slack](https://lucide.dev/icons/slack) | `slack` | `<Slack />` |
| [slash](https://lucide.dev/icons/slash) | `slash` | `<Slash />` |
| [spell-check](https://lucide.dev/icons/spell-check) | `spell-check` | `<SpellCheck />` |
| [spell-check-2](https://lucide.dev/icons/spell-check-2) | `spell-check-2` | `<SpellCheck2 />` |
| [split](https://lucide.dev/icons/split) | `split` | `<Split />` |
| [square-asterisk](https://lucide.dev/icons/square-asterisk) | `square-asterisk` | `<SquareAsterisk />` |
| [square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) | `square-bottom-dashed-scissors` | `<SquareBottomDashedScissors />` |
| [square-chart-gantt](https://lucide.dev/icons/square-chart-gantt) | `square-chart-gantt` | `<SquareChartGantt />` |
| [square-chevron-right](https://lucide.dev/icons/square-chevron-right) | `square-chevron-right` | `<SquareChevronRight />` |
| [square-code](https://lucide.dev/icons/square-code) | `square-code` | `<SquareCode />` |
| [square-dashed-bottom](https://lucide.dev/icons/square-dashed-bottom) | `square-dashed-bottom` | `<SquareDashedBottom />` |
| [square-dashed-bottom-code](https://lucide.dev/icons/square-dashed-bottom-code) | `square-dashed-bottom-code` | `<SquareDashedBottomCode />` |
| [square-dashed-kanban](https://lucide.dev/icons/square-dashed-kanban) | `square-dashed-kanban` | `<SquareDashedKanban />` |
| [square-dashed-mouse-pointer](https://lucide.dev/icons/square-dashed-mouse-pointer) | `square-dashed-mouse-pointer` | `<SquareDashedMousePointer />` |
| [square-dashed-top-solid](https://lucide.dev/icons/square-dashed-top-solid) | `square-dashed-top-solid` | `<SquareDashedTopSolid />` |
| [square-dot](https://lucide.dev/icons/square-dot) | `square-dot` | `<SquareDot />` |
| [square-function](https://lucide.dev/icons/square-function) | `square-function` | `<SquareFunction />` |
| [square-kanban](https://lucide.dev/icons/square-kanban) | `square-kanban` | `<SquareKanban />` |
| [square-library](https://lucide.dev/icons/square-library) | `square-library` | `<SquareLibrary />` |
| [square-minus](https://lucide.dev/icons/square-minus) | `square-minus` | `<SquareMinus />` |
| [square-mouse-pointer](https://lucide.dev/icons/square-mouse-pointer) | `square-mouse-pointer` | `<SquareMousePointer />` |
| [square-pi](https://lucide.dev/icons/square-pi) | `square-pi` | `<SquarePi />` |
| [square-plus](https://lucide.dev/icons/square-plus) | `square-plus` | `<SquarePlus />` |
| [square-radical](https://lucide.dev/icons/square-radical) | `square-radical` | `<SquareRadical />` |
| [square-round-corner](https://lucide.dev/icons/square-round-corner) | `square-round-corner` | `<SquareRoundCorner />` |
| [square-scissors](https://lucide.dev/icons/square-scissors) | `square-scissors` | `<SquareScissors />` |
| [square-slash](https://lucide.dev/icons/square-slash) | `square-slash` | `<SquareSlash />` |
| [square-stack](https://lucide.dev/icons/square-stack) | `square-stack` | `<SquareStack />` |
| [square-terminal](https://lucide.dev/icons/square-terminal) | `square-terminal` | `<SquareTerminal />` |
| [squircle-dashed](https://lucide.dev/icons/squircle-dashed) | `squircle-dashed` | `<SquircleDashed />` |
| [table-properties](https://lucide.dev/icons/table-properties) | `table-properties` | `<TableProperties />` |
| [tablet-smartphone](https://lucide.dev/icons/tablet-smartphone) | `tablet-smartphone` | `<TabletSmartphone />` |
| [telescope](https://lucide.dev/icons/telescope) | `telescope` | `<Telescope />` |
| [terminal](https://lucide.dev/icons/terminal) | `terminal` | `<Terminal />` |
| [toggle-left](https://lucide.dev/icons/toggle-left) | `toggle-left` | `<ToggleLeft />` |
| [toggle-right](https://lucide.dev/icons/toggle-right) | `toggle-right` | `<ToggleRight />` |
| [tool-case](https://lucide.dev/icons/tool-case) | `tool-case` | `<ToolCase />` |
| [toy-brick](https://lucide.dev/icons/toy-brick) | `toy-brick` | `<ToyBrick />` |
| [trello](https://lucide.dev/icons/trello) | `trello` | `<Trello />` |
| [triangle-alert](https://lucide.dev/icons/triangle-alert) | `triangle-alert` | `<TriangleAlert />` |
| [unplug](https://lucide.dev/icons/unplug) | `unplug` | `<Unplug />` |
| [variable](https://lucide.dev/icons/variable) | `variable` | `<Variable />` |
| [waypoints](https://lucide.dev/icons/waypoints) | `waypoints` | `<Waypoints />` |
| [webhook](https://lucide.dev/icons/webhook) | `webhook` | `<Webhook />` |
| [webhook-off](https://lucide.dev/icons/webhook-off) | `webhook-off` | `<WebhookOff />` |
| [workflow](https://lucide.dev/icons/workflow) | `workflow` | `<Workflow />` |
| [wrench](https://lucide.dev/icons/wrench) | `wrench` | `<Wrench />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "development"] or ["leptos", "all-icons"]
<Ampersand class="w-6 h-6 text-gray-600" />
```

### Devices

`devices` and `all-icons` features - 161 icons

| Icon | Name | Component |
|------|------|-----------|
| [airplay](https://lucide.dev/icons/airplay) | `airplay` | `<Airplay />` |
| [alarm-clock](https://lucide.dev/icons/alarm-clock) | `alarm-clock` | `<AlarmClock />` |
| [alarm-clock-check](https://lucide.dev/icons/alarm-clock-check) | `alarm-clock-check` | `<AlarmClockCheck />` |
| [alarm-clock-minus](https://lucide.dev/icons/alarm-clock-minus) | `alarm-clock-minus` | `<AlarmClockMinus />` |
| [alarm-clock-off](https://lucide.dev/icons/alarm-clock-off) | `alarm-clock-off` | `<AlarmClockOff />` |
| [alarm-clock-plus](https://lucide.dev/icons/alarm-clock-plus) | `alarm-clock-plus` | `<AlarmClockPlus />` |
| [alarm-smoke](https://lucide.dev/icons/alarm-smoke) | `alarm-smoke` | `<AlarmSmoke />` |
| [antenna](https://lucide.dev/icons/antenna) | `antenna` | `<Antenna />` |
| [battery](https://lucide.dev/icons/battery) | `battery` | `<Battery />` |
| [battery-charging](https://lucide.dev/icons/battery-charging) | `battery-charging` | `<BatteryCharging />` |
| [battery-full](https://lucide.dev/icons/battery-full) | `battery-full` | `<BatteryFull />` |
| [battery-low](https://lucide.dev/icons/battery-low) | `battery-low` | `<BatteryLow />` |
| [battery-medium](https://lucide.dev/icons/battery-medium) | `battery-medium` | `<BatteryMedium />` |
| [battery-plus](https://lucide.dev/icons/battery-plus) | `battery-plus` | `<BatteryPlus />` |
| [battery-warning](https://lucide.dev/icons/battery-warning) | `battery-warning` | `<BatteryWarning />` |
| [bell-electric](https://lucide.dev/icons/bell-electric) | `bell-electric` | `<BellElectric />` |
| [bluetooth](https://lucide.dev/icons/bluetooth) | `bluetooth` | `<Bluetooth />` |
| [bluetooth-connected](https://lucide.dev/icons/bluetooth-connected) | `bluetooth-connected` | `<BluetoothConnected />` |
| [bluetooth-off](https://lucide.dev/icons/bluetooth-off) | `bluetooth-off` | `<BluetoothOff />` |
| [bluetooth-searching](https://lucide.dev/icons/bluetooth-searching) | `bluetooth-searching` | `<BluetoothSearching />` |
| [boom-box](https://lucide.dev/icons/boom-box) | `boom-box` | `<BoomBox />` |
| [cable](https://lucide.dev/icons/cable) | `cable` | `<Cable />` |
| [calculator](https://lucide.dev/icons/calculator) | `calculator` | `<Calculator />` |
| [camera](https://lucide.dev/icons/camera) | `camera` | `<Camera />` |
| [camera-off](https://lucide.dev/icons/camera-off) | `camera-off` | `<CameraOff />` |
| [card-sim](https://lucide.dev/icons/card-sim) | `card-sim` | `<CardSim />` |
| [cassette-tape](https://lucide.dev/icons/cassette-tape) | `cassette-tape` | `<CassetteTape />` |
| [cast](https://lucide.dev/icons/cast) | `cast` | `<Cast />` |
| [cctv](https://lucide.dev/icons/cctv) | `cctv` | `<Cctv />` |
| [chevrons-left-right-ellipsis](https://lucide.dev/icons/chevrons-left-right-ellipsis) | `chevrons-left-right-ellipsis` | `<ChevronsLeftRightEllipsis />` |
| [computer](https://lucide.dev/icons/computer) | `computer` | `<Computer />` |
| [cpu](https://lucide.dev/icons/cpu) | `cpu` | `<Cpu />` |
| [database](https://lucide.dev/icons/database) | `database` | `<Database />` |
| [database-backup](https://lucide.dev/icons/database-backup) | `database-backup` | `<DatabaseBackup />` |
| [database-zap](https://lucide.dev/icons/database-zap) | `database-zap` | `<DatabaseZap />` |
| [diamond-minus](https://lucide.dev/icons/diamond-minus) | `diamond-minus` | `<DiamondMinus />` |
| [diamond-plus](https://lucide.dev/icons/diamond-plus) | `diamond-plus` | `<DiamondPlus />` |
| [disc](https://lucide.dev/icons/disc) | `disc` | `<Disc />` |
| [disc-2](https://lucide.dev/icons/disc-2) | `disc-2` | `<Disc2 />` |
| [disc-3](https://lucide.dev/icons/disc-3) | `disc-3` | `<Disc3 />` |
| [disc-album](https://lucide.dev/icons/disc-album) | `disc-album` | `<DiscAlbum />` |
| [drill](https://lucide.dev/icons/drill) | `drill` | `<Drill />` |
| [drone](https://lucide.dev/icons/drone) | `drone` | `<Drone />` |
| [drum](https://lucide.dev/icons/drum) | `drum` | `<Drum />` |
| [earth-lock](https://lucide.dev/icons/earth-lock) | `earth-lock` | `<EarthLock />` |
| [ethernet-port](https://lucide.dev/icons/ethernet-port) | `ethernet-port` | `<EthernetPort />` |
| [fingerprint](https://lucide.dev/icons/fingerprint) | `fingerprint` | `<Fingerprint />` |
| [flashlight](https://lucide.dev/icons/flashlight) | `flashlight` | `<Flashlight />` |
| [flashlight-off](https://lucide.dev/icons/flashlight-off) | `flashlight-off` | `<FlashlightOff />` |
| [gamepad](https://lucide.dev/icons/gamepad) | `gamepad` | `<Gamepad />` |
| [gamepad-2](https://lucide.dev/icons/gamepad-2) | `gamepad-2` | `<Gamepad2 />` |
| [globe-lock](https://lucide.dev/icons/globe-lock) | `globe-lock` | `<GlobeLock />` |
| [gpu](https://lucide.dev/icons/gpu) | `gpu` | `<Gpu />` |
| [hard-drive](https://lucide.dev/icons/hard-drive) | `hard-drive` | `<HardDrive />` |
| [hard-drive-download](https://lucide.dev/icons/hard-drive-download) | `hard-drive-download` | `<HardDriveDownload />` |
| [hard-drive-upload](https://lucide.dev/icons/hard-drive-upload) | `hard-drive-upload` | `<HardDriveUpload />` |
| [hdmi-port](https://lucide.dev/icons/hdmi-port) | `hdmi-port` | `<HdmiPort />` |
| [headphone-off](https://lucide.dev/icons/headphone-off) | `headphone-off` | `<HeadphoneOff />` |
| [headphones](https://lucide.dev/icons/headphones) | `headphones` | `<Headphones />` |
| [headset](https://lucide.dev/icons/headset) | `headset` | `<Headset />` |
| [heater](https://lucide.dev/icons/heater) | `heater` | `<Heater />` |
| [joystick](https://lucide.dev/icons/joystick) | `joystick` | `<Joystick />` |
| [keyboard](https://lucide.dev/icons/keyboard) | `keyboard` | `<Keyboard />` |
| [keyboard-music](https://lucide.dev/icons/keyboard-music) | `keyboard-music` | `<KeyboardMusic />` |
| [keyboard-off](https://lucide.dev/icons/keyboard-off) | `keyboard-off` | `<KeyboardOff />` |
| [laptop](https://lucide.dev/icons/laptop) | `laptop` | `<Laptop />` |
| [laptop-minimal](https://lucide.dev/icons/laptop-minimal) | `laptop-minimal` | `<LaptopMinimal />` |
| [laptop-minimal-check](https://lucide.dev/icons/laptop-minimal-check) | `laptop-minimal-check` | `<LaptopMinimalCheck />` |
| [memory-stick](https://lucide.dev/icons/memory-stick) | `memory-stick` | `<MemoryStick />` |
| [mic](https://lucide.dev/icons/mic) | `mic` | `<Mic />` |
| [mic-off](https://lucide.dev/icons/mic-off) | `mic-off` | `<MicOff />` |
| [mic-vocal](https://lucide.dev/icons/mic-vocal) | `mic-vocal` | `<MicVocal />` |
| [microchip](https://lucide.dev/icons/microchip) | `microchip` | `<Microchip />` |
| [monitor](https://lucide.dev/icons/monitor) | `monitor` | `<Monitor />` |
| [monitor-check](https://lucide.dev/icons/monitor-check) | `monitor-check` | `<MonitorCheck />` |
| [monitor-cog](https://lucide.dev/icons/monitor-cog) | `monitor-cog` | `<MonitorCog />` |
| [monitor-dot](https://lucide.dev/icons/monitor-dot) | `monitor-dot` | `<MonitorDot />` |
| [monitor-down](https://lucide.dev/icons/monitor-down) | `monitor-down` | `<MonitorDown />` |
| [monitor-off](https://lucide.dev/icons/monitor-off) | `monitor-off` | `<MonitorOff />` |
| [monitor-pause](https://lucide.dev/icons/monitor-pause) | `monitor-pause` | `<MonitorPause />` |
| [monitor-play](https://lucide.dev/icons/monitor-play) | `monitor-play` | `<MonitorPlay />` |
| [monitor-smartphone](https://lucide.dev/icons/monitor-smartphone) | `monitor-smartphone` | `<MonitorSmartphone />` |
| [monitor-speaker](https://lucide.dev/icons/monitor-speaker) | `monitor-speaker` | `<MonitorSpeaker />` |
| [monitor-stop](https://lucide.dev/icons/monitor-stop) | `monitor-stop` | `<MonitorStop />` |
| [monitor-up](https://lucide.dev/icons/monitor-up) | `monitor-up` | `<MonitorUp />` |
| [monitor-x](https://lucide.dev/icons/monitor-x) | `monitor-x` | `<MonitorX />` |
| [mouse](https://lucide.dev/icons/mouse) | `mouse` | `<Mouse />` |
| [mouse-off](https://lucide.dev/icons/mouse-off) | `mouse-off` | `<MouseOff />` |
| [nfc](https://lucide.dev/icons/nfc) | `nfc` | `<Nfc />` |
| [pc-case](https://lucide.dev/icons/pc-case) | `pc-case` | `<PcCase />` |
| [phone](https://lucide.dev/icons/phone) | `phone` | `<Phone />` |
| [phone-call](https://lucide.dev/icons/phone-call) | `phone-call` | `<PhoneCall />` |
| [phone-forwarded](https://lucide.dev/icons/phone-forwarded) | `phone-forwarded` | `<PhoneForwarded />` |
| [phone-incoming](https://lucide.dev/icons/phone-incoming) | `phone-incoming` | `<PhoneIncoming />` |
| [phone-missed](https://lucide.dev/icons/phone-missed) | `phone-missed` | `<PhoneMissed />` |
| [phone-off](https://lucide.dev/icons/phone-off) | `phone-off` | `<PhoneOff />` |
| [phone-outgoing](https://lucide.dev/icons/phone-outgoing) | `phone-outgoing` | `<PhoneOutgoing />` |
| [piano](https://lucide.dev/icons/piano) | `piano` | `<Piano />` |
| [plug](https://lucide.dev/icons/plug) | `plug` | `<Plug />` |
| [plug-2](https://lucide.dev/icons/plug-2) | `plug-2` | `<Plug2 />` |
| [plug-zap](https://lucide.dev/icons/plug-zap) | `plug-zap` | `<PlugZap />` |
| [presentation](https://lucide.dev/icons/presentation) | `presentation` | `<Presentation />` |
| [printer](https://lucide.dev/icons/printer) | `printer` | `<Printer />` |
| [printer-check](https://lucide.dev/icons/printer-check) | `printer-check` | `<PrinterCheck />` |
| [projector](https://lucide.dev/icons/projector) | `projector` | `<Projector />` |
| [proportions](https://lucide.dev/icons/proportions) | `proportions` | `<Proportions />` |
| [radio](https://lucide.dev/icons/radio) | `radio` | `<Radio />` |
| [radio-receiver](https://lucide.dev/icons/radio-receiver) | `radio-receiver` | `<RadioReceiver />` |
| [radio-tower](https://lucide.dev/icons/radio-tower) | `radio-tower` | `<RadioTower />` |
| [rectangle-goggles](https://lucide.dev/icons/rectangle-goggles) | `rectangle-goggles` | `<RectangleGoggles />` |
| [router](https://lucide.dev/icons/router) | `router` | `<Router />` |
| [satellite-dish](https://lucide.dev/icons/satellite-dish) | `satellite-dish` | `<SatelliteDish />` |
| [scan](https://lucide.dev/icons/scan) | `scan` | `<Scan />` |
| [scan-barcode](https://lucide.dev/icons/scan-barcode) | `scan-barcode` | `<ScanBarcode />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-face](https://lucide.dev/icons/scan-face) | `scan-face` | `<ScanFace />` |
| [scan-line](https://lucide.dev/icons/scan-line) | `scan-line` | `<ScanLine />` |
| [scan-qr-code](https://lucide.dev/icons/scan-qr-code) | `scan-qr-code` | `<ScanQrCode />` |
| [scan-text](https://lucide.dev/icons/scan-text) | `scan-text` | `<ScanText />` |
| [screen-share](https://lucide.dev/icons/screen-share) | `screen-share` | `<ScreenShare />` |
| [screen-share-off](https://lucide.dev/icons/screen-share-off) | `screen-share-off` | `<ScreenShareOff />` |
| [server](https://lucide.dev/icons/server) | `server` | `<Server />` |
| [server-cog](https://lucide.dev/icons/server-cog) | `server-cog` | `<ServerCog />` |
| [server-crash](https://lucide.dev/icons/server-crash) | `server-crash` | `<ServerCrash />` |
| [server-off](https://lucide.dev/icons/server-off) | `server-off` | `<ServerOff />` |
| [smartphone](https://lucide.dev/icons/smartphone) | `smartphone` | `<Smartphone />` |
| [smartphone-charging](https://lucide.dev/icons/smartphone-charging) | `smartphone-charging` | `<SmartphoneCharging />` |
| [smartphone-nfc](https://lucide.dev/icons/smartphone-nfc) | `smartphone-nfc` | `<SmartphoneNfc />` |
| [speaker](https://lucide.dev/icons/speaker) | `speaker` | `<Speaker />` |
| [spotlight](https://lucide.dev/icons/spotlight) | `spotlight` | `<Spotlight />` |
| [square-minus](https://lucide.dev/icons/square-minus) | `square-minus` | `<SquareMinus />` |
| [switch-camera](https://lucide.dev/icons/switch-camera) | `switch-camera` | `<SwitchCamera />` |
| [tablet](https://lucide.dev/icons/tablet) | `tablet` | `<Tablet />` |
| [tablet-smartphone](https://lucide.dev/icons/tablet-smartphone) | `tablet-smartphone` | `<TabletSmartphone />` |
| [toilet](https://lucide.dev/icons/toilet) | `toilet` | `<Toilet />` |
| [touchpad](https://lucide.dev/icons/touchpad) | `touchpad` | `<Touchpad />` |
| [touchpad-off](https://lucide.dev/icons/touchpad-off) | `touchpad-off` | `<TouchpadOff />` |
| [tv](https://lucide.dev/icons/tv) | `tv` | `<Tv />` |
| [tv-minimal](https://lucide.dev/icons/tv-minimal) | `tv-minimal` | `<TvMinimal />` |
| [tv-minimal-play](https://lucide.dev/icons/tv-minimal-play) | `tv-minimal-play` | `<TvMinimalPlay />` |
| [unplug](https://lucide.dev/icons/unplug) | `unplug` | `<Unplug />` |
| [usb](https://lucide.dev/icons/usb) | `usb` | `<Usb />` |
| [vibrate](https://lucide.dev/icons/vibrate) | `vibrate` | `<Vibrate />` |
| [vibrate-off](https://lucide.dev/icons/vibrate-off) | `vibrate-off` | `<VibrateOff />` |
| [video](https://lucide.dev/icons/video) | `video` | `<Video />` |
| [video-off](https://lucide.dev/icons/video-off) | `video-off` | `<VideoOff />` |
| [videotape](https://lucide.dev/icons/videotape) | `videotape` | `<Videotape />` |
| [voicemail](https://lucide.dev/icons/voicemail) | `voicemail` | `<Voicemail />` |
| [wallpaper](https://lucide.dev/icons/wallpaper) | `wallpaper` | `<Wallpaper />` |
| [washing-machine](https://lucide.dev/icons/washing-machine) | `washing-machine` | `<WashingMachine />` |
| [webcam](https://lucide.dev/icons/webcam) | `webcam` | `<Webcam />` |
| [wifi](https://lucide.dev/icons/wifi) | `wifi` | `<Wifi />` |
| [wifi-cog](https://lucide.dev/icons/wifi-cog) | `wifi-cog` | `<WifiCog />` |
| [wifi-high](https://lucide.dev/icons/wifi-high) | `wifi-high` | `<WifiHigh />` |
| [wifi-low](https://lucide.dev/icons/wifi-low) | `wifi-low` | `<WifiLow />` |
| [wifi-off](https://lucide.dev/icons/wifi-off) | `wifi-off` | `<WifiOff />` |
| [wifi-pen](https://lucide.dev/icons/wifi-pen) | `wifi-pen` | `<WifiPen />` |
| [wifi-sync](https://lucide.dev/icons/wifi-sync) | `wifi-sync` | `<WifiSync />` |
| [wifi-zero](https://lucide.dev/icons/wifi-zero) | `wifi-zero` | `<WifiZero />` |
| [zap](https://lucide.dev/icons/zap) | `zap` | `<Zap />` |
| [zap-off](https://lucide.dev/icons/zap-off) | `zap-off` | `<ZapOff />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "devices"] or ["leptos", "all-icons"]
<Airplay class="w-6 h-6 text-gray-600" />
```

### Emoji

`emoji` and `all-icons` features - 21 icons

| Icon | Name | Component |
|------|------|-----------|
| [angry](https://lucide.dev/icons/angry) | `angry` | `<Angry />` |
| [annoyed](https://lucide.dev/icons/annoyed) | `annoyed` | `<Annoyed />` |
| [biceps-flexed](https://lucide.dev/icons/biceps-flexed) | `biceps-flexed` | `<BicepsFlexed />` |
| [frown](https://lucide.dev/icons/frown) | `frown` | `<Frown />` |
| [hand-fist](https://lucide.dev/icons/hand-fist) | `hand-fist` | `<HandFist />` |
| [hand-helping](https://lucide.dev/icons/hand-helping) | `hand-helping` | `<HandHelping />` |
| [hand-metal](https://lucide.dev/icons/hand-metal) | `hand-metal` | `<HandMetal />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [heart-crack](https://lucide.dev/icons/heart-crack) | `heart-crack` | `<HeartCrack />` |
| [heart-handshake](https://lucide.dev/icons/heart-handshake) | `heart-handshake` | `<HeartHandshake />` |
| [laugh](https://lucide.dev/icons/laugh) | `laugh` | `<Laugh />` |
| [leafy-green](https://lucide.dev/icons/leafy-green) | `leafy-green` | `<LeafyGreen />` |
| [meh](https://lucide.dev/icons/meh) | `meh` | `<Meh />` |
| [party-popper](https://lucide.dev/icons/party-popper) | `party-popper` | `<PartyPopper />` |
| [ribbon](https://lucide.dev/icons/ribbon) | `ribbon` | `<Ribbon />` |
| [salad](https://lucide.dev/icons/salad) | `salad` | `<Salad />` |
| [smile](https://lucide.dev/icons/smile) | `smile` | `<Smile />` |
| [smile-plus](https://lucide.dev/icons/smile-plus) | `smile-plus` | `<SmilePlus />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [thumbs-down](https://lucide.dev/icons/thumbs-down) | `thumbs-down` | `<ThumbsDown />` |
| [thumbs-up](https://lucide.dev/icons/thumbs-up) | `thumbs-up` | `<ThumbsUp />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "emoji"] or ["leptos", "all-icons"]
<Angry class="w-6 h-6 text-gray-600" />
```

### Files

`files` and `all-icons` features - 164 icons

| Icon | Name | Component |
|------|------|-----------|
| [app-window](https://lucide.dev/icons/app-window) | `app-window` | `<AppWindow />` |
| [app-window-mac](https://lucide.dev/icons/app-window-mac) | `app-window-mac` | `<AppWindowMac />` |
| [archive](https://lucide.dev/icons/archive) | `archive` | `<Archive />` |
| [archive-restore](https://lucide.dev/icons/archive-restore) | `archive-restore` | `<ArchiveRestore />` |
| [archive-x](https://lucide.dev/icons/archive-x) | `archive-x` | `<ArchiveX />` |
| [arrow-big-down-dash](https://lucide.dev/icons/arrow-big-down-dash) | `arrow-big-down-dash` | `<ArrowBigDownDash />` |
| [arrow-down-from-line](https://lucide.dev/icons/arrow-down-from-line) | `arrow-down-from-line` | `<ArrowDownFromLine />` |
| [arrow-down-to-line](https://lucide.dev/icons/arrow-down-to-line) | `arrow-down-to-line` | `<ArrowDownToLine />` |
| [arrow-up-from-line](https://lucide.dev/icons/arrow-up-from-line) | `arrow-up-from-line` | `<ArrowUpFromLine />` |
| [arrow-up-to-line](https://lucide.dev/icons/arrow-up-to-line) | `arrow-up-to-line` | `<ArrowUpToLine />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [braces](https://lucide.dev/icons/braces) | `braces` | `<Braces />` |
| [brackets](https://lucide.dev/icons/brackets) | `brackets` | `<Brackets />` |
| [calendar-fold](https://lucide.dev/icons/calendar-fold) | `calendar-fold` | `<CalendarFold />` |
| [cassette-tape](https://lucide.dev/icons/cassette-tape) | `cassette-tape` | `<CassetteTape />` |
| [chart-pie](https://lucide.dev/icons/chart-pie) | `chart-pie` | `<ChartPie />` |
| [cloud-download](https://lucide.dev/icons/cloud-download) | `cloud-download` | `<CloudDownload />` |
| [cloud-upload](https://lucide.dev/icons/cloud-upload) | `cloud-upload` | `<CloudUpload />` |
| [combine](https://lucide.dev/icons/combine) | `combine` | `<Combine />` |
| [diff](https://lucide.dev/icons/diff) | `diff` | `<Diff />` |
| [dock](https://lucide.dev/icons/dock) | `dock` | `<Dock />` |
| [download](https://lucide.dev/icons/download) | `download` | `<Download />` |
| [file](https://lucide.dev/icons/file) | `file` | `<File />` |
| [file-archive](https://lucide.dev/icons/file-archive) | `file-archive` | `<FileArchive />` |
| [file-audio](https://lucide.dev/icons/file-audio) | `file-audio` | `<FileAudio />` |
| [file-audio-2](https://lucide.dev/icons/file-audio-2) | `file-audio-2` | `<FileAudio2 />` |
| [file-axis-3d](https://lucide.dev/icons/file-axis-3d) | `file-axis-3d` | `<FileAxis3D />` |
| [file-badge](https://lucide.dev/icons/file-badge) | `file-badge` | `<FileBadge />` |
| [file-badge-2](https://lucide.dev/icons/file-badge-2) | `file-badge-2` | `<FileBadge2 />` |
| [file-box](https://lucide.dev/icons/file-box) | `file-box` | `<FileBox />` |
| [file-chart-column](https://lucide.dev/icons/file-chart-column) | `file-chart-column` | `<FileChartColumn />` |
| [file-chart-column-increasing](https://lucide.dev/icons/file-chart-column-increasing) | `file-chart-column-increasing` | `<FileChartColumnIncreasing />` |
| [file-chart-line](https://lucide.dev/icons/file-chart-line) | `file-chart-line` | `<FileChartLine />` |
| [file-chart-pie](https://lucide.dev/icons/file-chart-pie) | `file-chart-pie` | `<FileChartPie />` |
| [file-check](https://lucide.dev/icons/file-check) | `file-check` | `<FileCheck />` |
| [file-check-2](https://lucide.dev/icons/file-check-2) | `file-check-2` | `<FileCheck2 />` |
| [file-clock](https://lucide.dev/icons/file-clock) | `file-clock` | `<FileClock />` |
| [file-code](https://lucide.dev/icons/file-code) | `file-code` | `<FileCode />` |
| [file-code-2](https://lucide.dev/icons/file-code-2) | `file-code-2` | `<FileCode2 />` |
| [file-cog](https://lucide.dev/icons/file-cog) | `file-cog` | `<FileCog />` |
| [file-diff](https://lucide.dev/icons/file-diff) | `file-diff` | `<FileDiff />` |
| [file-digit](https://lucide.dev/icons/file-digit) | `file-digit` | `<FileDigit />` |
| [file-down](https://lucide.dev/icons/file-down) | `file-down` | `<FileDown />` |
| [file-heart](https://lucide.dev/icons/file-heart) | `file-heart` | `<FileHeart />` |
| [file-image](https://lucide.dev/icons/file-image) | `file-image` | `<FileImage />` |
| [file-input](https://lucide.dev/icons/file-input) | `file-input` | `<FileInput />` |
| [file-json](https://lucide.dev/icons/file-json) | `file-json` | `<FileJson />` |
| [file-json-2](https://lucide.dev/icons/file-json-2) | `file-json-2` | `<FileJson2 />` |
| [file-key](https://lucide.dev/icons/file-key) | `file-key` | `<FileKey />` |
| [file-key-2](https://lucide.dev/icons/file-key-2) | `file-key-2` | `<FileKey2 />` |
| [file-lock](https://lucide.dev/icons/file-lock) | `file-lock` | `<FileLock />` |
| [file-lock-2](https://lucide.dev/icons/file-lock-2) | `file-lock-2` | `<FileLock2 />` |
| [file-minus](https://lucide.dev/icons/file-minus) | `file-minus` | `<FileMinus />` |
| [file-minus-2](https://lucide.dev/icons/file-minus-2) | `file-minus-2` | `<FileMinus2 />` |
| [file-music](https://lucide.dev/icons/file-music) | `file-music` | `<FileMusic />` |
| [file-output](https://lucide.dev/icons/file-output) | `file-output` | `<FileOutput />` |
| [file-pen](https://lucide.dev/icons/file-pen) | `file-pen` | `<FilePen />` |
| [file-pen-line](https://lucide.dev/icons/file-pen-line) | `file-pen-line` | `<FilePenLine />` |
| [file-play](https://lucide.dev/icons/file-play) | `file-play` | `<FilePlay />` |
| [file-plus](https://lucide.dev/icons/file-plus) | `file-plus` | `<FilePlus />` |
| [file-plus-2](https://lucide.dev/icons/file-plus-2) | `file-plus-2` | `<FilePlus2 />` |
| [file-question-mark](https://lucide.dev/icons/file-question-mark) | `file-question-mark` | `<FileQuestionMark />` |
| [file-scan](https://lucide.dev/icons/file-scan) | `file-scan` | `<FileScan />` |
| [file-search](https://lucide.dev/icons/file-search) | `file-search` | `<FileSearch />` |
| [file-search-2](https://lucide.dev/icons/file-search-2) | `file-search-2` | `<FileSearch2 />` |
| [file-sliders](https://lucide.dev/icons/file-sliders) | `file-sliders` | `<FileSliders />` |
| [file-spreadsheet](https://lucide.dev/icons/file-spreadsheet) | `file-spreadsheet` | `<FileSpreadsheet />` |
| [file-stack](https://lucide.dev/icons/file-stack) | `file-stack` | `<FileStack />` |
| [file-symlink](https://lucide.dev/icons/file-symlink) | `file-symlink` | `<FileSymlink />` |
| [file-terminal](https://lucide.dev/icons/file-terminal) | `file-terminal` | `<FileTerminal />` |
| [file-text](https://lucide.dev/icons/file-text) | `file-text` | `<FileText />` |
| [file-type](https://lucide.dev/icons/file-type) | `file-type` | `<FileType />` |
| [file-type-2](https://lucide.dev/icons/file-type-2) | `file-type-2` | `<FileType2 />` |
| [file-up](https://lucide.dev/icons/file-up) | `file-up` | `<FileUp />` |
| [file-user](https://lucide.dev/icons/file-user) | `file-user` | `<FileUser />` |
| [file-video-camera](https://lucide.dev/icons/file-video-camera) | `file-video-camera` | `<FileVideoCamera />` |
| [file-volume](https://lucide.dev/icons/file-volume) | `file-volume` | `<FileVolume />` |
| [file-volume-2](https://lucide.dev/icons/file-volume-2) | `file-volume-2` | `<FileVolume2 />` |
| [file-warning](https://lucide.dev/icons/file-warning) | `file-warning` | `<FileWarning />` |
| [file-x](https://lucide.dev/icons/file-x) | `file-x` | `<FileX />` |
| [file-x-2](https://lucide.dev/icons/file-x-2) | `file-x-2` | `<FileX2 />` |
| [files](https://lucide.dev/icons/files) | `files` | `<Files />` |
| [folder](https://lucide.dev/icons/folder) | `folder` | `<Folder />` |
| [folder-archive](https://lucide.dev/icons/folder-archive) | `folder-archive` | `<FolderArchive />` |
| [folder-check](https://lucide.dev/icons/folder-check) | `folder-check` | `<FolderCheck />` |
| [folder-clock](https://lucide.dev/icons/folder-clock) | `folder-clock` | `<FolderClock />` |
| [folder-closed](https://lucide.dev/icons/folder-closed) | `folder-closed` | `<FolderClosed />` |
| [folder-code](https://lucide.dev/icons/folder-code) | `folder-code` | `<FolderCode />` |
| [folder-cog](https://lucide.dev/icons/folder-cog) | `folder-cog` | `<FolderCog />` |
| [folder-dot](https://lucide.dev/icons/folder-dot) | `folder-dot` | `<FolderDot />` |
| [folder-down](https://lucide.dev/icons/folder-down) | `folder-down` | `<FolderDown />` |
| [folder-git](https://lucide.dev/icons/folder-git) | `folder-git` | `<FolderGit />` |
| [folder-git-2](https://lucide.dev/icons/folder-git-2) | `folder-git-2` | `<FolderGit2 />` |
| [folder-heart](https://lucide.dev/icons/folder-heart) | `folder-heart` | `<FolderHeart />` |
| [folder-input](https://lucide.dev/icons/folder-input) | `folder-input` | `<FolderInput />` |
| [folder-kanban](https://lucide.dev/icons/folder-kanban) | `folder-kanban` | `<FolderKanban />` |
| [folder-key](https://lucide.dev/icons/folder-key) | `folder-key` | `<FolderKey />` |
| [folder-lock](https://lucide.dev/icons/folder-lock) | `folder-lock` | `<FolderLock />` |
| [folder-minus](https://lucide.dev/icons/folder-minus) | `folder-minus` | `<FolderMinus />` |
| [folder-open](https://lucide.dev/icons/folder-open) | `folder-open` | `<FolderOpen />` |
| [folder-open-dot](https://lucide.dev/icons/folder-open-dot) | `folder-open-dot` | `<FolderOpenDot />` |
| [folder-output](https://lucide.dev/icons/folder-output) | `folder-output` | `<FolderOutput />` |
| [folder-pen](https://lucide.dev/icons/folder-pen) | `folder-pen` | `<FolderPen />` |
| [folder-plus](https://lucide.dev/icons/folder-plus) | `folder-plus` | `<FolderPlus />` |
| [folder-root](https://lucide.dev/icons/folder-root) | `folder-root` | `<FolderRoot />` |
| [folder-search](https://lucide.dev/icons/folder-search) | `folder-search` | `<FolderSearch />` |
| [folder-search-2](https://lucide.dev/icons/folder-search-2) | `folder-search-2` | `<FolderSearch2 />` |
| [folder-symlink](https://lucide.dev/icons/folder-symlink) | `folder-symlink` | `<FolderSymlink />` |
| [folder-sync](https://lucide.dev/icons/folder-sync) | `folder-sync` | `<FolderSync />` |
| [folder-tree](https://lucide.dev/icons/folder-tree) | `folder-tree` | `<FolderTree />` |
| [folder-up](https://lucide.dev/icons/folder-up) | `folder-up` | `<FolderUp />` |
| [folder-x](https://lucide.dev/icons/folder-x) | `folder-x` | `<FolderX />` |
| [folders](https://lucide.dev/icons/folders) | `folders` | `<Folders />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [group](https://lucide.dev/icons/group) | `group` | `<Group />` |
| [hard-drive-download](https://lucide.dev/icons/hard-drive-download) | `hard-drive-download` | `<HardDriveDownload />` |
| [hard-drive-upload](https://lucide.dev/icons/hard-drive-upload) | `hard-drive-upload` | `<HardDriveUpload />` |
| [headphones](https://lucide.dev/icons/headphones) | `headphones` | `<Headphones />` |
| [headset](https://lucide.dev/icons/headset) | `headset` | `<Headset />` |
| [image](https://lucide.dev/icons/image) | `image` | `<Image />` |
| [image-down](https://lucide.dev/icons/image-down) | `image-down` | `<ImageDown />` |
| [image-minus](https://lucide.dev/icons/image-minus) | `image-minus` | `<ImageMinus />` |
| [image-off](https://lucide.dev/icons/image-off) | `image-off` | `<ImageOff />` |
| [image-play](https://lucide.dev/icons/image-play) | `image-play` | `<ImagePlay />` |
| [image-plus](https://lucide.dev/icons/image-plus) | `image-plus` | `<ImagePlus />` |
| [image-up](https://lucide.dev/icons/image-up) | `image-up` | `<ImageUp />` |
| [images](https://lucide.dev/icons/images) | `images` | `<Images />` |
| [import](https://lucide.dev/icons/import) | `import` | `<Import />` |
| [list-tree](https://lucide.dev/icons/list-tree) | `list-tree` | `<ListTree />` |
| [message-square-diff](https://lucide.dev/icons/message-square-diff) | `message-square-diff` | `<MessageSquareDiff />` |
| [music](https://lucide.dev/icons/music) | `music` | `<Music />` |
| [music-2](https://lucide.dev/icons/music-2) | `music-2` | `<Music2 />` |
| [music-3](https://lucide.dev/icons/music-3) | `music-3` | `<Music3 />` |
| [music-4](https://lucide.dev/icons/music-4) | `music-4` | `<Music4 />` |
| [package](https://lucide.dev/icons/package) | `package` | `<Package />` |
| [package-2](https://lucide.dev/icons/package-2) | `package-2` | `<Package2 />` |
| [package-open](https://lucide.dev/icons/package-open) | `package-open` | `<PackageOpen />` |
| [package-search](https://lucide.dev/icons/package-search) | `package-search` | `<PackageSearch />` |
| [paperclip](https://lucide.dev/icons/paperclip) | `paperclip` | `<Paperclip />` |
| [parentheses](https://lucide.dev/icons/parentheses) | `parentheses` | `<Parentheses />` |
| [save](https://lucide.dev/icons/save) | `save` | `<Save />` |
| [save-all](https://lucide.dev/icons/save-all) | `save-all` | `<SaveAll />` |
| [save-off](https://lucide.dev/icons/save-off) | `save-off` | `<SaveOff />` |
| [sheet](https://lucide.dev/icons/sheet) | `sheet` | `<Sheet />` |
| [shredder](https://lucide.dev/icons/shredder) | `shredder` | `<Shredder />` |
| [square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) | `square-bottom-dashed-scissors` | `<SquareBottomDashedScissors />` |
| [square-dashed-bottom](https://lucide.dev/icons/square-dashed-bottom) | `square-dashed-bottom` | `<SquareDashedBottom />` |
| [square-dashed-bottom-code](https://lucide.dev/icons/square-dashed-bottom-code) | `square-dashed-bottom-code` | `<SquareDashedBottomCode />` |
| [square-scissors](https://lucide.dev/icons/square-scissors) | `square-scissors` | `<SquareScissors />` |
| [square-stack](https://lucide.dev/icons/square-stack) | `square-stack` | `<SquareStack />` |
| [table](https://lucide.dev/icons/table) | `table` | `<Table />` |
| [table-2](https://lucide.dev/icons/table-2) | `table-2` | `<Table2 />` |
| [table-cells-merge](https://lucide.dev/icons/table-cells-merge) | `table-cells-merge` | `<TableCellsMerge />` |
| [table-cells-split](https://lucide.dev/icons/table-cells-split) | `table-cells-split` | `<TableCellsSplit />` |
| [table-columns-split](https://lucide.dev/icons/table-columns-split) | `table-columns-split` | `<TableColumnsSplit />` |
| [table-properties](https://lucide.dev/icons/table-properties) | `table-properties` | `<TableProperties />` |
| [table-rows-split](https://lucide.dev/icons/table-rows-split) | `table-rows-split` | `<TableRowsSplit />` |
| [trash](https://lucide.dev/icons/trash) | `trash` | `<Trash />` |
| [trash-2](https://lucide.dev/icons/trash-2) | `trash-2` | `<Trash2 />` |
| [ungroup](https://lucide.dev/icons/ungroup) | `ungroup` | `<Ungroup />` |
| [upload](https://lucide.dev/icons/upload) | `upload` | `<Upload />` |
| [videotape](https://lucide.dev/icons/videotape) | `videotape` | `<Videotape />` |
| [wifi-cog](https://lucide.dev/icons/wifi-cog) | `wifi-cog` | `<WifiCog />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "files"] or ["leptos", "all-icons"]
<AppWindow class="w-6 h-6 text-gray-600" />
```

### Finance

`finance` and `all-icons` features - 55 icons

| Icon | Name | Component |
|------|------|-----------|
| [badge-cent](https://lucide.dev/icons/badge-cent) | `badge-cent` | `<BadgeCent />` |
| [badge-dollar-sign](https://lucide.dev/icons/badge-dollar-sign) | `badge-dollar-sign` | `<BadgeDollarSign />` |
| [badge-euro](https://lucide.dev/icons/badge-euro) | `badge-euro` | `<BadgeEuro />` |
| [badge-indian-rupee](https://lucide.dev/icons/badge-indian-rupee) | `badge-indian-rupee` | `<BadgeIndianRupee />` |
| [badge-japanese-yen](https://lucide.dev/icons/badge-japanese-yen) | `badge-japanese-yen` | `<BadgeJapaneseYen />` |
| [badge-percent](https://lucide.dev/icons/badge-percent) | `badge-percent` | `<BadgePercent />` |
| [badge-pound-sterling](https://lucide.dev/icons/badge-pound-sterling) | `badge-pound-sterling` | `<BadgePoundSterling />` |
| [badge-russian-ruble](https://lucide.dev/icons/badge-russian-ruble) | `badge-russian-ruble` | `<BadgeRussianRuble />` |
| [badge-swiss-franc](https://lucide.dev/icons/badge-swiss-franc) | `badge-swiss-franc` | `<BadgeSwissFranc />` |
| [badge-turkish-lira](https://lucide.dev/icons/badge-turkish-lira) | `badge-turkish-lira` | `<BadgeTurkishLira />` |
| [banknote](https://lucide.dev/icons/banknote) | `banknote` | `<Banknote />` |
| [banknote-arrow-down](https://lucide.dev/icons/banknote-arrow-down) | `banknote-arrow-down` | `<BanknoteArrowDown />` |
| [banknote-arrow-up](https://lucide.dev/icons/banknote-arrow-up) | `banknote-arrow-up` | `<BanknoteArrowUp />` |
| [banknote-x](https://lucide.dev/icons/banknote-x) | `banknote-x` | `<BanknoteX />` |
| [bitcoin](https://lucide.dev/icons/bitcoin) | `bitcoin` | `<Bitcoin />` |
| [chart-candlestick](https://lucide.dev/icons/chart-candlestick) | `chart-candlestick` | `<ChartCandlestick />` |
| [circle-dollar-sign](https://lucide.dev/icons/circle-dollar-sign) | `circle-dollar-sign` | `<CircleDollarSign />` |
| [circle-percent](https://lucide.dev/icons/circle-percent) | `circle-percent` | `<CirclePercent />` |
| [circle-pound-sterling](https://lucide.dev/icons/circle-pound-sterling) | `circle-pound-sterling` | `<CirclePoundSterling />` |
| [credit-card](https://lucide.dev/icons/credit-card) | `credit-card` | `<CreditCard />` |
| [currency](https://lucide.dev/icons/currency) | `currency` | `<Currency />` |
| [diamond-percent](https://lucide.dev/icons/diamond-percent) | `diamond-percent` | `<DiamondPercent />` |
| [dollar-sign](https://lucide.dev/icons/dollar-sign) | `dollar-sign` | `<DollarSign />` |
| [euro](https://lucide.dev/icons/euro) | `euro` | `<Euro />` |
| [gem](https://lucide.dev/icons/gem) | `gem` | `<Gem />` |
| [georgian-lari](https://lucide.dev/icons/georgian-lari) | `georgian-lari` | `<GeorgianLari />` |
| [hand-coins](https://lucide.dev/icons/hand-coins) | `hand-coins` | `<HandCoins />` |
| [handshake](https://lucide.dev/icons/handshake) | `handshake` | `<Handshake />` |
| [indian-rupee](https://lucide.dev/icons/indian-rupee) | `indian-rupee` | `<IndianRupee />` |
| [japanese-yen](https://lucide.dev/icons/japanese-yen) | `japanese-yen` | `<JapaneseYen />` |
| [landmark](https://lucide.dev/icons/landmark) | `landmark` | `<Landmark />` |
| [nfc](https://lucide.dev/icons/nfc) | `nfc` | `<Nfc />` |
| [percent](https://lucide.dev/icons/percent) | `percent` | `<Percent />` |
| [philippine-peso](https://lucide.dev/icons/philippine-peso) | `philippine-peso` | `<PhilippinePeso />` |
| [piggy-bank](https://lucide.dev/icons/piggy-bank) | `piggy-bank` | `<PiggyBank />` |
| [pound-sterling](https://lucide.dev/icons/pound-sterling) | `pound-sterling` | `<PoundSterling />` |
| [receipt](https://lucide.dev/icons/receipt) | `receipt` | `<Receipt />` |
| [receipt-cent](https://lucide.dev/icons/receipt-cent) | `receipt-cent` | `<ReceiptCent />` |
| [receipt-euro](https://lucide.dev/icons/receipt-euro) | `receipt-euro` | `<ReceiptEuro />` |
| [receipt-indian-rupee](https://lucide.dev/icons/receipt-indian-rupee) | `receipt-indian-rupee` | `<ReceiptIndianRupee />` |
| [receipt-japanese-yen](https://lucide.dev/icons/receipt-japanese-yen) | `receipt-japanese-yen` | `<ReceiptJapaneseYen />` |
| [receipt-pound-sterling](https://lucide.dev/icons/receipt-pound-sterling) | `receipt-pound-sterling` | `<ReceiptPoundSterling />` |
| [receipt-russian-ruble](https://lucide.dev/icons/receipt-russian-ruble) | `receipt-russian-ruble` | `<ReceiptRussianRuble />` |
| [receipt-swiss-franc](https://lucide.dev/icons/receipt-swiss-franc) | `receipt-swiss-franc` | `<ReceiptSwissFranc />` |
| [receipt-text](https://lucide.dev/icons/receipt-text) | `receipt-text` | `<ReceiptText />` |
| [receipt-turkish-lira](https://lucide.dev/icons/receipt-turkish-lira) | `receipt-turkish-lira` | `<ReceiptTurkishLira />` |
| [russian-ruble](https://lucide.dev/icons/russian-ruble) | `russian-ruble` | `<RussianRuble />` |
| [saudi-riyal](https://lucide.dev/icons/saudi-riyal) | `saudi-riyal` | `<SaudiRiyal />` |
| [smartphone-nfc](https://lucide.dev/icons/smartphone-nfc) | `smartphone-nfc` | `<SmartphoneNfc />` |
| [square-percent](https://lucide.dev/icons/square-percent) | `square-percent` | `<SquarePercent />` |
| [swiss-franc](https://lucide.dev/icons/swiss-franc) | `swiss-franc` | `<SwissFranc />` |
| [turkish-lira](https://lucide.dev/icons/turkish-lira) | `turkish-lira` | `<TurkishLira />` |
| [wallet](https://lucide.dev/icons/wallet) | `wallet` | `<Wallet />` |
| [wallet-cards](https://lucide.dev/icons/wallet-cards) | `wallet-cards` | `<WalletCards />` |
| [wallet-minimal](https://lucide.dev/icons/wallet-minimal) | `wallet-minimal` | `<WalletMinimal />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "finance"] or ["leptos", "all-icons"]
<BadgeCent class="w-6 h-6 text-gray-600" />
```

### Food Beverage

`food-beverage` and `all-icons` features - 69 icons

| Icon | Name | Component |
|------|------|-----------|
| [amphora](https://lucide.dev/icons/amphora) | `amphora` | `<Amphora />` |
| [apple](https://lucide.dev/icons/apple) | `apple` | `<Apple />` |
| [banana](https://lucide.dev/icons/banana) | `banana` | `<Banana />` |
| [barrel](https://lucide.dev/icons/barrel) | `barrel` | `<Barrel />` |
| [bean](https://lucide.dev/icons/bean) | `bean` | `<Bean />` |
| [bean-off](https://lucide.dev/icons/bean-off) | `bean-off` | `<BeanOff />` |
| [beef](https://lucide.dev/icons/beef) | `beef` | `<Beef />` |
| [beer](https://lucide.dev/icons/beer) | `beer` | `<Beer />` |
| [beer-off](https://lucide.dev/icons/beer-off) | `beer-off` | `<BeerOff />` |
| [bottle-wine](https://lucide.dev/icons/bottle-wine) | `bottle-wine` | `<BottleWine />` |
| [cake](https://lucide.dev/icons/cake) | `cake` | `<Cake />` |
| [cake-slice](https://lucide.dev/icons/cake-slice) | `cake-slice` | `<CakeSlice />` |
| [candy](https://lucide.dev/icons/candy) | `candy` | `<Candy />` |
| [candy-cane](https://lucide.dev/icons/candy-cane) | `candy-cane` | `<CandyCane />` |
| [candy-off](https://lucide.dev/icons/candy-off) | `candy-off` | `<CandyOff />` |
| [carrot](https://lucide.dev/icons/carrot) | `carrot` | `<Carrot />` |
| [chef-hat](https://lucide.dev/icons/chef-hat) | `chef-hat` | `<ChefHat />` |
| [cherry](https://lucide.dev/icons/cherry) | `cherry` | `<Cherry />` |
| [citrus](https://lucide.dev/icons/citrus) | `citrus` | `<Citrus />` |
| [coffee](https://lucide.dev/icons/coffee) | `coffee` | `<Coffee />` |
| [cookie](https://lucide.dev/icons/cookie) | `cookie` | `<Cookie />` |
| [cooking-pot](https://lucide.dev/icons/cooking-pot) | `cooking-pot` | `<CookingPot />` |
| [croissant](https://lucide.dev/icons/croissant) | `croissant` | `<Croissant />` |
| [cup-soda](https://lucide.dev/icons/cup-soda) | `cup-soda` | `<CupSoda />` |
| [dessert](https://lucide.dev/icons/dessert) | `dessert` | `<Dessert />` |
| [dna-off](https://lucide.dev/icons/dna-off) | `dna-off` | `<DnaOff />` |
| [donut](https://lucide.dev/icons/donut) | `donut` | `<Donut />` |
| [drumstick](https://lucide.dev/icons/drumstick) | `drumstick` | `<Drumstick />` |
| [egg](https://lucide.dev/icons/egg) | `egg` | `<Egg />` |
| [egg-fried](https://lucide.dev/icons/egg-fried) | `egg-fried` | `<EggFried />` |
| [egg-off](https://lucide.dev/icons/egg-off) | `egg-off` | `<EggOff />` |
| [fish](https://lucide.dev/icons/fish) | `fish` | `<Fish />` |
| [fish-off](https://lucide.dev/icons/fish-off) | `fish-off` | `<FishOff />` |
| [fish-symbol](https://lucide.dev/icons/fish-symbol) | `fish-symbol` | `<FishSymbol />` |
| [glass-water](https://lucide.dev/icons/glass-water) | `glass-water` | `<GlassWater />` |
| [grape](https://lucide.dev/icons/grape) | `grape` | `<Grape />` |
| [ham](https://lucide.dev/icons/ham) | `ham` | `<Ham />` |
| [hamburger](https://lucide.dev/icons/hamburger) | `hamburger` | `<Hamburger />` |
| [hand-platter](https://lucide.dev/icons/hand-platter) | `hand-platter` | `<HandPlatter />` |
| [hop](https://lucide.dev/icons/hop) | `hop` | `<Hop />` |
| [hop-off](https://lucide.dev/icons/hop-off) | `hop-off` | `<HopOff />` |
| [ice-cream-bowl](https://lucide.dev/icons/ice-cream-bowl) | `ice-cream-bowl` | `<IceCreamBowl />` |
| [ice-cream-cone](https://lucide.dev/icons/ice-cream-cone) | `ice-cream-cone` | `<IceCreamCone />` |
| [leafy-green](https://lucide.dev/icons/leafy-green) | `leafy-green` | `<LeafyGreen />` |
| [lollipop](https://lucide.dev/icons/lollipop) | `lollipop` | `<Lollipop />` |
| [martini](https://lucide.dev/icons/martini) | `martini` | `<Martini />` |
| [microwave](https://lucide.dev/icons/microwave) | `microwave` | `<Microwave />` |
| [milk](https://lucide.dev/icons/milk) | `milk` | `<Milk />` |
| [milk-off](https://lucide.dev/icons/milk-off) | `milk-off` | `<MilkOff />` |
| [nut](https://lucide.dev/icons/nut) | `nut` | `<Nut />` |
| [nut-off](https://lucide.dev/icons/nut-off) | `nut-off` | `<NutOff />` |
| [pizza](https://lucide.dev/icons/pizza) | `pizza` | `<Pizza />` |
| [popcorn](https://lucide.dev/icons/popcorn) | `popcorn` | `<Popcorn />` |
| [popsicle](https://lucide.dev/icons/popsicle) | `popsicle` | `<Popsicle />` |
| [refrigerator](https://lucide.dev/icons/refrigerator) | `refrigerator` | `<Refrigerator />` |
| [salad](https://lucide.dev/icons/salad) | `salad` | `<Salad />` |
| [sandwich](https://lucide.dev/icons/sandwich) | `sandwich` | `<Sandwich />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [snail](https://lucide.dev/icons/snail) | `snail` | `<Snail />` |
| [soup](https://lucide.dev/icons/soup) | `soup` | `<Soup />` |
| [torus](https://lucide.dev/icons/torus) | `torus` | `<Torus />` |
| [tractor](https://lucide.dev/icons/tractor) | `tractor` | `<Tractor />` |
| [utensils](https://lucide.dev/icons/utensils) | `utensils` | `<Utensils />` |
| [utensils-crossed](https://lucide.dev/icons/utensils-crossed) | `utensils-crossed` | `<UtensilsCrossed />` |
| [vegan](https://lucide.dev/icons/vegan) | `vegan` | `<Vegan />` |
| [wheat](https://lucide.dev/icons/wheat) | `wheat` | `<Wheat />` |
| [wheat-off](https://lucide.dev/icons/wheat-off) | `wheat-off` | `<WheatOff />` |
| [wine](https://lucide.dev/icons/wine) | `wine` | `<Wine />` |
| [wine-off](https://lucide.dev/icons/wine-off) | `wine-off` | `<WineOff />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "food-beverage"] or ["leptos", "all-icons"]
<Amphora class="w-6 h-6 text-gray-600" />
```

### Gaming

`gaming` and `all-icons` features - 140 icons

| Icon | Name | Component |
|------|------|-----------|
| [amphora](https://lucide.dev/icons/amphora) | `amphora` | `<Amphora />` |
| [anvil](https://lucide.dev/icons/anvil) | `anvil` | `<Anvil />` |
| [arrow-big-down](https://lucide.dev/icons/arrow-big-down) | `arrow-big-down` | `<ArrowBigDown />` |
| [arrow-big-down-dash](https://lucide.dev/icons/arrow-big-down-dash) | `arrow-big-down-dash` | `<ArrowBigDownDash />` |
| [arrow-big-left](https://lucide.dev/icons/arrow-big-left) | `arrow-big-left` | `<ArrowBigLeft />` |
| [arrow-big-left-dash](https://lucide.dev/icons/arrow-big-left-dash) | `arrow-big-left-dash` | `<ArrowBigLeftDash />` |
| [arrow-big-right](https://lucide.dev/icons/arrow-big-right) | `arrow-big-right` | `<ArrowBigRight />` |
| [arrow-big-right-dash](https://lucide.dev/icons/arrow-big-right-dash) | `arrow-big-right-dash` | `<ArrowBigRightDash />` |
| [arrow-big-up](https://lucide.dev/icons/arrow-big-up) | `arrow-big-up` | `<ArrowBigUp />` |
| [arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) | `arrow-big-up-dash` | `<ArrowBigUpDash />` |
| [award](https://lucide.dev/icons/award) | `award` | `<Award />` |
| [axe](https://lucide.dev/icons/axe) | `axe` | `<Axe />` |
| [backpack](https://lucide.dev/icons/backpack) | `backpack` | `<Backpack />` |
| [beaker](https://lucide.dev/icons/beaker) | `beaker` | `<Beaker />` |
| [bone](https://lucide.dev/icons/bone) | `bone` | `<Bone />` |
| [book](https://lucide.dev/icons/book) | `book` | `<Book />` |
| [book-a](https://lucide.dev/icons/book-a) | `book-a` | `<BookA />` |
| [book-alert](https://lucide.dev/icons/book-alert) | `book-alert` | `<BookAlert />` |
| [book-check](https://lucide.dev/icons/book-check) | `book-check` | `<BookCheck />` |
| [book-copy](https://lucide.dev/icons/book-copy) | `book-copy` | `<BookCopy />` |
| [book-heart](https://lucide.dev/icons/book-heart) | `book-heart` | `<BookHeart />` |
| [book-key](https://lucide.dev/icons/book-key) | `book-key` | `<BookKey />` |
| [book-lock](https://lucide.dev/icons/book-lock) | `book-lock` | `<BookLock />` |
| [book-marked](https://lucide.dev/icons/book-marked) | `book-marked` | `<BookMarked />` |
| [book-minus](https://lucide.dev/icons/book-minus) | `book-minus` | `<BookMinus />` |
| [book-open](https://lucide.dev/icons/book-open) | `book-open` | `<BookOpen />` |
| [book-open-check](https://lucide.dev/icons/book-open-check) | `book-open-check` | `<BookOpenCheck />` |
| [book-plus](https://lucide.dev/icons/book-plus) | `book-plus` | `<BookPlus />` |
| [book-text](https://lucide.dev/icons/book-text) | `book-text` | `<BookText />` |
| [book-type](https://lucide.dev/icons/book-type) | `book-type` | `<BookType />` |
| [book-x](https://lucide.dev/icons/book-x) | `book-x` | `<BookX />` |
| [bow-arrow](https://lucide.dev/icons/bow-arrow) | `bow-arrow` | `<BowArrow />` |
| [box](https://lucide.dev/icons/box) | `box` | `<Box />` |
| [boxes](https://lucide.dev/icons/boxes) | `boxes` | `<Boxes />` |
| [castle](https://lucide.dev/icons/castle) | `castle` | `<Castle />` |
| [chevron-down](https://lucide.dev/icons/chevron-down) | `chevron-down` | `<ChevronDown />` |
| [chevron-up](https://lucide.dev/icons/chevron-up) | `chevron-up` | `<ChevronUp />` |
| [chevrons-down](https://lucide.dev/icons/chevrons-down) | `chevrons-down` | `<ChevronsDown />` |
| [chevrons-left](https://lucide.dev/icons/chevrons-left) | `chevrons-left` | `<ChevronsLeft />` |
| [chevrons-left-right-ellipsis](https://lucide.dev/icons/chevrons-left-right-ellipsis) | `chevrons-left-right-ellipsis` | `<ChevronsLeftRightEllipsis />` |
| [chevrons-right](https://lucide.dev/icons/chevrons-right) | `chevrons-right` | `<ChevronsRight />` |
| [chevrons-up](https://lucide.dev/icons/chevrons-up) | `chevrons-up` | `<ChevronsUp />` |
| [circle-arrow-down](https://lucide.dev/icons/circle-arrow-down) | `circle-arrow-down` | `<CircleArrowDown />` |
| [circle-arrow-left](https://lucide.dev/icons/circle-arrow-left) | `circle-arrow-left` | `<CircleArrowLeft />` |
| [circle-arrow-right](https://lucide.dev/icons/circle-arrow-right) | `circle-arrow-right` | `<CircleArrowRight />` |
| [circle-arrow-up](https://lucide.dev/icons/circle-arrow-up) | `circle-arrow-up` | `<CircleArrowUp />` |
| [circle-plus](https://lucide.dev/icons/circle-plus) | `circle-plus` | `<CirclePlus />` |
| [circle-star](https://lucide.dev/icons/circle-star) | `circle-star` | `<CircleStar />` |
| [clover](https://lucide.dev/icons/clover) | `clover` | `<Clover />` |
| [club](https://lucide.dev/icons/club) | `club` | `<Club />` |
| [coins](https://lucide.dev/icons/coins) | `coins` | `<Coins />` |
| [computer](https://lucide.dev/icons/computer) | `computer` | `<Computer />` |
| [crown](https://lucide.dev/icons/crown) | `crown` | `<Crown />` |
| [diamond](https://lucide.dev/icons/diamond) | `diamond` | `<Diamond />` |
| [dice-1](https://lucide.dev/icons/dice-1) | `dice-1` | `<Dice1 />` |
| [dice-2](https://lucide.dev/icons/dice-2) | `dice-2` | `<Dice2 />` |
| [dice-3](https://lucide.dev/icons/dice-3) | `dice-3` | `<Dice3 />` |
| [dice-4](https://lucide.dev/icons/dice-4) | `dice-4` | `<Dice4 />` |
| [dice-5](https://lucide.dev/icons/dice-5) | `dice-5` | `<Dice5 />` |
| [dice-6](https://lucide.dev/icons/dice-6) | `dice-6` | `<Dice6 />` |
| [dices](https://lucide.dev/icons/dices) | `dices` | `<Dices />` |
| [droplet](https://lucide.dev/icons/droplet) | `droplet` | `<Droplet />` |
| [droplet-off](https://lucide.dev/icons/droplet-off) | `droplet-off` | `<DropletOff />` |
| [ethernet-port](https://lucide.dev/icons/ethernet-port) | `ethernet-port` | `<EthernetPort />` |
| [feather](https://lucide.dev/icons/feather) | `feather` | `<Feather />` |
| [flame](https://lucide.dev/icons/flame) | `flame` | `<Flame />` |
| [flame-kindling](https://lucide.dev/icons/flame-kindling) | `flame-kindling` | `<FlameKindling />` |
| [flask-conical](https://lucide.dev/icons/flask-conical) | `flask-conical` | `<FlaskConical />` |
| [flask-conical-off](https://lucide.dev/icons/flask-conical-off) | `flask-conical-off` | `<FlaskConicalOff />` |
| [flask-round](https://lucide.dev/icons/flask-round) | `flask-round` | `<FlaskRound />` |
| [flower](https://lucide.dev/icons/flower) | `flower` | `<Flower />` |
| [gamepad](https://lucide.dev/icons/gamepad) | `gamepad` | `<Gamepad />` |
| [gamepad-2](https://lucide.dev/icons/gamepad-2) | `gamepad-2` | `<Gamepad2 />` |
| [gem](https://lucide.dev/icons/gem) | `gem` | `<Gem />` |
| [ghost](https://lucide.dev/icons/ghost) | `ghost` | `<Ghost />` |
| [gift](https://lucide.dev/icons/gift) | `gift` | `<Gift />` |
| [goal](https://lucide.dev/icons/goal) | `goal` | `<Goal />` |
| [gpu](https://lucide.dev/icons/gpu) | `gpu` | `<Gpu />` |
| [hdmi-port](https://lucide.dev/icons/hdmi-port) | `hdmi-port` | `<HdmiPort />` |
| [headphone-off](https://lucide.dev/icons/headphone-off) | `headphone-off` | `<HeadphoneOff />` |
| [headphones](https://lucide.dev/icons/headphones) | `headphones` | `<Headphones />` |
| [headset](https://lucide.dev/icons/headset) | `headset` | `<Headset />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [heart-minus](https://lucide.dev/icons/heart-minus) | `heart-minus` | `<HeartMinus />` |
| [heart-plus](https://lucide.dev/icons/heart-plus) | `heart-plus` | `<HeartPlus />` |
| [hourglass](https://lucide.dev/icons/hourglass) | `hourglass` | `<Hourglass />` |
| [joystick](https://lucide.dev/icons/joystick) | `joystick` | `<Joystick />` |
| [land-plot](https://lucide.dev/icons/land-plot) | `land-plot` | `<LandPlot />` |
| [medal](https://lucide.dev/icons/medal) | `medal` | `<Medal />` |
| [memory-stick](https://lucide.dev/icons/memory-stick) | `memory-stick` | `<MemoryStick />` |
| [milestone](https://lucide.dev/icons/milestone) | `milestone` | `<Milestone />` |
| [mountain](https://lucide.dev/icons/mountain) | `mountain` | `<Mountain />` |
| [pc-case](https://lucide.dev/icons/pc-case) | `pc-case` | `<PcCase />` |
| [pickaxe](https://lucide.dev/icons/pickaxe) | `pickaxe` | `<Pickaxe />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [puzzle](https://lucide.dev/icons/puzzle) | `puzzle` | `<Puzzle />` |
| [rectangle-goggles](https://lucide.dev/icons/rectangle-goggles) | `rectangle-goggles` | `<RectangleGoggles />` |
| [rocket](https://lucide.dev/icons/rocket) | `rocket` | `<Rocket />` |
| [scan](https://lucide.dev/icons/scan) | `scan` | `<Scan />` |
| [scroll](https://lucide.dev/icons/scroll) | `scroll` | `<Scroll />` |
| [scroll-text](https://lucide.dev/icons/scroll-text) | `scroll-text` | `<ScrollText />` |
| [shapes](https://lucide.dev/icons/shapes) | `shapes` | `<Shapes />` |
| [shield](https://lucide.dev/icons/shield) | `shield` | `<Shield />` |
| [shield-alert](https://lucide.dev/icons/shield-alert) | `shield-alert` | `<ShieldAlert />` |
| [shield-ban](https://lucide.dev/icons/shield-ban) | `shield-ban` | `<ShieldBan />` |
| [shield-check](https://lucide.dev/icons/shield-check) | `shield-check` | `<ShieldCheck />` |
| [shield-ellipsis](https://lucide.dev/icons/shield-ellipsis) | `shield-ellipsis` | `<ShieldEllipsis />` |
| [shield-half](https://lucide.dev/icons/shield-half) | `shield-half` | `<ShieldHalf />` |
| [shield-minus](https://lucide.dev/icons/shield-minus) | `shield-minus` | `<ShieldMinus />` |
| [shield-off](https://lucide.dev/icons/shield-off) | `shield-off` | `<ShieldOff />` |
| [shield-plus](https://lucide.dev/icons/shield-plus) | `shield-plus` | `<ShieldPlus />` |
| [shield-question-mark](https://lucide.dev/icons/shield-question-mark) | `shield-question-mark` | `<ShieldQuestionMark />` |
| [shield-x](https://lucide.dev/icons/shield-x) | `shield-x` | `<ShieldX />` |
| [shovel](https://lucide.dev/icons/shovel) | `shovel` | `<Shovel />` |
| [signpost](https://lucide.dev/icons/signpost) | `signpost` | `<Signpost />` |
| [signpost-big](https://lucide.dev/icons/signpost-big) | `signpost-big` | `<SignpostBig />` |
| [skull](https://lucide.dev/icons/skull) | `skull` | `<Skull />` |
| [spade](https://lucide.dev/icons/spade) | `spade` | `<Spade />` |
| [sparkles](https://lucide.dev/icons/sparkles) | `sparkles` | `<Sparkles />` |
| [sprout](https://lucide.dev/icons/sprout) | `sprout` | `<Sprout />` |
| [square-arrow-down](https://lucide.dev/icons/square-arrow-down) | `square-arrow-down` | `<SquareArrowDown />` |
| [square-arrow-down-left](https://lucide.dev/icons/square-arrow-down-left) | `square-arrow-down-left` | `<SquareArrowDownLeft />` |
| [square-arrow-down-right](https://lucide.dev/icons/square-arrow-down-right) | `square-arrow-down-right` | `<SquareArrowDownRight />` |
| [square-star](https://lucide.dev/icons/square-star) | `square-star` | `<SquareStar />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [sword](https://lucide.dev/icons/sword) | `sword` | `<Sword />` |
| [swords](https://lucide.dev/icons/swords) | `swords` | `<Swords />` |
| [tally-1](https://lucide.dev/icons/tally-1) | `tally-1` | `<Tally1 />` |
| [tally-2](https://lucide.dev/icons/tally-2) | `tally-2` | `<Tally2 />` |
| [tally-3](https://lucide.dev/icons/tally-3) | `tally-3` | `<Tally3 />` |
| [tally-4](https://lucide.dev/icons/tally-4) | `tally-4` | `<Tally4 />` |
| [tally-5](https://lucide.dev/icons/tally-5) | `tally-5` | `<Tally5 />` |
| [target](https://lucide.dev/icons/target) | `target` | `<Target />` |
| [toy-brick](https://lucide.dev/icons/toy-brick) | `toy-brick` | `<ToyBrick />` |
| [trophy](https://lucide.dev/icons/trophy) | `trophy` | `<Trophy />` |
| [twitch](https://lucide.dev/icons/twitch) | `twitch` | `<Twitch />` |
| [venetian-mask](https://lucide.dev/icons/venetian-mask) | `venetian-mask` | `<VenetianMask />` |
| [volleyball](https://lucide.dev/icons/volleyball) | `volleyball` | `<Volleyball />` |
| [wand](https://lucide.dev/icons/wand) | `wand` | `<Wand />` |
| [wand-sparkles](https://lucide.dev/icons/wand-sparkles) | `wand-sparkles` | `<WandSparkles />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "gaming"] or ["leptos", "all-icons"]
<Amphora class="w-6 h-6 text-gray-600" />
```

### Home

`home` and `all-icons` features - 54 icons

| Icon | Name | Component |
|------|------|-----------|
| [air-vent](https://lucide.dev/icons/air-vent) | `air-vent` | `<AirVent />` |
| [alarm-smoke](https://lucide.dev/icons/alarm-smoke) | `alarm-smoke` | `<AlarmSmoke />` |
| [armchair](https://lucide.dev/icons/armchair) | `armchair` | `<Armchair />` |
| [bed](https://lucide.dev/icons/bed) | `bed` | `<Bed />` |
| [bed-double](https://lucide.dev/icons/bed-double) | `bed-double` | `<BedDouble />` |
| [bed-single](https://lucide.dev/icons/bed-single) | `bed-single` | `<BedSingle />` |
| [bell-electric](https://lucide.dev/icons/bell-electric) | `bell-electric` | `<BellElectric />` |
| [blinds](https://lucide.dev/icons/blinds) | `blinds` | `<Blinds />` |
| [bolt](https://lucide.dev/icons/bolt) | `bolt` | `<Bolt />` |
| [brick-wall](https://lucide.dev/icons/brick-wall) | `brick-wall` | `<BrickWall />` |
| [brick-wall-fire](https://lucide.dev/icons/brick-wall-fire) | `brick-wall-fire` | `<BrickWallFire />` |
| [brick-wall-shield](https://lucide.dev/icons/brick-wall-shield) | `brick-wall-shield` | `<BrickWallShield />` |
| [brush-cleaning](https://lucide.dev/icons/brush-cleaning) | `brush-cleaning` | `<BrushCleaning />` |
| [cooking-pot](https://lucide.dev/icons/cooking-pot) | `cooking-pot` | `<CookingPot />` |
| [door-closed](https://lucide.dev/icons/door-closed) | `door-closed` | `<DoorClosed />` |
| [door-closed-locked](https://lucide.dev/icons/door-closed-locked) | `door-closed-locked` | `<DoorClosedLocked />` |
| [door-open](https://lucide.dev/icons/door-open) | `door-open` | `<DoorOpen />` |
| [drill](https://lucide.dev/icons/drill) | `drill` | `<Drill />` |
| [fan](https://lucide.dev/icons/fan) | `fan` | `<Fan />` |
| [fence](https://lucide.dev/icons/fence) | `fence` | `<Fence />` |
| [fire-extinguisher](https://lucide.dev/icons/fire-extinguisher) | `fire-extinguisher` | `<FireExtinguisher />` |
| [hammer](https://lucide.dev/icons/hammer) | `hammer` | `<Hammer />` |
| [heater](https://lucide.dev/icons/heater) | `heater` | `<Heater />` |
| [house](https://lucide.dev/icons/house) | `house` | `<House />` |
| [house-heart](https://lucide.dev/icons/house-heart) | `house-heart` | `<HouseHeart />` |
| [house-plug](https://lucide.dev/icons/house-plug) | `house-plug` | `<HousePlug />` |
| [house-wifi](https://lucide.dev/icons/house-wifi) | `house-wifi` | `<HouseWifi />` |
| [lamp](https://lucide.dev/icons/lamp) | `lamp` | `<Lamp />` |
| [lamp-ceiling](https://lucide.dev/icons/lamp-ceiling) | `lamp-ceiling` | `<LampCeiling />` |
| [lamp-desk](https://lucide.dev/icons/lamp-desk) | `lamp-desk` | `<LampDesk />` |
| [lamp-floor](https://lucide.dev/icons/lamp-floor) | `lamp-floor` | `<LampFloor />` |
| [lamp-wall-down](https://lucide.dev/icons/lamp-wall-down) | `lamp-wall-down` | `<LampWallDown />` |
| [lamp-wall-up](https://lucide.dev/icons/lamp-wall-up) | `lamp-wall-up` | `<LampWallUp />` |
| [microwave](https://lucide.dev/icons/microwave) | `microwave` | `<Microwave />` |
| [paint-roller](https://lucide.dev/icons/paint-roller) | `paint-roller` | `<PaintRoller />` |
| [paintbrush](https://lucide.dev/icons/paintbrush) | `paintbrush` | `<Paintbrush />` |
| [paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) | `paintbrush-vertical` | `<PaintbrushVertical />` |
| [refrigerator](https://lucide.dev/icons/refrigerator) | `refrigerator` | `<Refrigerator />` |
| [rocking-chair](https://lucide.dev/icons/rocking-chair) | `rocking-chair` | `<RockingChair />` |
| [rose](https://lucide.dev/icons/rose) | `rose` | `<Rose />` |
| [router](https://lucide.dev/icons/router) | `router` | `<Router />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [shower-head](https://lucide.dev/icons/shower-head) | `shower-head` | `<ShowerHead />` |
| [soap-dispenser-droplet](https://lucide.dev/icons/soap-dispenser-droplet) | `soap-dispenser-droplet` | `<SoapDispenserDroplet />` |
| [sofa](https://lucide.dev/icons/sofa) | `sofa` | `<Sofa />` |
| [swatch-book](https://lucide.dev/icons/swatch-book) | `swatch-book` | `<SwatchBook />` |
| [toilet](https://lucide.dev/icons/toilet) | `toilet` | `<Toilet />` |
| [tool-case](https://lucide.dev/icons/tool-case) | `tool-case` | `<ToolCase />` |
| [turntable](https://lucide.dev/icons/turntable) | `turntable` | `<Turntable />` |
| [usb](https://lucide.dev/icons/usb) | `usb` | `<Usb />` |
| [utility-pole](https://lucide.dev/icons/utility-pole) | `utility-pole` | `<UtilityPole />` |
| [vault](https://lucide.dev/icons/vault) | `vault` | `<Vault />` |
| [washing-machine](https://lucide.dev/icons/washing-machine) | `washing-machine` | `<WashingMachine />` |
| [waves-ladder](https://lucide.dev/icons/waves-ladder) | `waves-ladder` | `<WavesLadder />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "home"] or ["leptos", "all-icons"]
<AirVent class="w-6 h-6 text-gray-600" />
```

### Layout

`layout` and `all-icons` features - 138 icons

| Icon | Name | Component |
|------|------|-----------|
| [align-center-horizontal](https://lucide.dev/icons/align-center-horizontal) | `align-center-horizontal` | `<AlignCenterHorizontal />` |
| [align-center-vertical](https://lucide.dev/icons/align-center-vertical) | `align-center-vertical` | `<AlignCenterVertical />` |
| [align-end-horizontal](https://lucide.dev/icons/align-end-horizontal) | `align-end-horizontal` | `<AlignEndHorizontal />` |
| [align-end-vertical](https://lucide.dev/icons/align-end-vertical) | `align-end-vertical` | `<AlignEndVertical />` |
| [align-horizontal-distribute-center](https://lucide.dev/icons/align-horizontal-distribute-center) | `align-horizontal-distribute-center` | `<AlignHorizontalDistributeCenter />` |
| [align-horizontal-distribute-end](https://lucide.dev/icons/align-horizontal-distribute-end) | `align-horizontal-distribute-end` | `<AlignHorizontalDistributeEnd />` |
| [align-horizontal-distribute-start](https://lucide.dev/icons/align-horizontal-distribute-start) | `align-horizontal-distribute-start` | `<AlignHorizontalDistributeStart />` |
| [align-horizontal-justify-center](https://lucide.dev/icons/align-horizontal-justify-center) | `align-horizontal-justify-center` | `<AlignHorizontalJustifyCenter />` |
| [align-horizontal-justify-end](https://lucide.dev/icons/align-horizontal-justify-end) | `align-horizontal-justify-end` | `<AlignHorizontalJustifyEnd />` |
| [align-horizontal-justify-start](https://lucide.dev/icons/align-horizontal-justify-start) | `align-horizontal-justify-start` | `<AlignHorizontalJustifyStart />` |
| [align-horizontal-space-around](https://lucide.dev/icons/align-horizontal-space-around) | `align-horizontal-space-around` | `<AlignHorizontalSpaceAround />` |
| [align-horizontal-space-between](https://lucide.dev/icons/align-horizontal-space-between) | `align-horizontal-space-between` | `<AlignHorizontalSpaceBetween />` |
| [align-start-horizontal](https://lucide.dev/icons/align-start-horizontal) | `align-start-horizontal` | `<AlignStartHorizontal />` |
| [align-start-vertical](https://lucide.dev/icons/align-start-vertical) | `align-start-vertical` | `<AlignStartVertical />` |
| [align-vertical-distribute-center](https://lucide.dev/icons/align-vertical-distribute-center) | `align-vertical-distribute-center` | `<AlignVerticalDistributeCenter />` |
| [align-vertical-distribute-end](https://lucide.dev/icons/align-vertical-distribute-end) | `align-vertical-distribute-end` | `<AlignVerticalDistributeEnd />` |
| [align-vertical-distribute-start](https://lucide.dev/icons/align-vertical-distribute-start) | `align-vertical-distribute-start` | `<AlignVerticalDistributeStart />` |
| [align-vertical-justify-center](https://lucide.dev/icons/align-vertical-justify-center) | `align-vertical-justify-center` | `<AlignVerticalJustifyCenter />` |
| [align-vertical-justify-end](https://lucide.dev/icons/align-vertical-justify-end) | `align-vertical-justify-end` | `<AlignVerticalJustifyEnd />` |
| [align-vertical-justify-start](https://lucide.dev/icons/align-vertical-justify-start) | `align-vertical-justify-start` | `<AlignVerticalJustifyStart />` |
| [align-vertical-space-around](https://lucide.dev/icons/align-vertical-space-around) | `align-vertical-space-around` | `<AlignVerticalSpaceAround />` |
| [align-vertical-space-between](https://lucide.dev/icons/align-vertical-space-between) | `align-vertical-space-between` | `<AlignVerticalSpaceBetween />` |
| [app-window](https://lucide.dev/icons/app-window) | `app-window` | `<AppWindow />` |
| [app-window-mac](https://lucide.dev/icons/app-window-mac) | `app-window-mac` | `<AppWindowMac />` |
| [arrow-down-0-1](https://lucide.dev/icons/arrow-down-0-1) | `arrow-down-0-1` | `<ArrowDown01 />` |
| [arrow-down-1-0](https://lucide.dev/icons/arrow-down-1-0) | `arrow-down-1-0` | `<ArrowDown10 />` |
| [arrow-down-a-z](https://lucide.dev/icons/arrow-down-a-z) | `arrow-down-a-z` | `<ArrowDownAZ />` |
| [arrow-down-narrow-wide](https://lucide.dev/icons/arrow-down-narrow-wide) | `arrow-down-narrow-wide` | `<ArrowDownNarrowWide />` |
| [arrow-down-wide-narrow](https://lucide.dev/icons/arrow-down-wide-narrow) | `arrow-down-wide-narrow` | `<ArrowDownWideNarrow />` |
| [arrow-down-z-a](https://lucide.dev/icons/arrow-down-z-a) | `arrow-down-z-a` | `<ArrowDownZA />` |
| [arrow-up-0-1](https://lucide.dev/icons/arrow-up-0-1) | `arrow-up-0-1` | `<ArrowUp01 />` |
| [arrow-up-1-0](https://lucide.dev/icons/arrow-up-1-0) | `arrow-up-1-0` | `<ArrowUp10 />` |
| [arrow-up-a-z](https://lucide.dev/icons/arrow-up-a-z) | `arrow-up-a-z` | `<ArrowUpAZ />` |
| [arrow-up-narrow-wide](https://lucide.dev/icons/arrow-up-narrow-wide) | `arrow-up-narrow-wide` | `<ArrowUpNarrowWide />` |
| [arrow-up-wide-narrow](https://lucide.dev/icons/arrow-up-wide-narrow) | `arrow-up-wide-narrow` | `<ArrowUpWideNarrow />` |
| [arrow-up-z-a](https://lucide.dev/icons/arrow-up-z-a) | `arrow-up-z-a` | `<ArrowUpZA />` |
| [between-horizontal-end](https://lucide.dev/icons/between-horizontal-end) | `between-horizontal-end` | `<BetweenHorizontalEnd />` |
| [between-horizontal-start](https://lucide.dev/icons/between-horizontal-start) | `between-horizontal-start` | `<BetweenHorizontalStart />` |
| [between-vertical-end](https://lucide.dev/icons/between-vertical-end) | `between-vertical-end` | `<BetweenVerticalEnd />` |
| [between-vertical-start](https://lucide.dev/icons/between-vertical-start) | `between-vertical-start` | `<BetweenVerticalStart />` |
| [blocks](https://lucide.dev/icons/blocks) | `blocks` | `<Blocks />` |
| [bring-to-front](https://lucide.dev/icons/bring-to-front) | `bring-to-front` | `<BringToFront />` |
| [circle-ellipsis](https://lucide.dev/icons/circle-ellipsis) | `circle-ellipsis` | `<CircleEllipsis />` |
| [columns-2](https://lucide.dev/icons/columns-2) | `columns-2` | `<Columns2 />` |
| [columns-3](https://lucide.dev/icons/columns-3) | `columns-3` | `<Columns3 />` |
| [columns-3-cog](https://lucide.dev/icons/columns-3-cog) | `columns-3-cog` | `<Columns3Cog />` |
| [columns-4](https://lucide.dev/icons/columns-4) | `columns-4` | `<Columns4 />` |
| [dock](https://lucide.dev/icons/dock) | `dock` | `<Dock />` |
| [ellipsis](https://lucide.dev/icons/ellipsis) | `ellipsis` | `<Ellipsis />` |
| [ellipsis-vertical](https://lucide.dev/icons/ellipsis-vertical) | `ellipsis-vertical` | `<EllipsisVertical />` |
| [fold-horizontal](https://lucide.dev/icons/fold-horizontal) | `fold-horizontal` | `<FoldHorizontal />` |
| [fold-vertical](https://lucide.dev/icons/fold-vertical) | `fold-vertical` | `<FoldVertical />` |
| [fullscreen](https://lucide.dev/icons/fullscreen) | `fullscreen` | `<Fullscreen />` |
| [funnel](https://lucide.dev/icons/funnel) | `funnel` | `<Funnel />` |
| [funnel-plus](https://lucide.dev/icons/funnel-plus) | `funnel-plus` | `<FunnelPlus />` |
| [funnel-x](https://lucide.dev/icons/funnel-x) | `funnel-x` | `<FunnelX />` |
| [gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) | `gallery-horizontal` | `<GalleryHorizontal />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) | `gallery-thumbnails` | `<GalleryThumbnails />` |
| [gallery-vertical](https://lucide.dev/icons/gallery-vertical) | `gallery-vertical` | `<GalleryVertical />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [grid-2x2](https://lucide.dev/icons/grid-2x2) | `grid-2x2` | `<Grid2X2 />` |
| [grid-2x2-check](https://lucide.dev/icons/grid-2x2-check) | `grid-2x2-check` | `<Grid2X2Check />` |
| [grid-2x2-plus](https://lucide.dev/icons/grid-2x2-plus) | `grid-2x2-plus` | `<Grid2X2Plus />` |
| [grid-2x2-x](https://lucide.dev/icons/grid-2x2-x) | `grid-2x2-x` | `<Grid2X2X />` |
| [grid-3x2](https://lucide.dev/icons/grid-3x2) | `grid-3x2` | `<Grid3X2 />` |
| [grid-3x3](https://lucide.dev/icons/grid-3x3) | `grid-3x3` | `<Grid3X3 />` |
| [grip](https://lucide.dev/icons/grip) | `grip` | `<Grip />` |
| [grip-horizontal](https://lucide.dev/icons/grip-horizontal) | `grip-horizontal` | `<GripHorizontal />` |
| [grip-vertical](https://lucide.dev/icons/grip-vertical) | `grip-vertical` | `<GripVertical />` |
| [hand-grab](https://lucide.dev/icons/hand-grab) | `hand-grab` | `<HandGrab />` |
| [layers](https://lucide.dev/icons/layers) | `layers` | `<Layers />` |
| [layers-2](https://lucide.dev/icons/layers-2) | `layers-2` | `<Layers2 />` |
| [layout-dashboard](https://lucide.dev/icons/layout-dashboard) | `layout-dashboard` | `<LayoutDashboard />` |
| [layout-grid](https://lucide.dev/icons/layout-grid) | `layout-grid` | `<LayoutGrid />` |
| [layout-list](https://lucide.dev/icons/layout-list) | `layout-list` | `<LayoutList />` |
| [layout-panel-left](https://lucide.dev/icons/layout-panel-left) | `layout-panel-left` | `<LayoutPanelLeft />` |
| [layout-panel-top](https://lucide.dev/icons/layout-panel-top) | `layout-panel-top` | `<LayoutPanelTop />` |
| [layout-template](https://lucide.dev/icons/layout-template) | `layout-template` | `<LayoutTemplate />` |
| [list-filter-plus](https://lucide.dev/icons/list-filter-plus) | `list-filter-plus` | `<ListFilterPlus />` |
| [list-tree](https://lucide.dev/icons/list-tree) | `list-tree` | `<ListTree />` |
| [loader](https://lucide.dev/icons/loader) | `loader` | `<Loader />` |
| [loader-circle](https://lucide.dev/icons/loader-circle) | `loader-circle` | `<LoaderCircle />` |
| [maximize](https://lucide.dev/icons/maximize) | `maximize` | `<Maximize />` |
| [maximize-2](https://lucide.dev/icons/maximize-2) | `maximize-2` | `<Maximize2 />` |
| [menu](https://lucide.dev/icons/menu) | `menu` | `<Menu />` |
| [minimize](https://lucide.dev/icons/minimize) | `minimize` | `<Minimize />` |
| [minimize-2](https://lucide.dev/icons/minimize-2) | `minimize-2` | `<Minimize2 />` |
| [panel-bottom](https://lucide.dev/icons/panel-bottom) | `panel-bottom` | `<PanelBottom />` |
| [panel-bottom-close](https://lucide.dev/icons/panel-bottom-close) | `panel-bottom-close` | `<PanelBottomClose />` |
| [panel-bottom-dashed](https://lucide.dev/icons/panel-bottom-dashed) | `panel-bottom-dashed` | `<PanelBottomDashed />` |
| [panel-bottom-open](https://lucide.dev/icons/panel-bottom-open) | `panel-bottom-open` | `<PanelBottomOpen />` |
| [panel-left](https://lucide.dev/icons/panel-left) | `panel-left` | `<PanelLeft />` |
| [panel-left-close](https://lucide.dev/icons/panel-left-close) | `panel-left-close` | `<PanelLeftClose />` |
| [panel-left-dashed](https://lucide.dev/icons/panel-left-dashed) | `panel-left-dashed` | `<PanelLeftDashed />` |
| [panel-left-open](https://lucide.dev/icons/panel-left-open) | `panel-left-open` | `<PanelLeftOpen />` |
| [panel-left-right-dashed](https://lucide.dev/icons/panel-left-right-dashed) | `panel-left-right-dashed` | `<PanelLeftRightDashed />` |
| [panel-right](https://lucide.dev/icons/panel-right) | `panel-right` | `<PanelRight />` |
| [panel-right-close](https://lucide.dev/icons/panel-right-close) | `panel-right-close` | `<PanelRightClose />` |
| [panel-right-dashed](https://lucide.dev/icons/panel-right-dashed) | `panel-right-dashed` | `<PanelRightDashed />` |
| [panel-right-open](https://lucide.dev/icons/panel-right-open) | `panel-right-open` | `<PanelRightOpen />` |
| [panel-top](https://lucide.dev/icons/panel-top) | `panel-top` | `<PanelTop />` |
| [panel-top-bottom-dashed](https://lucide.dev/icons/panel-top-bottom-dashed) | `panel-top-bottom-dashed` | `<PanelTopBottomDashed />` |
| [panel-top-close](https://lucide.dev/icons/panel-top-close) | `panel-top-close` | `<PanelTopClose />` |
| [panel-top-dashed](https://lucide.dev/icons/panel-top-dashed) | `panel-top-dashed` | `<PanelTopDashed />` |
| [panel-top-open](https://lucide.dev/icons/panel-top-open) | `panel-top-open` | `<PanelTopOpen />` |
| [panels-left-bottom](https://lucide.dev/icons/panels-left-bottom) | `panels-left-bottom` | `<PanelsLeftBottom />` |
| [panels-right-bottom](https://lucide.dev/icons/panels-right-bottom) | `panels-right-bottom` | `<PanelsRightBottom />` |
| [panels-top-left](https://lucide.dev/icons/panels-top-left) | `panels-top-left` | `<PanelsTopLeft />` |
| [pencil-ruler](https://lucide.dev/icons/pencil-ruler) | `pencil-ruler` | `<PencilRuler />` |
| [proportions](https://lucide.dev/icons/proportions) | `proportions` | `<Proportions />` |
| [ratio](https://lucide.dev/icons/ratio) | `ratio` | `<Ratio />` |
| [rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) | `rotate-ccw-square` | `<RotateCcwSquare />` |
| [rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) | `rotate-cw-square` | `<RotateCwSquare />` |
| [rows-2](https://lucide.dev/icons/rows-2) | `rows-2` | `<Rows2 />` |
| [rows-3](https://lucide.dev/icons/rows-3) | `rows-3` | `<Rows3 />` |
| [rows-4](https://lucide.dev/icons/rows-4) | `rows-4` | `<Rows4 />` |
| [ruler](https://lucide.dev/icons/ruler) | `ruler` | `<Ruler />` |
| [ruler-dimension-line](https://lucide.dev/icons/ruler-dimension-line) | `ruler-dimension-line` | `<RulerDimensionLine />` |
| [send-to-back](https://lucide.dev/icons/send-to-back) | `send-to-back` | `<SendToBack />` |
| [separator-horizontal](https://lucide.dev/icons/separator-horizontal) | `separator-horizontal` | `<SeparatorHorizontal />` |
| [separator-vertical](https://lucide.dev/icons/separator-vertical) | `separator-vertical` | `<SeparatorVertical />` |
| [shrink](https://lucide.dev/icons/shrink) | `shrink` | `<Shrink />` |
| [square-dashed-top-solid](https://lucide.dev/icons/square-dashed-top-solid) | `square-dashed-top-solid` | `<SquareDashedTopSolid />` |
| [square-menu](https://lucide.dev/icons/square-menu) | `square-menu` | `<SquareMenu />` |
| [square-round-corner](https://lucide.dev/icons/square-round-corner) | `square-round-corner` | `<SquareRoundCorner />` |
| [square-split-horizontal](https://lucide.dev/icons/square-split-horizontal) | `square-split-horizontal` | `<SquareSplitHorizontal />` |
| [square-split-vertical](https://lucide.dev/icons/square-split-vertical) | `square-split-vertical` | `<SquareSplitVertical />` |
| [square-square](https://lucide.dev/icons/square-square) | `square-square` | `<SquareSquare />` |
| [stretch-horizontal](https://lucide.dev/icons/stretch-horizontal) | `stretch-horizontal` | `<StretchHorizontal />` |
| [stretch-vertical](https://lucide.dev/icons/stretch-vertical) | `stretch-vertical` | `<StretchVertical />` |
| [text-cursor-input](https://lucide.dev/icons/text-cursor-input) | `text-cursor-input` | `<TextCursorInput />` |
| [toggle-left](https://lucide.dev/icons/toggle-left) | `toggle-left` | `<ToggleLeft />` |
| [toggle-right](https://lucide.dev/icons/toggle-right) | `toggle-right` | `<ToggleRight />` |
| [unfold-horizontal](https://lucide.dev/icons/unfold-horizontal) | `unfold-horizontal` | `<UnfoldHorizontal />` |
| [unfold-vertical](https://lucide.dev/icons/unfold-vertical) | `unfold-vertical` | `<UnfoldVertical />` |
| [zoom-in](https://lucide.dev/icons/zoom-in) | `zoom-in` | `<ZoomIn />` |
| [zoom-out](https://lucide.dev/icons/zoom-out) | `zoom-out` | `<ZoomOut />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "layout"] or ["leptos", "all-icons"]
<AlignCenterHorizontal class="w-6 h-6 text-gray-600" />
```

### Mail

`mail` and `all-icons` features - 26 icons

| Icon | Name | Component |
|------|------|-----------|
| [archive](https://lucide.dev/icons/archive) | `archive` | `<Archive />` |
| [archive-restore](https://lucide.dev/icons/archive-restore) | `archive-restore` | `<ArchiveRestore />` |
| [archive-x](https://lucide.dev/icons/archive-x) | `archive-x` | `<ArchiveX />` |
| [arrows-up-from-line](https://lucide.dev/icons/arrows-up-from-line) | `arrows-up-from-line` | `<ArrowsUpFromLine />` |
| [container](https://lucide.dev/icons/container) | `container` | `<Container />` |
| [forward](https://lucide.dev/icons/forward) | `forward` | `<Forward />` |
| [inbox](https://lucide.dev/icons/inbox) | `inbox` | `<Inbox />` |
| [mail](https://lucide.dev/icons/mail) | `mail` | `<Mail />` |
| [mail-check](https://lucide.dev/icons/mail-check) | `mail-check` | `<MailCheck />` |
| [mail-minus](https://lucide.dev/icons/mail-minus) | `mail-minus` | `<MailMinus />` |
| [mail-open](https://lucide.dev/icons/mail-open) | `mail-open` | `<MailOpen />` |
| [mail-plus](https://lucide.dev/icons/mail-plus) | `mail-plus` | `<MailPlus />` |
| [mail-question-mark](https://lucide.dev/icons/mail-question-mark) | `mail-question-mark` | `<MailQuestionMark />` |
| [mail-search](https://lucide.dev/icons/mail-search) | `mail-search` | `<MailSearch />` |
| [mail-warning](https://lucide.dev/icons/mail-warning) | `mail-warning` | `<MailWarning />` |
| [mail-x](https://lucide.dev/icons/mail-x) | `mail-x` | `<MailX />` |
| [mailbox](https://lucide.dev/icons/mailbox) | `mailbox` | `<Mailbox />` |
| [mails](https://lucide.dev/icons/mails) | `mails` | `<Mails />` |
| [paperclip](https://lucide.dev/icons/paperclip) | `paperclip` | `<Paperclip />` |
| [reply](https://lucide.dev/icons/reply) | `reply` | `<Reply />` |
| [reply-all](https://lucide.dev/icons/reply-all) | `reply-all` | `<ReplyAll />` |
| [send](https://lucide.dev/icons/send) | `send` | `<Send />` |
| [send-horizontal](https://lucide.dev/icons/send-horizontal) | `send-horizontal` | `<SendHorizontal />` |
| [shredder](https://lucide.dev/icons/shredder) | `shredder` | `<Shredder />` |
| [trash](https://lucide.dev/icons/trash) | `trash` | `<Trash />` |
| [trash-2](https://lucide.dev/icons/trash-2) | `trash-2` | `<Trash2 />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "mail"] or ["leptos", "all-icons"]
<Archive class="w-6 h-6 text-gray-600" />
```

### Math

`math` and `all-icons` features - 73 icons

| Icon | Name | Component |
|------|------|-----------|
| [asterisk](https://lucide.dev/icons/asterisk) | `asterisk` | `<Asterisk />` |
| [badge-percent](https://lucide.dev/icons/badge-percent) | `badge-percent` | `<BadgePercent />` |
| [box](https://lucide.dev/icons/box) | `box` | `<Box />` |
| [calculator](https://lucide.dev/icons/calculator) | `calculator` | `<Calculator />` |
| [chevron-right](https://lucide.dev/icons/chevron-right) | `chevron-right` | `<ChevronRight />` |
| [chevron-up](https://lucide.dev/icons/chevron-up) | `chevron-up` | `<ChevronUp />` |
| [circle-divide](https://lucide.dev/icons/circle-divide) | `circle-divide` | `<CircleDivide />` |
| [circle-equal](https://lucide.dev/icons/circle-equal) | `circle-equal` | `<CircleEqual />` |
| [circle-minus](https://lucide.dev/icons/circle-minus) | `circle-minus` | `<CircleMinus />` |
| [circle-percent](https://lucide.dev/icons/circle-percent) | `circle-percent` | `<CirclePercent />` |
| [circle-plus](https://lucide.dev/icons/circle-plus) | `circle-plus` | `<CirclePlus />` |
| [circle-slash](https://lucide.dev/icons/circle-slash) | `circle-slash` | `<CircleSlash />` |
| [circle-slash-2](https://lucide.dev/icons/circle-slash-2) | `circle-slash-2` | `<CircleSlash2 />` |
| [circle-x](https://lucide.dev/icons/circle-x) | `circle-x` | `<CircleX />` |
| [cone](https://lucide.dev/icons/cone) | `cone` | `<Cone />` |
| [copy-minus](https://lucide.dev/icons/copy-minus) | `copy-minus` | `<CopyMinus />` |
| [copy-plus](https://lucide.dev/icons/copy-plus) | `copy-plus` | `<CopyPlus />` |
| [copy-slash](https://lucide.dev/icons/copy-slash) | `copy-slash` | `<CopySlash />` |
| [copy-x](https://lucide.dev/icons/copy-x) | `copy-x` | `<CopyX />` |
| [cuboid](https://lucide.dev/icons/cuboid) | `cuboid` | `<Cuboid />` |
| [cylinder](https://lucide.dev/icons/cylinder) | `cylinder` | `<Cylinder />` |
| [decimals-arrow-left](https://lucide.dev/icons/decimals-arrow-left) | `decimals-arrow-left` | `<DecimalsArrowLeft />` |
| [decimals-arrow-right](https://lucide.dev/icons/decimals-arrow-right) | `decimals-arrow-right` | `<DecimalsArrowRight />` |
| [diameter](https://lucide.dev/icons/diameter) | `diameter` | `<Diameter />` |
| [diamond-percent](https://lucide.dev/icons/diamond-percent) | `diamond-percent` | `<DiamondPercent />` |
| [divide](https://lucide.dev/icons/divide) | `divide` | `<Divide />` |
| [drafting-compass](https://lucide.dev/icons/drafting-compass) | `drafting-compass` | `<DraftingCompass />` |
| [equal](https://lucide.dev/icons/equal) | `equal` | `<Equal />` |
| [equal-approximately](https://lucide.dev/icons/equal-approximately) | `equal-approximately` | `<EqualApproximately />` |
| [equal-not](https://lucide.dev/icons/equal-not) | `equal-not` | `<EqualNot />` |
| [grid-2x2](https://lucide.dev/icons/grid-2x2) | `grid-2x2` | `<Grid2X2 />` |
| [grid-2x2-check](https://lucide.dev/icons/grid-2x2-check) | `grid-2x2-check` | `<Grid2X2Check />` |
| [grid-2x2-plus](https://lucide.dev/icons/grid-2x2-plus) | `grid-2x2-plus` | `<Grid2X2Plus />` |
| [grid-2x2-x](https://lucide.dev/icons/grid-2x2-x) | `grid-2x2-x` | `<Grid2X2X />` |
| [grid-3x2](https://lucide.dev/icons/grid-3x2) | `grid-3x2` | `<Grid3X2 />` |
| [land-plot](https://lucide.dev/icons/land-plot) | `land-plot` | `<LandPlot />` |
| [line-squiggle](https://lucide.dev/icons/line-squiggle) | `line-squiggle` | `<LineSquiggle />` |
| [minus](https://lucide.dev/icons/minus) | `minus` | `<Minus />` |
| [octagon-x](https://lucide.dev/icons/octagon-x) | `octagon-x` | `<OctagonX />` |
| [omega](https://lucide.dev/icons/omega) | `omega` | `<Omega />` |
| [parentheses](https://lucide.dev/icons/parentheses) | `parentheses` | `<Parentheses />` |
| [percent](https://lucide.dev/icons/percent) | `percent` | `<Percent />` |
| [pi](https://lucide.dev/icons/pi) | `pi` | `<Pi />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [pyramid](https://lucide.dev/icons/pyramid) | `pyramid` | `<Pyramid />` |
| [radical](https://lucide.dev/icons/radical) | `radical` | `<Radical />` |
| [radius](https://lucide.dev/icons/radius) | `radius` | `<Radius />` |
| [sigma](https://lucide.dev/icons/sigma) | `sigma` | `<Sigma />` |
| [slash](https://lucide.dev/icons/slash) | `slash` | `<Slash />` |
| [square-asterisk](https://lucide.dev/icons/square-asterisk) | `square-asterisk` | `<SquareAsterisk />` |
| [square-chevron-up](https://lucide.dev/icons/square-chevron-up) | `square-chevron-up` | `<SquareChevronUp />` |
| [square-divide](https://lucide.dev/icons/square-divide) | `square-divide` | `<SquareDivide />` |
| [square-equal](https://lucide.dev/icons/square-equal) | `square-equal` | `<SquareEqual />` |
| [square-function](https://lucide.dev/icons/square-function) | `square-function` | `<SquareFunction />` |
| [square-minus](https://lucide.dev/icons/square-minus) | `square-minus` | `<SquareMinus />` |
| [square-percent](https://lucide.dev/icons/square-percent) | `square-percent` | `<SquarePercent />` |
| [square-pi](https://lucide.dev/icons/square-pi) | `square-pi` | `<SquarePi />` |
| [square-plus](https://lucide.dev/icons/square-plus) | `square-plus` | `<SquarePlus />` |
| [square-radical](https://lucide.dev/icons/square-radical) | `square-radical` | `<SquareRadical />` |
| [square-sigma](https://lucide.dev/icons/square-sigma) | `square-sigma` | `<SquareSigma />` |
| [square-slash](https://lucide.dev/icons/square-slash) | `square-slash` | `<SquareSlash />` |
| [square-x](https://lucide.dev/icons/square-x) | `square-x` | `<SquareX />` |
| [tally-1](https://lucide.dev/icons/tally-1) | `tally-1` | `<Tally1 />` |
| [tally-2](https://lucide.dev/icons/tally-2) | `tally-2` | `<Tally2 />` |
| [tally-3](https://lucide.dev/icons/tally-3) | `tally-3` | `<Tally3 />` |
| [tally-4](https://lucide.dev/icons/tally-4) | `tally-4` | `<Tally4 />` |
| [tally-5](https://lucide.dev/icons/tally-5) | `tally-5` | `<Tally5 />` |
| [tangent](https://lucide.dev/icons/tangent) | `tangent` | `<Tangent />` |
| [triangle-right](https://lucide.dev/icons/triangle-right) | `triangle-right` | `<TriangleRight />` |
| [variable](https://lucide.dev/icons/variable) | `variable` | `<Variable />` |
| [vector-square](https://lucide.dev/icons/vector-square) | `vector-square` | `<VectorSquare />` |
| [weight](https://lucide.dev/icons/weight) | `weight` | `<Weight />` |
| [x](https://lucide.dev/icons/x) | `x` | `<X />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "math"] or ["leptos", "all-icons"]
<Asterisk class="w-6 h-6 text-gray-600" />
```

### Medical

`medical` and `all-icons` features - 42 icons

| Icon | Name | Component |
|------|------|-----------|
| [accessibility](https://lucide.dev/icons/accessibility) | `accessibility` | `<Accessibility />` |
| [activity](https://lucide.dev/icons/activity) | `activity` | `<Activity />` |
| [ambulance](https://lucide.dev/icons/ambulance) | `ambulance` | `<Ambulance />` |
| [bandage](https://lucide.dev/icons/bandage) | `bandage` | `<Bandage />` |
| [bone](https://lucide.dev/icons/bone) | `bone` | `<Bone />` |
| [brain](https://lucide.dev/icons/brain) | `brain` | `<Brain />` |
| [briefcase-medical](https://lucide.dev/icons/briefcase-medical) | `briefcase-medical` | `<BriefcaseMedical />` |
| [cigarette](https://lucide.dev/icons/cigarette) | `cigarette` | `<Cigarette />` |
| [cigarette-off](https://lucide.dev/icons/cigarette-off) | `cigarette-off` | `<CigaretteOff />` |
| [circle-small](https://lucide.dev/icons/circle-small) | `circle-small` | `<CircleSmall />` |
| [clipboard-minus](https://lucide.dev/icons/clipboard-minus) | `clipboard-minus` | `<ClipboardMinus />` |
| [clipboard-plus](https://lucide.dev/icons/clipboard-plus) | `clipboard-plus` | `<ClipboardPlus />` |
| [dna](https://lucide.dev/icons/dna) | `dna` | `<Dna />` |
| [dna-off](https://lucide.dev/icons/dna-off) | `dna-off` | `<DnaOff />` |
| [ear](https://lucide.dev/icons/ear) | `ear` | `<Ear />` |
| [ear-off](https://lucide.dev/icons/ear-off) | `ear-off` | `<EarOff />` |
| [fingerprint](https://lucide.dev/icons/fingerprint) | `fingerprint` | `<Fingerprint />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [heart-minus](https://lucide.dev/icons/heart-minus) | `heart-minus` | `<HeartMinus />` |
| [heart-plus](https://lucide.dev/icons/heart-plus) | `heart-plus` | `<HeartPlus />` |
| [heart-pulse](https://lucide.dev/icons/heart-pulse) | `heart-pulse` | `<HeartPulse />` |
| [hospital](https://lucide.dev/icons/hospital) | `hospital` | `<Hospital />` |
| [house-heart](https://lucide.dev/icons/house-heart) | `house-heart` | `<HouseHeart />` |
| [house-plus](https://lucide.dev/icons/house-plus) | `house-plus` | `<HousePlus />` |
| [life-buoy](https://lucide.dev/icons/life-buoy) | `life-buoy` | `<LifeBuoy />` |
| [mars](https://lucide.dev/icons/mars) | `mars` | `<Mars />` |
| [mars-stroke](https://lucide.dev/icons/mars-stroke) | `mars-stroke` | `<MarsStroke />` |
| [microscope](https://lucide.dev/icons/microscope) | `microscope` | `<Microscope />` |
| [non-binary](https://lucide.dev/icons/non-binary) | `non-binary` | `<NonBinary />` |
| [pill](https://lucide.dev/icons/pill) | `pill` | `<Pill />` |
| [pill-bottle](https://lucide.dev/icons/pill-bottle) | `pill-bottle` | `<PillBottle />` |
| [ribbon](https://lucide.dev/icons/ribbon) | `ribbon` | `<Ribbon />` |
| [scan-heart](https://lucide.dev/icons/scan-heart) | `scan-heart` | `<ScanHeart />` |
| [shield-plus](https://lucide.dev/icons/shield-plus) | `shield-plus` | `<ShieldPlus />` |
| [siren](https://lucide.dev/icons/siren) | `siren` | `<Siren />` |
| [square-activity](https://lucide.dev/icons/square-activity) | `square-activity` | `<SquareActivity />` |
| [stethoscope](https://lucide.dev/icons/stethoscope) | `stethoscope` | `<Stethoscope />` |
| [syringe](https://lucide.dev/icons/syringe) | `syringe` | `<Syringe />` |
| [tablets](https://lucide.dev/icons/tablets) | `tablets` | `<Tablets />` |
| [transgender](https://lucide.dev/icons/transgender) | `transgender` | `<Transgender />` |
| [venus](https://lucide.dev/icons/venus) | `venus` | `<Venus />` |
| [venus-and-mars](https://lucide.dev/icons/venus-and-mars) | `venus-and-mars` | `<VenusAndMars />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "medical"] or ["leptos", "all-icons"]
<Accessibility class="w-6 h-6 text-gray-600" />
```

### Multimedia

`multimedia` and `all-icons` features - 137 icons

| Icon | Name | Component |
|------|------|-----------|
| [activity](https://lucide.dev/icons/activity) | `activity` | `<Activity />` |
| [airplay](https://lucide.dev/icons/airplay) | `airplay` | `<Airplay />` |
| [album](https://lucide.dev/icons/album) | `album` | `<Album />` |
| [antenna](https://lucide.dev/icons/antenna) | `antenna` | `<Antenna />` |
| [audio-lines](https://lucide.dev/icons/audio-lines) | `audio-lines` | `<AudioLines />` |
| [audio-waveform](https://lucide.dev/icons/audio-waveform) | `audio-waveform` | `<AudioWaveform />` |
| [book-audio](https://lucide.dev/icons/book-audio) | `book-audio` | `<BookAudio />` |
| [book-headphones](https://lucide.dev/icons/book-headphones) | `book-headphones` | `<BookHeadphones />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [boom-box](https://lucide.dev/icons/boom-box) | `boom-box` | `<BoomBox />` |
| [cable](https://lucide.dev/icons/cable) | `cable` | `<Cable />` |
| [captions](https://lucide.dev/icons/captions) | `captions` | `<Captions />` |
| [captions-off](https://lucide.dev/icons/captions-off) | `captions-off` | `<CaptionsOff />` |
| [card-sim](https://lucide.dev/icons/card-sim) | `card-sim` | `<CardSim />` |
| [cassette-tape](https://lucide.dev/icons/cassette-tape) | `cassette-tape` | `<CassetteTape />` |
| [chevron-first](https://lucide.dev/icons/chevron-first) | `chevron-first` | `<ChevronFirst />` |
| [chevron-last](https://lucide.dev/icons/chevron-last) | `chevron-last` | `<ChevronLast />` |
| [chevrons-left-right-ellipsis](https://lucide.dev/icons/chevrons-left-right-ellipsis) | `chevrons-left-right-ellipsis` | `<ChevronsLeftRightEllipsis />` |
| [circle-pause](https://lucide.dev/icons/circle-pause) | `circle-pause` | `<CirclePause />` |
| [circle-play](https://lucide.dev/icons/circle-play) | `circle-play` | `<CirclePlay />` |
| [circle-stop](https://lucide.dev/icons/circle-stop) | `circle-stop` | `<CircleStop />` |
| [clapperboard](https://lucide.dev/icons/clapperboard) | `clapperboard` | `<Clapperboard />` |
| [closed-caption](https://lucide.dev/icons/closed-caption) | `closed-caption` | `<ClosedCaption />` |
| [diamond-minus](https://lucide.dev/icons/diamond-minus) | `diamond-minus` | `<DiamondMinus />` |
| [diamond-plus](https://lucide.dev/icons/diamond-plus) | `diamond-plus` | `<DiamondPlus />` |
| [disc](https://lucide.dev/icons/disc) | `disc` | `<Disc />` |
| [disc-2](https://lucide.dev/icons/disc-2) | `disc-2` | `<Disc2 />` |
| [disc-3](https://lucide.dev/icons/disc-3) | `disc-3` | `<Disc3 />` |
| [disc-album](https://lucide.dev/icons/disc-album) | `disc-album` | `<DiscAlbum />` |
| [drama](https://lucide.dev/icons/drama) | `drama` | `<Drama />` |
| [drum](https://lucide.dev/icons/drum) | `drum` | `<Drum />` |
| [ethernet-port](https://lucide.dev/icons/ethernet-port) | `ethernet-port` | `<EthernetPort />` |
| [fast-forward](https://lucide.dev/icons/fast-forward) | `fast-forward` | `<FastForward />` |
| [file-music](https://lucide.dev/icons/file-music) | `file-music` | `<FileMusic />` |
| [film](https://lucide.dev/icons/film) | `film` | `<Film />` |
| [fullscreen](https://lucide.dev/icons/fullscreen) | `fullscreen` | `<Fullscreen />` |
| [gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) | `gallery-horizontal` | `<GalleryHorizontal />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) | `gallery-thumbnails` | `<GalleryThumbnails />` |
| [gallery-vertical](https://lucide.dev/icons/gallery-vertical) | `gallery-vertical` | `<GalleryVertical />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [guitar](https://lucide.dev/icons/guitar) | `guitar` | `<Guitar />` |
| [hand-metal](https://lucide.dev/icons/hand-metal) | `hand-metal` | `<HandMetal />` |
| [hdmi-port](https://lucide.dev/icons/hdmi-port) | `hdmi-port` | `<HdmiPort />` |
| [headphone-off](https://lucide.dev/icons/headphone-off) | `headphone-off` | `<HeadphoneOff />` |
| [headphones](https://lucide.dev/icons/headphones) | `headphones` | `<Headphones />` |
| [headset](https://lucide.dev/icons/headset) | `headset` | `<Headset />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [heart-minus](https://lucide.dev/icons/heart-minus) | `heart-minus` | `<HeartMinus />` |
| [heart-off](https://lucide.dev/icons/heart-off) | `heart-off` | `<HeartOff />` |
| [heart-plus](https://lucide.dev/icons/heart-plus) | `heart-plus` | `<HeartPlus />` |
| [image](https://lucide.dev/icons/image) | `image` | `<Image />` |
| [image-down](https://lucide.dev/icons/image-down) | `image-down` | `<ImageDown />` |
| [image-minus](https://lucide.dev/icons/image-minus) | `image-minus` | `<ImageMinus />` |
| [image-off](https://lucide.dev/icons/image-off) | `image-off` | `<ImageOff />` |
| [image-play](https://lucide.dev/icons/image-play) | `image-play` | `<ImagePlay />` |
| [image-plus](https://lucide.dev/icons/image-plus) | `image-plus` | `<ImagePlus />` |
| [image-up](https://lucide.dev/icons/image-up) | `image-up` | `<ImageUp />` |
| [image-upscale](https://lucide.dev/icons/image-upscale) | `image-upscale` | `<ImageUpscale />` |
| [images](https://lucide.dev/icons/images) | `images` | `<Images />` |
| [infinity](https://lucide.dev/icons/infinity) | `infinity` | `<Infinity />` |
| [keyboard-music](https://lucide.dev/icons/keyboard-music) | `keyboard-music` | `<KeyboardMusic />` |
| [lectern](https://lucide.dev/icons/lectern) | `lectern` | `<Lectern />` |
| [library](https://lucide.dev/icons/library) | `library` | `<Library />` |
| [library-big](https://lucide.dev/icons/library-big) | `library-big` | `<LibraryBig />` |
| [list-end](https://lucide.dev/icons/list-end) | `list-end` | `<ListEnd />` |
| [list-minus](https://lucide.dev/icons/list-minus) | `list-minus` | `<ListMinus />` |
| [list-music](https://lucide.dev/icons/list-music) | `list-music` | `<ListMusic />` |
| [list-plus](https://lucide.dev/icons/list-plus) | `list-plus` | `<ListPlus />` |
| [list-restart](https://lucide.dev/icons/list-restart) | `list-restart` | `<ListRestart />` |
| [list-start](https://lucide.dev/icons/list-start) | `list-start` | `<ListStart />` |
| [list-video](https://lucide.dev/icons/list-video) | `list-video` | `<ListVideo />` |
| [list-x](https://lucide.dev/icons/list-x) | `list-x` | `<ListX />` |
| [loader](https://lucide.dev/icons/loader) | `loader` | `<Loader />` |
| [loader-circle](https://lucide.dev/icons/loader-circle) | `loader-circle` | `<LoaderCircle />` |
| [megaphone](https://lucide.dev/icons/megaphone) | `megaphone` | `<Megaphone />` |
| [megaphone-off](https://lucide.dev/icons/megaphone-off) | `megaphone-off` | `<MegaphoneOff />` |
| [mic](https://lucide.dev/icons/mic) | `mic` | `<Mic />` |
| [mic-off](https://lucide.dev/icons/mic-off) | `mic-off` | `<MicOff />` |
| [mic-vocal](https://lucide.dev/icons/mic-vocal) | `mic-vocal` | `<MicVocal />` |
| [monitor-pause](https://lucide.dev/icons/monitor-pause) | `monitor-pause` | `<MonitorPause />` |
| [monitor-play](https://lucide.dev/icons/monitor-play) | `monitor-play` | `<MonitorPlay />` |
| [monitor-stop](https://lucide.dev/icons/monitor-stop) | `monitor-stop` | `<MonitorStop />` |
| [music](https://lucide.dev/icons/music) | `music` | `<Music />` |
| [music-2](https://lucide.dev/icons/music-2) | `music-2` | `<Music2 />` |
| [music-3](https://lucide.dev/icons/music-3) | `music-3` | `<Music3 />` |
| [music-4](https://lucide.dev/icons/music-4) | `music-4` | `<Music4 />` |
| [newspaper](https://lucide.dev/icons/newspaper) | `newspaper` | `<Newspaper />` |
| [octagon-pause](https://lucide.dev/icons/octagon-pause) | `octagon-pause` | `<OctagonPause />` |
| [pause](https://lucide.dev/icons/pause) | `pause` | `<Pause />` |
| [piano](https://lucide.dev/icons/piano) | `piano` | `<Piano />` |
| [picture-in-picture](https://lucide.dev/icons/picture-in-picture) | `picture-in-picture` | `<PictureInPicture />` |
| [picture-in-picture-2](https://lucide.dev/icons/picture-in-picture-2) | `picture-in-picture-2` | `<PictureInPicture2 />` |
| [play](https://lucide.dev/icons/play) | `play` | `<Play />` |
| [podcast](https://lucide.dev/icons/podcast) | `podcast` | `<Podcast />` |
| [popcorn](https://lucide.dev/icons/popcorn) | `popcorn` | `<Popcorn />` |
| [presentation](https://lucide.dev/icons/presentation) | `presentation` | `<Presentation />` |
| [projector](https://lucide.dev/icons/projector) | `projector` | `<Projector />` |
| [radio](https://lucide.dev/icons/radio) | `radio` | `<Radio />` |
| [radio-tower](https://lucide.dev/icons/radio-tower) | `radio-tower` | `<RadioTower />` |
| [rectangle-goggles](https://lucide.dev/icons/rectangle-goggles) | `rectangle-goggles` | `<RectangleGoggles />` |
| [repeat](https://lucide.dev/icons/repeat) | `repeat` | `<Repeat />` |
| [repeat-1](https://lucide.dev/icons/repeat-1) | `repeat-1` | `<Repeat1 />` |
| [repeat-2](https://lucide.dev/icons/repeat-2) | `repeat-2` | `<Repeat2 />` |
| [rewind](https://lucide.dev/icons/rewind) | `rewind` | `<Rewind />` |
| [satellite-dish](https://lucide.dev/icons/satellite-dish) | `satellite-dish` | `<SatelliteDish />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-search](https://lucide.dev/icons/scan-search) | `scan-search` | `<ScanSearch />` |
| [shuffle](https://lucide.dev/icons/shuffle) | `shuffle` | `<Shuffle />` |
| [skip-back](https://lucide.dev/icons/skip-back) | `skip-back` | `<SkipBack />` |
| [skip-forward](https://lucide.dev/icons/skip-forward) | `skip-forward` | `<SkipForward />` |
| [sparkles](https://lucide.dev/icons/sparkles) | `sparkles` | `<Sparkles />` |
| [speaker](https://lucide.dev/icons/speaker) | `speaker` | `<Speaker />` |
| [spotlight](https://lucide.dev/icons/spotlight) | `spotlight` | `<Spotlight />` |
| [square](https://lucide.dev/icons/square) | `square` | `<Square />` |
| [square-activity](https://lucide.dev/icons/square-activity) | `square-activity` | `<SquareActivity />` |
| [square-library](https://lucide.dev/icons/square-library) | `square-library` | `<SquareLibrary />` |
| [square-pause](https://lucide.dev/icons/square-pause) | `square-pause` | `<SquarePause />` |
| [square-play](https://lucide.dev/icons/square-play) | `square-play` | `<SquarePlay />` |
| [square-stop](https://lucide.dev/icons/square-stop) | `square-stop` | `<SquareStop />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [star-half](https://lucide.dev/icons/star-half) | `star-half` | `<StarHalf />` |
| [star-off](https://lucide.dev/icons/star-off) | `star-off` | `<StarOff />` |
| [step-back](https://lucide.dev/icons/step-back) | `step-back` | `<StepBack />` |
| [step-forward](https://lucide.dev/icons/step-forward) | `step-forward` | `<StepForward />` |
| [turntable](https://lucide.dev/icons/turntable) | `turntable` | `<Turntable />` |
| [tv](https://lucide.dev/icons/tv) | `tv` | `<Tv />` |
| [tv-minimal](https://lucide.dev/icons/tv-minimal) | `tv-minimal` | `<TvMinimal />` |
| [tv-minimal-play](https://lucide.dev/icons/tv-minimal-play) | `tv-minimal-play` | `<TvMinimalPlay />` |
| [usb](https://lucide.dev/icons/usb) | `usb` | `<Usb />` |
| [volume](https://lucide.dev/icons/volume) | `volume` | `<Volume />` |
| [volume-1](https://lucide.dev/icons/volume-1) | `volume-1` | `<Volume1 />` |
| [volume-2](https://lucide.dev/icons/volume-2) | `volume-2` | `<Volume2 />` |
| [volume-off](https://lucide.dev/icons/volume-off) | `volume-off` | `<VolumeOff />` |
| [volume-x](https://lucide.dev/icons/volume-x) | `volume-x` | `<VolumeX />` |
| [waves](https://lucide.dev/icons/waves) | `waves` | `<Waves />` |
| [youtube](https://lucide.dev/icons/youtube) | `youtube` | `<Youtube />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "multimedia"] or ["leptos", "all-icons"]
<Activity class="w-6 h-6 text-gray-600" />
```

### Nature

`nature` and `all-icons` features - 20 icons

| Icon | Name | Component |
|------|------|-----------|
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [cannabis](https://lucide.dev/icons/cannabis) | `cannabis` | `<Cannabis />` |
| [caravan](https://lucide.dev/icons/caravan) | `caravan` | `<Caravan />` |
| [flame-kindling](https://lucide.dev/icons/flame-kindling) | `flame-kindling` | `<FlameKindling />` |
| [flower](https://lucide.dev/icons/flower) | `flower` | `<Flower />` |
| [flower-2](https://lucide.dev/icons/flower-2) | `flower-2` | `<Flower2 />` |
| [leaf](https://lucide.dev/icons/leaf) | `leaf` | `<Leaf />` |
| [mountain](https://lucide.dev/icons/mountain) | `mountain` | `<Mountain />` |
| [mountain-snow](https://lucide.dev/icons/mountain-snow) | `mountain-snow` | `<MountainSnow />` |
| [rose](https://lucide.dev/icons/rose) | `rose` | `<Rose />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [shovel](https://lucide.dev/icons/shovel) | `shovel` | `<Shovel />` |
| [shrub](https://lucide.dev/icons/shrub) | `shrub` | `<Shrub />` |
| [sprout](https://lucide.dev/icons/sprout) | `sprout` | `<Sprout />` |
| [tent](https://lucide.dev/icons/tent) | `tent` | `<Tent />` |
| [tent-tree](https://lucide.dev/icons/tent-tree) | `tent-tree` | `<TentTree />` |
| [tree-deciduous](https://lucide.dev/icons/tree-deciduous) | `tree-deciduous` | `<TreeDeciduous />` |
| [tree-palm](https://lucide.dev/icons/tree-palm) | `tree-palm` | `<TreePalm />` |
| [tree-pine](https://lucide.dev/icons/tree-pine) | `tree-pine` | `<TreePine />` |
| [trees](https://lucide.dev/icons/trees) | `trees` | `<Trees />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "nature"] or ["leptos", "all-icons"]
<Binoculars class="w-6 h-6 text-gray-600" />
```

### Navigation

`navigation` and `all-icons` features - 140 icons

| Icon | Name | Component |
|------|------|-----------|
| [arrow-big-down](https://lucide.dev/icons/arrow-big-down) | `arrow-big-down` | `<ArrowBigDown />` |
| [arrow-big-down-dash](https://lucide.dev/icons/arrow-big-down-dash) | `arrow-big-down-dash` | `<ArrowBigDownDash />` |
| [arrow-big-left](https://lucide.dev/icons/arrow-big-left) | `arrow-big-left` | `<ArrowBigLeft />` |
| [arrow-big-left-dash](https://lucide.dev/icons/arrow-big-left-dash) | `arrow-big-left-dash` | `<ArrowBigLeftDash />` |
| [arrow-big-right](https://lucide.dev/icons/arrow-big-right) | `arrow-big-right` | `<ArrowBigRight />` |
| [arrow-big-right-dash](https://lucide.dev/icons/arrow-big-right-dash) | `arrow-big-right-dash` | `<ArrowBigRightDash />` |
| [arrow-big-up](https://lucide.dev/icons/arrow-big-up) | `arrow-big-up` | `<ArrowBigUp />` |
| [arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) | `arrow-big-up-dash` | `<ArrowBigUpDash />` |
| [arrow-down](https://lucide.dev/icons/arrow-down) | `arrow-down` | `<ArrowDown />` |
| [arrow-down-from-line](https://lucide.dev/icons/arrow-down-from-line) | `arrow-down-from-line` | `<ArrowDownFromLine />` |
| [arrow-down-left](https://lucide.dev/icons/arrow-down-left) | `arrow-down-left` | `<ArrowDownLeft />` |
| [arrow-down-right](https://lucide.dev/icons/arrow-down-right) | `arrow-down-right` | `<ArrowDownRight />` |
| [arrow-down-to-dot](https://lucide.dev/icons/arrow-down-to-dot) | `arrow-down-to-dot` | `<ArrowDownToDot />` |
| [arrow-down-to-line](https://lucide.dev/icons/arrow-down-to-line) | `arrow-down-to-line` | `<ArrowDownToLine />` |
| [arrow-down-up](https://lucide.dev/icons/arrow-down-up) | `arrow-down-up` | `<ArrowDownUp />` |
| [arrow-left](https://lucide.dev/icons/arrow-left) | `arrow-left` | `<ArrowLeft />` |
| [arrow-left-from-line](https://lucide.dev/icons/arrow-left-from-line) | `arrow-left-from-line` | `<ArrowLeftFromLine />` |
| [arrow-left-right](https://lucide.dev/icons/arrow-left-right) | `arrow-left-right` | `<ArrowLeftRight />` |
| [arrow-left-to-line](https://lucide.dev/icons/arrow-left-to-line) | `arrow-left-to-line` | `<ArrowLeftToLine />` |
| [arrow-right](https://lucide.dev/icons/arrow-right) | `arrow-right` | `<ArrowRight />` |
| [arrow-right-from-line](https://lucide.dev/icons/arrow-right-from-line) | `arrow-right-from-line` | `<ArrowRightFromLine />` |
| [arrow-right-left](https://lucide.dev/icons/arrow-right-left) | `arrow-right-left` | `<ArrowRightLeft />` |
| [arrow-right-to-line](https://lucide.dev/icons/arrow-right-to-line) | `arrow-right-to-line` | `<ArrowRightToLine />` |
| [arrow-up](https://lucide.dev/icons/arrow-up) | `arrow-up` | `<ArrowUp />` |
| [arrow-up-down](https://lucide.dev/icons/arrow-up-down) | `arrow-up-down` | `<ArrowUpDown />` |
| [arrow-up-from-dot](https://lucide.dev/icons/arrow-up-from-dot) | `arrow-up-from-dot` | `<ArrowUpFromDot />` |
| [arrow-up-from-line](https://lucide.dev/icons/arrow-up-from-line) | `arrow-up-from-line` | `<ArrowUpFromLine />` |
| [arrow-up-left](https://lucide.dev/icons/arrow-up-left) | `arrow-up-left` | `<ArrowUpLeft />` |
| [arrow-up-right](https://lucide.dev/icons/arrow-up-right) | `arrow-up-right` | `<ArrowUpRight />` |
| [arrow-up-to-line](https://lucide.dev/icons/arrow-up-to-line) | `arrow-up-to-line` | `<ArrowUpToLine />` |
| [barrel](https://lucide.dev/icons/barrel) | `barrel` | `<Barrel />` |
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [chevron-down](https://lucide.dev/icons/chevron-down) | `chevron-down` | `<ChevronDown />` |
| [chevron-left](https://lucide.dev/icons/chevron-left) | `chevron-left` | `<ChevronLeft />` |
| [chevron-right](https://lucide.dev/icons/chevron-right) | `chevron-right` | `<ChevronRight />` |
| [chevron-up](https://lucide.dev/icons/chevron-up) | `chevron-up` | `<ChevronUp />` |
| [chevrons-down](https://lucide.dev/icons/chevrons-down) | `chevrons-down` | `<ChevronsDown />` |
| [chevrons-left](https://lucide.dev/icons/chevrons-left) | `chevrons-left` | `<ChevronsLeft />` |
| [chevrons-right](https://lucide.dev/icons/chevrons-right) | `chevrons-right` | `<ChevronsRight />` |
| [chevrons-up](https://lucide.dev/icons/chevrons-up) | `chevrons-up` | `<ChevronsUp />` |
| [church](https://lucide.dev/icons/church) | `church` | `<Church />` |
| [circle-arrow-down](https://lucide.dev/icons/circle-arrow-down) | `circle-arrow-down` | `<CircleArrowDown />` |
| [circle-arrow-left](https://lucide.dev/icons/circle-arrow-left) | `circle-arrow-left` | `<CircleArrowLeft />` |
| [circle-arrow-out-down-left](https://lucide.dev/icons/circle-arrow-out-down-left) | `circle-arrow-out-down-left` | `<CircleArrowOutDownLeft />` |
| [circle-arrow-out-down-right](https://lucide.dev/icons/circle-arrow-out-down-right) | `circle-arrow-out-down-right` | `<CircleArrowOutDownRight />` |
| [circle-arrow-out-up-left](https://lucide.dev/icons/circle-arrow-out-up-left) | `circle-arrow-out-up-left` | `<CircleArrowOutUpLeft />` |
| [circle-arrow-out-up-right](https://lucide.dev/icons/circle-arrow-out-up-right) | `circle-arrow-out-up-right` | `<CircleArrowOutUpRight />` |
| [circle-arrow-right](https://lucide.dev/icons/circle-arrow-right) | `circle-arrow-right` | `<CircleArrowRight />` |
| [circle-arrow-up](https://lucide.dev/icons/circle-arrow-up) | `circle-arrow-up` | `<CircleArrowUp />` |
| [circle-chevron-down](https://lucide.dev/icons/circle-chevron-down) | `circle-chevron-down` | `<CircleChevronDown />` |
| [circle-chevron-left](https://lucide.dev/icons/circle-chevron-left) | `circle-chevron-left` | `<CircleChevronLeft />` |
| [circle-chevron-right](https://lucide.dev/icons/circle-chevron-right) | `circle-chevron-right` | `<CircleChevronRight />` |
| [circle-chevron-up](https://lucide.dev/icons/circle-chevron-up) | `circle-chevron-up` | `<CircleChevronUp />` |
| [circle-parking](https://lucide.dev/icons/circle-parking) | `circle-parking` | `<CircleParking />` |
| [circle-parking-off](https://lucide.dev/icons/circle-parking-off) | `circle-parking-off` | `<CircleParkingOff />` |
| [compass](https://lucide.dev/icons/compass) | `compass` | `<Compass />` |
| [dumbbell](https://lucide.dev/icons/dumbbell) | `dumbbell` | `<Dumbbell />` |
| [earth](https://lucide.dev/icons/earth) | `earth` | `<Earth />` |
| [ev-charger](https://lucide.dev/icons/ev-charger) | `ev-charger` | `<EvCharger />` |
| [ferris-wheel](https://lucide.dev/icons/ferris-wheel) | `ferris-wheel` | `<FerrisWheel />` |
| [flag-triangle-left](https://lucide.dev/icons/flag-triangle-left) | `flag-triangle-left` | `<FlagTriangleLeft />` |
| [flag-triangle-right](https://lucide.dev/icons/flag-triangle-right) | `flag-triangle-right` | `<FlagTriangleRight />` |
| [footprints](https://lucide.dev/icons/footprints) | `footprints` | `<Footprints />` |
| [fuel](https://lucide.dev/icons/fuel) | `fuel` | `<Fuel />` |
| [gavel](https://lucide.dev/icons/gavel) | `gavel` | `<Gavel />` |
| [git-commit-horizontal](https://lucide.dev/icons/git-commit-horizontal) | `git-commit-horizontal` | `<GitCommitHorizontal />` |
| [git-commit-vertical](https://lucide.dev/icons/git-commit-vertical) | `git-commit-vertical` | `<GitCommitVertical />` |
| [globe](https://lucide.dev/icons/globe) | `globe` | `<Globe />` |
| [hospital](https://lucide.dev/icons/hospital) | `hospital` | `<Hospital />` |
| [hotel](https://lucide.dev/icons/hotel) | `hotel` | `<Hotel />` |
| [landmark](https://lucide.dev/icons/landmark) | `landmark` | `<Landmark />` |
| [library](https://lucide.dev/icons/library) | `library` | `<Library />` |
| [library-big](https://lucide.dev/icons/library-big) | `library-big` | `<LibraryBig />` |
| [locate](https://lucide.dev/icons/locate) | `locate` | `<Locate />` |
| [locate-fixed](https://lucide.dev/icons/locate-fixed) | `locate-fixed` | `<LocateFixed />` |
| [locate-off](https://lucide.dev/icons/locate-off) | `locate-off` | `<LocateOff />` |
| [map](https://lucide.dev/icons/map) | `map` | `<Map />` |
| [map-minus](https://lucide.dev/icons/map-minus) | `map-minus` | `<MapMinus />` |
| [map-pin](https://lucide.dev/icons/map-pin) | `map-pin` | `<MapPin />` |
| [map-pin-check](https://lucide.dev/icons/map-pin-check) | `map-pin-check` | `<MapPinCheck />` |
| [map-pin-check-inside](https://lucide.dev/icons/map-pin-check-inside) | `map-pin-check-inside` | `<MapPinCheckInside />` |
| [map-pin-house](https://lucide.dev/icons/map-pin-house) | `map-pin-house` | `<MapPinHouse />` |
| [map-pin-minus](https://lucide.dev/icons/map-pin-minus) | `map-pin-minus` | `<MapPinMinus />` |
| [map-pin-minus-inside](https://lucide.dev/icons/map-pin-minus-inside) | `map-pin-minus-inside` | `<MapPinMinusInside />` |
| [map-pin-off](https://lucide.dev/icons/map-pin-off) | `map-pin-off` | `<MapPinOff />` |
| [map-pin-pen](https://lucide.dev/icons/map-pin-pen) | `map-pin-pen` | `<MapPinPen />` |
| [map-pin-plus](https://lucide.dev/icons/map-pin-plus) | `map-pin-plus` | `<MapPinPlus />` |
| [map-pin-plus-inside](https://lucide.dev/icons/map-pin-plus-inside) | `map-pin-plus-inside` | `<MapPinPlusInside />` |
| [map-pin-x](https://lucide.dev/icons/map-pin-x) | `map-pin-x` | `<MapPinX />` |
| [map-pin-x-inside](https://lucide.dev/icons/map-pin-x-inside) | `map-pin-x-inside` | `<MapPinXInside />` |
| [map-pinned](https://lucide.dev/icons/map-pinned) | `map-pinned` | `<MapPinned />` |
| [map-plus](https://lucide.dev/icons/map-plus) | `map-plus` | `<MapPlus />` |
| [milestone](https://lucide.dev/icons/milestone) | `milestone` | `<Milestone />` |
| [navigation](https://lucide.dev/icons/navigation) | `navigation` | `<Navigation />` |
| [navigation-2](https://lucide.dev/icons/navigation-2) | `navigation-2` | `<Navigation2 />` |
| [navigation-2-off](https://lucide.dev/icons/navigation-2-off) | `navigation-2-off` | `<Navigation2Off />` |
| [navigation-off](https://lucide.dev/icons/navigation-off) | `navigation-off` | `<NavigationOff />` |
| [parking-meter](https://lucide.dev/icons/parking-meter) | `parking-meter` | `<ParkingMeter />` |
| [pin](https://lucide.dev/icons/pin) | `pin` | `<Pin />` |
| [pin-off](https://lucide.dev/icons/pin-off) | `pin-off` | `<PinOff />` |
| [radar](https://lucide.dev/icons/radar) | `radar` | `<Radar />` |
| [rail-symbol](https://lucide.dev/icons/rail-symbol) | `rail-symbol` | `<RailSymbol />` |
| [roller-coaster](https://lucide.dev/icons/roller-coaster) | `roller-coaster` | `<RollerCoaster />` |
| [route](https://lucide.dev/icons/route) | `route` | `<Route />` |
| [route-off](https://lucide.dev/icons/route-off) | `route-off` | `<RouteOff />` |
| [scale](https://lucide.dev/icons/scale) | `scale` | `<Scale />` |
| [school](https://lucide.dev/icons/school) | `school` | `<School />` |
| [ship](https://lucide.dev/icons/ship) | `ship` | `<Ship />` |
| [ship-wheel](https://lucide.dev/icons/ship-wheel) | `ship-wheel` | `<ShipWheel />` |
| [signpost](https://lucide.dev/icons/signpost) | `signpost` | `<Signpost />` |
| [signpost-big](https://lucide.dev/icons/signpost-big) | `signpost-big` | `<SignpostBig />` |
| [square-arrow-down](https://lucide.dev/icons/square-arrow-down) | `square-arrow-down` | `<SquareArrowDown />` |
| [square-arrow-down-left](https://lucide.dev/icons/square-arrow-down-left) | `square-arrow-down-left` | `<SquareArrowDownLeft />` |
| [square-arrow-down-right](https://lucide.dev/icons/square-arrow-down-right) | `square-arrow-down-right` | `<SquareArrowDownRight />` |
| [square-arrow-left](https://lucide.dev/icons/square-arrow-left) | `square-arrow-left` | `<SquareArrowLeft />` |
| [square-arrow-out-down-left](https://lucide.dev/icons/square-arrow-out-down-left) | `square-arrow-out-down-left` | `<SquareArrowOutDownLeft />` |
| [square-arrow-out-down-right](https://lucide.dev/icons/square-arrow-out-down-right) | `square-arrow-out-down-right` | `<SquareArrowOutDownRight />` |
| [square-arrow-out-up-left](https://lucide.dev/icons/square-arrow-out-up-left) | `square-arrow-out-up-left` | `<SquareArrowOutUpLeft />` |
| [square-arrow-out-up-right](https://lucide.dev/icons/square-arrow-out-up-right) | `square-arrow-out-up-right` | `<SquareArrowOutUpRight />` |
| [square-arrow-right](https://lucide.dev/icons/square-arrow-right) | `square-arrow-right` | `<SquareArrowRight />` |
| [square-arrow-up](https://lucide.dev/icons/square-arrow-up) | `square-arrow-up` | `<SquareArrowUp />` |
| [square-arrow-up-left](https://lucide.dev/icons/square-arrow-up-left) | `square-arrow-up-left` | `<SquareArrowUpLeft />` |
| [square-arrow-up-right](https://lucide.dev/icons/square-arrow-up-right) | `square-arrow-up-right` | `<SquareArrowUpRight />` |
| [square-chevron-down](https://lucide.dev/icons/square-chevron-down) | `square-chevron-down` | `<SquareChevronDown />` |
| [square-chevron-left](https://lucide.dev/icons/square-chevron-left) | `square-chevron-left` | `<SquareChevronLeft />` |
| [square-chevron-right](https://lucide.dev/icons/square-chevron-right) | `square-chevron-right` | `<SquareChevronRight />` |
| [square-chevron-up](https://lucide.dev/icons/square-chevron-up) | `square-chevron-up` | `<SquareChevronUp />` |
| [square-library](https://lucide.dev/icons/square-library) | `square-library` | `<SquareLibrary />` |
| [square-m](https://lucide.dev/icons/square-m) | `square-m` | `<SquareM />` |
| [square-parking](https://lucide.dev/icons/square-parking) | `square-parking` | `<SquareParking />` |
| [square-parking-off](https://lucide.dev/icons/square-parking-off) | `square-parking-off` | `<SquareParkingOff />` |
| [store](https://lucide.dev/icons/store) | `store` | `<Store />` |
| [train-front-tunnel](https://lucide.dev/icons/train-front-tunnel) | `train-front-tunnel` | `<TrainFrontTunnel />` |
| [train-track](https://lucide.dev/icons/train-track) | `train-track` | `<TrainTrack />` |
| [university](https://lucide.dev/icons/university) | `university` | `<University />` |
| [utensils](https://lucide.dev/icons/utensils) | `utensils` | `<Utensils />` |
| [utensils-crossed](https://lucide.dev/icons/utensils-crossed) | `utensils-crossed` | `<UtensilsCrossed />` |
| [warehouse](https://lucide.dev/icons/warehouse) | `warehouse` | `<Warehouse />` |
| [waves](https://lucide.dev/icons/waves) | `waves` | `<Waves />` |
| [waypoints](https://lucide.dev/icons/waypoints) | `waypoints` | `<Waypoints />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "navigation"] or ["leptos", "all-icons"]
<ArrowBigDown class="w-6 h-6 text-gray-600" />
```

### Notifications

`notifications` and `all-icons` features - 39 icons

| Icon | Name | Component |
|------|------|-----------|
| [alarm-clock](https://lucide.dev/icons/alarm-clock) | `alarm-clock` | `<AlarmClock />` |
| [alarm-clock-check](https://lucide.dev/icons/alarm-clock-check) | `alarm-clock-check` | `<AlarmClockCheck />` |
| [alarm-clock-minus](https://lucide.dev/icons/alarm-clock-minus) | `alarm-clock-minus` | `<AlarmClockMinus />` |
| [alarm-clock-off](https://lucide.dev/icons/alarm-clock-off) | `alarm-clock-off` | `<AlarmClockOff />` |
| [alarm-clock-plus](https://lucide.dev/icons/alarm-clock-plus) | `alarm-clock-plus` | `<AlarmClockPlus />` |
| [bell](https://lucide.dev/icons/bell) | `bell` | `<Bell />` |
| [bell-dot](https://lucide.dev/icons/bell-dot) | `bell-dot` | `<BellDot />` |
| [bell-electric](https://lucide.dev/icons/bell-electric) | `bell-electric` | `<BellElectric />` |
| [bell-minus](https://lucide.dev/icons/bell-minus) | `bell-minus` | `<BellMinus />` |
| [bell-off](https://lucide.dev/icons/bell-off) | `bell-off` | `<BellOff />` |
| [bell-plus](https://lucide.dev/icons/bell-plus) | `bell-plus` | `<BellPlus />` |
| [bell-ring](https://lucide.dev/icons/bell-ring) | `bell-ring` | `<BellRing />` |
| [check](https://lucide.dev/icons/check) | `check` | `<Check />` |
| [check-check](https://lucide.dev/icons/check-check) | `check-check` | `<CheckCheck />` |
| [check-line](https://lucide.dev/icons/check-line) | `check-line` | `<CheckLine />` |
| [circle-alert](https://lucide.dev/icons/circle-alert) | `circle-alert` | `<CircleAlert />` |
| [circle-check](https://lucide.dev/icons/circle-check) | `circle-check` | `<CircleCheck />` |
| [circle-check-big](https://lucide.dev/icons/circle-check-big) | `circle-check-big` | `<CircleCheckBig />` |
| [circle-question-mark](https://lucide.dev/icons/circle-question-mark) | `circle-question-mark` | `<CircleQuestionMark />` |
| [copy-check](https://lucide.dev/icons/copy-check) | `copy-check` | `<CopyCheck />` |
| [copy-x](https://lucide.dev/icons/copy-x) | `copy-x` | `<CopyX />` |
| [file-warning](https://lucide.dev/icons/file-warning) | `file-warning` | `<FileWarning />` |
| [info](https://lucide.dev/icons/info) | `info` | `<Info />` |
| [laptop-minimal-check](https://lucide.dev/icons/laptop-minimal-check) | `laptop-minimal-check` | `<LaptopMinimalCheck />` |
| [megaphone](https://lucide.dev/icons/megaphone) | `megaphone` | `<Megaphone />` |
| [megaphone-off](https://lucide.dev/icons/megaphone-off) | `megaphone-off` | `<MegaphoneOff />` |
| [message-circle-warning](https://lucide.dev/icons/message-circle-warning) | `message-circle-warning` | `<MessageCircleWarning />` |
| [message-square-dot](https://lucide.dev/icons/message-square-dot) | `message-square-dot` | `<MessageSquareDot />` |
| [message-square-warning](https://lucide.dev/icons/message-square-warning) | `message-square-warning` | `<MessageSquareWarning />` |
| [octagon-alert](https://lucide.dev/icons/octagon-alert) | `octagon-alert` | `<OctagonAlert />` |
| [octagon-x](https://lucide.dev/icons/octagon-x) | `octagon-x` | `<OctagonX />` |
| [shield-alert](https://lucide.dev/icons/shield-alert) | `shield-alert` | `<ShieldAlert />` |
| [smile-plus](https://lucide.dev/icons/smile-plus) | `smile-plus` | `<SmilePlus />` |
| [square-check](https://lucide.dev/icons/square-check) | `square-check` | `<SquareCheck />` |
| [square-check-big](https://lucide.dev/icons/square-check-big) | `square-check-big` | `<SquareCheckBig />` |
| [square-x](https://lucide.dev/icons/square-x) | `square-x` | `<SquareX />` |
| [triangle-alert](https://lucide.dev/icons/triangle-alert) | `triangle-alert` | `<TriangleAlert />` |
| [vibrate](https://lucide.dev/icons/vibrate) | `vibrate` | `<Vibrate />` |
| [x](https://lucide.dev/icons/x) | `x` | `<X />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "notifications"] or ["leptos", "all-icons"]
<AlarmClock class="w-6 h-6 text-gray-600" />
```

### People

`people` and `all-icons` features - 3 icons

| Icon | Name | Component |
|------|------|-----------|
| [baby](https://lucide.dev/icons/baby) | `baby` | `<Baby />` |
| [hand-platter](https://lucide.dev/icons/hand-platter) | `hand-platter` | `<HandPlatter />` |
| [person-standing](https://lucide.dev/icons/person-standing) | `person-standing` | `<PersonStanding />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "people"] or ["leptos", "all-icons"]
<Baby class="w-6 h-6 text-gray-600" />
```

### Photography

`photography` and `all-icons` features - 75 icons

| Icon | Name | Component |
|------|------|-----------|
| [album](https://lucide.dev/icons/album) | `album` | `<Album />` |
| [aperture](https://lucide.dev/icons/aperture) | `aperture` | `<Aperture />` |
| [backpack](https://lucide.dev/icons/backpack) | `backpack` | `<Backpack />` |
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [blend](https://lucide.dev/icons/blend) | `blend` | `<Blend />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [camera](https://lucide.dev/icons/camera) | `camera` | `<Camera />` |
| [camera-off](https://lucide.dev/icons/camera-off) | `camera-off` | `<CameraOff />` |
| [cctv](https://lucide.dev/icons/cctv) | `cctv` | `<Cctv />` |
| [contrast](https://lucide.dev/icons/contrast) | `contrast` | `<Contrast />` |
| [crop](https://lucide.dev/icons/crop) | `crop` | `<Crop />` |
| [crosshair](https://lucide.dev/icons/crosshair) | `crosshair` | `<Crosshair />` |
| [database-backup](https://lucide.dev/icons/database-backup) | `database-backup` | `<DatabaseBackup />` |
| [diamond-minus](https://lucide.dev/icons/diamond-minus) | `diamond-minus` | `<DiamondMinus />` |
| [diamond-plus](https://lucide.dev/icons/diamond-plus) | `diamond-plus` | `<DiamondPlus />` |
| [eclipse](https://lucide.dev/icons/eclipse) | `eclipse` | `<Eclipse />` |
| [eye](https://lucide.dev/icons/eye) | `eye` | `<Eye />` |
| [eye-closed](https://lucide.dev/icons/eye-closed) | `eye-closed` | `<EyeClosed />` |
| [eye-off](https://lucide.dev/icons/eye-off) | `eye-off` | `<EyeOff />` |
| [film](https://lucide.dev/icons/film) | `film` | `<Film />` |
| [flashlight](https://lucide.dev/icons/flashlight) | `flashlight` | `<Flashlight />` |
| [flashlight-off](https://lucide.dev/icons/flashlight-off) | `flashlight-off` | `<FlashlightOff />` |
| [flip-horizontal](https://lucide.dev/icons/flip-horizontal) | `flip-horizontal` | `<FlipHorizontal />` |
| [flip-horizontal-2](https://lucide.dev/icons/flip-horizontal-2) | `flip-horizontal-2` | `<FlipHorizontal2 />` |
| [flip-vertical](https://lucide.dev/icons/flip-vertical) | `flip-vertical` | `<FlipVertical />` |
| [flip-vertical-2](https://lucide.dev/icons/flip-vertical-2) | `flip-vertical-2` | `<FlipVertical2 />` |
| [focus](https://lucide.dev/icons/focus) | `focus` | `<Focus />` |
| [frame](https://lucide.dev/icons/frame) | `frame` | `<Frame />` |
| [fullscreen](https://lucide.dev/icons/fullscreen) | `fullscreen` | `<Fullscreen />` |
| [gallery-horizontal](https://lucide.dev/icons/gallery-horizontal) | `gallery-horizontal` | `<GalleryHorizontal />` |
| [gallery-horizontal-end](https://lucide.dev/icons/gallery-horizontal-end) | `gallery-horizontal-end` | `<GalleryHorizontalEnd />` |
| [gallery-thumbnails](https://lucide.dev/icons/gallery-thumbnails) | `gallery-thumbnails` | `<GalleryThumbnails />` |
| [gallery-vertical](https://lucide.dev/icons/gallery-vertical) | `gallery-vertical` | `<GalleryVertical />` |
| [gallery-vertical-end](https://lucide.dev/icons/gallery-vertical-end) | `gallery-vertical-end` | `<GalleryVerticalEnd />` |
| [image](https://lucide.dev/icons/image) | `image` | `<Image />` |
| [image-down](https://lucide.dev/icons/image-down) | `image-down` | `<ImageDown />` |
| [image-minus](https://lucide.dev/icons/image-minus) | `image-minus` | `<ImageMinus />` |
| [image-off](https://lucide.dev/icons/image-off) | `image-off` | `<ImageOff />` |
| [image-play](https://lucide.dev/icons/image-play) | `image-play` | `<ImagePlay />` |
| [image-plus](https://lucide.dev/icons/image-plus) | `image-plus` | `<ImagePlus />` |
| [image-up](https://lucide.dev/icons/image-up) | `image-up` | `<ImageUp />` |
| [image-upscale](https://lucide.dev/icons/image-upscale) | `image-upscale` | `<ImageUpscale />` |
| [images](https://lucide.dev/icons/images) | `images` | `<Images />` |
| [instagram](https://lucide.dev/icons/instagram) | `instagram` | `<Instagram />` |
| [layout-list](https://lucide.dev/icons/layout-list) | `layout-list` | `<LayoutList />` |
| [library](https://lucide.dev/icons/library) | `library` | `<Library />` |
| [library-big](https://lucide.dev/icons/library-big) | `library-big` | `<LibraryBig />` |
| [lightbulb](https://lucide.dev/icons/lightbulb) | `lightbulb` | `<Lightbulb />` |
| [lightbulb-off](https://lucide.dev/icons/lightbulb-off) | `lightbulb-off` | `<LightbulbOff />` |
| [paintbrush](https://lucide.dev/icons/paintbrush) | `paintbrush` | `<Paintbrush />` |
| [paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) | `paintbrush-vertical` | `<PaintbrushVertical />` |
| [palette](https://lucide.dev/icons/palette) | `palette` | `<Palette />` |
| [presentation](https://lucide.dev/icons/presentation) | `presentation` | `<Presentation />` |
| [projector](https://lucide.dev/icons/projector) | `projector` | `<Projector />` |
| [proportions](https://lucide.dev/icons/proportions) | `proportions` | `<Proportions />` |
| [ratio](https://lucide.dev/icons/ratio) | `ratio` | `<Ratio />` |
| [rotate-ccw](https://lucide.dev/icons/rotate-ccw) | `rotate-ccw` | `<RotateCcw />` |
| [rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) | `rotate-ccw-square` | `<RotateCcwSquare />` |
| [rotate-cw](https://lucide.dev/icons/rotate-cw) | `rotate-cw` | `<RotateCw />` |
| [rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) | `rotate-cw-square` | `<RotateCwSquare />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-search](https://lucide.dev/icons/scan-search) | `scan-search` | `<ScanSearch />` |
| [spotlight](https://lucide.dev/icons/spotlight) | `spotlight` | `<Spotlight />` |
| [square-library](https://lucide.dev/icons/square-library) | `square-library` | `<SquareLibrary />` |
| [swatch-book](https://lucide.dev/icons/swatch-book) | `swatch-book` | `<SwatchBook />` |
| [video](https://lucide.dev/icons/video) | `video` | `<Video />` |
| [video-off](https://lucide.dev/icons/video-off) | `video-off` | `<VideoOff />` |
| [videotape](https://lucide.dev/icons/videotape) | `videotape` | `<Videotape />` |
| [view](https://lucide.dev/icons/view) | `view` | `<View />` |
| [wand](https://lucide.dev/icons/wand) | `wand` | `<Wand />` |
| [wand-sparkles](https://lucide.dev/icons/wand-sparkles) | `wand-sparkles` | `<WandSparkles />` |
| [zap](https://lucide.dev/icons/zap) | `zap` | `<Zap />` |
| [zap-off](https://lucide.dev/icons/zap-off) | `zap-off` | `<ZapOff />` |
| [zoom-in](https://lucide.dev/icons/zoom-in) | `zoom-in` | `<ZoomIn />` |
| [zoom-out](https://lucide.dev/icons/zoom-out) | `zoom-out` | `<ZoomOut />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "photography"] or ["leptos", "all-icons"]
<Album class="w-6 h-6 text-gray-600" />
```

### Science

`science` and `all-icons` features - 30 icons

| Icon | Name | Component |
|------|------|-----------|
| [activity](https://lucide.dev/icons/activity) | `activity` | `<Activity />` |
| [atom](https://lucide.dev/icons/atom) | `atom` | `<Atom />` |
| [beaker](https://lucide.dev/icons/beaker) | `beaker` | `<Beaker />` |
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [biohazard](https://lucide.dev/icons/biohazard) | `biohazard` | `<Biohazard />` |
| [brain](https://lucide.dev/icons/brain) | `brain` | `<Brain />` |
| [brain-circuit](https://lucide.dev/icons/brain-circuit) | `brain-circuit` | `<BrainCircuit />` |
| [brain-cog](https://lucide.dev/icons/brain-cog) | `brain-cog` | `<BrainCog />` |
| [circle-gauge](https://lucide.dev/icons/circle-gauge) | `circle-gauge` | `<CircleGauge />` |
| [circuit-board](https://lucide.dev/icons/circuit-board) | `circuit-board` | `<CircuitBoard />` |
| [eclipse](https://lucide.dev/icons/eclipse) | `eclipse` | `<Eclipse />` |
| [flask-conical](https://lucide.dev/icons/flask-conical) | `flask-conical` | `<FlaskConical />` |
| [flask-conical-off](https://lucide.dev/icons/flask-conical-off) | `flask-conical-off` | `<FlaskConicalOff />` |
| [flask-round](https://lucide.dev/icons/flask-round) | `flask-round` | `<FlaskRound />` |
| [gauge](https://lucide.dev/icons/gauge) | `gauge` | `<Gauge />` |
| [microscope](https://lucide.dev/icons/microscope) | `microscope` | `<Microscope />` |
| [omega](https://lucide.dev/icons/omega) | `omega` | `<Omega />` |
| [orbit](https://lucide.dev/icons/orbit) | `orbit` | `<Orbit />` |
| [pipette](https://lucide.dev/icons/pipette) | `pipette` | `<Pipette />` |
| [radiation](https://lucide.dev/icons/radiation) | `radiation` | `<Radiation />` |
| [satellite](https://lucide.dev/icons/satellite) | `satellite` | `<Satellite />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [sigma](https://lucide.dev/icons/sigma) | `sigma` | `<Sigma />` |
| [square-activity](https://lucide.dev/icons/square-activity) | `square-activity` | `<SquareActivity />` |
| [stethoscope](https://lucide.dev/icons/stethoscope) | `stethoscope` | `<Stethoscope />` |
| [syringe](https://lucide.dev/icons/syringe) | `syringe` | `<Syringe />` |
| [telescope](https://lucide.dev/icons/telescope) | `telescope` | `<Telescope />` |
| [test-tube](https://lucide.dev/icons/test-tube) | `test-tube` | `<TestTube />` |
| [test-tube-diagonal](https://lucide.dev/icons/test-tube-diagonal) | `test-tube-diagonal` | `<TestTubeDiagonal />` |
| [test-tubes](https://lucide.dev/icons/test-tubes) | `test-tubes` | `<TestTubes />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "science"] or ["leptos", "all-icons"]
<Activity class="w-6 h-6 text-gray-600" />
```

### Seasons

`seasons` and `all-icons` features - 5 icons

| Icon | Name | Component |
|------|------|-----------|
| [flower-2](https://lucide.dev/icons/flower-2) | `flower-2` | `<Flower2 />` |
| [leaf](https://lucide.dev/icons/leaf) | `leaf` | `<Leaf />` |
| [rose](https://lucide.dev/icons/rose) | `rose` | `<Rose />` |
| [snowflake](https://lucide.dev/icons/snowflake) | `snowflake` | `<Snowflake />` |
| [sun](https://lucide.dev/icons/sun) | `sun` | `<Sun />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "seasons"] or ["leptos", "all-icons"]
<Flower2 class="w-6 h-6 text-gray-600" />
```

### Security

`security` and `all-icons` features - 57 icons

| Icon | Name | Component |
|------|------|-----------|
| [bomb](https://lucide.dev/icons/bomb) | `bomb` | `<Bomb />` |
| [book-key](https://lucide.dev/icons/book-key) | `book-key` | `<BookKey />` |
| [book-lock](https://lucide.dev/icons/book-lock) | `book-lock` | `<BookLock />` |
| [brick-wall-fire](https://lucide.dev/icons/brick-wall-fire) | `brick-wall-fire` | `<BrickWallFire />` |
| [brick-wall-shield](https://lucide.dev/icons/brick-wall-shield) | `brick-wall-shield` | `<BrickWallShield />` |
| [cctv](https://lucide.dev/icons/cctv) | `cctv` | `<Cctv />` |
| [columns-4](https://lucide.dev/icons/columns-4) | `columns-4` | `<Columns4 />` |
| [door-closed](https://lucide.dev/icons/door-closed) | `door-closed` | `<DoorClosed />` |
| [door-closed-locked](https://lucide.dev/icons/door-closed-locked) | `door-closed-locked` | `<DoorClosedLocked />` |
| [door-open](https://lucide.dev/icons/door-open) | `door-open` | `<DoorOpen />` |
| [earth-lock](https://lucide.dev/icons/earth-lock) | `earth-lock` | `<EarthLock />` |
| [eye](https://lucide.dev/icons/eye) | `eye` | `<Eye />` |
| [eye-closed](https://lucide.dev/icons/eye-closed) | `eye-closed` | `<EyeClosed />` |
| [eye-off](https://lucide.dev/icons/eye-off) | `eye-off` | `<EyeOff />` |
| [file-key](https://lucide.dev/icons/file-key) | `file-key` | `<FileKey />` |
| [file-key-2](https://lucide.dev/icons/file-key-2) | `file-key-2` | `<FileKey2 />` |
| [file-lock](https://lucide.dev/icons/file-lock) | `file-lock` | `<FileLock />` |
| [file-lock-2](https://lucide.dev/icons/file-lock-2) | `file-lock-2` | `<FileLock2 />` |
| [fingerprint](https://lucide.dev/icons/fingerprint) | `fingerprint` | `<Fingerprint />` |
| [folder-key](https://lucide.dev/icons/folder-key) | `folder-key` | `<FolderKey />` |
| [folder-lock](https://lucide.dev/icons/folder-lock) | `folder-lock` | `<FolderLock />` |
| [globe-lock](https://lucide.dev/icons/globe-lock) | `globe-lock` | `<GlobeLock />` |
| [handshake](https://lucide.dev/icons/handshake) | `handshake` | `<Handshake />` |
| [hat-glasses](https://lucide.dev/icons/hat-glasses) | `hat-glasses` | `<HatGlasses />` |
| [heart-handshake](https://lucide.dev/icons/heart-handshake) | `heart-handshake` | `<HeartHandshake />` |
| [id-card](https://lucide.dev/icons/id-card) | `id-card` | `<IdCard />` |
| [id-card-lanyard](https://lucide.dev/icons/id-card-lanyard) | `id-card-lanyard` | `<IdCardLanyard />` |
| [key](https://lucide.dev/icons/key) | `key` | `<Key />` |
| [key-round](https://lucide.dev/icons/key-round) | `key-round` | `<KeyRound />` |
| [key-square](https://lucide.dev/icons/key-square) | `key-square` | `<KeySquare />` |
| [lock](https://lucide.dev/icons/lock) | `lock` | `<Lock />` |
| [lock-keyhole](https://lucide.dev/icons/lock-keyhole) | `lock-keyhole` | `<LockKeyhole />` |
| [lock-keyhole-open](https://lucide.dev/icons/lock-keyhole-open) | `lock-keyhole-open` | `<LockKeyholeOpen />` |
| [lock-open](https://lucide.dev/icons/lock-open) | `lock-open` | `<LockOpen />` |
| [radar](https://lucide.dev/icons/radar) | `radar` | `<Radar />` |
| [rotate-ccw-key](https://lucide.dev/icons/rotate-ccw-key) | `rotate-ccw-key` | `<RotateCcwKey />` |
| [scan](https://lucide.dev/icons/scan) | `scan` | `<Scan />` |
| [scan-eye](https://lucide.dev/icons/scan-eye) | `scan-eye` | `<ScanEye />` |
| [scan-face](https://lucide.dev/icons/scan-face) | `scan-face` | `<ScanFace />` |
| [scan-qr-code](https://lucide.dev/icons/scan-qr-code) | `scan-qr-code` | `<ScanQrCode />` |
| [shield](https://lucide.dev/icons/shield) | `shield` | `<Shield />` |
| [shield-alert](https://lucide.dev/icons/shield-alert) | `shield-alert` | `<ShieldAlert />` |
| [shield-ban](https://lucide.dev/icons/shield-ban) | `shield-ban` | `<ShieldBan />` |
| [shield-check](https://lucide.dev/icons/shield-check) | `shield-check` | `<ShieldCheck />` |
| [shield-ellipsis](https://lucide.dev/icons/shield-ellipsis) | `shield-ellipsis` | `<ShieldEllipsis />` |
| [shield-half](https://lucide.dev/icons/shield-half) | `shield-half` | `<ShieldHalf />` |
| [shield-minus](https://lucide.dev/icons/shield-minus) | `shield-minus` | `<ShieldMinus />` |
| [shield-off](https://lucide.dev/icons/shield-off) | `shield-off` | `<ShieldOff />` |
| [shield-plus](https://lucide.dev/icons/shield-plus) | `shield-plus` | `<ShieldPlus />` |
| [shield-question-mark](https://lucide.dev/icons/shield-question-mark) | `shield-question-mark` | `<ShieldQuestionMark />` |
| [shield-user](https://lucide.dev/icons/shield-user) | `shield-user` | `<ShieldUser />` |
| [shield-x](https://lucide.dev/icons/shield-x) | `shield-x` | `<ShieldX />` |
| [square-asterisk](https://lucide.dev/icons/square-asterisk) | `square-asterisk` | `<SquareAsterisk />` |
| [user-lock](https://lucide.dev/icons/user-lock) | `user-lock` | `<UserLock />` |
| [vault](https://lucide.dev/icons/vault) | `vault` | `<Vault />` |
| [waypoints](https://lucide.dev/icons/waypoints) | `waypoints` | `<Waypoints />` |
| [worm](https://lucide.dev/icons/worm) | `worm` | `<Worm />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "security"] or ["leptos", "all-icons"]
<Bomb class="w-6 h-6 text-gray-600" />
```

### Shapes

`shapes` and `all-icons` features - 47 icons

| Icon | Name | Component |
|------|------|-----------|
| [badge](https://lucide.dev/icons/badge) | `badge` | `<Badge />` |
| [badge-question-mark](https://lucide.dev/icons/badge-question-mark) | `badge-question-mark` | `<BadgeQuestionMark />` |
| [blocks](https://lucide.dev/icons/blocks) | `blocks` | `<Blocks />` |
| [box](https://lucide.dev/icons/box) | `box` | `<Box />` |
| [boxes](https://lucide.dev/icons/boxes) | `boxes` | `<Boxes />` |
| [circle](https://lucide.dev/icons/circle) | `circle` | `<Circle />` |
| [circle-dashed](https://lucide.dev/icons/circle-dashed) | `circle-dashed` | `<CircleDashed />` |
| [circle-dot](https://lucide.dev/icons/circle-dot) | `circle-dot` | `<CircleDot />` |
| [circle-dot-dashed](https://lucide.dev/icons/circle-dot-dashed) | `circle-dot-dashed` | `<CircleDotDashed />` |
| [circle-off](https://lucide.dev/icons/circle-off) | `circle-off` | `<CircleOff />` |
| [circle-slash-2](https://lucide.dev/icons/circle-slash-2) | `circle-slash-2` | `<CircleSlash2 />` |
| [circle-small](https://lucide.dev/icons/circle-small) | `circle-small` | `<CircleSmall />` |
| [club](https://lucide.dev/icons/club) | `club` | `<Club />` |
| [cone](https://lucide.dev/icons/cone) | `cone` | `<Cone />` |
| [cross](https://lucide.dev/icons/cross) | `cross` | `<Cross />` |
| [cuboid](https://lucide.dev/icons/cuboid) | `cuboid` | `<Cuboid />` |
| [cylinder](https://lucide.dev/icons/cylinder) | `cylinder` | `<Cylinder />` |
| [diameter](https://lucide.dev/icons/diameter) | `diameter` | `<Diameter />` |
| [diamond](https://lucide.dev/icons/diamond) | `diamond` | `<Diamond />` |
| [dot](https://lucide.dev/icons/dot) | `dot` | `<Dot />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [hexagon](https://lucide.dev/icons/hexagon) | `hexagon` | `<Hexagon />` |
| [line-squiggle](https://lucide.dev/icons/line-squiggle) | `line-squiggle` | `<LineSquiggle />` |
| [octagon](https://lucide.dev/icons/octagon) | `octagon` | `<Octagon />` |
| [octagon-alert](https://lucide.dev/icons/octagon-alert) | `octagon-alert` | `<OctagonAlert />` |
| [octagon-pause](https://lucide.dev/icons/octagon-pause) | `octagon-pause` | `<OctagonPause />` |
| [pentagon](https://lucide.dev/icons/pentagon) | `pentagon` | `<Pentagon />` |
| [pyramid](https://lucide.dev/icons/pyramid) | `pyramid` | `<Pyramid />` |
| [radius](https://lucide.dev/icons/radius) | `radius` | `<Radius />` |
| [rectangle-horizontal](https://lucide.dev/icons/rectangle-horizontal) | `rectangle-horizontal` | `<RectangleHorizontal />` |
| [rectangle-vertical](https://lucide.dev/icons/rectangle-vertical) | `rectangle-vertical` | `<RectangleVertical />` |
| [shapes](https://lucide.dev/icons/shapes) | `shapes` | `<Shapes />` |
| [shield](https://lucide.dev/icons/shield) | `shield` | `<Shield />` |
| [spade](https://lucide.dev/icons/spade) | `spade` | `<Spade />` |
| [sparkle](https://lucide.dev/icons/sparkle) | `sparkle` | `<Sparkle />` |
| [square](https://lucide.dev/icons/square) | `square` | `<Square />` |
| [squircle](https://lucide.dev/icons/squircle) | `squircle` | `<Squircle />` |
| [squircle-dashed](https://lucide.dev/icons/squircle-dashed) | `squircle-dashed` | `<SquircleDashed />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [tangent](https://lucide.dev/icons/tangent) | `tangent` | `<Tangent />` |
| [torus](https://lucide.dev/icons/torus) | `torus` | `<Torus />` |
| [triangle](https://lucide.dev/icons/triangle) | `triangle` | `<Triangle />` |
| [triangle-alert](https://lucide.dev/icons/triangle-alert) | `triangle-alert` | `<TriangleAlert />` |
| [triangle-dashed](https://lucide.dev/icons/triangle-dashed) | `triangle-dashed` | `<TriangleDashed />` |
| [triangle-right](https://lucide.dev/icons/triangle-right) | `triangle-right` | `<TriangleRight />` |
| [ungroup](https://lucide.dev/icons/ungroup) | `ungroup` | `<Ungroup />` |
| [vector-square](https://lucide.dev/icons/vector-square) | `vector-square` | `<VectorSquare />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "shapes"] or ["leptos", "all-icons"]
<Badge class="w-6 h-6 text-gray-600" />
```

### Shopping

`shopping` and `all-icons` features - 27 icons

| Icon | Name | Component |
|------|------|-----------|
| [badge-cent](https://lucide.dev/icons/badge-cent) | `badge-cent` | `<BadgeCent />` |
| [badge-dollar-sign](https://lucide.dev/icons/badge-dollar-sign) | `badge-dollar-sign` | `<BadgeDollarSign />` |
| [badge-euro](https://lucide.dev/icons/badge-euro) | `badge-euro` | `<BadgeEuro />` |
| [badge-indian-rupee](https://lucide.dev/icons/badge-indian-rupee) | `badge-indian-rupee` | `<BadgeIndianRupee />` |
| [badge-japanese-yen](https://lucide.dev/icons/badge-japanese-yen) | `badge-japanese-yen` | `<BadgeJapaneseYen />` |
| [badge-percent](https://lucide.dev/icons/badge-percent) | `badge-percent` | `<BadgePercent />` |
| [badge-pound-sterling](https://lucide.dev/icons/badge-pound-sterling) | `badge-pound-sterling` | `<BadgePoundSterling />` |
| [badge-russian-ruble](https://lucide.dev/icons/badge-russian-ruble) | `badge-russian-ruble` | `<BadgeRussianRuble />` |
| [badge-swiss-franc](https://lucide.dev/icons/badge-swiss-franc) | `badge-swiss-franc` | `<BadgeSwissFranc />` |
| [badge-turkish-lira](https://lucide.dev/icons/badge-turkish-lira) | `badge-turkish-lira` | `<BadgeTurkishLira />` |
| [barcode](https://lucide.dev/icons/barcode) | `barcode` | `<Barcode />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [circle-percent](https://lucide.dev/icons/circle-percent) | `circle-percent` | `<CirclePercent />` |
| [diamond-percent](https://lucide.dev/icons/diamond-percent) | `diamond-percent` | `<DiamondPercent />` |
| [handbag](https://lucide.dev/icons/handbag) | `handbag` | `<Handbag />` |
| [percent](https://lucide.dev/icons/percent) | `percent` | `<Percent />` |
| [scan](https://lucide.dev/icons/scan) | `scan` | `<Scan />` |
| [scan-barcode](https://lucide.dev/icons/scan-barcode) | `scan-barcode` | `<ScanBarcode />` |
| [scan-line](https://lucide.dev/icons/scan-line) | `scan-line` | `<ScanLine />` |
| [scan-qr-code](https://lucide.dev/icons/scan-qr-code) | `scan-qr-code` | `<ScanQrCode />` |
| [shirt](https://lucide.dev/icons/shirt) | `shirt` | `<Shirt />` |
| [shopping-bag](https://lucide.dev/icons/shopping-bag) | `shopping-bag` | `<ShoppingBag />` |
| [shopping-basket](https://lucide.dev/icons/shopping-basket) | `shopping-basket` | `<ShoppingBasket />` |
| [shopping-cart](https://lucide.dev/icons/shopping-cart) | `shopping-cart` | `<ShoppingCart />` |
| [square-percent](https://lucide.dev/icons/square-percent) | `square-percent` | `<SquarePercent />` |
| [store](https://lucide.dev/icons/store) | `store` | `<Store />` |
| [ticket-percent](https://lucide.dev/icons/ticket-percent) | `ticket-percent` | `<TicketPercent />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "shopping"] or ["leptos", "all-icons"]
<BadgeCent class="w-6 h-6 text-gray-600" />
```

### Social

`social` and `all-icons` features - 118 icons

| Icon | Name | Component |
|------|------|-----------|
| [activity](https://lucide.dev/icons/activity) | `activity` | `<Activity />` |
| [badge](https://lucide.dev/icons/badge) | `badge` | `<Badge />` |
| [badge-alert](https://lucide.dev/icons/badge-alert) | `badge-alert` | `<BadgeAlert />` |
| [badge-check](https://lucide.dev/icons/badge-check) | `badge-check` | `<BadgeCheck />` |
| [badge-info](https://lucide.dev/icons/badge-info) | `badge-info` | `<BadgeInfo />` |
| [badge-minus](https://lucide.dev/icons/badge-minus) | `badge-minus` | `<BadgeMinus />` |
| [badge-percent](https://lucide.dev/icons/badge-percent) | `badge-percent` | `<BadgePercent />` |
| [badge-plus](https://lucide.dev/icons/badge-plus) | `badge-plus` | `<BadgePlus />` |
| [badge-question-mark](https://lucide.dev/icons/badge-question-mark) | `badge-question-mark` | `<BadgeQuestionMark />` |
| [badge-x](https://lucide.dev/icons/badge-x) | `badge-x` | `<BadgeX />` |
| [book-heart](https://lucide.dev/icons/book-heart) | `book-heart` | `<BookHeart />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [book-user](https://lucide.dev/icons/book-user) | `book-user` | `<BookUser />` |
| [boom-box](https://lucide.dev/icons/boom-box) | `boom-box` | `<BoomBox />` |
| [bot](https://lucide.dev/icons/bot) | `bot` | `<Bot />` |
| [bot-message-square](https://lucide.dev/icons/bot-message-square) | `bot-message-square` | `<BotMessageSquare />` |
| [bot-off](https://lucide.dev/icons/bot-off) | `bot-off` | `<BotOff />` |
| [cake](https://lucide.dev/icons/cake) | `cake` | `<Cake />` |
| [cake-slice](https://lucide.dev/icons/cake-slice) | `cake-slice` | `<CakeSlice />` |
| [circle-fading-plus](https://lucide.dev/icons/circle-fading-plus) | `circle-fading-plus` | `<CircleFadingPlus />` |
| [circle-percent](https://lucide.dev/icons/circle-percent) | `circle-percent` | `<CirclePercent />` |
| [contact](https://lucide.dev/icons/contact) | `contact` | `<Contact />` |
| [contact-round](https://lucide.dev/icons/contact-round) | `contact-round` | `<ContactRound />` |
| [diamond-percent](https://lucide.dev/icons/diamond-percent) | `diamond-percent` | `<DiamondPercent />` |
| [dribbble](https://lucide.dev/icons/dribbble) | `dribbble` | `<Dribbble />` |
| [external-link](https://lucide.dev/icons/external-link) | `external-link` | `<ExternalLink />` |
| [facebook](https://lucide.dev/icons/facebook) | `facebook` | `<Facebook />` |
| [flag](https://lucide.dev/icons/flag) | `flag` | `<Flag />` |
| [flag-off](https://lucide.dev/icons/flag-off) | `flag-off` | `<FlagOff />` |
| [flame](https://lucide.dev/icons/flame) | `flame` | `<Flame />` |
| [flame-kindling](https://lucide.dev/icons/flame-kindling) | `flame-kindling` | `<FlameKindling />` |
| [hand-fist](https://lucide.dev/icons/hand-fist) | `hand-fist` | `<HandFist />` |
| [hand-heart](https://lucide.dev/icons/hand-heart) | `hand-heart` | `<HandHeart />` |
| [handshake](https://lucide.dev/icons/handshake) | `handshake` | `<Handshake />` |
| [hash](https://lucide.dev/icons/hash) | `hash` | `<Hash />` |
| [hat-glasses](https://lucide.dev/icons/hat-glasses) | `hat-glasses` | `<HatGlasses />` |
| [heart](https://lucide.dev/icons/heart) | `heart` | `<Heart />` |
| [heart-minus](https://lucide.dev/icons/heart-minus) | `heart-minus` | `<HeartMinus />` |
| [heart-off](https://lucide.dev/icons/heart-off) | `heart-off` | `<HeartOff />` |
| [heart-plus](https://lucide.dev/icons/heart-plus) | `heart-plus` | `<HeartPlus />` |
| [instagram](https://lucide.dev/icons/instagram) | `instagram` | `<Instagram />` |
| [linkedin](https://lucide.dev/icons/linkedin) | `linkedin` | `<Linkedin />` |
| [message-circle](https://lucide.dev/icons/message-circle) | `message-circle` | `<MessageCircle />` |
| [message-circle-code](https://lucide.dev/icons/message-circle-code) | `message-circle-code` | `<MessageCircleCode />` |
| [message-circle-dashed](https://lucide.dev/icons/message-circle-dashed) | `message-circle-dashed` | `<MessageCircleDashed />` |
| [message-circle-heart](https://lucide.dev/icons/message-circle-heart) | `message-circle-heart` | `<MessageCircleHeart />` |
| [message-circle-more](https://lucide.dev/icons/message-circle-more) | `message-circle-more` | `<MessageCircleMore />` |
| [message-circle-off](https://lucide.dev/icons/message-circle-off) | `message-circle-off` | `<MessageCircleOff />` |
| [message-circle-plus](https://lucide.dev/icons/message-circle-plus) | `message-circle-plus` | `<MessageCirclePlus />` |
| [message-circle-question-mark](https://lucide.dev/icons/message-circle-question-mark) | `message-circle-question-mark` | `<MessageCircleQuestionMark />` |
| [message-circle-reply](https://lucide.dev/icons/message-circle-reply) | `message-circle-reply` | `<MessageCircleReply />` |
| [message-circle-warning](https://lucide.dev/icons/message-circle-warning) | `message-circle-warning` | `<MessageCircleWarning />` |
| [message-circle-x](https://lucide.dev/icons/message-circle-x) | `message-circle-x` | `<MessageCircleX />` |
| [message-square](https://lucide.dev/icons/message-square) | `message-square` | `<MessageSquare />` |
| [message-square-code](https://lucide.dev/icons/message-square-code) | `message-square-code` | `<MessageSquareCode />` |
| [message-square-dashed](https://lucide.dev/icons/message-square-dashed) | `message-square-dashed` | `<MessageSquareDashed />` |
| [message-square-diff](https://lucide.dev/icons/message-square-diff) | `message-square-diff` | `<MessageSquareDiff />` |
| [message-square-dot](https://lucide.dev/icons/message-square-dot) | `message-square-dot` | `<MessageSquareDot />` |
| [message-square-heart](https://lucide.dev/icons/message-square-heart) | `message-square-heart` | `<MessageSquareHeart />` |
| [message-square-lock](https://lucide.dev/icons/message-square-lock) | `message-square-lock` | `<MessageSquareLock />` |
| [message-square-more](https://lucide.dev/icons/message-square-more) | `message-square-more` | `<MessageSquareMore />` |
| [message-square-off](https://lucide.dev/icons/message-square-off) | `message-square-off` | `<MessageSquareOff />` |
| [message-square-plus](https://lucide.dev/icons/message-square-plus) | `message-square-plus` | `<MessageSquarePlus />` |
| [message-square-quote](https://lucide.dev/icons/message-square-quote) | `message-square-quote` | `<MessageSquareQuote />` |
| [message-square-reply](https://lucide.dev/icons/message-square-reply) | `message-square-reply` | `<MessageSquareReply />` |
| [message-square-share](https://lucide.dev/icons/message-square-share) | `message-square-share` | `<MessageSquareShare />` |
| [message-square-text](https://lucide.dev/icons/message-square-text) | `message-square-text` | `<MessageSquareText />` |
| [message-square-warning](https://lucide.dev/icons/message-square-warning) | `message-square-warning` | `<MessageSquareWarning />` |
| [message-square-x](https://lucide.dev/icons/message-square-x) | `message-square-x` | `<MessageSquareX />` |
| [messages-square](https://lucide.dev/icons/messages-square) | `messages-square` | `<MessagesSquare />` |
| [notebook](https://lucide.dev/icons/notebook) | `notebook` | `<Notebook />` |
| [notebook-pen](https://lucide.dev/icons/notebook-pen) | `notebook-pen` | `<NotebookPen />` |
| [notebook-tabs](https://lucide.dev/icons/notebook-tabs) | `notebook-tabs` | `<NotebookTabs />` |
| [notebook-text](https://lucide.dev/icons/notebook-text) | `notebook-text` | `<NotebookText />` |
| [notepad-text](https://lucide.dev/icons/notepad-text) | `notepad-text` | `<NotepadText />` |
| [notepad-text-dashed](https://lucide.dev/icons/notepad-text-dashed) | `notepad-text-dashed` | `<NotepadTextDashed />` |
| [podcast](https://lucide.dev/icons/podcast) | `podcast` | `<Podcast />` |
| [qr-code](https://lucide.dev/icons/qr-code) | `qr-code` | `<QrCode />` |
| [radio](https://lucide.dev/icons/radio) | `radio` | `<Radio />` |
| [radio-tower](https://lucide.dev/icons/radio-tower) | `radio-tower` | `<RadioTower />` |
| [repeat-2](https://lucide.dev/icons/repeat-2) | `repeat-2` | `<Repeat2 />` |
| [ribbon](https://lucide.dev/icons/ribbon) | `ribbon` | `<Ribbon />` |
| [rose](https://lucide.dev/icons/rose) | `rose` | `<Rose />` |
| [rss](https://lucide.dev/icons/rss) | `rss` | `<Rss />` |
| [scan](https://lucide.dev/icons/scan) | `scan` | `<Scan />` |
| [scan-face](https://lucide.dev/icons/scan-face) | `scan-face` | `<ScanFace />` |
| [search](https://lucide.dev/icons/search) | `search` | `<Search />` |
| [search-check](https://lucide.dev/icons/search-check) | `search-check` | `<SearchCheck />` |
| [search-code](https://lucide.dev/icons/search-code) | `search-code` | `<SearchCode />` |
| [search-slash](https://lucide.dev/icons/search-slash) | `search-slash` | `<SearchSlash />` |
| [search-x](https://lucide.dev/icons/search-x) | `search-x` | `<SearchX />` |
| [share](https://lucide.dev/icons/share) | `share` | `<Share />` |
| [share-2](https://lucide.dev/icons/share-2) | `share-2` | `<Share2 />` |
| [slack](https://lucide.dev/icons/slack) | `slack` | `<Slack />` |
| [smile-plus](https://lucide.dev/icons/smile-plus) | `smile-plus` | `<SmilePlus />` |
| [spool](https://lucide.dev/icons/spool) | `spool` | `<Spool />` |
| [square-activity](https://lucide.dev/icons/square-activity) | `square-activity` | `<SquareActivity />` |
| [square-arrow-out-up-right](https://lucide.dev/icons/square-arrow-out-up-right) | `square-arrow-out-up-right` | `<SquareArrowOutUpRight />` |
| [square-arrow-up-right](https://lucide.dev/icons/square-arrow-up-right) | `square-arrow-up-right` | `<SquareArrowUpRight />` |
| [square-percent](https://lucide.dev/icons/square-percent) | `square-percent` | `<SquarePercent />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [star-half](https://lucide.dev/icons/star-half) | `star-half` | `<StarHalf />` |
| [star-off](https://lucide.dev/icons/star-off) | `star-off` | `<StarOff />` |
| [sticker](https://lucide.dev/icons/sticker) | `sticker` | `<Sticker />` |
| [sticky-note](https://lucide.dev/icons/sticky-note) | `sticky-note` | `<StickyNote />` |
| [theater](https://lucide.dev/icons/theater) | `theater` | `<Theater />` |
| [thumbs-down](https://lucide.dev/icons/thumbs-down) | `thumbs-down` | `<ThumbsDown />` |
| [thumbs-up](https://lucide.dev/icons/thumbs-up) | `thumbs-up` | `<ThumbsUp />` |
| [twitch](https://lucide.dev/icons/twitch) | `twitch` | `<Twitch />` |
| [twitter](https://lucide.dev/icons/twitter) | `twitter` | `<Twitter />` |
| [user-round-search](https://lucide.dev/icons/user-round-search) | `user-round-search` | `<UserRoundSearch />` |
| [user-search](https://lucide.dev/icons/user-search) | `user-search` | `<UserSearch />` |
| [voicemail](https://lucide.dev/icons/voicemail) | `voicemail` | `<Voicemail />` |
| [vote](https://lucide.dev/icons/vote) | `vote` | `<Vote />` |
| [waypoints](https://lucide.dev/icons/waypoints) | `waypoints` | `<Waypoints />` |
| [webhook](https://lucide.dev/icons/webhook) | `webhook` | `<Webhook />` |
| [webhook-off](https://lucide.dev/icons/webhook-off) | `webhook-off` | `<WebhookOff />` |
| [youtube](https://lucide.dev/icons/youtube) | `youtube` | `<Youtube />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "social"] or ["leptos", "all-icons"]
<Activity class="w-6 h-6 text-gray-600" />
```

### Sports

`sports` and `all-icons` features - 12 icons

| Icon | Name | Component |
|------|------|-----------|
| [award](https://lucide.dev/icons/award) | `award` | `<Award />` |
| [circle-gauge](https://lucide.dev/icons/circle-gauge) | `circle-gauge` | `<CircleGauge />` |
| [circle-star](https://lucide.dev/icons/circle-star) | `circle-star` | `<CircleStar />` |
| [dumbbell](https://lucide.dev/icons/dumbbell) | `dumbbell` | `<Dumbbell />` |
| [gauge](https://lucide.dev/icons/gauge) | `gauge` | `<Gauge />` |
| [hand-fist](https://lucide.dev/icons/hand-fist) | `hand-fist` | `<HandFist />` |
| [land-plot](https://lucide.dev/icons/land-plot) | `land-plot` | `<LandPlot />` |
| [medal](https://lucide.dev/icons/medal) | `medal` | `<Medal />` |
| [square-star](https://lucide.dev/icons/square-star) | `square-star` | `<SquareStar />` |
| [trophy](https://lucide.dev/icons/trophy) | `trophy` | `<Trophy />` |
| [volleyball](https://lucide.dev/icons/volleyball) | `volleyball` | `<Volleyball />` |
| [waves-ladder](https://lucide.dev/icons/waves-ladder) | `waves-ladder` | `<WavesLadder />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "sports"] or ["leptos", "all-icons"]
<Award class="w-6 h-6 text-gray-600" />
```

### Sustainability

`sustainability` and `all-icons` features - 21 icons

| Icon | Name | Component |
|------|------|-----------|
| [dam](https://lucide.dev/icons/dam) | `dam` | `<Dam />` |
| [flower](https://lucide.dev/icons/flower) | `flower` | `<Flower />` |
| [flower-2](https://lucide.dev/icons/flower-2) | `flower-2` | `<Flower2 />` |
| [house-plug](https://lucide.dev/icons/house-plug) | `house-plug` | `<HousePlug />` |
| [leaf](https://lucide.dev/icons/leaf) | `leaf` | `<Leaf />` |
| [leafy-green](https://lucide.dev/icons/leafy-green) | `leafy-green` | `<LeafyGreen />` |
| [recycle](https://lucide.dev/icons/recycle) | `recycle` | `<Recycle />` |
| [rose](https://lucide.dev/icons/rose) | `rose` | `<Rose />` |
| [sprout](https://lucide.dev/icons/sprout) | `sprout` | `<Sprout />` |
| [sun](https://lucide.dev/icons/sun) | `sun` | `<Sun />` |
| [tent](https://lucide.dev/icons/tent) | `tent` | `<Tent />` |
| [tractor](https://lucide.dev/icons/tractor) | `tractor` | `<Tractor />` |
| [tree-deciduous](https://lucide.dev/icons/tree-deciduous) | `tree-deciduous` | `<TreeDeciduous />` |
| [tree-palm](https://lucide.dev/icons/tree-palm) | `tree-palm` | `<TreePalm />` |
| [tree-pine](https://lucide.dev/icons/tree-pine) | `tree-pine` | `<TreePine />` |
| [trees](https://lucide.dev/icons/trees) | `trees` | `<Trees />` |
| [utility-pole](https://lucide.dev/icons/utility-pole) | `utility-pole` | `<UtilityPole />` |
| [vegan](https://lucide.dev/icons/vegan) | `vegan` | `<Vegan />` |
| [waves](https://lucide.dev/icons/waves) | `waves` | `<Waves />` |
| [wind](https://lucide.dev/icons/wind) | `wind` | `<Wind />` |
| [wind-arrow-down](https://lucide.dev/icons/wind-arrow-down) | `wind-arrow-down` | `<WindArrowDown />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "sustainability"] or ["leptos", "all-icons"]
<Dam class="w-6 h-6 text-gray-600" />
```

### Text

`text` and `all-icons` features - 244 icons

| Icon | Name | Component |
|------|------|-----------|
| [a-arrow-down](https://lucide.dev/icons/a-arrow-down) | `a-arrow-down` | `<AArrowDown />` |
| [a-arrow-up](https://lucide.dev/icons/a-arrow-up) | `a-arrow-up` | `<AArrowUp />` |
| [a-large-small](https://lucide.dev/icons/a-large-small) | `a-large-small` | `<ALargeSmall />` |
| [ampersand](https://lucide.dev/icons/ampersand) | `ampersand` | `<Ampersand />` |
| [ampersands](https://lucide.dev/icons/ampersands) | `ampersands` | `<Ampersands />` |
| [anchor](https://lucide.dev/icons/anchor) | `anchor` | `<Anchor />` |
| [arrow-big-up](https://lucide.dev/icons/arrow-big-up) | `arrow-big-up` | `<ArrowBigUp />` |
| [arrow-big-up-dash](https://lucide.dev/icons/arrow-big-up-dash) | `arrow-big-up-dash` | `<ArrowBigUpDash />` |
| [arrow-down-0-1](https://lucide.dev/icons/arrow-down-0-1) | `arrow-down-0-1` | `<ArrowDown01 />` |
| [arrow-down-1-0](https://lucide.dev/icons/arrow-down-1-0) | `arrow-down-1-0` | `<ArrowDown10 />` |
| [arrow-down-a-z](https://lucide.dev/icons/arrow-down-a-z) | `arrow-down-a-z` | `<ArrowDownAZ />` |
| [arrow-down-narrow-wide](https://lucide.dev/icons/arrow-down-narrow-wide) | `arrow-down-narrow-wide` | `<ArrowDownNarrowWide />` |
| [arrow-down-wide-narrow](https://lucide.dev/icons/arrow-down-wide-narrow) | `arrow-down-wide-narrow` | `<ArrowDownWideNarrow />` |
| [arrow-down-z-a](https://lucide.dev/icons/arrow-down-z-a) | `arrow-down-z-a` | `<ArrowDownZA />` |
| [arrow-up-0-1](https://lucide.dev/icons/arrow-up-0-1) | `arrow-up-0-1` | `<ArrowUp01 />` |
| [arrow-up-1-0](https://lucide.dev/icons/arrow-up-1-0) | `arrow-up-1-0` | `<ArrowUp10 />` |
| [arrow-up-a-z](https://lucide.dev/icons/arrow-up-a-z) | `arrow-up-a-z` | `<ArrowUpAZ />` |
| [arrow-up-narrow-wide](https://lucide.dev/icons/arrow-up-narrow-wide) | `arrow-up-narrow-wide` | `<ArrowUpNarrowWide />` |
| [arrow-up-wide-narrow](https://lucide.dev/icons/arrow-up-wide-narrow) | `arrow-up-wide-narrow` | `<ArrowUpWideNarrow />` |
| [arrow-up-z-a](https://lucide.dev/icons/arrow-up-z-a) | `arrow-up-z-a` | `<ArrowUpZA />` |
| [asterisk](https://lucide.dev/icons/asterisk) | `asterisk` | `<Asterisk />` |
| [at-sign](https://lucide.dev/icons/at-sign) | `at-sign` | `<AtSign />` |
| [baseline](https://lucide.dev/icons/baseline) | `baseline` | `<Baseline />` |
| [binary](https://lucide.dev/icons/binary) | `binary` | `<Binary />` |
| [bold](https://lucide.dev/icons/bold) | `bold` | `<Bold />` |
| [book](https://lucide.dev/icons/book) | `book` | `<Book />` |
| [book-a](https://lucide.dev/icons/book-a) | `book-a` | `<BookA />` |
| [book-alert](https://lucide.dev/icons/book-alert) | `book-alert` | `<BookAlert />` |
| [book-audio](https://lucide.dev/icons/book-audio) | `book-audio` | `<BookAudio />` |
| [book-check](https://lucide.dev/icons/book-check) | `book-check` | `<BookCheck />` |
| [book-copy](https://lucide.dev/icons/book-copy) | `book-copy` | `<BookCopy />` |
| [book-headphones](https://lucide.dev/icons/book-headphones) | `book-headphones` | `<BookHeadphones />` |
| [book-heart](https://lucide.dev/icons/book-heart) | `book-heart` | `<BookHeart />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [book-marked](https://lucide.dev/icons/book-marked) | `book-marked` | `<BookMarked />` |
| [book-minus](https://lucide.dev/icons/book-minus) | `book-minus` | `<BookMinus />` |
| [book-open](https://lucide.dev/icons/book-open) | `book-open` | `<BookOpen />` |
| [book-open-check](https://lucide.dev/icons/book-open-check) | `book-open-check` | `<BookOpenCheck />` |
| [book-open-text](https://lucide.dev/icons/book-open-text) | `book-open-text` | `<BookOpenText />` |
| [book-plus](https://lucide.dev/icons/book-plus) | `book-plus` | `<BookPlus />` |
| [book-text](https://lucide.dev/icons/book-text) | `book-text` | `<BookText />` |
| [book-type](https://lucide.dev/icons/book-type) | `book-type` | `<BookType />` |
| [book-x](https://lucide.dev/icons/book-x) | `book-x` | `<BookX />` |
| [brush](https://lucide.dev/icons/brush) | `brush` | `<Brush />` |
| [case-lower](https://lucide.dev/icons/case-lower) | `case-lower` | `<CaseLower />` |
| [case-sensitive](https://lucide.dev/icons/case-sensitive) | `case-sensitive` | `<CaseSensitive />` |
| [case-upper](https://lucide.dev/icons/case-upper) | `case-upper` | `<CaseUpper />` |
| [circle-question-mark](https://lucide.dev/icons/circle-question-mark) | `circle-question-mark` | `<CircleQuestionMark />` |
| [clipboard](https://lucide.dev/icons/clipboard) | `clipboard` | `<Clipboard />` |
| [clipboard-check](https://lucide.dev/icons/clipboard-check) | `clipboard-check` | `<ClipboardCheck />` |
| [clipboard-clock](https://lucide.dev/icons/clipboard-clock) | `clipboard-clock` | `<ClipboardClock />` |
| [clipboard-copy](https://lucide.dev/icons/clipboard-copy) | `clipboard-copy` | `<ClipboardCopy />` |
| [clipboard-list](https://lucide.dev/icons/clipboard-list) | `clipboard-list` | `<ClipboardList />` |
| [clipboard-minus](https://lucide.dev/icons/clipboard-minus) | `clipboard-minus` | `<ClipboardMinus />` |
| [clipboard-paste](https://lucide.dev/icons/clipboard-paste) | `clipboard-paste` | `<ClipboardPaste />` |
| [clipboard-pen](https://lucide.dev/icons/clipboard-pen) | `clipboard-pen` | `<ClipboardPen />` |
| [clipboard-pen-line](https://lucide.dev/icons/clipboard-pen-line) | `clipboard-pen-line` | `<ClipboardPenLine />` |
| [clipboard-plus](https://lucide.dev/icons/clipboard-plus) | `clipboard-plus` | `<ClipboardPlus />` |
| [clipboard-type](https://lucide.dev/icons/clipboard-type) | `clipboard-type` | `<ClipboardType />` |
| [clipboard-x](https://lucide.dev/icons/clipboard-x) | `clipboard-x` | `<ClipboardX />` |
| [code](https://lucide.dev/icons/code) | `code` | `<Code />` |
| [code-xml](https://lucide.dev/icons/code-xml) | `code-xml` | `<CodeXml />` |
| [columns-2](https://lucide.dev/icons/columns-2) | `columns-2` | `<Columns2 />` |
| [columns-3](https://lucide.dev/icons/columns-3) | `columns-3` | `<Columns3 />` |
| [columns-4](https://lucide.dev/icons/columns-4) | `columns-4` | `<Columns4 />` |
| [copy](https://lucide.dev/icons/copy) | `copy` | `<Copy />` |
| [copy-check](https://lucide.dev/icons/copy-check) | `copy-check` | `<CopyCheck />` |
| [copy-minus](https://lucide.dev/icons/copy-minus) | `copy-minus` | `<CopyMinus />` |
| [copy-plus](https://lucide.dev/icons/copy-plus) | `copy-plus` | `<CopyPlus />` |
| [copy-slash](https://lucide.dev/icons/copy-slash) | `copy-slash` | `<CopySlash />` |
| [copyleft](https://lucide.dev/icons/copyleft) | `copyleft` | `<Copyleft />` |
| [copyright](https://lucide.dev/icons/copyright) | `copyright` | `<Copyright />` |
| [corner-down-right](https://lucide.dev/icons/corner-down-right) | `corner-down-right` | `<CornerDownRight />` |
| [creative-commons](https://lucide.dev/icons/creative-commons) | `creative-commons` | `<CreativeCommons />` |
| [decimals-arrow-left](https://lucide.dev/icons/decimals-arrow-left) | `decimals-arrow-left` | `<DecimalsArrowLeft />` |
| [decimals-arrow-right](https://lucide.dev/icons/decimals-arrow-right) | `decimals-arrow-right` | `<DecimalsArrowRight />` |
| [delete](https://lucide.dev/icons/delete) | `delete` | `<Delete />` |
| [dot](https://lucide.dev/icons/dot) | `dot` | `<Dot />` |
| [eraser](https://lucide.dev/icons/eraser) | `eraser` | `<Eraser />` |
| [expand](https://lucide.dev/icons/expand) | `expand` | `<Expand />` |
| [external-link](https://lucide.dev/icons/external-link) | `external-link` | `<ExternalLink />` |
| [file-text](https://lucide.dev/icons/file-text) | `file-text` | `<FileText />` |
| [file-type](https://lucide.dev/icons/file-type) | `file-type` | `<FileType />` |
| [file-type-2](https://lucide.dev/icons/file-type-2) | `file-type-2` | `<FileType2 />` |
| [grid-2x2](https://lucide.dev/icons/grid-2x2) | `grid-2x2` | `<Grid2X2 />` |
| [grid-2x2-check](https://lucide.dev/icons/grid-2x2-check) | `grid-2x2-check` | `<Grid2X2Check />` |
| [grid-2x2-plus](https://lucide.dev/icons/grid-2x2-plus) | `grid-2x2-plus` | `<Grid2X2Plus />` |
| [grid-2x2-x](https://lucide.dev/icons/grid-2x2-x) | `grid-2x2-x` | `<Grid2X2X />` |
| [grid-3x2](https://lucide.dev/icons/grid-3x2) | `grid-3x2` | `<Grid3X2 />` |
| [grid-3x3](https://lucide.dev/icons/grid-3x3) | `grid-3x3` | `<Grid3X3 />` |
| [hash](https://lucide.dev/icons/hash) | `hash` | `<Hash />` |
| [heading](https://lucide.dev/icons/heading) | `heading` | `<Heading />` |
| [heading-1](https://lucide.dev/icons/heading-1) | `heading-1` | `<Heading1 />` |
| [heading-2](https://lucide.dev/icons/heading-2) | `heading-2` | `<Heading2 />` |
| [heading-3](https://lucide.dev/icons/heading-3) | `heading-3` | `<Heading3 />` |
| [heading-4](https://lucide.dev/icons/heading-4) | `heading-4` | `<Heading4 />` |
| [heading-5](https://lucide.dev/icons/heading-5) | `heading-5` | `<Heading5 />` |
| [heading-6](https://lucide.dev/icons/heading-6) | `heading-6` | `<Heading6 />` |
| [highlighter](https://lucide.dev/icons/highlighter) | `highlighter` | `<Highlighter />` |
| [image](https://lucide.dev/icons/image) | `image` | `<Image />` |
| [image-down](https://lucide.dev/icons/image-down) | `image-down` | `<ImageDown />` |
| [image-play](https://lucide.dev/icons/image-play) | `image-play` | `<ImagePlay />` |
| [image-up](https://lucide.dev/icons/image-up) | `image-up` | `<ImageUp />` |
| [images](https://lucide.dev/icons/images) | `images` | `<Images />` |
| [italic](https://lucide.dev/icons/italic) | `italic` | `<Italic />` |
| [keyboard](https://lucide.dev/icons/keyboard) | `keyboard` | `<Keyboard />` |
| [keyboard-off](https://lucide.dev/icons/keyboard-off) | `keyboard-off` | `<KeyboardOff />` |
| [languages](https://lucide.dev/icons/languages) | `languages` | `<Languages />` |
| [layout-list](https://lucide.dev/icons/layout-list) | `layout-list` | `<LayoutList />` |
| [library](https://lucide.dev/icons/library) | `library` | `<Library />` |
| [library-big](https://lucide.dev/icons/library-big) | `library-big` | `<LibraryBig />` |
| [ligature](https://lucide.dev/icons/ligature) | `ligature` | `<Ligature />` |
| [link](https://lucide.dev/icons/link) | `link` | `<Link />` |
| [link-2](https://lucide.dev/icons/link-2) | `link-2` | `<Link2 />` |
| [link-2-off](https://lucide.dev/icons/link-2-off) | `link-2-off` | `<Link2Off />` |
| [list](https://lucide.dev/icons/list) | `list` | `<List />` |
| [list-check](https://lucide.dev/icons/list-check) | `list-check` | `<ListCheck />` |
| [list-checks](https://lucide.dev/icons/list-checks) | `list-checks` | `<ListChecks />` |
| [list-chevrons-down-up](https://lucide.dev/icons/list-chevrons-down-up) | `list-chevrons-down-up` | `<ListChevronsDownUp />` |
| [list-chevrons-up-down](https://lucide.dev/icons/list-chevrons-up-down) | `list-chevrons-up-down` | `<ListChevronsUpDown />` |
| [list-collapse](https://lucide.dev/icons/list-collapse) | `list-collapse` | `<ListCollapse />` |
| [list-end](https://lucide.dev/icons/list-end) | `list-end` | `<ListEnd />` |
| [list-filter](https://lucide.dev/icons/list-filter) | `list-filter` | `<ListFilter />` |
| [list-filter-plus](https://lucide.dev/icons/list-filter-plus) | `list-filter-plus` | `<ListFilterPlus />` |
| [list-indent-decrease](https://lucide.dev/icons/list-indent-decrease) | `list-indent-decrease` | `<ListIndentDecrease />` |
| [list-indent-increase](https://lucide.dev/icons/list-indent-increase) | `list-indent-increase` | `<ListIndentIncrease />` |
| [list-minus](https://lucide.dev/icons/list-minus) | `list-minus` | `<ListMinus />` |
| [list-ordered](https://lucide.dev/icons/list-ordered) | `list-ordered` | `<ListOrdered />` |
| [list-plus](https://lucide.dev/icons/list-plus) | `list-plus` | `<ListPlus />` |
| [list-restart](https://lucide.dev/icons/list-restart) | `list-restart` | `<ListRestart />` |
| [list-start](https://lucide.dev/icons/list-start) | `list-start` | `<ListStart />` |
| [list-todo](https://lucide.dev/icons/list-todo) | `list-todo` | `<ListTodo />` |
| [list-tree](https://lucide.dev/icons/list-tree) | `list-tree` | `<ListTree />` |
| [list-x](https://lucide.dev/icons/list-x) | `list-x` | `<ListX />` |
| [logs](https://lucide.dev/icons/logs) | `logs` | `<Logs />` |
| [mail](https://lucide.dev/icons/mail) | `mail` | `<Mail />` |
| [map](https://lucide.dev/icons/map) | `map` | `<Map />` |
| [message-square-quote](https://lucide.dev/icons/message-square-quote) | `message-square-quote` | `<MessageSquareQuote />` |
| [minus](https://lucide.dev/icons/minus) | `minus` | `<Minus />` |
| [notebook](https://lucide.dev/icons/notebook) | `notebook` | `<Notebook />` |
| [notebook-pen](https://lucide.dev/icons/notebook-pen) | `notebook-pen` | `<NotebookPen />` |
| [notebook-text](https://lucide.dev/icons/notebook-text) | `notebook-text` | `<NotebookText />` |
| [notepad-text](https://lucide.dev/icons/notepad-text) | `notepad-text` | `<NotepadText />` |
| [notepad-text-dashed](https://lucide.dev/icons/notepad-text-dashed) | `notepad-text-dashed` | `<NotepadTextDashed />` |
| [omega](https://lucide.dev/icons/omega) | `omega` | `<Omega />` |
| [paint-roller](https://lucide.dev/icons/paint-roller) | `paint-roller` | `<PaintRoller />` |
| [paintbrush](https://lucide.dev/icons/paintbrush) | `paintbrush` | `<Paintbrush />` |
| [paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) | `paintbrush-vertical` | `<PaintbrushVertical />` |
| [palette](https://lucide.dev/icons/palette) | `palette` | `<Palette />` |
| [paperclip](https://lucide.dev/icons/paperclip) | `paperclip` | `<Paperclip />` |
| [pen](https://lucide.dev/icons/pen) | `pen` | `<Pen />` |
| [pen-line](https://lucide.dev/icons/pen-line) | `pen-line` | `<PenLine />` |
| [pen-off](https://lucide.dev/icons/pen-off) | `pen-off` | `<PenOff />` |
| [pen-tool](https://lucide.dev/icons/pen-tool) | `pen-tool` | `<PenTool />` |
| [pencil](https://lucide.dev/icons/pencil) | `pencil` | `<Pencil />` |
| [pencil-line](https://lucide.dev/icons/pencil-line) | `pencil-line` | `<PencilLine />` |
| [pencil-off](https://lucide.dev/icons/pencil-off) | `pencil-off` | `<PencilOff />` |
| [pencil-ruler](https://lucide.dev/icons/pencil-ruler) | `pencil-ruler` | `<PencilRuler />` |
| [phone](https://lucide.dev/icons/phone) | `phone` | `<Phone />` |
| [pilcrow](https://lucide.dev/icons/pilcrow) | `pilcrow` | `<Pilcrow />` |
| [pilcrow-left](https://lucide.dev/icons/pilcrow-left) | `pilcrow-left` | `<PilcrowLeft />` |
| [pilcrow-right](https://lucide.dev/icons/pilcrow-right) | `pilcrow-right` | `<PilcrowRight />` |
| [pipette](https://lucide.dev/icons/pipette) | `pipette` | `<Pipette />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [quote](https://lucide.dev/icons/quote) | `quote` | `<Quote />` |
| [rectangle-circle](https://lucide.dev/icons/rectangle-circle) | `rectangle-circle` | `<RectangleCircle />` |
| [rectangle-ellipsis](https://lucide.dev/icons/rectangle-ellipsis) | `rectangle-ellipsis` | `<RectangleEllipsis />` |
| [redo](https://lucide.dev/icons/redo) | `redo` | `<Redo />` |
| [redo-2](https://lucide.dev/icons/redo-2) | `redo-2` | `<Redo2 />` |
| [redo-dot](https://lucide.dev/icons/redo-dot) | `redo-dot` | `<RedoDot />` |
| [regex](https://lucide.dev/icons/regex) | `regex` | `<Regex />` |
| [remove-formatting](https://lucide.dev/icons/remove-formatting) | `remove-formatting` | `<RemoveFormatting />` |
| [replace](https://lucide.dev/icons/replace) | `replace` | `<Replace />` |
| [replace-all](https://lucide.dev/icons/replace-all) | `replace-all` | `<ReplaceAll />` |
| [rows-2](https://lucide.dev/icons/rows-2) | `rows-2` | `<Rows2 />` |
| [rows-3](https://lucide.dev/icons/rows-3) | `rows-3` | `<Rows3 />` |
| [rows-4](https://lucide.dev/icons/rows-4) | `rows-4` | `<Rows4 />` |
| [save](https://lucide.dev/icons/save) | `save` | `<Save />` |
| [save-all](https://lucide.dev/icons/save-all) | `save-all` | `<SaveAll />` |
| [save-off](https://lucide.dev/icons/save-off) | `save-off` | `<SaveOff />` |
| [scan-text](https://lucide.dev/icons/scan-text) | `scan-text` | `<ScanText />` |
| [scissors](https://lucide.dev/icons/scissors) | `scissors` | `<Scissors />` |
| [scroll](https://lucide.dev/icons/scroll) | `scroll` | `<Scroll />` |
| [scroll-text](https://lucide.dev/icons/scroll-text) | `scroll-text` | `<ScrollText />` |
| [search](https://lucide.dev/icons/search) | `search` | `<Search />` |
| [search-check](https://lucide.dev/icons/search-check) | `search-check` | `<SearchCheck />` |
| [search-code](https://lucide.dev/icons/search-code) | `search-code` | `<SearchCode />` |
| [search-slash](https://lucide.dev/icons/search-slash) | `search-slash` | `<SearchSlash />` |
| [search-x](https://lucide.dev/icons/search-x) | `search-x` | `<SearchX />` |
| [section](https://lucide.dev/icons/section) | `section` | `<Section />` |
| [separator-horizontal](https://lucide.dev/icons/separator-horizontal) | `separator-horizontal` | `<SeparatorHorizontal />` |
| [separator-vertical](https://lucide.dev/icons/separator-vertical) | `separator-vertical` | `<SeparatorVertical />` |
| [sheet](https://lucide.dev/icons/sheet) | `sheet` | `<Sheet />` |
| [sigma](https://lucide.dev/icons/sigma) | `sigma` | `<Sigma />` |
| [signature](https://lucide.dev/icons/signature) | `signature` | `<Signature />` |
| [space](https://lucide.dev/icons/space) | `space` | `<Space />` |
| [spell-check](https://lucide.dev/icons/spell-check) | `spell-check` | `<SpellCheck />` |
| [spell-check-2](https://lucide.dev/icons/spell-check-2) | `spell-check-2` | `<SpellCheck2 />` |
| [square-asterisk](https://lucide.dev/icons/square-asterisk) | `square-asterisk` | `<SquareAsterisk />` |
| [square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) | `square-bottom-dashed-scissors` | `<SquareBottomDashedScissors />` |
| [square-code](https://lucide.dev/icons/square-code) | `square-code` | `<SquareCode />` |
| [square-dashed](https://lucide.dev/icons/square-dashed) | `square-dashed` | `<SquareDashed />` |
| [square-library](https://lucide.dev/icons/square-library) | `square-library` | `<SquareLibrary />` |
| [square-minus](https://lucide.dev/icons/square-minus) | `square-minus` | `<SquareMinus />` |
| [square-pen](https://lucide.dev/icons/square-pen) | `square-pen` | `<SquarePen />` |
| [square-pilcrow](https://lucide.dev/icons/square-pilcrow) | `square-pilcrow` | `<SquarePilcrow />` |
| [square-plus](https://lucide.dev/icons/square-plus) | `square-plus` | `<SquarePlus />` |
| [square-scissors](https://lucide.dev/icons/square-scissors) | `square-scissors` | `<SquareScissors />` |
| [square-sigma](https://lucide.dev/icons/square-sigma) | `square-sigma` | `<SquareSigma />` |
| [square-stack](https://lucide.dev/icons/square-stack) | `square-stack` | `<SquareStack />` |
| [sticky-note](https://lucide.dev/icons/sticky-note) | `sticky-note` | `<StickyNote />` |
| [strikethrough](https://lucide.dev/icons/strikethrough) | `strikethrough` | `<Strikethrough />` |
| [subscript](https://lucide.dev/icons/subscript) | `subscript` | `<Subscript />` |
| [superscript](https://lucide.dev/icons/superscript) | `superscript` | `<Superscript />` |
| [table](https://lucide.dev/icons/table) | `table` | `<Table />` |
| [table-2](https://lucide.dev/icons/table-2) | `table-2` | `<Table2 />` |
| [table-cells-merge](https://lucide.dev/icons/table-cells-merge) | `table-cells-merge` | `<TableCellsMerge />` |
| [table-cells-split](https://lucide.dev/icons/table-cells-split) | `table-cells-split` | `<TableCellsSplit />` |
| [table-columns-split](https://lucide.dev/icons/table-columns-split) | `table-columns-split` | `<TableColumnsSplit />` |
| [table-of-contents](https://lucide.dev/icons/table-of-contents) | `table-of-contents` | `<TableOfContents />` |
| [table-properties](https://lucide.dev/icons/table-properties) | `table-properties` | `<TableProperties />` |
| [table-rows-split](https://lucide.dev/icons/table-rows-split) | `table-rows-split` | `<TableRowsSplit />` |
| [text-align-center](https://lucide.dev/icons/text-align-center) | `text-align-center` | `<TextAlignCenter />` |
| [text-align-end](https://lucide.dev/icons/text-align-end) | `text-align-end` | `<TextAlignEnd />` |
| [text-align-justify](https://lucide.dev/icons/text-align-justify) | `text-align-justify` | `<TextAlignJustify />` |
| [text-align-start](https://lucide.dev/icons/text-align-start) | `text-align-start` | `<TextAlignStart />` |
| [text-cursor](https://lucide.dev/icons/text-cursor) | `text-cursor` | `<TextCursor />` |
| [text-cursor-input](https://lucide.dev/icons/text-cursor-input) | `text-cursor-input` | `<TextCursorInput />` |
| [text-initial](https://lucide.dev/icons/text-initial) | `text-initial` | `<TextInitial />` |
| [text-quote](https://lucide.dev/icons/text-quote) | `text-quote` | `<TextQuote />` |
| [text-search](https://lucide.dev/icons/text-search) | `text-search` | `<TextSearch />` |
| [text-select](https://lucide.dev/icons/text-select) | `text-select` | `<TextSelect />` |
| [text-wrap](https://lucide.dev/icons/text-wrap) | `text-wrap` | `<TextWrap />` |
| [type](https://lucide.dev/icons/type) | `type` | `<Type />` |
| [type-outline](https://lucide.dev/icons/type-outline) | `type-outline` | `<TypeOutline />` |
| [underline](https://lucide.dev/icons/underline) | `underline` | `<Underline />` |
| [undo](https://lucide.dev/icons/undo) | `undo` | `<Undo />` |
| [undo-2](https://lucide.dev/icons/undo-2) | `undo-2` | `<Undo2 />` |
| [undo-dot](https://lucide.dev/icons/undo-dot) | `undo-dot` | `<UndoDot />` |
| [unlink](https://lucide.dev/icons/unlink) | `unlink` | `<Unlink />` |
| [unlink-2](https://lucide.dev/icons/unlink-2) | `unlink-2` | `<Unlink2 />` |
| [whole-word](https://lucide.dev/icons/whole-word) | `whole-word` | `<WholeWord />` |
| [zoom-in](https://lucide.dev/icons/zoom-in) | `zoom-in` | `<ZoomIn />` |
| [zoom-out](https://lucide.dev/icons/zoom-out) | `zoom-out` | `<ZoomOut />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "text"] or ["leptos", "all-icons"]
<AArrowDown class="w-6 h-6 text-gray-600" />
```

### Time

`time` and `all-icons` features - 56 icons

| Icon | Name | Component |
|------|------|-----------|
| [alarm-clock](https://lucide.dev/icons/alarm-clock) | `alarm-clock` | `<AlarmClock />` |
| [alarm-clock-check](https://lucide.dev/icons/alarm-clock-check) | `alarm-clock-check` | `<AlarmClockCheck />` |
| [alarm-clock-minus](https://lucide.dev/icons/alarm-clock-minus) | `alarm-clock-minus` | `<AlarmClockMinus />` |
| [alarm-clock-off](https://lucide.dev/icons/alarm-clock-off) | `alarm-clock-off` | `<AlarmClockOff />` |
| [alarm-clock-plus](https://lucide.dev/icons/alarm-clock-plus) | `alarm-clock-plus` | `<AlarmClockPlus />` |
| [calendar](https://lucide.dev/icons/calendar) | `calendar` | `<Calendar />` |
| [calendar-1](https://lucide.dev/icons/calendar-1) | `calendar-1` | `<Calendar1 />` |
| [calendar-arrow-down](https://lucide.dev/icons/calendar-arrow-down) | `calendar-arrow-down` | `<CalendarArrowDown />` |
| [calendar-arrow-up](https://lucide.dev/icons/calendar-arrow-up) | `calendar-arrow-up` | `<CalendarArrowUp />` |
| [calendar-check](https://lucide.dev/icons/calendar-check) | `calendar-check` | `<CalendarCheck />` |
| [calendar-check-2](https://lucide.dev/icons/calendar-check-2) | `calendar-check-2` | `<CalendarCheck2 />` |
| [calendar-clock](https://lucide.dev/icons/calendar-clock) | `calendar-clock` | `<CalendarClock />` |
| [calendar-cog](https://lucide.dev/icons/calendar-cog) | `calendar-cog` | `<CalendarCog />` |
| [calendar-days](https://lucide.dev/icons/calendar-days) | `calendar-days` | `<CalendarDays />` |
| [calendar-fold](https://lucide.dev/icons/calendar-fold) | `calendar-fold` | `<CalendarFold />` |
| [calendar-heart](https://lucide.dev/icons/calendar-heart) | `calendar-heart` | `<CalendarHeart />` |
| [calendar-minus](https://lucide.dev/icons/calendar-minus) | `calendar-minus` | `<CalendarMinus />` |
| [calendar-minus-2](https://lucide.dev/icons/calendar-minus-2) | `calendar-minus-2` | `<CalendarMinus2 />` |
| [calendar-off](https://lucide.dev/icons/calendar-off) | `calendar-off` | `<CalendarOff />` |
| [calendar-plus](https://lucide.dev/icons/calendar-plus) | `calendar-plus` | `<CalendarPlus />` |
| [calendar-plus-2](https://lucide.dev/icons/calendar-plus-2) | `calendar-plus-2` | `<CalendarPlus2 />` |
| [calendar-range](https://lucide.dev/icons/calendar-range) | `calendar-range` | `<CalendarRange />` |
| [calendar-search](https://lucide.dev/icons/calendar-search) | `calendar-search` | `<CalendarSearch />` |
| [calendar-sync](https://lucide.dev/icons/calendar-sync) | `calendar-sync` | `<CalendarSync />` |
| [calendar-x](https://lucide.dev/icons/calendar-x) | `calendar-x` | `<CalendarX />` |
| [calendar-x-2](https://lucide.dev/icons/calendar-x-2) | `calendar-x-2` | `<CalendarX2 />` |
| [chart-no-axes-gantt](https://lucide.dev/icons/chart-no-axes-gantt) | `chart-no-axes-gantt` | `<ChartNoAxesGantt />` |
| [clipboard-clock](https://lucide.dev/icons/clipboard-clock) | `clipboard-clock` | `<ClipboardClock />` |
| [clock](https://lucide.dev/icons/clock) | `clock` | `<Clock />` |
| [clock-1](https://lucide.dev/icons/clock-1) | `clock-1` | `<Clock1 />` |
| [clock-10](https://lucide.dev/icons/clock-10) | `clock-10` | `<Clock10 />` |
| [clock-11](https://lucide.dev/icons/clock-11) | `clock-11` | `<Clock11 />` |
| [clock-12](https://lucide.dev/icons/clock-12) | `clock-12` | `<Clock12 />` |
| [clock-2](https://lucide.dev/icons/clock-2) | `clock-2` | `<Clock2 />` |
| [clock-3](https://lucide.dev/icons/clock-3) | `clock-3` | `<Clock3 />` |
| [clock-4](https://lucide.dev/icons/clock-4) | `clock-4` | `<Clock4 />` |
| [clock-5](https://lucide.dev/icons/clock-5) | `clock-5` | `<Clock5 />` |
| [clock-6](https://lucide.dev/icons/clock-6) | `clock-6` | `<Clock6 />` |
| [clock-7](https://lucide.dev/icons/clock-7) | `clock-7` | `<Clock7 />` |
| [clock-8](https://lucide.dev/icons/clock-8) | `clock-8` | `<Clock8 />` |
| [clock-9](https://lucide.dev/icons/clock-9) | `clock-9` | `<Clock9 />` |
| [clock-alert](https://lucide.dev/icons/clock-alert) | `clock-alert` | `<ClockAlert />` |
| [clock-arrow-down](https://lucide.dev/icons/clock-arrow-down) | `clock-arrow-down` | `<ClockArrowDown />` |
| [clock-arrow-up](https://lucide.dev/icons/clock-arrow-up) | `clock-arrow-up` | `<ClockArrowUp />` |
| [clock-fading](https://lucide.dev/icons/clock-fading) | `clock-fading` | `<ClockFading />` |
| [clock-plus](https://lucide.dev/icons/clock-plus) | `clock-plus` | `<ClockPlus />` |
| [file-clock](https://lucide.dev/icons/file-clock) | `file-clock` | `<FileClock />` |
| [folder-clock](https://lucide.dev/icons/folder-clock) | `folder-clock` | `<FolderClock />` |
| [history](https://lucide.dev/icons/history) | `history` | `<History />` |
| [hourglass](https://lucide.dev/icons/hourglass) | `hourglass` | `<Hourglass />` |
| [square-chart-gantt](https://lucide.dev/icons/square-chart-gantt) | `square-chart-gantt` | `<SquareChartGantt />` |
| [sunrise](https://lucide.dev/icons/sunrise) | `sunrise` | `<Sunrise />` |
| [timer](https://lucide.dev/icons/timer) | `timer` | `<Timer />` |
| [timer-off](https://lucide.dev/icons/timer-off) | `timer-off` | `<TimerOff />` |
| [timer-reset](https://lucide.dev/icons/timer-reset) | `timer-reset` | `<TimerReset />` |
| [watch](https://lucide.dev/icons/watch) | `watch` | `<Watch />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "time"] or ["leptos", "all-icons"]
<AlarmClock class="w-6 h-6 text-gray-600" />
```

### Tools

`tools` and `all-icons` features - 65 icons

| Icon | Name | Component |
|------|------|-----------|
| [anvil](https://lucide.dev/icons/anvil) | `anvil` | `<Anvil />` |
| [axe](https://lucide.dev/icons/axe) | `axe` | `<Axe />` |
| [between-horizontal-end](https://lucide.dev/icons/between-horizontal-end) | `between-horizontal-end` | `<BetweenHorizontalEnd />` |
| [between-horizontal-start](https://lucide.dev/icons/between-horizontal-start) | `between-horizontal-start` | `<BetweenHorizontalStart />` |
| [between-vertical-end](https://lucide.dev/icons/between-vertical-end) | `between-vertical-end` | `<BetweenVerticalEnd />` |
| [between-vertical-start](https://lucide.dev/icons/between-vertical-start) | `between-vertical-start` | `<BetweenVerticalStart />` |
| [blend](https://lucide.dev/icons/blend) | `blend` | `<Blend />` |
| [bolt](https://lucide.dev/icons/bolt) | `bolt` | `<Bolt />` |
| [bomb](https://lucide.dev/icons/bomb) | `bomb` | `<Bomb />` |
| [bow-arrow](https://lucide.dev/icons/bow-arrow) | `bow-arrow` | `<BowArrow />` |
| [brush](https://lucide.dev/icons/brush) | `brush` | `<Brush />` |
| [brush-cleaning](https://lucide.dev/icons/brush-cleaning) | `brush-cleaning` | `<BrushCleaning />` |
| [diameter](https://lucide.dev/icons/diameter) | `diameter` | `<Diameter />` |
| [diamond-minus](https://lucide.dev/icons/diamond-minus) | `diamond-minus` | `<DiamondMinus />` |
| [diamond-plus](https://lucide.dev/icons/diamond-plus) | `diamond-plus` | `<DiamondPlus />` |
| [drafting-compass](https://lucide.dev/icons/drafting-compass) | `drafting-compass` | `<DraftingCompass />` |
| [drill](https://lucide.dev/icons/drill) | `drill` | `<Drill />` |
| [fire-extinguisher](https://lucide.dev/icons/fire-extinguisher) | `fire-extinguisher` | `<FireExtinguisher />` |
| [gavel](https://lucide.dev/icons/gavel) | `gavel` | `<Gavel />` |
| [hammer](https://lucide.dev/icons/hammer) | `hammer` | `<Hammer />` |
| [hard-hat](https://lucide.dev/icons/hard-hat) | `hard-hat` | `<HardHat />` |
| [inspection-panel](https://lucide.dev/icons/inspection-panel) | `inspection-panel` | `<InspectionPanel />` |
| [land-plot](https://lucide.dev/icons/land-plot) | `land-plot` | `<LandPlot />` |
| [minus](https://lucide.dev/icons/minus) | `minus` | `<Minus />` |
| [paint-bucket](https://lucide.dev/icons/paint-bucket) | `paint-bucket` | `<PaintBucket />` |
| [paint-roller](https://lucide.dev/icons/paint-roller) | `paint-roller` | `<PaintRoller />` |
| [paintbrush](https://lucide.dev/icons/paintbrush) | `paintbrush` | `<Paintbrush />` |
| [paintbrush-vertical](https://lucide.dev/icons/paintbrush-vertical) | `paintbrush-vertical` | `<PaintbrushVertical />` |
| [pen](https://lucide.dev/icons/pen) | `pen` | `<Pen />` |
| [pen-line](https://lucide.dev/icons/pen-line) | `pen-line` | `<PenLine />` |
| [pen-off](https://lucide.dev/icons/pen-off) | `pen-off` | `<PenOff />` |
| [pencil](https://lucide.dev/icons/pencil) | `pencil` | `<Pencil />` |
| [pencil-line](https://lucide.dev/icons/pencil-line) | `pencil-line` | `<PencilLine />` |
| [pencil-off](https://lucide.dev/icons/pencil-off) | `pencil-off` | `<PencilOff />` |
| [pencil-ruler](https://lucide.dev/icons/pencil-ruler) | `pencil-ruler` | `<PencilRuler />` |
| [pickaxe](https://lucide.dev/icons/pickaxe) | `pickaxe` | `<Pickaxe />` |
| [plus](https://lucide.dev/icons/plus) | `plus` | `<Plus />` |
| [pocket-knife](https://lucide.dev/icons/pocket-knife) | `pocket-knife` | `<PocketKnife />` |
| [radius](https://lucide.dev/icons/radius) | `radius` | `<Radius />` |
| [rotate-ccw-square](https://lucide.dev/icons/rotate-ccw-square) | `rotate-ccw-square` | `<RotateCcwSquare />` |
| [rotate-cw-square](https://lucide.dev/icons/rotate-cw-square) | `rotate-cw-square` | `<RotateCwSquare />` |
| [ruler](https://lucide.dev/icons/ruler) | `ruler` | `<Ruler />` |
| [ruler-dimension-line](https://lucide.dev/icons/ruler-dimension-line) | `ruler-dimension-line` | `<RulerDimensionLine />` |
| [scissors](https://lucide.dev/icons/scissors) | `scissors` | `<Scissors />` |
| [scissors-line-dashed](https://lucide.dev/icons/scissors-line-dashed) | `scissors-line-dashed` | `<ScissorsLineDashed />` |
| [shovel](https://lucide.dev/icons/shovel) | `shovel` | `<Shovel />` |
| [spline-pointer](https://lucide.dev/icons/spline-pointer) | `spline-pointer` | `<SplinePointer />` |
| [spool](https://lucide.dev/icons/spool) | `spool` | `<Spool />` |
| [spray-can](https://lucide.dev/icons/spray-can) | `spray-can` | `<SprayCan />` |
| [square-bottom-dashed-scissors](https://lucide.dev/icons/square-bottom-dashed-scissors) | `square-bottom-dashed-scissors` | `<SquareBottomDashedScissors />` |
| [square-dashed-mouse-pointer](https://lucide.dev/icons/square-dashed-mouse-pointer) | `square-dashed-mouse-pointer` | `<SquareDashedMousePointer />` |
| [square-minus](https://lucide.dev/icons/square-minus) | `square-minus` | `<SquareMinus />` |
| [square-mouse-pointer](https://lucide.dev/icons/square-mouse-pointer) | `square-mouse-pointer` | `<SquareMousePointer />` |
| [square-plus](https://lucide.dev/icons/square-plus) | `square-plus` | `<SquarePlus />` |
| [square-scissors](https://lucide.dev/icons/square-scissors) | `square-scissors` | `<SquareScissors />` |
| [stamp](https://lucide.dev/icons/stamp) | `stamp` | `<Stamp />` |
| [sword](https://lucide.dev/icons/sword) | `sword` | `<Sword />` |
| [swords](https://lucide.dev/icons/swords) | `swords` | `<Swords />` |
| [tablet-smartphone](https://lucide.dev/icons/tablet-smartphone) | `tablet-smartphone` | `<TabletSmartphone />` |
| [tangent](https://lucide.dev/icons/tangent) | `tangent` | `<Tangent />` |
| [telescope](https://lucide.dev/icons/telescope) | `telescope` | `<Telescope />` |
| [tool-case](https://lucide.dev/icons/tool-case) | `tool-case` | `<ToolCase />` |
| [torus](https://lucide.dev/icons/torus) | `torus` | `<Torus />` |
| [vector-square](https://lucide.dev/icons/vector-square) | `vector-square` | `<VectorSquare />` |
| [wrench](https://lucide.dev/icons/wrench) | `wrench` | `<Wrench />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "tools"] or ["leptos", "all-icons"]
<Anvil class="w-6 h-6 text-gray-600" />
```

### Transportation

`transportation` and `all-icons` features - 60 icons

| Icon | Name | Component |
|------|------|-----------|
| [ambulance](https://lucide.dev/icons/ambulance) | `ambulance` | `<Ambulance />` |
| [anchor](https://lucide.dev/icons/anchor) | `anchor` | `<Anchor />` |
| [arrows-up-from-line](https://lucide.dev/icons/arrows-up-from-line) | `arrows-up-from-line` | `<ArrowsUpFromLine />` |
| [baggage-claim](https://lucide.dev/icons/baggage-claim) | `baggage-claim` | `<BaggageClaim />` |
| [bike](https://lucide.dev/icons/bike) | `bike` | `<Bike />` |
| [briefcase](https://lucide.dev/icons/briefcase) | `briefcase` | `<Briefcase />` |
| [briefcase-business](https://lucide.dev/icons/briefcase-business) | `briefcase-business` | `<BriefcaseBusiness />` |
| [briefcase-conveyor-belt](https://lucide.dev/icons/briefcase-conveyor-belt) | `briefcase-conveyor-belt` | `<BriefcaseConveyorBelt />` |
| [briefcase-medical](https://lucide.dev/icons/briefcase-medical) | `briefcase-medical` | `<BriefcaseMedical />` |
| [bus](https://lucide.dev/icons/bus) | `bus` | `<Bus />` |
| [bus-front](https://lucide.dev/icons/bus-front) | `bus-front` | `<BusFront />` |
| [cable-car](https://lucide.dev/icons/cable-car) | `cable-car` | `<CableCar />` |
| [car](https://lucide.dev/icons/car) | `car` | `<Car />` |
| [car-front](https://lucide.dev/icons/car-front) | `car-front` | `<CarFront />` |
| [car-taxi-front](https://lucide.dev/icons/car-taxi-front) | `car-taxi-front` | `<CarTaxiFront />` |
| [caravan](https://lucide.dev/icons/caravan) | `caravan` | `<Caravan />` |
| [cigarette](https://lucide.dev/icons/cigarette) | `cigarette` | `<Cigarette />` |
| [cigarette-off](https://lucide.dev/icons/cigarette-off) | `cigarette-off` | `<CigaretteOff />` |
| [circle-gauge](https://lucide.dev/icons/circle-gauge) | `circle-gauge` | `<CircleGauge />` |
| [circle-parking](https://lucide.dev/icons/circle-parking) | `circle-parking` | `<CircleParking />` |
| [circle-parking-off](https://lucide.dev/icons/circle-parking-off) | `circle-parking-off` | `<CircleParkingOff />` |
| [container](https://lucide.dev/icons/container) | `container` | `<Container />` |
| [drone](https://lucide.dev/icons/drone) | `drone` | `<Drone />` |
| [ev-charger](https://lucide.dev/icons/ev-charger) | `ev-charger` | `<EvCharger />` |
| [forklift](https://lucide.dev/icons/forklift) | `forklift` | `<Forklift />` |
| [fuel](https://lucide.dev/icons/fuel) | `fuel` | `<Fuel />` |
| [gauge](https://lucide.dev/icons/gauge) | `gauge` | `<Gauge />` |
| [handbag](https://lucide.dev/icons/handbag) | `handbag` | `<Handbag />` |
| [kayak](https://lucide.dev/icons/kayak) | `kayak` | `<Kayak />` |
| [luggage](https://lucide.dev/icons/luggage) | `luggage` | `<Luggage />` |
| [octagon-minus](https://lucide.dev/icons/octagon-minus) | `octagon-minus` | `<OctagonMinus />` |
| [parking-meter](https://lucide.dev/icons/parking-meter) | `parking-meter` | `<ParkingMeter />` |
| [plane](https://lucide.dev/icons/plane) | `plane` | `<Plane />` |
| [plane-landing](https://lucide.dev/icons/plane-landing) | `plane-landing` | `<PlaneLanding />` |
| [plane-takeoff](https://lucide.dev/icons/plane-takeoff) | `plane-takeoff` | `<PlaneTakeoff />` |
| [rail-symbol](https://lucide.dev/icons/rail-symbol) | `rail-symbol` | `<RailSymbol />` |
| [sailboat](https://lucide.dev/icons/sailboat) | `sailboat` | `<Sailboat />` |
| [ship](https://lucide.dev/icons/ship) | `ship` | `<Ship />` |
| [ship-wheel](https://lucide.dev/icons/ship-wheel) | `ship-wheel` | `<ShipWheel />` |
| [square-m](https://lucide.dev/icons/square-m) | `square-m` | `<SquareM />` |
| [square-parking](https://lucide.dev/icons/square-parking) | `square-parking` | `<SquareParking />` |
| [square-parking-off](https://lucide.dev/icons/square-parking-off) | `square-parking-off` | `<SquareParkingOff />` |
| [ticket](https://lucide.dev/icons/ticket) | `ticket` | `<Ticket />` |
| [ticket-check](https://lucide.dev/icons/ticket-check) | `ticket-check` | `<TicketCheck />` |
| [ticket-minus](https://lucide.dev/icons/ticket-minus) | `ticket-minus` | `<TicketMinus />` |
| [ticket-percent](https://lucide.dev/icons/ticket-percent) | `ticket-percent` | `<TicketPercent />` |
| [ticket-plus](https://lucide.dev/icons/ticket-plus) | `ticket-plus` | `<TicketPlus />` |
| [ticket-slash](https://lucide.dev/icons/ticket-slash) | `ticket-slash` | `<TicketSlash />` |
| [ticket-x](https://lucide.dev/icons/ticket-x) | `ticket-x` | `<TicketX />` |
| [tickets](https://lucide.dev/icons/tickets) | `tickets` | `<Tickets />` |
| [tickets-plane](https://lucide.dev/icons/tickets-plane) | `tickets-plane` | `<TicketsPlane />` |
| [tower-control](https://lucide.dev/icons/tower-control) | `tower-control` | `<TowerControl />` |
| [tractor](https://lucide.dev/icons/tractor) | `tractor` | `<Tractor />` |
| [traffic-cone](https://lucide.dev/icons/traffic-cone) | `traffic-cone` | `<TrafficCone />` |
| [train-front](https://lucide.dev/icons/train-front) | `train-front` | `<TrainFront />` |
| [train-front-tunnel](https://lucide.dev/icons/train-front-tunnel) | `train-front-tunnel` | `<TrainFrontTunnel />` |
| [train-track](https://lucide.dev/icons/train-track) | `train-track` | `<TrainTrack />` |
| [tram-front](https://lucide.dev/icons/tram-front) | `tram-front` | `<TramFront />` |
| [truck](https://lucide.dev/icons/truck) | `truck` | `<Truck />` |
| [truck-electric](https://lucide.dev/icons/truck-electric) | `truck-electric` | `<TruckElectric />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "transportation"] or ["leptos", "all-icons"]
<Ambulance class="w-6 h-6 text-gray-600" />
```

### Travel

`travel` and `all-icons` features - 65 icons

| Icon | Name | Component |
|------|------|-----------|
| [alarm-smoke](https://lucide.dev/icons/alarm-smoke) | `alarm-smoke` | `<AlarmSmoke />` |
| [backpack](https://lucide.dev/icons/backpack) | `backpack` | `<Backpack />` |
| [baggage-claim](https://lucide.dev/icons/baggage-claim) | `baggage-claim` | `<BaggageClaim />` |
| [bath](https://lucide.dev/icons/bath) | `bath` | `<Bath />` |
| [binoculars](https://lucide.dev/icons/binoculars) | `binoculars` | `<Binoculars />` |
| [book-image](https://lucide.dev/icons/book-image) | `book-image` | `<BookImage />` |
| [briefcase-conveyor-belt](https://lucide.dev/icons/briefcase-conveyor-belt) | `briefcase-conveyor-belt` | `<BriefcaseConveyorBelt />` |
| [cable-car](https://lucide.dev/icons/cable-car) | `cable-car` | `<CableCar />` |
| [caravan](https://lucide.dev/icons/caravan) | `caravan` | `<Caravan />` |
| [cigarette](https://lucide.dev/icons/cigarette) | `cigarette` | `<Cigarette />` |
| [cigarette-off](https://lucide.dev/icons/cigarette-off) | `cigarette-off` | `<CigaretteOff />` |
| [compass](https://lucide.dev/icons/compass) | `compass` | `<Compass />` |
| [concierge-bell](https://lucide.dev/icons/concierge-bell) | `concierge-bell` | `<ConciergeBell />` |
| [door-closed](https://lucide.dev/icons/door-closed) | `door-closed` | `<DoorClosed />` |
| [door-closed-locked](https://lucide.dev/icons/door-closed-locked) | `door-closed-locked` | `<DoorClosedLocked />` |
| [door-open](https://lucide.dev/icons/door-open) | `door-open` | `<DoorOpen />` |
| [fire-extinguisher](https://lucide.dev/icons/fire-extinguisher) | `fire-extinguisher` | `<FireExtinguisher />` |
| [heater](https://lucide.dev/icons/heater) | `heater` | `<Heater />` |
| [hospital](https://lucide.dev/icons/hospital) | `hospital` | `<Hospital />` |
| [hotel](https://lucide.dev/icons/hotel) | `hotel` | `<Hotel />` |
| [luggage](https://lucide.dev/icons/luggage) | `luggage` | `<Luggage />` |
| [map-minus](https://lucide.dev/icons/map-minus) | `map-minus` | `<MapMinus />` |
| [map-pin](https://lucide.dev/icons/map-pin) | `map-pin` | `<MapPin />` |
| [map-pin-check](https://lucide.dev/icons/map-pin-check) | `map-pin-check` | `<MapPinCheck />` |
| [map-pin-check-inside](https://lucide.dev/icons/map-pin-check-inside) | `map-pin-check-inside` | `<MapPinCheckInside />` |
| [map-pin-house](https://lucide.dev/icons/map-pin-house) | `map-pin-house` | `<MapPinHouse />` |
| [map-pin-minus](https://lucide.dev/icons/map-pin-minus) | `map-pin-minus` | `<MapPinMinus />` |
| [map-pin-minus-inside](https://lucide.dev/icons/map-pin-minus-inside) | `map-pin-minus-inside` | `<MapPinMinusInside />` |
| [map-pin-off](https://lucide.dev/icons/map-pin-off) | `map-pin-off` | `<MapPinOff />` |
| [map-pin-pen](https://lucide.dev/icons/map-pin-pen) | `map-pin-pen` | `<MapPinPen />` |
| [map-pin-plus](https://lucide.dev/icons/map-pin-plus) | `map-pin-plus` | `<MapPinPlus />` |
| [map-pin-plus-inside](https://lucide.dev/icons/map-pin-plus-inside) | `map-pin-plus-inside` | `<MapPinPlusInside />` |
| [map-pin-x](https://lucide.dev/icons/map-pin-x) | `map-pin-x` | `<MapPinX />` |
| [map-pin-x-inside](https://lucide.dev/icons/map-pin-x-inside) | `map-pin-x-inside` | `<MapPinXInside />` |
| [map-pinned](https://lucide.dev/icons/map-pinned) | `map-pinned` | `<MapPinned />` |
| [plane](https://lucide.dev/icons/plane) | `plane` | `<Plane />` |
| [plane-landing](https://lucide.dev/icons/plane-landing) | `plane-landing` | `<PlaneLanding />` |
| [plane-takeoff](https://lucide.dev/icons/plane-takeoff) | `plane-takeoff` | `<PlaneTakeoff />` |
| [pyramid](https://lucide.dev/icons/pyramid) | `pyramid` | `<Pyramid />` |
| [receipt](https://lucide.dev/icons/receipt) | `receipt` | `<Receipt />` |
| [receipt-cent](https://lucide.dev/icons/receipt-cent) | `receipt-cent` | `<ReceiptCent />` |
| [receipt-euro](https://lucide.dev/icons/receipt-euro) | `receipt-euro` | `<ReceiptEuro />` |
| [receipt-indian-rupee](https://lucide.dev/icons/receipt-indian-rupee) | `receipt-indian-rupee` | `<ReceiptIndianRupee />` |
| [receipt-japanese-yen](https://lucide.dev/icons/receipt-japanese-yen) | `receipt-japanese-yen` | `<ReceiptJapaneseYen />` |
| [receipt-pound-sterling](https://lucide.dev/icons/receipt-pound-sterling) | `receipt-pound-sterling` | `<ReceiptPoundSterling />` |
| [receipt-russian-ruble](https://lucide.dev/icons/receipt-russian-ruble) | `receipt-russian-ruble` | `<ReceiptRussianRuble />` |
| [receipt-swiss-franc](https://lucide.dev/icons/receipt-swiss-franc) | `receipt-swiss-franc` | `<ReceiptSwissFranc />` |
| [receipt-text](https://lucide.dev/icons/receipt-text) | `receipt-text` | `<ReceiptText />` |
| [receipt-turkish-lira](https://lucide.dev/icons/receipt-turkish-lira) | `receipt-turkish-lira` | `<ReceiptTurkishLira />` |
| [sailboat](https://lucide.dev/icons/sailboat) | `sailboat` | `<Sailboat />` |
| [shell](https://lucide.dev/icons/shell) | `shell` | `<Shell />` |
| [ship](https://lucide.dev/icons/ship) | `ship` | `<Ship />` |
| [ship-wheel](https://lucide.dev/icons/ship-wheel) | `ship-wheel` | `<ShipWheel />` |
| [shower-head](https://lucide.dev/icons/shower-head) | `shower-head` | `<ShowerHead />` |
| [soap-dispenser-droplet](https://lucide.dev/icons/soap-dispenser-droplet) | `soap-dispenser-droplet` | `<SoapDispenserDroplet />` |
| [tent](https://lucide.dev/icons/tent) | `tent` | `<Tent />` |
| [tent-tree](https://lucide.dev/icons/tent-tree) | `tent-tree` | `<TentTree />` |
| [tickets](https://lucide.dev/icons/tickets) | `tickets` | `<Tickets />` |
| [tickets-plane](https://lucide.dev/icons/tickets-plane) | `tickets-plane` | `<TicketsPlane />` |
| [tower-control](https://lucide.dev/icons/tower-control) | `tower-control` | `<TowerControl />` |
| [utensils](https://lucide.dev/icons/utensils) | `utensils` | `<Utensils />` |
| [utensils-crossed](https://lucide.dev/icons/utensils-crossed) | `utensils-crossed` | `<UtensilsCrossed />` |
| [vault](https://lucide.dev/icons/vault) | `vault` | `<Vault />` |
| [volleyball](https://lucide.dev/icons/volleyball) | `volleyball` | `<Volleyball />` |
| [washing-machine](https://lucide.dev/icons/washing-machine) | `washing-machine` | `<WashingMachine />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "travel"] or ["leptos", "all-icons"]
<AlarmSmoke class="w-6 h-6 text-gray-600" />
```

### Weather

`weather` and `all-icons` features - 42 icons

| Icon | Name | Component |
|------|------|-----------|
| [bubbles](https://lucide.dev/icons/bubbles) | `bubbles` | `<Bubbles />` |
| [cloud](https://lucide.dev/icons/cloud) | `cloud` | `<Cloud />` |
| [cloud-drizzle](https://lucide.dev/icons/cloud-drizzle) | `cloud-drizzle` | `<CloudDrizzle />` |
| [cloud-fog](https://lucide.dev/icons/cloud-fog) | `cloud-fog` | `<CloudFog />` |
| [cloud-hail](https://lucide.dev/icons/cloud-hail) | `cloud-hail` | `<CloudHail />` |
| [cloud-lightning](https://lucide.dev/icons/cloud-lightning) | `cloud-lightning` | `<CloudLightning />` |
| [cloud-moon](https://lucide.dev/icons/cloud-moon) | `cloud-moon` | `<CloudMoon />` |
| [cloud-moon-rain](https://lucide.dev/icons/cloud-moon-rain) | `cloud-moon-rain` | `<CloudMoonRain />` |
| [cloud-off](https://lucide.dev/icons/cloud-off) | `cloud-off` | `<CloudOff />` |
| [cloud-rain](https://lucide.dev/icons/cloud-rain) | `cloud-rain` | `<CloudRain />` |
| [cloud-rain-wind](https://lucide.dev/icons/cloud-rain-wind) | `cloud-rain-wind` | `<CloudRainWind />` |
| [cloud-snow](https://lucide.dev/icons/cloud-snow) | `cloud-snow` | `<CloudSnow />` |
| [cloud-sun](https://lucide.dev/icons/cloud-sun) | `cloud-sun` | `<CloudSun />` |
| [cloud-sun-rain](https://lucide.dev/icons/cloud-sun-rain) | `cloud-sun-rain` | `<CloudSunRain />` |
| [cloudy](https://lucide.dev/icons/cloudy) | `cloudy` | `<Cloudy />` |
| [droplet](https://lucide.dev/icons/droplet) | `droplet` | `<Droplet />` |
| [droplet-off](https://lucide.dev/icons/droplet-off) | `droplet-off` | `<DropletOff />` |
| [droplets](https://lucide.dev/icons/droplets) | `droplets` | `<Droplets />` |
| [flame](https://lucide.dev/icons/flame) | `flame` | `<Flame />` |
| [haze](https://lucide.dev/icons/haze) | `haze` | `<Haze />` |
| [moon-star](https://lucide.dev/icons/moon-star) | `moon-star` | `<MoonStar />` |
| [rainbow](https://lucide.dev/icons/rainbow) | `rainbow` | `<Rainbow />` |
| [snowflake](https://lucide.dev/icons/snowflake) | `snowflake` | `<Snowflake />` |
| [sparkles](https://lucide.dev/icons/sparkles) | `sparkles` | `<Sparkles />` |
| [star](https://lucide.dev/icons/star) | `star` | `<Star />` |
| [sun](https://lucide.dev/icons/sun) | `sun` | `<Sun />` |
| [sun-dim](https://lucide.dev/icons/sun-dim) | `sun-dim` | `<SunDim />` |
| [sun-medium](https://lucide.dev/icons/sun-medium) | `sun-medium` | `<SunMedium />` |
| [sun-snow](https://lucide.dev/icons/sun-snow) | `sun-snow` | `<SunSnow />` |
| [sunrise](https://lucide.dev/icons/sunrise) | `sunrise` | `<Sunrise />` |
| [sunset](https://lucide.dev/icons/sunset) | `sunset` | `<Sunset />` |
| [thermometer](https://lucide.dev/icons/thermometer) | `thermometer` | `<Thermometer />` |
| [thermometer-snowflake](https://lucide.dev/icons/thermometer-snowflake) | `thermometer-snowflake` | `<ThermometerSnowflake />` |
| [thermometer-sun](https://lucide.dev/icons/thermometer-sun) | `thermometer-sun` | `<ThermometerSun />` |
| [tornado](https://lucide.dev/icons/tornado) | `tornado` | `<Tornado />` |
| [umbrella](https://lucide.dev/icons/umbrella) | `umbrella` | `<Umbrella />` |
| [umbrella-off](https://lucide.dev/icons/umbrella-off) | `umbrella-off` | `<UmbrellaOff />` |
| [waves](https://lucide.dev/icons/waves) | `waves` | `<Waves />` |
| [wind](https://lucide.dev/icons/wind) | `wind` | `<Wind />` |
| [wind-arrow-down](https://lucide.dev/icons/wind-arrow-down) | `wind-arrow-down` | `<WindArrowDown />` |
| [zap](https://lucide.dev/icons/zap) | `zap` | `<Zap />` |
| [zap-off](https://lucide.dev/icons/zap-off) | `zap-off` | `<ZapOff />` |

**Usage example:**

```rust
// Add to Cargo.toml: features = ["leptos", "weather"] or ["leptos", "all-icons"]
<Bubbles class="w-6 h-6 text-gray-600" />
```

---

*This file is automatically generated by the `generate-icons` script.*
