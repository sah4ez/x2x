# План рефакторинга x2x с C на Rust

## Обзор проекта

**Текущее состояние:**
- ~4,284 строки C кода
- Основной файл: `x2x.c` (116 KB, ~3,600 строк)
- Дополнительные файлы: `keymap.c`, `keymap.h`, `winmsg.c`
- Зависимости: X11, Xext, Xtst (X Test extension)
- Опционально: Win32/Cygwin поддержка через `WIN_2_X`

**Назначение:**
Утилита для X Window System, позволяющая использовать клавиатуру и мышь одного X-дисплея для управления другим, с поддержкой clipboard sharing.

## Архитектура текущего C кода

### Основные структуры данных

```c
DPYINFO     // Глобальное состояние обоих дисплеев
SHADOW      // Теневые дисплеи для управления несколькими экранами
FAKE        // Очередь симулированных событий (клавиши/кнопки)
STICKY      // Sticky keys
```

### Основные функции

```c
main()           // Инициализация, открытие дисплеев
DoX2X()          // Основной event loop
ProcessEvent()  // Диспетчер событий
ProcessMotionNotify()  // Обработка движения мыши
ProcessButtonPress()    // Обработка нажатий кнопок
ProcessKeyEvent()      // Обработка клавиатурных событий
ProcessSelectionRequest() // Clipboard handling
FakeThingsUp()   // Эмуляция событий через XTest
```

### Ключевые компоненты

1. **Event Loop**: select + XPending для обработки событий
2. **Coordinate Mapping**: Таблицы преобразования координат
3. **XTest Integration**: Эмуляция ввода
4. **Clipboard Sharing**: X Selection механизм
5. **Multi-screen**: Поддержка нескольких мониторов
6. **Win32 Support**: Cygwin/Windows интеграция

## План миграции на Rust

### Фаза 1: Подготовка окружения (1-2 дня)

#### 1.1 Выбор Rust библиотек для X11

**Обязательные зависимости:**
- `x11-dl` - FFI биндинги к Xlib (или `x11-rs`)
- `x11-clipboard` - Работа с X clipboard
- `nix` - Системные вызовы (select, signal handling)
- `anyhow` / `thiserror` - Обработка ошибок

**Опциональные зависимости:**
- `clap` - CLI аргументы
- `log` + `env_logger` - Логирование
- `tokio` / `async-std` - Асинхронный event loop (опционально)

#### 1.2 Создание структуры проекта

```
x2x-rust/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── x11/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── event.rs
│   │   ├── extension.rs
│   │   └── selection.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── dpy_info.rs
│   │   ├── coord_mapping.rs
│   │   └── state.rs
│   ├── input/
│   │   ├── mod.rs
│   │   ├── keyboard.rs
│   │   ├── mouse.rs
│   │   └── fake.rs
│   ├── clipboard/
│   │   ├── mod.rs
│   │   └── x11_selection.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── errors.rs
│   │   └── config.rs
│   └── win32/ (опционально, через feature flag)
│       ├── mod.rs
│       ├── keymap.rs
│       └── window.rs
├── docs/
│   ├── host_based_auth.md
│   ├── ssh_tunneling.md
│   └── usage.md
└── tests/
    ├── integration/
    └── unit/
```

#### 1.3 Feature Flags для гибкости

```toml
[features]
default = ["x11"]
x11 = []
win32 = ["x11", "dep:windows"]
```

---

### Фаза 2: X11 биндинги и базовый фреймворк (2-3 дня)

#### 2.1 Создание безопасных оберток над Xlib

**Задачи:**
- Обернуть X11 connection в RAII тип
- Безопасная работа с Window, Display, GC
- Методы для основных X11 операций

**Пример структуры:**

```rust
// src/x11/connection.rs
pub struct X11Connection {
    display: *mut Display,
    screen: i32,
}

impl X11Connection {
    pub fn open(display_name: Option<&str>) -> Result<Self, X11Error>;
    pub fn screen(&self) -> i32;
    pub fn root_window(&self) -> Window;
    pub fn flush(&self) -> Result<(), X11Error>;
    pub fn pending(&self) -> i32;
    pub fn next_event(&self) -> XEvent;
}

impl Drop for X11Connection {
    fn close(&mut self) {
        unsafe { XCloseDisplay(self.display) };
    }
}
```

