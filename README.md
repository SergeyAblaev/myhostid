# myhostid

Небольшая замена GNU/Linux-команды `hostid` для macOS. Вместо вычисления
идентификатора утилита печатает строку из конфигурационного файла.

По умолчанию читается `/etc/myhostid.conf`. Другой путь можно указать через
переменную окружения `MYHOSTID_CONFIG`. Файл должен содержать ровно одну
непустую строку. Значение не приводится к нижнему регистру и не ограничивается
восемью символами.

## Сборка и проверка

```sh
cargo test
cargo build --release
```

Готовая программа находится в `target/release/hostid`.

## Установка на macOS

Пример системной установки (команды `install` попросят пароль администратора):

```sh
sudo install -m 0755 target/release/hostid /usr/local/bin/hostid
printf '%s\n' 'BED0745A0764' | sudo tee /etc/myhostid.conf >/dev/null
sudo chmod 0644 /etc/myhostid.conf
```

Убедитесь, что `/usr/local/bin` расположен в `PATH` раньше каталога, в котором
находится несовместимая версия `hostid`:

```sh
command -v hostid
hostid
```

Ожидаемый вывод:

```text
BED0745A0764
```

Для установки без записи в `/etc` задайте путь при запуске:

```sh
MYHOSTID_CONFIG="$HOME/.config/myhostid.conf" hostid
```

Утилита также поддерживает совместимые с GNU `hostid` вызовы `--help` и
`--version`. Лишние аргументы считаются ошибкой.
