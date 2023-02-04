# Changelog

### Latest/Nightly (this branch)

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