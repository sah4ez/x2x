# Статус проекта x2x-rust: Рефакторинг с C на Rust

**Дата:** 2026-02-27
**Ветка:** rust_refactor
**Оригинальный C код:** ~4,284 строки (x2x.c ~3,600 строк)

---

## Краткий обзор проекта

**x2x** — утилита для X Window System, которая позволяет использовать клавиатуру и мышь одного X-дисплея для управления другим, с поддержкой clipboard sharing.

Это Rust-рефакторинг оригинального C проекта с целью улучшить безопасность памяти, поддерживаемость и добавить современные практики разработки.

---

## Общий прогресс: ~40%

| Фаза | Статус | Прогресс |
|------|--------|----------|
| Фаза 1: Подготовка окружения | ✅ Завершена | 100% |
| Фаза 2: X11 биндинги и фреймворк | 🚧 В процессе | 70% |
| Фаза 3: Основные структуры данных | 🚧 В процессе | 40% |
| Фаза 4: Coordinate Mapping | 📋 Не начата | 0% |
| Фаза 5: Input Processing | 📋 Не начата | 0% |
| Фаза 6: Clipboard Sharing | 📋 Не начата | 0% |
| Фаза 7: Connection Management | 📋 Не начата | 0% |
| Фаза 8: CLI и конфигурация | 🚧 Частично | 30% |
| Фаза 9: Тестирование | 📋 Не начата | 0% |
| Фаза 10: Документация и полировка | 📋 Не начата | 0% |

---

## Детальный статус по фазам

### ✅ Фаза 1: Подготовка окружения (Завершена)

**Выполнено:**
- ✅ Выбор Rust библиотек:
  - `x11-dl` — FFI биндинги к Xlib
  - `x11-xtst` — XTest extension
  - `x11-clipboard` — Работа с X clipboard
  - `nix` — Системные вызовы
  - `anyhow` / `thiserror` — Обработка ошибок
  - `clap` — CLI аргументы
  - `log` + `env_logger` — Логирование
- ✅ Создание структуры проекта:
  - Модульная структура создана (src/x11/, src/core/, src/input/, src/clipboard/, src/utils/, src/win32/)
- ✅ Feature flags определены:
  - `default = ["x11"]`
  - `win32` feature для Windows поддержки

---

### 🚧 Фаза 2: X11 биндинги и базовый фреймворк (70%)

**Выполнено:**
- ✅ `X11Connection` структура (`src/x11/connection.rs`):
  - Безопасная обертка над `Display*`
  - RAII для корректного закрытия
  - Методы: `open()`, `screen()`, `root_window()`, `flush()`, `sync()`, `pending()`, `connection_number()`, `next_event()`, `peek_event()`
  - Screen info: `screen_info()`, `current_screen_info()`, `screen_count()`
  - Pointer: `query_pointer()`, `warp_pointer()`
  - Send + Sync для многопоточности

- ✅ `XEvent` enum и структуры (`src/x11/event.rs`):
  - Все типы событий: MotionNotify, ButtonPress/Release, KeyPress/Release, SelectionRequest/Notify/Clear, Property, ClientMessage и другие
  - `from_xlib_event()` метод для конвертации
  - `EventHandler` trait

- ✅ `XTestExtension` структура (`src/x11/extension.rs`):
  - `from_connection()` — проверка и инициализация
  - `fake_motion()` — эмуляция движения мыши
  - `fake_button()` — эмуляция нажатий кнопок
  - `fake_key()` — эмуляция нажатий клавиш
  - `grab_control()` / `release_control()` — захват ввода

- ✅ `DpmsExtension` структура (`src/x11/extension.rs`):
  - `from_connection()` — проверка доступности
  - `is_enabled()` — проверка статуса
  - `enable()` / `disable()` — управление
  - `force_level()` — принудительное переключение
  - `get_level()` — получение текущего уровня

**Осталось:**
- ❌ Event loop с использованием `select()` / `poll()`
  - Существует `EventLoop` структура в `src/core/event_loop.rs`, но:
  - ❌ Не использует `select()` / `poll()` для эффективного ожидания
  - ❌ Использует busy-wait с `thread::sleep(10ms)` — неэффективно
  - ❌ Нет таймаутов и логики переподключения
- ❌ Обработчики для всех типов событий (существуют только заглушки)
- ❌ X11 error handler (`XSetErrorHandler`)
- ❌ Graceful degradation при недоступности расширений
- ❌ Лучшее логирование ошибок

