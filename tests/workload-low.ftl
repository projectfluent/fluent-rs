
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


# browser/locales/en-US/browser/menubar.ftl

[[ File menu ]]

file-menu =
    [xul/label]     File
    [xul/accesskey] F
tab-menuitem =
    [xul/label]      New Tab
    [xul/accesskey]  T
tab-key =
    [xul/key]        t
new-user-context-menu =
    [xul/label]      New Container Tab
    [xul/accesskey]  C
new-navigator-menuitem =
    [xul/label]      New Window
    [xul/accesskey]  N
new-navigator-key =
    [xul/key]        N
new-private-window-menuitem =
    [xul/label]      New Private Window
    [xul/accesskey]  W
new-non-remote-window-menuitem =
    [xul/label]      New Non-e10s Window

# Only displayed on OS X, and only on windows that aren't main browser windows,
# or when there are no windows but Firefox is still running.
open-location-menuitem =
    [xul/label]      Open Location…
open-file-menuitem =
    [xul/label]      Open File…
    [xul/accesskey]  O
open-file-key =
    [xul/key]        o

close-menuitem =
    [xul/label]      Close
    [xul/accesskey]  C
close-key =
    [xul/key]        W
close-window-menuitem =
    [xul/label]      Close Window
    [xul/accesskey]  d

# [xul/accesskey2] is for content area context menu
save-page-menuitem =
    [xul/label]      Save Page As…
    [xul/accesskey]  A
    [xul/accesskey2] P
save-page-key =
    [xul/key]        s

email-page-menuitem =
    [xul/label]      Email Link…
    [xul/accesskey]  E

print-setup-menuitem =
    [xul/label]      Page Setup…
    [xul/accesskey]  u
print-preview-menuitem =
    [xul/label]      Print Preview…
    [xul/accesskey]  v
print-menuitem =
    [xul/label]      Print…
    [xul/accesskey]  P
print-key =
    [xul/key]        p

go-offline-menuitem =
    [xul/label]      Work Offline
    [xul/accesskey]  k

quit-application-menuitem =
    [xul/label]      Quit
    [xul/accesskey]  Q
quit-application-menuitem-win =
    [xul/label]      Exit
    [xul/accesskey]  x
quit-application-menuitem-mac =
    [xul/label]      Quit { brand-shorter-name }
# Used by both Linux and OSX builds
quit-application-key-unix =
    [xul/key]        Q

[[ Edit menu ]]

edit-menu =
    [xul/label]      Edit
    [xul/accesskey]  E
undo-menuitem =
    [xul/label]      Undo
    [xul/accesskey]  U
undo-key =
    [xul/key]        Z
redo-menuitem =
    [xul/label]      Redo
    [xul/accesskey]  R
redo-key =
    [xul/key]        Y
cut-menuitem =
    [xul/label]      Cut
    [xul/accesskey]  t
cut-key =
    [xul/key]        X
copy-menuitem =
    [xul/label]      Copy
    [xul/accesskey]  C
copy-key =
    [xul/key]        C
paste-menuitem =
    [xul/label]      Paste
    [xul/accesskey]  P
paste-key =
    [xul/key]        V
delete-menuitem =
    [xul/label]      Delete
    [xul/accesskey]  D
delete-key =
    [xul/key]        D
select-all-menuitem =
    [xul/label]      Select All
    [xul/accesskey]  A
select-all-key =
    [xul/key]        A

find-on-menuitem =
    [xul/label]      Find in This Page…
    [xul/accesskey]  F
find-on-key =
    [xul/key]        f
find-again-menuitem =
    [xul/label]      Find Again
    [xul/accesskey]  g
find-again-key1 =
    [xul/key]        g
find-again-key2 =
    [xul/keycode]    VK_F3
find-selection-key =
    [xul/key]        e

bidi-switch-text-direction-menuitem =
    [xul/label]      Switch Text Direction
    [xul/accesskey]  w
bidi-switch-text-direction-key =
    [xul/key]        X

preferences-menuitem =
    [xul/label]      Options
    [xul/accesskey]  O
preferences-menuitem-unix =
    [xul/label]      Preferences
    [xul/accesskey]  n


[[ View menu ]]