#### 2.2 Event handling система

**Задачи:**
- Enum для всех типов X событий
- Трейт для обработки событий
- Event loop с использованием `select` / `poll`

```rust
// src/x11/event.rs
pub enum XEvent {
    MotionNotify(XMotionEvent),
    ButtonPress(XButtonEvent),
    KeyPress(XKeyEvent),
    SelectionRequest(XSelectionRequestEvent),
    // ... остальные события
}

pub trait EventHandler {
    fn handle(&self, event: &XEvent, ctx: &mut Context) -> Result<bool, Error>;
}

// Event loop
pub fn run_event_loop(
    from_conn: &X11Connection,
    to_conn: &X11Connection,
    handlers: Vec<Box<dyn EventHandler>>,
) -> Result<(), Error>;
```

#### 2.3 XTest extension интеграция

**Задачи:**
- Проверка наличия XTest extension
- Безопасные обертки для XTestFakeMotionEvent, XTestFakeButtonEvent, XTestFakeKeyEvent

```rust
// src/x11/extension.rs
pub struct XTestExtension;

impl XTestExtension {
    pub fn check_available(conn: &X11Connection) -> bool;
    pub fn fake_motion(
        conn: &X11Connection,
        screen: i32,
        x: i32,
        y: i32,
    ) -> Result<(), Error>;

    pub fn fake_button(
        conn: &X11Connection,
        button: u32,
        is_press: bool,
    ) -> Result<(), Error>;

    pub fn fake_key(
        conn: &X11Connection,
        keycode: u8,
        is_press: bool,
    ) -> Result<(), Error>;
}
```

---

### Фаза 3: Портинг основных структур данных (2-3 дня)

#### 3.1 DPYINFO → DpyInfo

**Задачи:**
- Перевести C struct в Rust struct с Option для nullable полей
- Использовать `Arc<Mutex<>>` для потокобезопасности (если нужно)
- Безопасные типы для координат, KeySym, KeyCode

```rust
// src/core/dpy_info.rs
use std::collections::VecDeque;

#[derive(Debug)]
pub struct DpyInfo {
    // From display
    pub from_conn: Arc<X11Connection>,
    pub from_root: Window,
    pub from_trigger: Option<Window>,
    pub from_big: Option<Window>,
    pub cursor: Option<Cursor>,

    // To display
    pub to_conn: Arc<X11Connection>,
    pub to_root: Window,

    // State
    pub mode: ConnectionMode,
    pub to_screen: usize,
    pub last_from_coord: i32,
    pub unreasonable_delta: i32,

    // Coordinate mapping
    pub x_tables: Vec<Vec<Short>>,
    pub y_tables: Vec<Vec<Short>>,
    pub from_conn_coord: i32,
    pub from_disc_coord: i32,

    // Selection state
    pub selection_state: SelectionState,

    // Input state tracking
    pub fake_queue: VecDeque<FakeEvent>,
    pub button_mapping: [u8; 20],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionMode {
    Disconnected,
    Connected,
}
```

#### 3.2 SHADOW → ShadowDisplay

```rust
// src/core/dpy_info.rs
pub struct ShadowDisplay {
    pub name: String,
    pub conn: Arc<X11Connection>,
    pub led_mask: u64,
    pub flush_required: bool,
    pub dpms_status: DpmsStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DpmsStatus {
    Unknown,
    NotSupported,
    Supported { level: u16 },
}
```

#### 3.3 FAKE → FakeEvent

```rust
// src/core/state.rs
#[derive(Debug, Clone)]
pub enum FakeEvent {
    Key {
        keysym: KeySym,
        keycode: u8,
        is_press: bool,
    },
    Button {
        button: u32,
        is_press: bool,
    },
}

pub struct FakeQueue {
    events: VecDeque<FakeEvent>,
}
```

---

### Фаза 4: Coordinate Mapping (2-3 дня)

#### 4.1 Портинг таблиц преобразования

