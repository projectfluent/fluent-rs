
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


// browser/locales/en-US/browser/menubar.ftl

[[ File menu ]]

file-menu
    .label =     File
    .accesskey = F
tab-menuitem
    .label =      New Tab
    .accesskey =  T
tab-key
    .key =        t
new-user-context-menu
    .label =      New Container Tab
    .accesskey =  C
new-navigator-menuitem
    .label =      New Window
    .accesskey =  N
new-navigator-key
    .key =        N
new-private-window-menuitem
    .label =      New Private Window
    .accesskey =  W
new-non-remote-window-menuitem
    .label =      New Non-e10s Window

// Only displayed on OS X, and only on windows that aren't main browser windows,
// or when there are no windows but Firefox is still running.
open-location-menuitem
    .label =      Open Location…
open-file-menuitem
    .label =      Open File…
    .accesskey =  O
open-file-key
    .key =        o

close-menuitem
    .label =      Close
    .accesskey =  C
close-key
    .key =        W
close-window-menuitem
    .label =      Close Window
    .accesskey =  d

// .accesskey2 = is for content area context menu
save-page-menuitem
    .label =      Save Page As…
    .accesskey =  A
    .accesskey2 = P
save-page-key
    .key =        s

email-page-menuitem
    .label =      Email Link…
    .accesskey =  E

print-setup-menuitem
    .label =      Page Setup…
    .accesskey =  u
print-preview-menuitem
    .label =      Print Preview…
    .accesskey =  v
print-menuitem
    .label =      Print…
    .accesskey =  P
print-key
    .key =        p

go-offline-menuitem
    .label =      Work Offline
    .accesskey =  k

quit-application-menuitem
    .label =      Quit
    .accesskey =  Q
quit-application-menuitem-win
    .label =      Exit
    .accesskey =  x
quit-application-menuitem-mac
    .label =      Quit { brand-shorter-name }
// Used by both Linux and OSX builds
quit-application-key-unix
    .key =        Q

[[ Edit menu ]]

edit-menu
    .label =      Edit
    .accesskey =  E
undo-menuitem
    .label =      Undo
    .accesskey =  U
undo-key
    .key =        Z
redo-menuitem
    .label =      Redo
    .accesskey =  R
redo-key
    .key =        Y
cut-menuitem
    .label =      Cut
    .accesskey =  t
cut-key
    .key =        X
copy-menuitem
    .label =      Copy
    .accesskey =  C
copy-key
    .key =        C
paste-menuitem
    .label =      Paste
    .accesskey =  P
paste-key
    .key =        V
delete-menuitem
    .label =      Delete
    .accesskey =  D
delete-key
    .key =        D
select-all-menuitem
    .label =      Select All
    .accesskey =  A
select-all-key
    .key =        A

find-on-menuitem
    .label =      Find in This Page…
    .accesskey =  F
find-on-key
    .key =        f
find-again-menuitem
    .label =      Find Again
    .accesskey =  g
find-again-key1
    .key =        g
find-again-key2
    .keycode =    VK_F3
find-selection-key
    .key =        e

bidi-switch-text-direction-menuitem
    .label =      Switch Text Direction
    .accesskey =  w
bidi-switch-text-direction-key
    .key =        X

preferences-menuitem
    .label =      Options
    .accesskey =  O
preferences-menuitem-unix
    .label =      Preferences
    .accesskey =  n


[[ View menu ]]

view-menu
    .label =      View
    .accesskey =  V
view-toolbars-menu
    .label =      Toolbars
    .accesskey =  T
view-sidebar-menu
    .label =      Sidebar
    .accesskey =  e
view-customize-toolbar-menuitem
    .label =      Customize…
    .accesskey =  C

full-zoom-menu
    .label =      Zoom
    .accesskey =  Z
full-zoom-enlarge-menuitem
    .label =      Zoom In
    .accesskey =  I
full-zoom-enlarge-key1
    .key =        +
full-zoom-enlarge-key2
    .key =        =
full-zoom-enlarge-key3
    .key =        {""}
full-zoom-reduce-menuitem
    .label =      Zoom Out
    .accesskey =  O
full-zoom-reduce-key1
    .key =        -
full-zoom-reduce-key2
    .key =        {""}
full-zoom-reset-menuitem
    .label =      Reset
    .accesskey =  R
full-zoom-reset-key1
    .key =        0