view-menu =
    [xul/label]      View
    [xul/accesskey]  V
view-toolbars-menu =
    [xul/label]      Toolbars
    [xul/accesskey]  T
view-sidebar-menu =
    [xul/label]      Sidebar
    [xul/accesskey]  e
view-customize-toolbar-menuitem =
    [xul/label]      Customize…
    [xul/accesskey]  C

full-zoom-menu =
    [xul/label]      Zoom
    [xul/accesskey]  Z
full-zoom-enlarge-menuitem =
    [xul/label]      Zoom In
    [xul/accesskey]  I
full-zoom-enlarge-key1 =
    [xul/key]        +
full-zoom-enlarge-key2 =
    [xul/key]        =
full-zoom-enlarge-key3 =
    [xul/key]        ""
full-zoom-reduce-menuitem =
    [xul/label]      Zoom Out
    [xul/accesskey]  O
full-zoom-reduce-key1 =
    [xul/key]        -
full-zoom-reduce-key2 =
    [xul/key]        ""
full-zoom-reset-menuitem =
    [xul/label]      Reset
    [xul/accesskey]  R
full-zoom-reset-key1 =
    [xul/key]        0
full-zoom-reset-key2 =
    [xul/key]        ""
full-zoom-toggle-menuitem =
    [xul/label]      Zoom Text Only
    [xul/accesskey]  T

page-style-menu =
    [xul/label]      Page Style
    [xul/accesskey]  y
page-style-no-style-menuitem =
    [xul/label]      No Style
    [xul/accesskey]  n
page-style-persistent-only-menuitem =
    [xul/label]      Basic Page Style
    [xul/accesskey]  b

show-all-tabs-menuitem =
    [xul/label]      Show All Tabs
    [xul/accesskey]  A
bidi-switch-page-direction-menuitem =
    [xul/label]      Switch Page Direction
    [xul/accesskey]  D

# Match what Safari and other Apple applications use on OS X Lion.
[[ Full Screen controls ]]

enter-full-screen-menuitem =
    [xul/label]      Enter Full Screen
    [xul/accesskey]  F
exit-full-screen-menuitem =
    [xul/label]      Exit Full Screen
    [xul/accesskey]  F
full-screen-menuitem =
    [xul/label]      Full Screen
    [xul/accesskey]  F
full-screen-key =
    [xul/key]        f


[[ History menu ]]

history-menu =
    [xul/label]        History
    [xul/accesskey]    s
show-all-history-menuitem =
    [xul/label]        Show All History
show-all-history-key =
    [xul/key]          H
clear-recent-history-menuitem =
    [xul/label]        Clean Recent History…
history-synced-tabs-menuitem =
    [xul/label]        Synced Tabs
history-restore-last-session-menuitem =
    [xul/label]        Restore Previous Session
history-undo-menu =
    [xul/label]        Recently Closed Tabs
history-undo-window-menu =
    [xul/label]        Recently Closed Windows


[[ Bookmarks menu ]]

bookmarks-menu =
    [xul/label]      Bookmarks
    [xul/accesskey]  B
show-all-bookmarks-menuitem =
    [xul/label]      Show All Bookmarks
show-all-bookmarks-key =
    [xul/key]        b
# [xul/key] should not contain the letters A-F since the are reserved shortcut
# keys on Linux.
show-all-bookmarks-key-gtk =
    [xul/key]        o
bookmark-this-page-broadcaster =
    [xul/label]      Bookmark This Page
edit-this-page-broadcaster =
    [xul/label]      Edit This Page
bookmark-this-page-key =
    [xul/key]        d
subscribe-to-page-menuitem =
    [xul/label]      Subscribe to This Page…
subscribe-to-page-menupopup =
    [xul/label]      Subscribe to This Page…
add-cur-pages-menuitem =
    [xul/label]      Bookmark All Tabs…
recent-bookmarks-menuitem =
    [xul/label]      Recently Bookmarked

other-bookmarks-menu =
    [xul/label]      Other Bookmarks
personalbar-menu =
    [xul/label]      Bookmarks Toolbar
    [xul/accesskey]  B


[[ Tools menu ]]

tools-menu =
    [xul/label]      Tools
    [xul/accesskey]  T