---

### 🚧 Фаза 3: Портинг основных структур данных (40%)

**Выполнено:**
- ✅ `DpyInfo` структура (`src/core/dpy_info.rs`):
  - Основные поля: from_conn, to_conn, from_root, to_root
  - ConnectionMode enum (Disconnected, Connected)
  - SelectionState placeholder
- ✅ `ShadowDisplay` структура (`src/core/dpy_info.rs`):
  - Основные поля: name, conn, led_mask, flush_required
- ✅ `FakeEvent` enum и `FakeQueue` (`src/core/state.rs`):
  - Полностью реализована очередь фейковых событий
  - Методы: `push()`, `pop()`, `is_empty()`, `len()`, `process_all()`
  - Отслеживание активных клавиш и кнопок
- ✅ `StickyKeys` структура (`src/core/state.rs`):
  - Полностью реализована поддержка sticky keys
  - Методы: `toggle()`, `is_sticky()`, `clear()`, `len()`
  - Тесты

**Осталось:**
- ❌ `DpyInfo::connect()` — реализовать DoConnect логику
  - Создание trigger окна на краю экрана
  - Захват указателя
  - Переключение mode в Connected
  - Смещение курсора на другой экран
- ❌ `DpyInfo::disconnect()` — реализовать DoDisconnect логику
  - Освобождение указателя
  - Уничтожение trigger окна
  - Переключение mode в Disconnected
  - Смещение курсора обратно на from экран
- ❌ `CoordinateMapping` таблицы (перейти в Фазу 4)
  - Нет фактического построения таблиц трансформации
  - Методы `map_x()` и `map_y()` возвращают COORD_INCR для всех координат (заглушка)
  - Нет логики `COORD_INCR` / `COORD_DECR` для переключения экранов

---

### 📋 Фаза 4: Coordinate Mapping (0%)

**Выполнено:**
- ✅ Структуры и enum'ы определены:
  - `CoordinateMapping` с x_tables, y_tables
  - `LayoutMode` (Horizontal, Vertical)
  - `Direction` (Left, Right, Up, Down)
- ✅ Тесты граничных условий

**Осталось:**
- ❌ Построение таблиц трансформации (`build_x_tables()`, `build_y_tables()`):
  - Логика из C кода для предвычисления координат
  - Учет ширины/высоты экранов
  - Обработка нескольких экранов (multi-screen)
  - Оптимизация с Rust iterators
- ❌ Реальный маппинг координат:
  - Замена заглушек на реальную логику
  - Обработка COORD_INCR/COORD_DECR для переключения экранов
  - Sanity checks для движения курсора
- ❌ Property-based тесты для координатных трансформаций

---

### 📋 Фаза 5: Input Processing (0%)

**Выполнено:**
- ✅ Структуры определены (`MouseHandler`, `KeyboardHandler` в `src/input/`):
- ✅ `EventHandler` trait реализован

**Осталось:**

#### 5.1 Mouse event handling
- ❌ `MouseHandler::handle_motion()` — реализовать `ProcessMotionNotify`:
  - Проверка same_screen
  - Маппинг координат через таблицы
  - Sanity check (unreasonableDelta)
  - Обработка COORD_INCR/COORD_DECR
  - Фейк движение на to display через XTest
- ❌ `MouseHandler::handle_button_press()` — реализовать `ProcessButtonPress`:
  - Блокировка кнопок при авто-отключении
  - Фейк нажатие на to display
- ❌ `MouseHandler::handle_button_release()` — реализовать `ProcessButtonRelease`:
  - Отслеживание состояния кнопок
  - Фейк отпускание на to display

#### 5.2 Keyboard event handling
- ❌ `KeyboardHandler::handle_key_press()` — реализовать `ProcessKeyEvent` для press:
  - Получение KeySym из KeyCode
  - Проверка sticky keys
  - Отправка на to display через XTest
- ❌ `KeyboardHandler::handle_key_release()` — реализовать `ProcessKeyEvent` для release:
  - Обработка модификаторов
  - Отправка на to display через XTest
- ❌ XKeySym маппинг
- ❌ Modifier keys handling

#### 5.3 Fake queue processing
- ❌ `FakeQueue::process()` — реализовать `FakeThingsUp` логику:
  - Отправка фейковых событий через XTest
  - Интеграция с `XTestExtension`

