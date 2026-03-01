# План реализации: X11 Window Creation

## Обзор

Цель: Реализовать создание X11 окон для x2x-rust аналогично C версии.

C код: функция `InitDpyInfo` (строки ~1150-1400 в x2x.c)

---

## Компоненты для реализации

### 1. Trigger Window

**Назначение:** Главное окно приложения для захвата событий

**Типы окон:**
- **Edge window:** Когда включена опция `-edge` (north/south/east/west)
- **Normal window:** Обычное окно с текстом (метка, название дисплея)

#### 1.1 Edge Window Creation

C код:
```c
trigger = pDpyInfo->trigger =
  XCreateWindow(fromDpy, root,
                vertical ? triggerw : triggerLoc,
                vertical ? triggerLoc : triggerw,
                vertical ? fromWidth - (2*triggerw) : triggerw,
                vertical ? triggerw : fromHeight - (2*triggerw),
                0, 0, InputOutput, 0,
                CWOverrideRedirect, &xswa);
```

**Параметры:**
- `override_redirect: true` - окно не управляется window manager
- Тип окна: `InputOutput` - для возможности рисования (Exposure events)
- Позиция и размер зависят от направления edge

**Расчет позиции:**

| Direction | X | Y | Width | Height |
|-----------|---|---|-------|--------|
| NORTH | `triggerw` | `fromHeight - triggerw` | `fromWidth - 2*triggerw` | `triggerw` |
| SOUTH | `triggerw` | `0` | `fromWidth - 2*triggerw` | `triggerw` |
| EAST | `fromWidth - triggerw` | `triggerw` | `triggerw` | `fromHeight - 2*triggerw` |
| WEST | `0` | `triggerw` | `triggerw` | `fromHeight - 2*triggerw` |

**Константы:**
- `triggerw: u32` - ширина trigger зоны (по умолчанию 2px)
- `fromWidth: u32` - ширина from-дисплея
- `fromHeight: u32` - высота from-дисплея

#### 1.2 Normal Window Creation

C код:
```c
trigger = pDpyInfo->trigger =
  XCreateSimpleWindow(fromDpy, root, xoff, yoff, width, height,
                      0, black, white);
```

**Параметры:**
- Тип окна: обычное (не override_redirect)
- С размером под текстовую метку
- С черным бордюром (border_width=0)
- Белый фон

**Расчет размера:**
```c
width = twidth + 4;  // текст + padding
height = theight + 4;
```

**Geometry parsing:**
- `-geometry <GEOMETRY>` опция
- `XParseGeometry()` для парсинга
- Gravity: NorthWestGravity, NorthEastGravity, SouthWestGravity, SouthEastGravity

---

### 2. Big Window Creation

**Назначение:** Полноэкранное прозрачное окно для захвата всей области экрана

**Опция:** `-big`

C код:
```c
big = pDpyInfo->big =
  XCreateWindow(fromDpy, root, 0, 0, fromWidth, fromHeight, 0,
                0, InputOnly, 0, CWOverrideRedirect, &xswa);
```

**Параметры:**
- `x: 0, y: 0` - левый верхний угол
- `width: fromWidth, height: fromHeight` - весь экран
- Тип окна: `InputOnly` - только для захвата событий, не рисует
- `override_redirect: true`

**Использование:**
- Mapping/Unmapping при connect/disconnect
- Захват pointer/keyboard через этот window

---

### 3. Window Properties

#### 3.1 Basic Properties

**Window Name & Icon Name:**
```c
XStoreName(fromDpy, trigger, windowName);
XSetIconName(fromDpy, trigger, windowName);
```

**Имя окна:**
- Если задана опция `-title`: используется title
- Иначе: `"{programStr} {toDpyName}"` → `"x2x localhost:1"`

**WM_NORMAL_HINTS:**
```c
xsh->x           = xoff;
xsh->y           = yoff;
xsh->base_width  = width;
xsh->base_height = height;
xsh->win_gravity = gravity;
xsh->flags       = (PPosition|PBaseSize|PWinGravity);
XSetWMNormalHints(fromDpy, trigger, xsh);
```

#### 3.2 WM_PROTOCOLS

**WM_DELETE_WINDOW:**
```c
pDpyInfo->wmpAtom = XInternAtom(fromDpy, "WM_PROTOCOLS", True);
pDpyInfo->wmdwAtom = XInternAtom(fromDpy, "WM_DELETE_WINDOW", True);
XSetWMProtocols(fromDpy, trigger, &(pDpyInfo->wmdwAtom), 1);
```

**Назначение:** Получать событие закрытия окна (ClientMessage)

