try to create program which prints something like

```
===============================
01.02.2023 Monday
-------------------------------
01.02.2023 Tuesday
-------------------------------
01.02.2023 Wednesday
-------------------------------
01.02.2023 Thursday
-------------------------------
01.02.2023 Friday
-------------------------------
01.02.2023 Saturday						##
-------------------------------
01.02.2023 Sunday							##
===============================
```

New things I have learned:

- constants declare constant values. These represent a value, not a memory address. This is the most common thing one would reach for and would replace static as we know it today in almost all cases.
- statics declare global variables. These represent a memory address. They would be rarely used: the primary use cases are global locks, global atomic counters, and interfacing with legacy C libraries.