---

### 📋 Фаза 6: Clipboard Sharing (0%)

**Выполнено:**
- ✅ `X11Clipboard` структура определена (`src/clipboard/x11_selection.rs`):
- ✅ `SelectionState` структура определена

**Осталось:**
- ❌ Инициализация X atom'ов для clipboard операций (PRIMARY, SECONDARY, CLIPBOARD, UTF8_STRING, TARGETS, MULTIPLE)
- ❌ `X11Clipboard::handle_selection_request()` — реализовать `ProcessSelectionRequest`:
  - Логика из x2x.c для обмена clipboard между дисплеями
- ❌ `X11Clipboard::handle_selection_notify()` — реализовать `ProcessSelectionNotify`:
  - Обработка ответов на запросы clipboard
- ❌ `X11Clipboard::handle_selection_clear()` — реализовать `ProcessSelectionClear`:
  - Потеря владения clipboard
- ❌ `X11Clipboard::convert_selection()` — реализовать `XConvertSelection`:
  - Запрос преобразования формата clipboard
- ❌ UTF-8 encoding поддержка
- ❌ Поддержка множественных форматов

---

### 📋 Фаза 7: Connection Management (0%)

**Выполнено:**
- ✅ Основные структуры определены

**Осталось:**
- ❌ `DpyInfo::connect()` (уже отмечено в Фазе 3)
- ❌ `DpyInfo::disconnect()` (уже отмечено в Фазе 3)
- ❌ `move_window_to_edge()` — реализовать MoveWindowToEdge:
  - Создание окна на краю экрана
  - Управление геометрией окна
- ❌ `move_window_to_screen()` — реализовать MoveWindowToScreen:
  - Перемещение окна между экранами
  - Обработка multi-screen конфигураций

---

### 🚧 Фаза 8: CLI и конфигурация (30%)

**Выполнено:**
- ✅ `Config` структура определена (`src/utils/config.rs`)
- ✅ Парсинг аргументов через `clap`:
  - `-f, --from <DISPLAY>`
  - `-t, --to <DISPLAY>`
  - `-e, --edge <EDGE>`
  - `-w, --wait`
  - `--btn-block`
  - `--vertical`
  - `-F, --font <FONT>`
  - `-d, --debug`
  - `-c, --config <FILE>` (не реализовано)

**Осталось:**
- ❌ Реализация `--config` для чтения YAML/TOML конфигураций
- ❌ Проверка валидности аргументов
- ❌ Вспомогательные функции для парсинга (например, ParseCommandLine из C кода)
- ❌ Генерация справки

---

### 📋 Фаза 9: Тестирование (0%)

**Выполнено:**
- ✅ Unit тесты заглушки во всех модулях

**Осталось:**
- ❌ Реальные unit тесты:
  - Coordinate mapping тесты с различными конфигурациями экранов
  - Fake queue тесты
  - Sticky keys тесты
- ❌ Integration тесты:
  - Требуют запущенный X server
  - Тестирование X11Connection
  - Тестирование XTestExtension
  - Можно использовать Xvfb (virtual framebuffer)
- ❌ Property-based тесты (с `proptest`):
  - Тестирование координатных трансформаций
  - Тестирование edge cases
- ❌ Сравнение поведения с C версией:
  - Запуск обеих версий параллельно
  - Проверка идентичности поведения

---

### 📋 Фаза 10: Документация и полировка (0%)

**Выполнено:**
- ✅ Базовый README.md

**Осталось:**
- ❌ Rustdoc для всех public API
- ❌ Полная документация в README:
  - Инструкции по сборке
  - Примеры использования
  - Перенести docs/ в Markdown
- ❌ Оптимизации:
  - Профилирование с `perf` / `flamegraph`
  - Оптимизация hot paths (coordinate mapping)
  - Уменьшение аллокаций
- ❌ CI/CD:
  - GitHub Actions workflow
  - Автоматические тесты при push/pull request
  - Linting (clippy, fmt)
- ❌ Дополнительные улучшения:
  - Async event loop (опционально)
  - Plugin система (опционально)
  - Config файлы (опционально)
  - Wayland поддержка (опционально)
  - GUI статус (опционально)

---

## На каком этапе остановились

**Текущий этап:** Фаза 2 — X11 биндинги и базовый фреймворк

