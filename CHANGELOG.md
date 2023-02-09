# Changelog

### Latest/Nightly (this branch)
- Features
    - Added changelog to About/Help page.
    - Header Tool and Server Info Tool fields can now be copied by clicking them, closes #17.
    - Most table fields can now be selected & copied, closes #18.
- Changes
    - Flags, Hi-Value, and Lo-Value for SendTable fields now display in hex.
- Internal
    - Set egui to version 0.21.0
    - Set egui_extras to version 0.21.0
    - Set source-demo-tool to version 0.8.0

### v0.7.0
- Features
    - Added Sign On Frames tool.
    - Added DataTables viewmodel.

### v0.6.0
- Features
    - UserMessages and GameEvents now display their sub-message names in the Frames tool, closes #21.
    - Added Help/About page, closes #23.
- Bug Fixes
    - Fixed filters not playing nice with "Goto" links, closes #26.
- Changes
    - Moved gui initialization to its own thread (stops loading spinner from hitching), closes #24.
    - Made 'hide None values' sticky, closes #20.
- Internal
    - set source-demo-tool version to 0.7.3

### v0.5.2
- Features
    - Game Events now link from frames to individual entries in the Game Events Tool and vice-versa, closes #13.
    - Added filtering for User Messages, closes #10.
    - Added filtering for Game Events, closes #15.
- Internal
    - set source-demo-tool version to 0.7.1

### v0.5.1
- Features
    - User Messages Tool now lists tick and time.
- Changes
    - User Messages Tool ui now behaves closer to frames/game events too, closes #12
    - consolidated various table widths, closes #14
- Internal
    - set source-demo-tool version to 0.5.0

### v0.5.0
- Features
    - Added Game Events Tool
- Internal
    - set source-demo-tool version to 0.4.2 (doesn't break anything).

### v0.4.2
- Features
    - Added Server Info tool. close #3
- Internal
    - set dependency RFD version to 0.11.0

### v0.4.1
- Features
    - Protobuf messages (frames/user messages tool) now show their message index.
    - Added UI Scale (pixels per point/ppt) header. closes #5

### v0.4.0
- Features
    - User messages now link out to there respective frame and message. #2
    - User messages inside the frames tool now link to their respective entry in the user messages tool. #2
- Bug Fixes
    - Fixed selecting non-packet frame not removing frame detail panel. closes #1
- Internal
    - set source-demo-tool version to 0.4.1.
        - Affects UserMessagesToolViewModel, addressed in message index feature.

### v0.3.0
- Internal
    - set source-demo-tool version to 0.3.0.
        - Affects ProtobufMessagesViewModel. No functional change.