downloads-menuitem =
    [xul/label]      Downloads
    [xul/accesskey]  D
downloads-key =
    [xul/key]        j
downloads-key-unix =
    [xul/key]        y
addons-menuitem =
    [xul/label]      Add-ons
    [xul/accesskey]  A
addons-key =
    [xul/key]        A

sync-sign-in-menuitem =
    [xul/label]      Sign In To { sync-brand-short-name }…
    [xul/accesskey]  Y
sync-sync-now-menuitem =
    [xul/label]      Sync Now
    [xul/accesskey]  S
sync-re-auth-menuitem =
    [xul/label]      Reconnect to { sync-brand-short-name }…
    [xul/accesskey]  R
sync-toolbar-button =
    [xul/label]      Sync

web-developer-menu =
    [xul/label]      Web Developer
    [xul/accesskey]  W

page-source-broadcaster =
    [xul/label]      Page Source
    [xul/accesskey]  o
page-source-key =
    [xul/key]        u
page-info-menuitem =
    [xul/label]      Page Info
    [xul/accesskey]  I
page-info-key =
    [xul/key]        i
mirror-tab-menu =
    [xul/label]      Mirror Tab
    [xul/accesskey]  m


# browser/locales/en-US/browser/toolbar.ftl

urlbar-textbox =
    [xul/placeholder] Search or enter address
    [xul/accesskey]   d


[[ Toolbar items ]]

view-bookmarks-broadcaster =
    [xul/label]      Bookmarks
view-bookmarks-key =
    [xul/key]        b
view-bookmarks-key-win =
    [xul/key]        i

view-history-broadcaster =
    [xul/label]      History
view-history-key =
    [xul/key]        h
view-tabs-broadcaster =
    [xul/label]      Synced Tabs


# browser/branding/official/locales/en-US/brand.ftl

brand-shorter-name    = Firefox
brand-short-name      = Firefox
brand-full-name       = Mozilla Firefox
vendor-short-name     = Mozilla

trademark-info        = 
  | Firefox and the Firefox logos are trademarks of the Mozilla Foundation.

sync-brand-short-name = Sync
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


# browser/locales/en-US/browser/menubar.ftl

[[ File menu ]]

file-menu =
    [xul/label]     File
    [xul/accesskey] F
tab-menuitem =
    [xul/label]      New Tab
    [xul/accesskey]  T
tab-key =
    [xul/key]        t
new-user-context-menu =
    [xul/label]      New Container Tab
    [xul/accesskey]  C
new-navigator-menuitem =
    [xul/label]      New Window
    [xul/accesskey]  N
new-navigator-key =
    [xul/key]        N
new-private-window-menuitem =
    [xul/label]      New Private Window
    [xul/accesskey]  W
new-non-remote-window-menuitem =
    [xul/label]      New Non-e10s Window

# Only displayed on OS X, and only on windows that aren't main browser windows,
# or when there are no windows but Firefox is still running.
open-location-menuitem =
    [xul/label]      Open Location…
open-file-menuitem =
    [xul/label]      Open File…
    [xul/accesskey]  O
open-file-key =
    [xul/key]        o

close-menuitem =
    [xul/label]      Close
    [xul/accesskey]  C
close-key =
    [xul/key]        W
close-window-menuitem =
    [xul/label]      Close Window
    [xul/accesskey]  d

# [xul/accesskey2] is for content area context menu
save-page-menuitem =
    [xul/label]      Save Page As…
    [xul/accesskey]  A
    [xul/accesskey2] P
save-page-key =
    [xul/key]        s

email-page-menuitem =
    [xul/label]      Email Link…
    [xul/accesskey]  E

print-setup-menuitem =
    [xul/label]      Page Setup…
    [xul/accesskey]  u
print-preview-menuitem =
    [xul/label]      Print Preview…
    [xul/accesskey]  v
print-menuitem =
    [xul/label]      Print…
    [xul/accesskey]  P
print-key =
    [xul/key]        p

go-offline-menuitem =
    [xul/label]      Work Offline
    [xul/accesskey]  k

quit-application-menuitem =
    [xul/label]      Quit
    [xul/accesskey]  Q