#### 3.3 EWMH Hints

**_NET_WM_WINDOW_TYPE:**
```c
pDpyInfo->netWmWindowTypeAtom = XInternAtom(fromDpy, "_NET_WM_WINDOW_TYPE", True);
pDpyInfo->netWmWindowTypeDockAtom = XInternAtom(fromDpy, "_NET_WM_WINDOW_TYPE_DOCK", True);
XChangeProperty(fromDpy, trigger, pDpyInfo->netWmWindowTypeAtom,
                XA_ATOM, 32, PropModeReplace,
                (unsigned char *)&pDpyInfo->netWmWindowTypeDockAtom, 1);
```

**Тип:** `_NET_WM_WINDOW_TYPE_DOCK` - для edge window

**_NET_WM_STRUT (опция `-struts`):**
```c
unsigned long struts[4] = { doEdge == EDGE_WEST ? triggerw : 0
                          , doEdge == EDGE_EAST ? triggerw : 0
                          , doEdge == EDGE_NORTH ? triggerw : 0
                          , doEdge == EDGE_SOUTH ? triggerw : 0
                          };
pDpyInfo->netWmStrutAtom = XInternAtom(fromDpy, "_NET_WM_STRUT", True);
XChangeProperty(fromDpy, trigger, pDpyInfo->netWmStrutAtom,
                XA_CARDINAL, 32, PropModeReplace,
                (unsigned char *)&struts, 4);
```

**Назначение:** Резервировать место на экране для edge window

#### 3.4 Window Transparency (опция `-win-transparent`)

**_NET_WM_WINDOW_OPACITY:**
```c
if (winTransparent) {
  u_int32_t cardinal_alpha = (u_int32_t) (0);
  XChangeProperty(fromDpy, trigger,
    XInternAtom(fromDpy, "_NET_WM_WINDOW_OPACITY", 0),
    XA_CARDINAL, 32, PropModeReplace, (u_int8_t*) &cardinal_alpha,1);
}
```

**Значения:**
- `0` - полностью прозрачный
- `0xFFFFFFFF` - полностью непрозрачный

---

### 4. Cursor Creation

**Grab Cursor:**
```c
pDpyInfo->grabCursor = XCreateFontCursor(fromDpy, XC_exchange);
```

**Цели:**
- Для edge window: прозрачный курсор (из pixmap)
- Для normal window: курсор обмена (XC_exchange)

**Transparent cursor:**
```c
nullPixmap = XCreatePixmap(fromDpy, root, 1, 1, 1);
pDpyInfo->grabCursor =
  XCreatePixmapCursor(fromDpy, nullPixmap, nullPixmap,
                      &dummyColor, &dummyColor, 0, 0);
```

---

### 5. Font and Text Rendering (для normal window)

**Font Loading:**
```c
if (((fid = XLoadFont(fromDpy, fontName)) != 0) ||
    ((fid = XLoadFont(fromDpy, defaultFN)) != 0) ||
    ((fid = XLoadFont(fromDpy, "fixed")) != 0)) {
```

**Fallback order:**
1. Опция `-font <FONT>`
2. Default: `-*-times-bold-r-*-*-*-180-*-*-*-*-*-*`
3. `"fixed"`

**Text Measurement:**
```c
XQueryTextExtents(fromDpy, fid, label, strlen(label),
                  &direction, &ascent, &descent, &overall);
twidth = -overall.lbearing + overall.rbearing;
theight = ascent + descent;
tascent = ascent;
```

**GC Creation:**
```c
textGC = pDpyInfo->textGC = XCreateGC(fromDpy, root, 0, NULL);
XSetState(fromDpy, textGC, black, white, GXcopy, AllPlanes);
XSetFont(fromDpy, textGC, fid);
```

**Drawing:**
```c
XDrawImageString(pDpyInfo->fromDpy, pDpyInfo->trigger, pDpyInfo->textGC,
                MAX(0, ((pDpyInfo->width - pDpyInfo->twidth) / 2)),
                MAX(0, ((pDpyInfo->height - pDpyInfo->theight) / 2)) +
                pDpyInfo->tascent,
                label, strlen(label));
```

---

## Реализация в Rust

### Новые структуры

```rust
// src/x11/window.rs
pub struct X11Window {
    display: *mut Display,
    window: Window,
}

pub struct WindowAttributes {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    border_width: u32,
    override_redirect: bool,
    input_only: bool,
}

pub struct WindowHints {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    min_width: Option<u32>,
    min_height: Option<u32>,
    base_width: Option<u32>,
    base_height: Option<u32>,
    win_gravity: Gravity,
    flags: HintFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gravity {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

bitflags! {
    pub struct HintFlags: u32 {
        const POSITION = 1 << 0;
        const SIZE = 1 << 1;
        const MIN_SIZE = 1 << 2;
        const BASE_SIZE = 1 << 3;
        const WIN_GRAVITY = 1 << 4;
    }
}
```

