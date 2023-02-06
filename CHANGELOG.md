# Changelog

### Latest/Nightly (this branch)

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