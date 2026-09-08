@echo off
chcp 65001 >nul
echo ========================================================
echo  Запуск Treasure Towers 3D (Sony Ericsson, MascotCapsule v3)
echo ========================================================
echo.
echo Управление:
echo   Стрелка Вверх / 2 / W             : Карабкаться вверх по башне
echo   Стрелка Вниз / 8 / S              : Спуститься / скользить вниз
echo   Стрелка Влево / 4 / A             : Движение влево вокруг башни
echo   Стрелка Вправо / 6 / D            : Движение вправо вокруг башни
echo   Enter / Пробел / 5                : Прыжок / Действие / Выбор
echo   F1 / Z / Q                        : Левая софт-клавиша (ОК / Старт)
echo   F2 / X / E                        : Правая софт-клавиша (Назад / Пауза)
echo.

.\target\release\rust_java.exe -jar "apk\spaces-java\all-games\56644136-TreasureTowers.jar"