**Задачи:**
- Перенести логику построения таблиц `xTables`/`yTables`
- Оптимизировать с использованием Rust iterators
- Добавить тесты для edge cases

```rust
// src/core/coord_mapping.rs
pub struct CoordinateMapping {
    pub x_tables: Vec<Vec<i16>>,
    pub y_tables: Vec<Vec<i16>>,
    pub n_screens: usize,
    pub from_width: u32,
    pub from_height: u32,
}

impl CoordinateMapping {
    pub fn new(
        from_screen: &ScreenInfo,
        to_screens: &[ScreenInfo],
        mode: LayoutMode,
    ) -> Result<Self, Error> {
        // Построение таблиц преобразования
        let x_tables = Self::build_x_tables(from_screen, to_screens, mode)?;
        let y_tables = Self::build_y_tables(from_screen, to_screens, mode)?;

        Ok(Self {
            x_tables,
            y_tables,
            n_screens: to_screens.len(),
            from_width: from_screen.width,
            from_height: from_screen.height,
        })
    }

    pub fn map_x(&self, from_x: i32, to_screen: usize) -> i32 {
        // Логика маппинга с проверкой COORD_INCR/COORD_DECR
    }

    pub fn map_y(&self, from_y: i32, to_screen: usize) -> i32 {
        // Логика маппинга
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Horizontal { direction: Direction },
    Vertical { direction: Direction },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
```

---

### Фаза 5: Input Processing (3-4 дня)

#### 5.1 Mouse event handling

**Задачи:**
- Перенести `ProcessMotionNotify` логику
- Обработка multi-screen переключений
- Sanity checks для движения курсора

```rust
// src/input/mouse.rs
pub struct MouseHandler {
    mapping: Arc<CoordinateMapping>,
}

impl EventHandler for MouseHandler {
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool, Error> {
        match event {
            XEvent::MotionNotify(motion) => {
                self.handle_motion(motion, ctx)
            }
            XEvent::ButtonPress(btn) => {
                self.handle_button_press(btn, ctx)
            }
            XEvent::ButtonRelease(btn) => {
                self.handle_button_release(btn, ctx)
            }
            _ => Ok(false),
        }
    }
}

impl MouseHandler {
    fn handle_motion(
        &self,
        event: &XMotionEvent,
        ctx: &mut DpyInfo,
    ) -> Result<bool, Error> {
        // Логика ProcessMotionNotify:
        // 1. Проверка same_screen
        // 2. Маппинг координат через таблицы
        // 3. Sanity check (unreasonableDelta)
        // 4. Обработка COORD_INCR/COORD_DECR
        // 5. Фейк движение на to display
    }
}
```

#### 5.2 Keyboard event handling

**Задачи:**
- Перенести `ProcessKeyEvent`
- XKeySym маппинг
- Sticky keys поддержка
- Modifier keys handling

```rust
// src/input/keyboard.rs
pub struct KeyboardHandler {
    sticky_keys: HashSet<KeySym>,
    modifier_map: ModifierMap,
}

impl EventHandler for KeyboardHandler {
    fn handle(&self, event: &XEvent, ctx: &mut DpyInfo) -> Result<bool, Error> {
        match event {
            XEvent::KeyPress(key) => {
                self.handle_key_press(key, ctx)
            }
            XEvent::KeyRelease(key) => {
                self.handle_key_release(key, ctx)
            }
            _ => Ok(false),
        }
    }
}

impl KeyboardHandler {
    fn handle_key_press(
        &self,
        event: &XKeyEvent,
        ctx: &mut DpyInfo,
    ) -> Result<bool, Error> {
        // Получить KeySym из KeyCode
        // Проверить sticky keys
        // Отправить на to display через XTest
    }
}
```

#### 5.3 Fake queue processing

**Задачи:**
- Перенести `FakeThingsUp` логику
- Отслеживание состояния клавиш/кнопок

