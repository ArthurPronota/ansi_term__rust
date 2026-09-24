# Применение `ansi_term` в Rust

## Что такое `ansi_term`

**`ansi_term`** — крейт для **цветного** и **стилизованного** вывода в терминал через **ANSI escape-коды**. Позволяет делать текст **жирным**, **подчёркнутым**, **курсивом**, с **цветом** и **фоном**.

## Установка

```toml
[dependencies]
ansi_term = "0.12"
```

Или:

```bash
cargo add ansi_term
```

## Основные типы

| Тип | Назначение |
|---|---|
| **`Color`** | **Цвет** текста или фона |
| **`Style`** | **Стиль** (жирный, курсив, подчёркивание, ...) |
| **`ANSIString`** | Строка с ANSI-кодами (результат `paint`) |

## Разбор примера

### 1. Цветной текст

```rust
use ansi_term::Color;

println!("Red: {}, Blue: {}, Green: {}", 
    Color::Red.paint("red"),
    Color::Blue.paint("blue"),
    Color::Green.paint("green"),
);
```

**Вывод:** `red` — **красный**, `blue` — **синий**, `green` — **зелёный**.

- **`Color::Red`** — цвет.
- **`.paint("...")`** — применяет цвет к строке.
- Возвращает **`ANSIString`**, который реализует `Display`.

### 2. Жирный текст

```rust
use ansi_term::Style;

println!("Bold text: {}", Style::new().bold().paint("bold color"));
```

**Вывод:** `bold color` — **жирный**.

- **`Style::new()`** — новый стиль.
- **`.bold()`** — добавить **жирный**.
- **`.paint("...")`** — применить.

### 3. Цвет + жирный

```rust
use ansi_term::Color;

println!("{}", Color::Yellow.bold().paint("bold yellow"));
```

**Вывод:** `bold yellow` — **жёлтый** + **жирный**.

- **`Color::Yellow`** — цвет.
- **`.bold()`** — метод `Color`, возвращает `Style` с цветом + жирным.
- **`.paint("...")`** — применить.

## Доступные цвета

### Обычные (8 цветов)

| Цвет | Код |
|---|---|
| **Чёрный** | `Color::Black` |
| **Красный** | `Color::Red` |
| **Зелёный** | `Color::Green` |
| **Жёлтый** | `Color::Yellow` |
| **Синий** | `Color::Blue` |
| **Пурпурный** | `Color::Purple` |
| **Голубой** | `Color::Cyan` |
| **Белый** | `Color::White` |

### Яркие (bright)

| Цвет | Код |
|---|---|
| **Ярко-чёрный** | `Color::Fixed(8)` |
| **Ярко-красный** | `Color::Fixed(9)` |
| **Ярко-зелёный** | `Color::Fixed(10)` |
| **Ярко-жёлтый** | `Color::Fixed(11)` |
| **Ярко-синий** | `Color::Fixed(12)` |
| **Ярко-пурпурный** | `Color::Fixed(13)` |
| **Ярко-голубой** | `Color::Fixed(14)` |
| **Ярко-белый** | `Color::Fixed(15)` |

### 256 цветов

```rust
Color::Fixed(196)   // ярко-красный из 256-палитры
```

### RGB

```rust
Color::RGB(255, 128, 0)   // оранжевый
```

## Доступные стили

| Метод | Что делает |
|---|---|
| **`.bold()`** | **Жирный** |
| **`.dimmed()`** | **Тусклый** |
| **`.italic()`** | **Курсив** |
| **`.underline()`** | **Подчёркнутый** |
| **`.blink()`** | **Мигающий** |
| **`.reverse()`** | **Инвертированный** |
| **`.hidden()`** | **Скрытый** |
| **`.strikethrough()`** | **Зачёркнутый** |

### Комбинирование

```rust
Style::new()
    .bold()
    .italic()
    .underline()
    .paint("bold italic underline")
```

## Фон

### `.on(color)` — цвет фона

```rust
Color::White.on(Color::Red).paint("White on Red")
```

**Вывод:** **белый** текст на **красном** фоне.

### С `Style`

```rust
Style::new()
    .bold()
    .on(Color::Blue)
    .paint("Bold on Blue")
```

## Полный пример

