# Creating TextFSM Templates

TextFSM templates are the core of the parsing engine. They define how raw text is processed and extracted into structured data. A template consists of **Value Definitions** and **State Definitions**.

## 1. Value Definitions

These lines appear at the top of the file and define the fields you want to extract.

**Syntax:**
```textfsm
Value [Option[,Option...]] Name (Regex)
```

-   **Name**: The variable name (e.g., `Version`, `Interface`).
-   **Regex**: A Regular Expression (PCRE/Rust-style) to capture the value. It must be enclosed in parentheses if you want to capture specific parts, though TextFSM often treats the whole regex as the capture group if not specified.
-   **Options** (Optional): Comma-separated flags affecting behavior.

### Supported Options

| Option | Description |
| :--- | :--- |
| `Key` | Marks this value as part of the unique identifier (primary key) for the record. |
| `Required` | The record will only be saved if this value has been matched. |
| `Filldown` | Once matched, the value is copied to subsequent records until changed (useful for headers like "VLAN ID"). |
| `Fillup` | The value is copied *upwards* to previous records in the current block (less common). |
| `List` | Allows multiple matches for this value within a single record (e.g., multiple IP addresses per interface). |

**Examples:**
```textfsm
Value Required Hostname (\S+)
Value List IpAddress ([0-9.]+)
Value Filldown Interface (\S+)
```

## 2. State Definitions

After values are defined, the template must define at least one state: `Start`.

**Syntax:**
```textfsm
StateName
  ^RuleRegex -> Action
```

-   **StateName**: A label for a block of rules. The parser starts in the `Start` state.
-   **RuleRegex**: A regex to match the current line. It can contain variables like `${Name}`.
-   **Action** (Optional): What to do when the line matches.

### Rules

Rules are processed in order. If a line matches a rule, the Action is executed. If no action is specified, `Next` is implied (move to next line).

**Syntax:** `^Regex [-> Action]`

-   Use `${ValueName}` in the regex to capture data into a defined Value.
-   The regex usually starts with `^` to match the start of the line.

### Actions

Actions control the flow of the state machine and record generation.

**Format:** `Action` or `NextState` or `Action.NextState`

| Action | Description |
| :--- | :--- |
| `Next` | (Default) Finish with the current line and read the next line of input. |
| `Continue` | Keep processing the *current* line with subsequent rules in the same state. |
| `Record` | Save the current values as a `DataRecord`, clear non-Filldown values, and start a new record. |
| `NoRecord` | Do not save the record (often used with state transitions). |
| `Clear` | Clear current values (except Filldown). |
| `Clearall` | Clear *all* values (including Filldown). |
| `StateName` | Transition to a new state definition (e.g., `Start`, `ParseInterface`, `EOF`). |

**Examples:**
```textfsm
  # Capture uptime and move to next line (default action)
  ^Uptime is ${Uptime}

  # Capture Interface and transition to ParseInterface state
  ^Interface ${Interface} -> ParseInterface

  # Record the current data and continue parsing
  ^  IP Address: ${IpAddress} -> Record
```

## 3. Special States

-   **`Start`**: Mandatory. The parser begins here.
-   **`EOF`**: Optional. Implicitly called when input ends. Useful for saving the last record if it wasn't triggered by a `Record` action.

## 4. Complete Example

**Input (`sh version`):**
```text
Cisco IOS Software, C3750 Software (C3750-IPSERVICESK9-M), Version 12.2(46)SE, RELEASE SOFTWARE (fc2)
Copyright (c) 1986-2008 by Cisco Systems, Inc.
cisco WS-C3750G-24TS (PowerPC405) processor (revision R0) with 131072K bytes of memory.
Model number: WS-C3750G-24TS-1U
System serial number: FDO12345678
```

**Template:**
```textfsm
# Define Values
Value Version (\S+)
Value Model (\S+)
Value Serial (\S+)

# Start State
Start
  ^Cisco IOS Software.*Version ${Version}, -> Continue
  ^Model number: ${Model}
  ^System serial number: ${Serial} -> Record
```

**Explanation:**
1.  `Value` definitions create slots for Version, Model, and Serial.
2.  `Start` state begins processing lines.
3.  Line 1 matches `Cisco IOS...`. It extracts `12.2(46)SE` into `Version`. `-> Continue` means it keeps looking for other rules on *this same line* (though none match here).
4.  Line 4 matches `Model number`. Extracts `WS-C3750G-24TS-1U`.
5.  Line 5 matches `System serial`. Extracts `FDO12345678`. `-> Record` saves the row {Version, Model, Serial} and resets variables.

## 5. Tips & Best Practices

1.  **Anchor Regexes**: Always start rules with `^` to avoid accidental mid-line matches.
2.  **Order Matters**: Put specific rules before general ones. The first match wins unless `Continue` is used.
3.  **Whitespace**: Use `\s+` or explicit spaces in regex to match whitespace in the input.
4.  **Testing**: Use the `textfsm-rs` tests or standard regex tools (like Regex101, selecting Python flavor) to verify your regex patterns.
5.  **Variables**: Use `${Var}` for simple matching.