### Новые методы в X11Connection

```rust
// src/x11/connection.rs
impl X11Connection {
    pub fn create_window(
        &self,
        parent: Window,
        attrs: &WindowAttributes,
    ) -> Result<Window> {
        // XCreateWindow / XCreateSimpleWindow
    }

    pub fn map_window(&self, window: Window) -> Result<()> {
        // XMapWindow
    }

    pub fn map_raised(&self, window: Window) -> Result<()> {
        // XMapRaised
    }

    pub fn unmap_window(&self, window: Window) -> Result<()> {
        // XUnmapWindow
    }

    pub fn set_window_name(&self, window: Window, name: &str) -> Result<()> {
        // XStoreName
    }

    pub fn set_icon_name(&self, window: Window, name: &str) -> Result<()> {
        // XSetIconName
    }

    pub fn set_wm_hints(&self, window: Window, hints: &WindowHints) -> Result<()> {
        // XSetWMNormalHints
    }

    pub fn set_wm_protocols(&self, window: Window, protocols: &[Atom]) -> Result<()> {
        // XSetWMProtocols
    }

    pub fn intern_atom(&self, name: &str, only_if_exists: bool) -> Result<Atom> {
        // XInternAtom
    }

    pub fn change_property(
        &self,
        window: Window,
        property: Atom,
        property_type: Atom,
        format: i32,
        mode: PropMode,
        data: &[u8],
    ) -> Result<()> {
        // XChangeProperty
    }

    pub fn create_font_cursor(&self, cursor_font: u32) -> Result<Cursor> {
        // XCreateFontCursor
    }

    pub fn create_pixmap_cursor(
        &self,
        source_pixmap: Pixmap,
        mask_pixmap: Pixmap,
        fg_color: &XColor,
        bg_color: &XColor,
        x_hot: u32,
        y_hot: u32,
    ) -> Result<Cursor> {
        // XCreatePixmapCursor
    }

    pub fn load_font(&self, font_name: &str) -> Result<Font> {
        // XLoadFont
    }

    pub fn query_text_extents(
        &self,
        font: Font,
        text: &str,
    ) -> Result<TextExtents> {
        // XQueryTextExtents
    }

    pub fn create_gc(&self, drawable: Drawable, mask: u64, values: &GCValues) -> Result<GC> {
        // XCreateGC
    }

    pub fn draw_image_string(
        &self,
        drawable: Drawable,
        gc: GC,
        x: i32,
        y: i32,
        text: &str,
    ) -> Result<()> {
        // XDrawImageString
    }
}
```

### Обновление DpyInfo

```rust
// src/core/dpy_info.rs
pub struct DpyInfo {
    // From display
    pub from_conn: Arc<X11Connection>,
    pub from_root: Window,
    pub trigger: Option<Window>,  // Главное окно
    pub big: Option<Window>,       // Полноэкранное окно (-big)

    // Window properties
    pub trigger_width: u32,
    pub trigger_height: u32,
    pub grab_cursor: Option<Cursor>,

    // Text rendering (для normal window)
    pub font: Option<Font>,
    pub text_gc: Option<GC>,
    pub text_width: u32,
    pub text_height: u32,
    pub text_ascent: i32,

    // Window atoms
    pub wm_protocols_atom: Option<Atom>,
    pub wm_delete_window_atom: Option<Atom>,
    pub net_wm_window_type_atom: Option<Atom>,
    pub net_wm_dock_atom: Option<Atom>,
    pub net_wm_strut_atom: Option<Atom>,
    pub net_wm_opacity_atom: Option<Atom>,

    // ... остальные поля
}

impl DpyInfo {
    pub fn create_windows(&mut self, config: &Config) -> Result<()> {
        // Создание trigger окна
        // Создание big окна (если -big)
        // Настройка properties
        Ok(())
    }
}
```

---

## Порядок реализации

### Шаг 1: Базовые X11 wrappers (2-3 дня)

1. Добавить `X11Window` структуру
2. Реализовать `X11Connection::create_window()`
3. Реализовать `X11Connection::map_window()`, `map_raised()`, `unmap_window()`
4. Добавить тесты для создания простого окна

### Шаг 2: Window Properties (2-3 дня)