full-zoom-reset-key2
    .key =       {""}
full-zoom-toggle-menuitem
    .label =      Zoom Text Only
    .accesskey =  T

page-style-menu
    .label =      Page Style
    .accesskey =  y
page-style-no-style-menuitem
    .label =      No Style
    .accesskey =  n
page-style-persistent-only-menuitem
    .label =      Basic Page Style
    .accesskey =  b

show-all-tabs-menuitem
    .label =      Show All Tabs
    .accesskey =  A
bidi-switch-page-direction-menuitem
    .label =      Switch Page Direction
    .accesskey =  D

// Match what Safari and other Apple applications use on OS X Lion.
[[ Full Screen controls ]]

enter-full-screen-menuitem
    .label =      Enter Full Screen
    .accesskey =  F
exit-full-screen-menuitem
    .label =      Exit Full Screen
    .accesskey =  F
full-screen-menuitem
    .label =      Full Screen
    .accesskey =  F
full-screen-key
    .key =        f


[[ History menu ]]

history-menu
    .label =        History
    .accesskey =    s
show-all-history-menuitem
    .label =        Show All History
show-all-history-key
    .key =          H
clear-recent-history-menuitem
    .label =        Clean Recent History…
history-synced-tabs-menuitem
    .label =        Synced Tabs
history-restore-last-session-menuitem
    .label =        Restore Previous Session
history-undo-menu
    .label =        Recently Closed Tabs
history-undo-window-menu
    .label =        Recently Closed Windows


[[ Bookmarks menu ]]

bookmarks-menu
    .label =      Bookmarks
    .accesskey =  B
show-all-bookmarks-menuitem
    .label =      Show All Bookmarks
show-all-bookmarks-key
    .key =        b
// .key = should not contain the letters A-F since the are reserved shortcut
// keys on Linux.
show-all-bookmarks-key-gtk
    .key =        o
bookmark-this-page-broadcaster
    .label =      Bookmark This Page
edit-this-page-broadcaster
    .label =      Edit This Page
bookmark-this-page-key
    .key =        d
subscribe-to-page-menuitem
    .label =      Subscribe to This Page…
subscribe-to-page-menupopup
    .label =      Subscribe to This Page…
add-cur-pages-menuitem
    .label =      Bookmark All Tabs…
recent-bookmarks-menuitem
    .label =      Recently Bookmarked

other-bookmarks-menu
    .label =      Other Bookmarks
personalbar-menu
    .label =      Bookmarks Toolbar
    .accesskey =  B


[[ Tools menu ]]

tools-menu
    .label =      Tools
    .accesskey =  T
downloads-menuitem
    .label =      Downloads
    .accesskey =  D
downloads-key
    .key =        j
downloads-key-unix
    .key =        y
addons-menuitem
    .label =      Add-ons
    .accesskey =  A
addons-key
    .key =        A

sync-sign-in-menuitem
    .label =      Sign In To { sync-brand-short-name }…
    .accesskey =  Y
sync-sync-now-menuitem
    .label =      Sync Now
    .accesskey =  S
sync-re-auth-menuitem
    .label =      Reconnect to { sync-brand-short-name }…
    .accesskey =  R
sync-toolbar-button
    .label =      Sync

web-developer-menu
    .label =      Web Developer
    .accesskey =  W

page-source-broadcaster
    .label =      Page Source
    .accesskey =  o
page-source-key
    .key =        u
page-info-menuitem
    .label =      Page Info
    .accesskey =  I
page-info-key
    .key =        i
mirror-tab-menu
    .label =      Mirror Tab
    .accesskey =  m


// browser/locales/en-US/browser/toolbar.ftl

urlbar-textbox
    .placeholder = Search or enter address
    .accesskey =   d


[[ Toolbar items ]]

view-bookmarks-broadcaster
    .label =      Bookmarks
view-bookmarks-key
    .key =        b
view-bookmarks-key-win
    .key =        i

view-history-broadcaster
    .label =      History
view-history-key
    .key =        h
view-tabs-broadcaster
    .label =      Synced Tabs


// browser/branding/official/locales/en-US/brand.ftl

brand-shorter-name    = Firefox
brand-short-name      = Firefox
brand-full-name       = Mozilla Firefox
vendor-short-name     = Mozilla

trademark-info =
  | Firefox and the Firefox logos are trademarks of the Mozilla Foundation.