```rust
use ansi_term::{Color, Style};

fn main() {
    // === Цвета ===
    println!("{}", Color::Black.paint("Black"));
    println!("{}", Color::Red.paint("Red"));
    println!("{}", Color::Green.paint("Green"));
    println!("{}", Color::Yellow.paint("Yellow"));
    println!("{}", Color::Blue.paint("Blue"));
    println!("{}", Color::Purple.paint("Purple"));
    println!("{}", Color::Cyan.paint("Cyan"));
    println!("{}", Color::White.paint("White"));

    // === Яркие (256) ===
    println!("{}", Color::Fixed(196).paint("Bright Red"));
    println!("{}", Color::Fixed(46).paint("Bright Green"));

    // === RGB ===
    println!("{}", Color::RGB(255, 128, 0).paint("Orange RGB"));

    // === Стили ===
    println!("{}", Style::new().bold().paint("Bold"));
    println!("{}", Style::new().italic().paint("Italic"));
    println!("{}", Style::new().underline().paint("Underline"));
    println!("{}", Style::new().dimmed().paint("Dimmed"));
    println!("{}", Style::new().strikethrough().paint("Strikethrough"));

    // === Комбинации ===
    println!("{}", Color::Yellow.bold().paint("Bold Yellow"));
    println!("{}", Color::Red.italic().underline().paint("Red Italic Underline"));

    // === Фон ===
    println!("{}", Color::White.on(Color::Red).paint("White on Red"));
    println!("{}", Color::Black.on(Color::Yellow).paint("Black on Yellow"));

    // === Стиль + фон ===
    println!("{}", Style::new().bold().on(Color::Blue).paint("Bold on Blue"));
}
```

## Полезные приёмы

### 1. Сохранение в переменную

```rust
let s = Color::Red.paint("Error");
println!("{}", s);
```

**`ANSIString`** можно **сохранить** и **использовать** многократно.

### 2. Форматирование с `format!`

```rust
let msg = format!("{} {}", 
    Color::Red.paint("ERROR:"), 
    "something failed"
);
println!("{}", msg);
```

### 3. Отключение цветов

```rust
use ansi_term::enable_ansi_support;

// На Windows 10+ — включить ANSI
#[cfg(windows)]
enable_ansi_support().ok();
```

### 4. Проверка, поддерживает ли терминал цвета

```rust
use std::env;

fn supports_color() -> bool {
    env::var("NO_COLOR").is_err()
}
```

**Стандарт:** переменная окружения `NO_COLOR` — если установлена, цвета **отключаются**.

## Сравнение с другими крейтами

| Крейт | Синтаксис | Особенности |
|---|---|---|
| **`ansi_term`** | `Color::Red.paint("x")` | Старый, простой |
| **`colored`** | `"x".red()` | **Проще**, метод на строке |
| **`owo-colors`** | `"x".red()` | **Быстрый**, без аллокаций |
| **`termcolor`** | `Color::Red` | **Кроссплатформенный** |
| **`crossterm`** | `style("x").red()` | Полный TUI |

### Пример с `colored`

```rust
use colored::Colorize;

println!("{}", "Red".red());
println!("{}", "Bold".bold());
println!("{}", "Bold Red".red().bold());
```

**Короче** — методы прямо на строке.

## Проблемы

### 1. **Windows cmd.exe**

Старый `cmd.exe` **не поддерживает** ANSI по умолчанию.

**Решение:**

- **Windows 10+** — работает в Windows Terminal, PowerShell 7+.
- Включить ANSI через `enable_ansi_support`.

### 2. **Вывод в файл**

ANSI-коды **попадут** в файл:

```
\x1b[31mRed\x1b[0m
```

**Решение:** отключайте цвета при `NO_COLOR` или проверке `isatty`.

### 3. **Логи**

Если логи пишутся в файл — **цвета не нужны**.

## Сводная таблица

| Элемент | Описание |
|---|---|
| **`Color::Red`** | Цвет |
| **`Style::new()`** | Стиль |
| **`.bold()`** | Жирный |
| **`.italic()`** | Курсив |
| **`.underline()`** | Подчёркивание |
| **`.on(color)`** | Фон |
| **`.paint("...")`** | Применить |
| **`Color::Fixed(n)`** | 256-палитра |
| **`Color::RGB(r,g,b)`** | RGB |

## Итог

- **`ansi_term`** — крейт для **цветного** вывода в терминал.
- **`Color`** — цвет текста, **`Style`** — стиль.
- **`.paint("...")`** — применить к строке.
- **8 обычных** + **256** + **RGB** цветов.
- **Стили:** bold, italic, underline, dimmed, strikethrough и др.
- **Фон:** `.on(Color::Red)`.
- **Комбинации:** `Color::Yellow.bold().paint(...)`.
- **Проблемы:** Windows cmd.exe, вывод в файл.
- **Альтернативы:** `colored`, `owo-colors`, `termcolor` — **проще** и **быстрее**.
- **Стандарт:** `NO_COLOR` — для отключения цветов.
