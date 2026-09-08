@echo off
chcp 65001 >nul
echo ========================================================
echo  Запуск Prince of Persia 2008 (240x320, Русская версия)
echo ========================================================
echo.
echo Управление:
echo   Стрелки / WASD / Numpad 4, 6, 2, 8 : Движение (влево, вправо, прыжок/вверх, присесть/вниз)
echo   Enter / Пробел / Numpad 5          : Действие / Удар мечом / Выбор
echo   F1 / Z / Q                          : Левая софт-клавиша (ОК / Меню)
echo   F2 / X / E                          : Правая софт-клавиша (Назад / Пауза)
echo.

.\target\release\rust_java.exe -jar "apk\spaces-java\all-games\41720016-Prince_of_Persia_2008_RU.jar"
