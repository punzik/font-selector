# Font Selector

English version: [`README.md`](README.md)

Font Selector — это простое настольное приложение на GTK4 и Rust для Linux (включая окружения с Xorg). Оно позволяет просматривать установленные в системе шрифты и видеть, как выглядит текст с разными гарнитурами и размером.

![screenshot](screenshot.png "Font Selector")

## Возможности

- Список установленных шрифтов
- Быстрая фильтрация шрифтов по имени
- Навигация по списку шрифтов из поля фильтра (`Up`/`Down`)
- Фокус на поле фильтра по `Ctrl+F`
- Очистка фильтра по `Esc`
- Копирование имени выбранного шрифта через контекстное меню или `Ctrl+C`
- Многострочное поле тестового текста
- Область предпросмотра с белым фоном
- Изменение размера шрифта
- Встроенная локализация: русский, английский, немецкий, французский, испанский и эсперанто

## Требования

- Linux с установленными runtime/dev-библиотеками GTK4
- Rust toolchain (`rustc`, `cargo`)
- `pkg-config`

Если вы используете Nix, в репозитории есть:

- `shell.nix` для `nix-shell`
- `font-selector.nix` для упаковки в NixOS

## Сборка и запуск

### Обычный workflow через Cargo

```bash
cargo check
cargo run
```

### Через Nix shell

```bash
nix-shell
cargo run
```

## Установка в NixOS

Программу можно добавить в `environment.systemPackages` через `font-selector.nix`.

```nix
{ config, pkgs, ... }:

let
  font-selector = pkgs.callPackage /path/to/font-selector/font-selector.nix {};
in {
  environment.systemPackages = with pkgs; [
    font-selector
  ];
}
```

После этого примените конфигурацию:

```bash
sudo nixos-rebuild switch
```

## Локализация

Язык выбирается в таком порядке:

1. `FONT_SELECTOR_LANG`
2. `LC_ALL`
3. `LANG`

Поддерживаемые коды языков:

- `ru`
- `en`
- `de`
- `fr`
- `es`
- `eo`

Примеры:

```bash
FONT_SELECTOR_LANG=en cargo run
LANG=de_DE.UTF-8 cargo run
```

Файлы переводов:

- `i18n/ru.lang`
- `i18n/en.lang`
- `i18n/de.lang`
- `i18n/fr.lang`
- `i18n/es.lang`
- `i18n/eo.lang`

Каждый файл перевода использует простой формат `key = value`.

## Структура проекта

- `src/main.rs` - UI и логика приложения
- `i18n/*.lang` - файлы переводов
- `shell.nix` - dev-shell для `nix-shell`
- `font-selector.nix` - derivation для установки в систему NixOS
