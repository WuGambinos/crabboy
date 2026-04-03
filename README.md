# CrabBoy
Gameboy emulator written in Rust using imgui-rs for GUI

## Build
```bash
    git clone https://github.com/WuGambinos/crabboy.git
    cd crabboy
    cargo build --release
```

## :joystick: Run
```bash 
    cd crabboy
    cd crabboy-gui
    cargo run --release -- --rom /path/to/rom
```
# Features 
* Save states
  
## TODO 
* Audio 
* GBC 

## Tests

### Blargg's

| Test              | passed/failed/NA |
| ----------------- | ---------------- |
| cpu_instrs        | ✅        |
| dmg_sound         | NA               |
| instr_timing      | ✅             |
| interrupt_time    | Failed               |
| mem_timing        | ✅        |
| mem_timing-2      | ✅           |
| oam_bug           | Failed               |
| dmg-acid2         | ✅ (1 bug)   |
| halt_bug          | Failed           |