sync-brand-short-name = Sync
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


// browser/locales/en-US/browser/menubar.ftl

[[ File menu ]]

file-menu
    .label =     File
    .accesskey = F
tab-menuitem
    .label =      New Tab
    .accesskey =  T
tab-key
    .key =        t
new-user-context-menu
    .label =      New Container Tab
    .accesskey =  C
new-navigator-menuitem
    .label =      New Window
    .accesskey =  N
new-navigator-key
    .key =        N
new-private-window-menuitem
    .label =      New Private Window
    .accesskey =  W
new-non-remote-window-menuitem
    .label =      New Non-e10s Window

// Only displayed on OS X, and only on windows that aren't main browser windows,
// or when there are no windows but Firefox is still running.
open-location-menuitem
    .label =      Open Location…
open-file-menuitem
    .label =      Open File…
    .accesskey =  O
open-file-key
    .key =        o

close-menuitem
    .label =      Close
    .accesskey =  C
close-key
    .key =        W
close-window-menuitem
    .label =      Close Window
    .accesskey =  d

// .accesskey2 = is for content area context menu
save-page-menuitem
    .label =      Save Page As…
    .accesskey =  A
    .accesskey2 = P
save-page-key
    .key =        s

email-page-menuitem
    .label =      Email Link…
    .accesskey =  E

print-setup-menuitem
    .label =      Page Setup…
    .accesskey =  u
print-preview-menuitem
    .label =      Print Preview…
    .accesskey =  v
print-menuitem
    .label =      Print…
    .accesskey =  P
print-key
    .key =        p

go-offline-menuitem
    .label =      Work Offline
    .accesskey =  k

quit-application-menuitem
    .label =      Quit
    .accesskey =  Q
quit-application-menuitem-win
    .label =      Exit
    .accesskey =  x
quit-application-menuitem-mac
    .label =      Quit { brand-shorter-name }
// Used by both Linux and OSX builds
quit-application-key-unix
    .key =        Q

[[ Edit menu ]]

edit-menu
    .label =      Edit
    .accesskey =  E
undo-menuitem
    .label =      Undo
    .accesskey =  U
undo-key
    .key =        Z
redo-menuitem
    .label =      Redo
    .accesskey =  R
redo-key
    .key =        Y
cut-menuitem
    .label =      Cut
    .accesskey =  t
cut-key
    .key =        X
copy-menuitem
    .label =      Copy
    .accesskey =  C
copy-key
    .key =        C
paste-menuitem
    .label =      Paste
    .accesskey =  P
paste-key
    .key =        V
delete-menuitem
    .label =      Delete
    .accesskey =  D
delete-key
    .key =        D
select-all-menuitem
    .label =      Select All
    .accesskey =  A
select-all-key
    .key =        A

find-on-menuitem
    .label =      Find in This Page…
    .accesskey =  F
find-on-key
    .key =        f
find-again-menuitem
    .label =      Find Again
    .accesskey =  g
find-again-key1
    .key =        g
find-again-key2
    .keycode =    VK_F3
find-selection-key
    .key =        e

bidi-switch-text-direction-menuitem
    .label =      Switch Text Direction
    .accesskey =  w
bidi-switch-text-direction-key
    .key =        X

preferences-menuitem
    .label =      Options
    .accesskey =  O
preferences-menuitem-unix
    .label =      Preferences
    .accesskey =  n


[[ View menu ]]

view-menu
    .label =      View
    .accesskey =  V
view-toolbars-menu
    .label =      Toolbars
    .accesskey =  T
view-sidebar-menu
    .label =      Sidebar
    .accesskey =  e
view-customize-toolbar-menuitem
    .label =      Customize…
    .accesskey =  C

full-zoom-menu
    .label =      Zoom
    .accesskey =  Z
full-zoom-enlarge-menuitem
    .label =      Zoom In
    .accesskey =  I
full-zoom-enlarge-key1
    .key =        +
full-zoom-enlarge-key2
    .key =        =
full-zoom-enlarge-key3
    .key =        {""}
full-zoom-reduce-menuitem
    .label =      Zoom Out
    .accesskey =  O
full-zoom-reduce-key1
    .key =        -
full-zoom-reduce-key2
    .key =        {""}
full-zoom-reset-menuitem
    .label =      Reset
    .accesskey =  R
full-zoom-reset-key1
    .key =        0
full-zoom-reset-key2
    .key =        {""}