```rust
// src/input/fake.rs
pub struct FakeQueue {
    events: VecDeque<FakeEvent>,
    active_keys: HashSet<KeySym>,
    active_buttons: HashSet<u32>,
}

impl FakeQueue {
    pub fn process(&mut self, conn: &X11Connection) -> Result<(), Error> {
        while let Some(event) = self.events.pop_front() {
            match event {
                FakeEvent::Key { keysym, keycode, is_press } => {
                    self.process_key_event(conn, keysym, keycode, is_press)?;
                }
                FakeEvent::Button { button, is_press } => {
                    self.process_button_event(conn, button, is_press)?;
                }
            }
        }
        Ok(())
    }
}
```

---

### Фаза 6: Clipboard Sharing (3-4 дня)

#### 6.1 X Selection mechanism

**Задачи:**
- Перенести clipboard логику
- Поддержка PRIMARY, SECONDARY, CLIPBOARD
- UTF-8 encoding

```rust
// src/clipboard/x11_selection.rs
pub struct X11Clipboard {
    conn: Arc<X11Connection>,
    selections: HashMap<Atom, SelectionState>,
}

impl X11Clipboard {
    pub fn new(conn: Arc<X11Connection>) -> Result<Self, Error> {
        // Инициализация atom'ов
        Ok(Self {
            conn,
            selections: HashMap::new(),
        })
    }

    pub fn handle_selection_request(
        &mut self,
        event: &XSelectionRequestEvent,
        ctx: &mut DpyInfo,
    ) -> Result<(), Error> {
        // Логика ProcessSelectionRequest
    }

    pub fn handle_selection_notify(
        &mut self,
        event: &XSelectionEvent,
        ctx: &mut DpyInfo,
    ) -> Result<(), Error> {
        // Логика ProcessSelectionNotify
    }

    pub fn handle_selection_clear(
        &mut self,
        event: &XSelectionClearEvent,
        ctx: &mut DpyInfo,
    ) -> Result<(), Error> {
        // Логика ProcessSelectionClear
    }

    pub fn convert_selection(
        &mut self,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Time,
    ) -> Result<(), Error> {
        // XConvertSelection вызов
    }
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub owner: Option<Window>,
    pub data: Option<Vec<u8>>,
    pub timestamp: Time,
    pub revision: u32,
}
```

---

### Фаза 7: Connection Management (2-3 дня)

#### 7.1 Connect/Disconnect логика

**Задачи:**
- Перенести `DoConnect` / `DoDisconnect`
- Создание trigger окна на краю экрана
- Управление режимами соединения

```rust
// src/core/dpy_info.rs
impl DpyInfo {
    pub fn connect(&mut self) -> Result<(), Error> {
        // Логика DoConnect:
        // 1. Создать trigger окно на краю экрана
        // 2. Захватить указатель
        // 3. Переключить mode в Connected
        // 4. Сместить курсор на другой экран
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        // Логика DoDisconnect:
        // 1. Освободить указатель
        // 2. Уничтожить trigger окно
        // 3. Переключить mode в Disconnected
        // 4. Сместить курсор обратно на from экран
    }

    pub fn move_window_to_edge(&mut self) -> Result<(), Error> {
        // MoveWindowToEdge логика
    }

    pub fn move_window_to_screen(&mut self, screen_num: usize) -> Result<(), Error> {
        // MoveWindowToScreen логика
    }
}
```

---

### Фаза 8: CLI и конфигурация (1-2 дня)

#### 8.1 Парсинг аргументов

**Задачи:**
- Перенести `ParseCommandLine` логику
- Использовать `clap` для CLI

```rust
// src/utils/config.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "x2x")]
#[command(about = "Control X displays with shared keyboard/mouse", long_about = None)]
pub struct Config {
    /// Display to control from
    #[arg(short = 'from', long)]
    pub from_display: Option<String>,

    /// Display to control to
    #[arg(short = 'to', long)]
    pub to_display: String,

    /// Connect on screen edge: east, west, north, south
    #[arg(long)]
    pub edge: Option<EdgeDirection>,

    /// Wait for display if unavailable
    #[arg(short = 'w', long)]
    pub wait: bool,

    /// Block mouse buttons during auto-disconnect
    #[arg(long)]
    pub btn_block: bool,

    /// Vertical screen layout
    #[arg(long)]
    pub vertical: bool,

    /// Font name for status window
    #[arg(long)]
    pub font: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum EdgeDirection {
    East,
    West,
    North,
    South,
}
```

