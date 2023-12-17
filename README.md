# Embedded Rust Example for ESP32


## Useful Commands for ARM Cortex-M Development and Debugging

### Setting Up and Running OpenOCD

- **Starting OpenOCD for STMicroelectronics NUCLEO Boards:**
  ```bash
  openocd -f board/st_nucleo_f7.cfg

### Starting and Using GDB

- **Starting GDB with Your Binary:**
  ```bash
  arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/your_project_name
  ```

- **Connecting GDB to OpenOCD:**
  ```gdb
  target remote :3333
  ```

- **Loading the Program onto the Board:**
  ```gdb
  load
  ```

- **Running the Program:**
  ```gdb
  continue
  ```

- **Interrupting the Program Execution:**
  ```gdb
  Ctrl+C
  ```

- **Detaching GDB from the Target:**
  ```gdb
  detach
  ```

- **Exiting GDB:**
  ```gdb
  quit
  ```

### Additional Debugging Commands

- **Disassembling the Code:**
  ```bash
  arm-none-eabi-objdump -Cd target/thumbv7em-none-eabihf/debug/your_project_name
  ```

- **Reading ELF File Headers:**
  ```bash
  arm-none-eabi-readelf -h target/thumbv7em-none-eabihf/debug/your_project_name
  ```

- **Setting a Breakpoint:**
  ```gdb
  break [function_name or file_name:line_number]
  ```

- **Viewing the Source Code:**
  ```gdb
  list
  ```

- **Stepping Through the Code:**
    - **Next Line (Step Over):**
      ```gdb
      next
      ```
    - **Into a Function (Step In):**
      ```gdb
      step
      ```

- **Inspecting Variables:**
  ```gdb
  print [variable_name]
  ```

- **Examining Memory:**
  ```gdb
  x/[length][format] [address]
  ```

- **Viewing Register Values:**
  ```gdb
  info registers
  ```