full-zoom-toggle-menuitem
    .label =      Zoom Text Only
    .accesskey =  T

page-style-menu
    .label =      Page Style
    .accesskey =  y
page-style-no-style-menuitem
    .label =      No Style
    .accesskey =  n
page-style-persistent-only-menuitem
    .label =      Basic Page Style
    .accesskey =  b

show-all-tabs-menuitem
    .label =      Show All Tabs
    .accesskey =  A
bidi-switch-page-direction-menuitem
    .label =      Switch Page Direction
    .accesskey =  D

// Match what Safari and other Apple applications use on OS X Lion.
[[ Full Screen controls ]]

enter-full-screen-menuitem
    .label =      Enter Full Screen
    .accesskey =  F
exit-full-screen-menuitem
    .label =      Exit Full Screen
    .accesskey =  F
full-screen-menuitem
    .label =      Full Screen
    .accesskey =  F
full-screen-key
    .key =        f


[[ History menu ]]

history-menu
    .label =        History
    .accesskey =    s
show-all-history-menuitem
    .label =        Show All History
show-all-history-key
    .key =          H
clear-recent-history-menuitem
    .label =        Clean Recent History…
history-synced-tabs-menuitem
    .label =        Synced Tabs
history-restore-last-session-menuitem
    .label =        Restore Previous Session
history-undo-menu
    .label =        Recently Closed Tabs
history-undo-window-menu
    .label =        Recently Closed Windows


[[ Bookmarks menu ]]

bookmarks-menu
    .label =      Bookmarks
    .accesskey =  B
show-all-bookmarks-menuitem
    .label =      Show All Bookmarks
show-all-bookmarks-key
    .key =        b
// .key = should not contain the letters A-F since the are reserved shortcut
// keys on Linux.
show-all-bookmarks-key-gtk
    .key =        o
bookmark-this-page-broadcaster
    .label =      Bookmark This Page
edit-this-page-broadcaster
    .label =      Edit This Page
bookmark-this-page-key
    .key =        d
subscribe-to-page-menuitem
    .label =      Subscribe to This Page…
subscribe-to-page-menupopup
    .label =      Subscribe to This Page…
add-cur-pages-menuitem
    .label =      Bookmark All Tabs…
recent-bookmarks-menuitem
    .label =      Recently Bookmarked

other-bookmarks-menu
    .label =      Other Bookmarks
personalbar-menu
    .label =      Bookmarks Toolbar
    .accesskey =  B


[[ Tools menu ]]

tools-menu
    .label =      Tools
    .accesskey =  T
downloads-menuitem
    .label =      Downloads
    .accesskey =  D
downloads-key
    .key =        j
downloads-key-unix
    .key =        y
addons-menuitem
    .label =      Add-ons
    .accesskey =  A
addons-key
    .key =        A

sync-sign-in-menuitem
    .label =      Sign In To { sync-brand-short-name }…
    .accesskey =  Y
sync-sync-now-menuitem
    .label =      Sync Now
    .accesskey =  S
sync-re-auth-menuitem
    .label =      Reconnect to { sync-brand-short-name }…
    .accesskey =  R
sync-toolbar-button
    .label =      Sync

web-developer-menu
    .label =      Web Developer
    .accesskey =  W

page-source-broadcaster
    .label =      Page Source
    .accesskey =  o
page-source-key
    .key =        u
page-info-menuitem
    .label =      Page Info
    .accesskey =  I
page-info-key
    .key =        i
mirror-tab-menu
    .label =      Mirror Tab
    .accesskey =  m


// browser/locales/en-US/browser/toolbar.ftl

urlbar-textbox
    .placeholder = Search or enter address
    .accesskey =   d


[[ Toolbar items ]]

view-bookmarks-broadcaster
    .label =      Bookmarks
view-bookmarks-key
    .key =        b
view-bookmarks-key-win
    .key =        i

view-history-broadcaster
    .label =      History
view-history-key
    .key =        h
view-tabs-broadcaster
    .label =      Synced Tabs


// browser/branding/official/locales/en-US/brand.ftl

brand-shorter-name    = Firefox
brand-short-name      = Firefox
brand-full-name       = Mozilla Firefox
vendor-short-name     = Mozilla

trademark-info =
  | Firefox and the Firefox logos are trademarks of the Mozilla Foundation.

sync-brand-short-name = Sync
