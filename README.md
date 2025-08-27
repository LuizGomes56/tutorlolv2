# TutorLoL v2

### Pending features
- Code to generate champions automatically after every update
- Code to generate item damages automatically
- Check if calculations are acceptably accurate
- Save data from `realtime` in the database to allow others recover it by using `game_id` or `game_code`

### Current stage
- Overlay for `realtime` feature
- Keyboard controls to enable/disable Window during the game (it must be in Borderless mode)

### What is done
- Calculator component
- Formulas
- Help guide
- Settings (maybe)
- Code import/export to frontend

### Current binary sizes
- Frontend: 1.4MB (raw), 385Kb (compressed)
- Backend: 3MB (raw), 6.5Kb (including images)
- Calculator average payload size = 1.2Kb
- Realtime average payload size = 150Kb
- 85Kb static cached bytes (formulas as html). Expands to 5.6Mb on initialization

### Performance
- 10 micro-seconds for each `calculator` call
- 220 micro-seconds for each `realtime` call
- 0.21s page startup time
- 12 hours to convert all images to AVIF