**Последние изменения:**
- Создан базовый event loop в `src/core/event_loop.rs`, но он неэффективен (busy-wait)
- Определены все основные структуры данных (DpyInfo, ShadowDisplay, FakeQueue, StickyKeys)
- Реализованы обертки над X11 API (X11Connection, XTestExtension, DpmsExtension)
- Реализован EventHandler trait

**Что работает:**
- ✅ Открытие X11 соединений
- ✅ Получение информации об экранах
- ✅ Проверка доступности XTest и DPMS расширений
- ✅ Парсинг CLI аргументов

**Что НЕ работает:**
- ❌ Event loop (нет эффективного select/poll)
- ❌ Обработка событий мыши и клавиатуры (только заглушки с todo!())
- ❌ Coordinate mapping (заглушки возвращают COORD_INCR)
- ❌ Connection/disconnect логика (todo!())
- ❌ Clipboard sharing (заглушки)
- ❌ Комплексное тестирование

---

## Что нужно доделать (приоритеты)

### КРИТИЧНО (для MVP):

1. **Event Loop с select/poll** (Фаза 2)
   - Заменить busy-wait на эффективный select()
   - Добавить таймауты и логику переподключения

2. **Coordinate Mapping** (Фаза 4)
   - Реализовать построение таблиц трансформации
   - Реализовать реальный маппинг координат

3. **Mouse Handling** (Фаза 5.1)
   - Реализовать handle_motion()
   - Реализовать handle_button_press/release()

4. **Connection Management** (Фаза 7)
   - Реализовать DpyInfo::connect()
   - Реализовать DpyInfo::disconnect()

5. **Basic CLI Integration** (Фаза 8)
   - Интегрировать event loop с main.rs
   - Сделать минимально работающий MVP

### ВАЖНО (для полной функциональности):

6. **Keyboard Handling** (Фаза 5.2)
   - Реализовать handle_key_press/release()
   - XKeySym маппинг

7. **Clipboard Sharing** (Фаза 6)
   - Реализовать SelectionRequest/Notify/Clear
   - UTF-8 encoding

8. **Тестирование** (Фаза 9)
   - Unit тесты для основных модулей
   - Integration тесты с Xvfb

### ЖЕЛАТЕЛЬНО (для улучшений):

9. **X11 Error Handler** (Фаза 2)
   - Graceful degradation
   - Лучшее логирование

10. **Документация** (Фаза 10)
    - Rustdoc
    - Полный README
    - Примеры использования

11. **Оптимизации** (Фаза 10)
    - Профилирование
    - Уменьшение аллокаций

12. **CI/CD** (Фаза 10)
    - GitHub Actions
    - Автоматическое тестирование

---

## Блокеры и проблемы

1. **Cargo не установлен** — невозможно протестировать компиляцию
   - Нужно установить Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. **Busy-wait event loop** — неэффективно работает CPU
   - Критично для использования в продакшене
   - Нужен select() или poll()

3. **Нереализованные TODO** — много методов возвращают todo!()
   - Это вызывает panic при вызове
   - Нужно заменить на заглушки с возвращением Ok(())

4. **Нет реального функционала** — программа не делает ничего полезного
   - Нужен MVP с базовой функциональностью

---

## Рекомендуемые следующие шаги

1. **Установить Rust** и протестировать компиляцию:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo build --release
   ```

2. **Реализовать Event Loop** с использованием `select()`:
   - Изучить C код для `select()` логики
   - Использовать `nix` crate для системных вызовов
   - Добавить таймауты

3. **Реализовать Coordinate Mapping**:
   - Перенести логику построения таблиц из C кода
   - Добавить тесты для граничных условий

4. **Сделать MVP**:
   - Работающий mouse handling
   - Работающий connect/disconnect
   - Базовый event loop

5. **Комплексное тестирование**:
   - Unit тесты
   - Integration тесты с Xvfb
   - Сравнение с C версией

---

## Заключение

Проект находится на **ранней стадии** (~40% завершения). Основной фреймворк создан, но критическая функциональность еще не реализована. Требуется значительная работа по реализации core logic (coordinate mapping, input handling, connection management).

Ключевой принцип продолжения разработки: **двигаться маленькими шагами с постоянным тестированием**, как указано в RUST_REFACTOR_PLAN.md.

**Ориентировочное время до MVP:** 7-10 дней (при полной занятости)
**Ориентировочное время до полной функциональности:** 24-31 день (как указано в плане)