---

### Фаза 9: Win32 Support (опционально, 3-4 дня)

#### 9.1 Windows интеграция

**Задачи:**
- Перенести `keymap.c` логику в Rust
- Обработка Windows сообщений через `windows` crate
- Clipboard интеграция

```rust
// src/win32/keymap.rs
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct Win32KeyMap {
    mappings: HashMap<VIRTUAL_KEY, Vec<KeySym>>,
}

impl Win32KeyMap {
    pub fn new() -> Self {
        // Построение таблицы из C кода
        Self {
            mappings: Self::build_keymap_table(),
        }
    }

    pub fn map_virtual_key(
        &self,
        vk: VIRTUAL_KEY,
        key_data: u32,
    ) -> KeyAction {
        // Логика из PCtoX
    }
}

pub struct KeyAction {
    pub keysyms: Vec<KeySym>,
    pub release_modifiers: ModifierFlags,
}
```

```rust
// src/win32/window.rs
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct Win32Window {
    hwnd: HWND,
    edge_wnd: HWND,
    big_wnd: HWND,
}

impl Win32Window {
    pub fn create(&mut self) -> Result<(), Win32Error> {
        // Создание edge окна
        // Создание big окна для статуса
    }

    pub fn process_messages(&mut self) -> Result<bool, Win32Error> {
        // GetMessage / DispatchMessage loop
    }
}
```

---

### Фаза 10: Тестирование и отладка (4-5 дней)

#### 10.1 Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_mapping_horizontal() {
        let from = ScreenInfo { width: 1920, height: 1080 };
        let to = vec![ScreenInfo { width: 1920, height: 1080 }];

        let mapping = CoordinateMapping::new(
            &from,
            &to,
            LayoutMode::Horizontal { direction: Direction::Right },
        ).unwrap();

        // Проверки граничных условий
        assert_eq!(mapping.map_x(1919, 0), 0); // Порог переключения
        assert_eq!(mapping.map_x(1920, 0), COORD_INCR);
    }

    #[test]
    fn test_fake_queue() {
        let mut queue = FakeQueue::new();
        queue.push(FakeEvent::Key { keysym: XK_a, keycode: 10, is_press: true });

        // Проверить что события обрабатываются правильно
    }
}
```

#### 10.2 Integration тесты

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    #[ignore] // Требует запущенный X server
    fn test_x11_connection() {
        let conn = X11Connection::open(None).unwrap();
        assert!(conn.screen() >= 0);
    }
}
```

#### 10.3 Property-based testing (опционально)

Использовать `proptest` для тестирования координатных трансформаций.

---

### Фаза 11: Документация и полировка (2-3 дня)

#### 11.1 Документация

- Rustdoc для всех public API
- README с инструкциями по сборке
- Примеры использования
- Перенести docs/ в Markdown

#### 11.2 Оптимизации

- Профилирование с `perf` / `flamegraph`
- Оптимизация hot paths (coordinate mapping)
- Уменьшение аллокаций

#### 11.3 CI/CD

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      - run: sudo apt-get install -y libx11-dev libxext-dev libxtst-dev
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

---

## Оценка сроков

| Фаза | Срок | Зависимости |
|------|------|-------------|
| 1. Подготовка окружения | 1-2 дня | - |
| 2. X11 биндинги | 2-3 дня | Фаза 1 |
| 3. Основные структуры | 2-3 дня | Фаза 2 |
| 4. Coordinate Mapping | 2-3 дня | Фаза 3 |
| 5. Input Processing | 3-4 дня | Фаза 4 |
| 6. Clipboard Sharing | 3-4 дня | Фаза 2, 3 |
| 7. Connection Management | 2-3 дня | Фаза 3, 4 |
| 8. CLI и конфигурация | 1-2 дня | Фаза 1 |
| 9. Win32 Support | 3-4 дня | Фаза 2, 5, 8 |
| 10. Тестирование | 4-5 дней | Фазы 1-9 |
| 11. Документация | 2-3 дня | Все фазы |
| **Итого (без Win32)** | **24-31 день** | |
| **Итого (с Win32)** | **27-35 дней** | |