quit-application-menuitem-win =
    [xul/label]      Exit
    [xul/accesskey]  x
quit-application-menuitem-mac =
    [xul/label]      Quit { brand-shorter-name }
# Used by both Linux and OSX builds
quit-application-key-unix =
    [xul/key]        Q

[[ Edit menu ]]

edit-menu =
    [xul/label]      Edit
    [xul/accesskey]  E
undo-menuitem =
    [xul/label]      Undo
    [xul/accesskey]  U
undo-key =
    [xul/key]        Z
redo-menuitem =
    [xul/label]      Redo
    [xul/accesskey]  R
redo-key =
    [xul/key]        Y
cut-menuitem =
    [xul/label]      Cut
    [xul/accesskey]  t
cut-key =
    [xul/key]        X
copy-menuitem =
    [xul/label]      Copy
    [xul/accesskey]  C
copy-key =
    [xul/key]        C
paste-menuitem =
    [xul/label]      Paste
    [xul/accesskey]  P
paste-key =
    [xul/key]        V
delete-menuitem =
    [xul/label]      Delete
    [xul/accesskey]  D
delete-key =
    [xul/key]        D
select-all-menuitem =
    [xul/label]      Select All
    [xul/accesskey]  A
select-all-key =
    [xul/key]        A

find-on-menuitem =
    [xul/label]      Find in This Page…
    [xul/accesskey]  F
find-on-key =
    [xul/key]        f
find-again-menuitem =
    [xul/label]      Find Again
    [xul/accesskey]  g
find-again-key1 =
    [xul/key]        g
find-again-key2 =
    [xul/keycode]    VK_F3
find-selection-key =
    [xul/key]        e

bidi-switch-text-direction-menuitem =
    [xul/label]      Switch Text Direction
    [xul/accesskey]  w
bidi-switch-text-direction-key =
    [xul/key]        X

preferences-menuitem =
    [xul/label]      Options
    [xul/accesskey]  O
preferences-menuitem-unix =
    [xul/label]      Preferences
    [xul/accesskey]  n


[[ View menu ]]

view-menu =
    [xul/label]      View
    [xul/accesskey]  V
view-toolbars-menu =
    [xul/label]      Toolbars
    [xul/accesskey]  T
view-sidebar-menu =
    [xul/label]      Sidebar
    [xul/accesskey]  e
view-customize-toolbar-menuitem =
    [xul/label]      Customize…
    [xul/accesskey]  C

full-zoom-menu =
    [xul/label]      Zoom
    [xul/accesskey]  Z
full-zoom-enlarge-menuitem =
    [xul/label]      Zoom In
    [xul/accesskey]  I
full-zoom-enlarge-key1 =
    [xul/key]        +
full-zoom-enlarge-key2 =
    [xul/key]        =
full-zoom-enlarge-key3 =
    [xul/key]        ""
full-zoom-reduce-menuitem =
    [xul/label]      Zoom Out
    [xul/accesskey]  O
full-zoom-reduce-key1 =
    [xul/key]        -
full-zoom-reduce-key2 =
    [xul/key]        ""
full-zoom-reset-menuitem =
    [xul/label]      Reset
    [xul/accesskey]  R
full-zoom-reset-key1 =
    [xul/key]        0
full-zoom-reset-key2 =
    [xul/key]        ""
full-zoom-toggle-menuitem =
    [xul/label]      Zoom Text Only
    [xul/accesskey]  T

page-style-menu =
    [xul/label]      Page Style
    [xul/accesskey]  y
page-style-no-style-menuitem =
    [xul/label]      No Style
    [xul/accesskey]  n
page-style-persistent-only-menuitem =
    [xul/label]      Basic Page Style
    [xul/accesskey]  b

show-all-tabs-menuitem =
    [xul/label]      Show All Tabs
    [xul/accesskey]  A
bidi-switch-page-direction-menuitem =
    [xul/label]      Switch Page Direction
    [xul/accesskey]  D

# Match what Safari and other Apple applications use on OS X Lion.
[[ Full Screen controls ]]

enter-full-screen-menuitem =
    [xul/label]      Enter Full Screen
    [xul/accesskey]  F