1. Реализовать `intern_atom()`
2. Реализовать `set_window_name()`, `set_icon_name()`
3. Реализовать `set_wm_hints()` (нужна структура `WindowHints`)
4. Реализовать `set_wm_protocols()`
5. Реализовать `change_property()`
6. Добавить тесты для properties

### Шаг 3: Trigger Window Creation (2-3 дня)

1. Добавить логику для edge window
   - Расчет позиции по direction
   - Установка `override_redirect`
   - Установка EWMH hints (_NET_WM_WINDOW_TYPE_DOCK, _NET_WM_STRUT)
2. Добавить логику для normal window
   - Расчет размера по тексту (пока заглушка)
   - Geometry parsing (пока заглушка)
3. Интегрировать в `DpyInfo::create_windows()`

### Шаг 4: Big Window Creation (1 день)

1. Реализовать создание InputOnly окна
2. Добавить поддержку опции `-big`
3. Интегрировать в `DpyInfo::create_windows()`

### Шаг 5: Cursor Creation (1-2 дня)

1. Реализовать `create_font_cursor()`
2. Реализовать `create_pixmap_cursor()`
3. Добавить `create_pixmap()` для прозрачного курсора
4. Интегрировать в trigger window

### Шаг 6: Transparency & Optional Features (1-2 дня)

1. Добавить поддержку `-win-transparent` опции
2. Реализовать установку _NET_WM_WINDOW_OPACITY
3. Добавить поддержку `-struts` опции

### Шаг 7: Font & Text Rendering (2-3 дня)

1. Реализовать `load_font()`
2. Реализовать `query_text_extents()`
3. Реализовать `create_gc()`
4. Реализовать `draw_image_string()`
5. Интегрировать в normal window creation

---

## Файлы для создания/изменения

```
src/x11/
  - connection.rs      # Добавить методы создания окон
  - window.rs        # Новый файл для структур окон
  - font.rs          # Новый файл для работы со шрифтами (опционально)

src/core/
  - dpy_info.rs      # Добавить поля окон, метод create_windows()

src/utils/
  - config.rs        # Добавить опции: -big, -win-transparent, -struts, -title

tests/
  - x11_window.rs   # Тесты для создания окон
```

---

## Тестирование

### Unit Tests

```rust
#[test]
fn test_create_simple_window() {
    let conn = X11Connection::open(None).unwrap();
    let root = conn.root_window();

    let attrs = WindowAttributes {
        x: 100,
        y: 100,
        width: 200,
        height: 150,
        border_width: 0,
        override_redirect: false,
        input_only: false,
    };

    let window = conn.create_window(root, &attrs).unwrap();
    conn.map_window(window).unwrap();

    // Проверить, что окно существует (XGetWindowAttributes)
}
```

### Integration Tests

```rust
#[test]
fn test_edge_window_creation() {
    // Создать edge window для NORTH
    // Проверить позицию
    // Проверить _NET_WM_WINDOW_TYPE_DOCK
}

#[test]
fn test_normal_window_creation() {
    // Создать normal window с текстом
    // Проверить размер
    // Проверить WM_NAME
}
```

---

## Зависимости

- ✅ `x11-dl` - уже есть в dependencies
- ✅ `anyhow` - уже есть
- ⚠️ Может понадобиться `bitflags` для HintFlags

---

## Риски

1. **Несовместимость типов между x11-dl и Xlib**
   - Решение: тщательно проверить типы данных

2. **Memory leaks**
   - Решение: использовать RAII для X11 ресурсов (Drop trait)

3. **Отсутствие EWMH hints на некоторых WM**
   - Решение: игнорировать ошибки при установке свойств

4. **Сложность с InputOnly окнами**
   - Решение: тестировать на разных X server

---

## Пример использования

```rust
use x2x_rust::core::DpyInfo;
use x2x_rust::utils::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    let from_conn = X11Connection::open(Some(":0"))?;
    let to_conn = X11Connection::open(Some(":1"))?;

    let mut dpy_info = DpyInfo::new(from_conn, to_conn)?;

    // Создать окна
    dpy_info.create_windows(&config)?;

    // Показать окно
    if let Some(trigger) = dpy_info.trigger {
        dpy_info.from_conn.map_raised(trigger)?;
    }

    Ok(())
}
```

---

## Критерий завершения

✅ Триггерное окно создается и отображается на экране
✅ Большое окно (если -big) создается корректно
✅ Window properties установлены (WM_NAME, WM_PROTOCOLS, EWMH hints)
✅ Тесты проходят
✅ Код компилируется без warnings

---

**Предполагаемое время:** 10-15 дней для полной реализации
**Сложность:** Средняя
**Приоритет:** Критический (блокирует Event Loop и обработку событий)