---

## Риски и стратегии mitigations

### 1. X11 FFI сложность

**Риск:** Работа с unsafe кодом, Xlib может быть нестабильной.

**Mitigation:**
- Изолировать unsafe код в минимальные модули
- Использовать существующие обертки (`x11-rs`, `x11-dl`)
- Много тестов

### 2. Производительность

**Риск:** Rust abstractions могут добавить оверхед.

**Mitigation:**
- Профилирование на ранних этапах
- Использовать `#[inline]` для hot paths
- Избегать избыточных аллокаций

### 3. Потеря функциональности

**Риск:** Несоответствие поведения с C версией.

**Mitigation:**
- Побежать обе версии параллельно и сравнить поведение
- Комплексное тестирование на разных конфигурациях
- Сохранить C версию как fallback

### 4. Legacy X11 особенности

**Риск:** Современные WM/DE ведут себя иначе чем в 1997.

**Mitigation:**
- Тестирование на популярных WM (i3, gnome, kde)
- Учесть современные X extensions (RandR, Xinerama)
- Возможно добавить Wayland поддержку в будущем

---

## Преимущества Rust версии

### Безопасность памяти

- Отсутствие buffer overflows
- Отсутствие use-after-free
- Thread safety без гонок данных

### Современный инструментари

- Cargo для зависимостей и сборки
- Linting с clippy
- Мощная система типов
- Pattern matching

### Поддерживаемость

- Модульная архитектура
- Автоматическое документирование
- Легче для новых контрибьюторов
- Меньше implicit behaviors

---

## Следующие шаги

1. **Создать начальный Cargo проект:**
   ```bash
   cargo new x2x-rust --lib
   cd x2x-rust
   ```

2. **Добавить базовые зависимости:**
   ```toml
   [dependencies]
   x11-dl = "2.21"
   x11-clipboard = "0.7"
   nix = "0.27"
   anyhow = "1.0"
   thiserror = "1.0"
   clap = { version = "4.4", features = ["derive"] }
   ```

3. **Начать с Фазы 1-2** и двигаться итеративно.

---

## Рекомендуемый порядок разработки

**MVP (Minimum Viable Product):**
1. Фазы 1-2 (X11 фреймворк)
2. Фаза 3 (Основные структуры)
3. Фаза 4 (Coordinate mapping)
4. Фаза 5.1 (Mouse handling)
5. Фаза 8 (CLI)
6. Базовый тест

**После MVP:**
1. Фаза 5.2 (Keyboard handling)
2. Фаза 7 (Connection management)
3. Фаза 6 (Clipboard sharing)
4. Фаза 10 (Comprehensive testing)

**Опционально:**
1. Фаза 9 (Win32 support)
2. Additional features (multi-display enhancements, etc.)

---

## Дополнительные идеи для улучшений

### 1. Async event loop

Вместо blocking `select()` использовать `tokio` + async IO:
```rust
async fn run_async_event_loop(
    from_conn: &X11Connection,
    to_conn: &X11Connection,
) -> Result<(), Error>;
```

### 2. Plugin система

Расширяемость через dynamic loading:
```rust
pub trait InputPlugin {
    fn handle_event(&self, event: &XEvent, ctx: &mut Context) -> Result<bool, Error>;
}
```

### 3. Config файлы

Поддержка YAML/TOML конфигураций:
```yaml
displays:
  from: ":0"
  to: ":1"
  layout: horizontal
  edge: east
clipboard:
  enabled: true
  formats: [text, utf8]
```

### 4. Wayland поддержка

В будущем можно добавить:
- `wayland-client` crate
- Virtual input protocol
- Поддержка compositor-specific features

### 5. GUI статус

Улучшенный статус window с:
- Rust GUI framework (`egui`, `iced`)
- Animated transitions
- Connection stats

---

## Заключение

Этот план предусматривает постепенную, итеративную миграцию с сохранением функциональности и добавлением преимуществ Rust. Основной фокус на безопасности памяти, поддерживаемости и современных практиках разработки.

Ключевой принцип: **не переписывать всё сразу**, а двигаться маленькими шагами с постоянным тестированием.