exit-full-screen-menuitem =
    [xul/label]      Exit Full Screen
    [xul/accesskey]  F
full-screen-menuitem =
    [xul/label]      Full Screen
    [xul/accesskey]  F
full-screen-key =
    [xul/key]        f


[[ History menu ]]

history-menu =
    [xul/label]        History
    [xul/accesskey]    s
show-all-history-menuitem =
    [xul/label]        Show All History
show-all-history-key =
    [xul/key]          H
clear-recent-history-menuitem =
    [xul/label]        Clean Recent History…
history-synced-tabs-menuitem =
    [xul/label]        Synced Tabs
history-restore-last-session-menuitem =
    [xul/label]        Restore Previous Session
history-undo-menu =
    [xul/label]        Recently Closed Tabs
history-undo-window-menu =
    [xul/label]        Recently Closed Windows


[[ Bookmarks menu ]]

bookmarks-menu =
    [xul/label]      Bookmarks
    [xul/accesskey]  B
show-all-bookmarks-menuitem =
    [xul/label]      Show All Bookmarks
show-all-bookmarks-key =
    [xul/key]        b
# [xul/key] should not contain the letters A-F since the are reserved shortcut
# keys on Linux.
show-all-bookmarks-key-gtk =
    [xul/key]        o
bookmark-this-page-broadcaster =
    [xul/label]      Bookmark This Page
edit-this-page-broadcaster =
    [xul/label]      Edit This Page
bookmark-this-page-key =
    [xul/key]        d
subscribe-to-page-menuitem =
    [xul/label]      Subscribe to This Page…
subscribe-to-page-menupopup =
    [xul/label]      Subscribe to This Page…
add-cur-pages-menuitem =
    [xul/label]      Bookmark All Tabs…
recent-bookmarks-menuitem =
    [xul/label]      Recently Bookmarked

other-bookmarks-menu =
    [xul/label]      Other Bookmarks
personalbar-menu =
    [xul/label]      Bookmarks Toolbar
    [xul/accesskey]  B


[[ Tools menu ]]

tools-menu =
    [xul/label]      Tools
    [xul/accesskey]  T
downloads-menuitem =
    [xul/label]      Downloads
    [xul/accesskey]  D
downloads-key =
    [xul/key]        j
downloads-key-unix =
    [xul/key]        y
addons-menuitem =
    [xul/label]      Add-ons
    [xul/accesskey]  A
addons-key =
    [xul/key]        A

sync-sign-in-menuitem =
    [xul/label]      Sign In To { sync-brand-short-name }…
    [xul/accesskey]  Y
sync-sync-now-menuitem =
    [xul/label]      Sync Now
    [xul/accesskey]  S
sync-re-auth-menuitem =
    [xul/label]      Reconnect to { sync-brand-short-name }…
    [xul/accesskey]  R
sync-toolbar-button =
    [xul/label]      Sync

web-developer-menu =
    [xul/label]      Web Developer
    [xul/accesskey]  W

page-source-broadcaster =
    [xul/label]      Page Source
    [xul/accesskey]  o
page-source-key =
    [xul/key]        u
page-info-menuitem =
    [xul/label]      Page Info
    [xul/accesskey]  I
page-info-key =
    [xul/key]        i
mirror-tab-menu =
    [xul/label]      Mirror Tab
    [xul/accesskey]  m


# browser/locales/en-US/browser/toolbar.ftl

urlbar-textbox =
    [xul/placeholder] Search or enter address
    [xul/accesskey]   d


[[ Toolbar items ]]

view-bookmarks-broadcaster =
    [xul/label]      Bookmarks
view-bookmarks-key =
    [xul/key]        b
view-bookmarks-key-win =
    [xul/key]        i

view-history-broadcaster =
    [xul/label]      History
view-history-key =
    [xul/key]        h
view-tabs-broadcaster =
    [xul/label]      Synced Tabs


# browser/branding/official/locales/en-US/brand.ftl

brand-shorter-name    = Firefox
brand-short-name      = Firefox
brand-full-name       = Mozilla Firefox
vendor-short-name     = Mozilla

trademark-info        = 
  | Firefox and the Firefox logos are trademarks of the Mozilla Foundation.

sync-brand-short-name = Sync
