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
    cargo run --release
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


## References

* https://gbdev.io/pandocs/
* https://gekkio.fi/files/gb-docs/gbctr.pdf
* https://github.com/retrio/gb-test-roms
* https://izik1.github.io/gbops/
* https://www.youtube.com/watch?v=HyzD8pNlpwI
