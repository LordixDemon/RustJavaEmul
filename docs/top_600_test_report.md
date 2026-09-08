# Отчёт о масштабном тестировании Топ-600 культовых J2ME игр на RustJava

**Дата тестирования:** 2026-09-08 15:08:45
**Бинарный файл:** `target\release\rust_java.exe`
**Всего протестировано игр:** 600
**Успешно запустились (PASS):** 595 (99.2%)
**Ошибки Missing API:** 1
**Исключения (Exception):** 0
**Паники (Panic):** 0
**Таймауты / Прочее:** 4
**Общее время тестирования:** 55.5 с (0.9 мин)

## 1. Сводка по категориям издателей и жанров

| Категория | Всего | PASS | Missing API | Exception | Panic | Таймаут/Exit | Pass Rate |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **3D_M3G** | 47 | 46 | 0 | 0 | 0 | 1 | **97.9%** |
| **3D_MascotCapsule** | 6 | 6 | 0 | 0 | 0 | 0 | **100.0%** |
| **Benchmark** | 2 | 2 | 0 | 0 | 0 | 0 | **100.0%** |
| **Classics** | 94 | 93 | 0 | 0 | 0 | 1 | **98.9%** |
| **Disney_Pixar** | 21 | 21 | 0 | 0 | 0 | 0 | **100.0%** |
| **EA_Mobile** | 170 | 168 | 1 | 0 | 0 | 1 | **98.8%** |
| **Gameloft** | 144 | 143 | 0 | 0 | 0 | 1 | **99.3%** |
| **HandyGames_HeroCraft** | 38 | 38 | 0 | 0 | 0 | 0 | **100.0%** |
| **PopCap** | 14 | 14 | 0 | 0 | 0 | 0 | **100.0%** |
| **SEGA_Konami_Capcom** | 64 | 64 | 0 | 0 | 0 | 0 | **100.0%** |

## 2. Поддержка специальных графических API и платформ

| API / Технология | Найдено в JAR | Успешно запустилось (PASS) | Pass Rate |
| :--- | :---: | :---: | :---: |
| **JSR-184 (M3G 3D)** | 90 | 89 | **98.9%** |
| **MascotCapsule Micro3D v3** | 21 | 19 | **90.5%** |
| **Nokia UI API (DirectGraphics/Sound)** | 138 | 137 | **99.3%** |

## 3. Анализ проблем и недостающих API (требуют реализации/доработки)

| # | Статус | Игра | Категория | Ошибка / Причина |
| :---: | :---: | :--- | :--- | :--- |
| 89 | **TIMEOUT** | Fruit Ninja | Classics | `Process timeout > 45s` |
| 115 | **TIMEOUT** | Time Crisis 3D | 3D_M3G | `Process timeout > 45s` |
| 142 | **TIMEOUT** | Splinter Cell: Chaos Theory | Gameloft | `Process timeout > 45s` |
| 242 | **TIMEOUT** | Battlefield: Bad Company 2 | EA_Mobile | `Process timeout > 45s` |
| 245 | **MISSING_API** | Worms Forts: Under Siege | EA_Mobile | `Java Exception: java/lang/NoClassDefFoundError: com/nokia/mid/ui/DeviceControl` |

## 4. Полная таблица результатов тестирования Топ-600 игр

| # | Статус | Название игры | Категория | Размер (КБ) | 3D Движок | Издатель | Время (с) | Детали запуска |
| :---: | :---: | :--- | :--- | :---: | :---: | :--- | :---: | :--- |
| 1 | PASS | **Galaxy on Fire 2** | 3D_M3G | 1422.4 | Mascot3D | www.fishlabs.net | 0.05 | ok (clean exit) |
| 2 | PASS | **Rally Master Pro 3D** | 3D_M3G | 959.8 | M3G | www.fishlabs.net | 0.05 | ok (clean exit) |
| 3 | PASS | **DOOM II RPG** | 3D_M3G | 788.2 | 2D | Electronic Arts MobP | 0.16 | ok (clean exit) |
| 4 | PASS | **DOOM RPG** | 3D_M3G | 323.9 | 2D | JAMDAT Mobile Inc. | 0.09 | ok (clean exit) |
| 5 | PASS | **Wolfenstein RPG** | 3D_M3G | 725.2 | 2D | Electronic Arts | 0.35 | ok (clean exit) |
| 6 | PASS | **Need for Speed: Shift 3D** | 3D_M3G | 890.1 | Mascot3D | Electronic Arts | 0.37 | ok (clean exit) |
| 7 | PASS | **Need for Speed: Carbon 3D** | 3D_M3G | 390.9 | M3G | Unknown | 0.05 | ok (clean exit) |
| 8 | PASS | **Need for Speed: Most Wanted 3D** | 3D_M3G | 628.3 | M3G | Electronics Art, | 0.15 | ok (clean exit) |
| 9 | PASS | **SEGA Rally 3D** | 3D_M3G | 958.3 | M3G | SEGA/Dedomil | 0.05 | ok (clean exit) |
| 10 | PASS | **Dead Space 3D** | 3D_M3G | 754.0 | M3G | sodita, Wixel | 0.04 | ok (clean exit) |
| 11 | PASS | **Air War 3D** | 3D_M3G | 455.9 | M3G | 7Seas Entertainment  | 0.07 | ok (clean exit) |
| 12 | PASS | **Outland 3D** | 3D_M3G | 401.7 | M3G | Tracebit Ltd | 0.22 | ok (clean exit) |
| 13 | PASS | **Solid Weapon 3D** | 3D_M3G | 486.1 | Mascot3D | LemonQuest | 0.14 | ok (clean exit) |
| 14 | PASS | **Formula Extreme 3D** | 3D_M3G | 710.9 | M3G | Baltoro Games Studio | 0.33 | ok (clean exit) |
| 15 | PASS | **Blades & Magic 3D** | 3D_MascotCapsule | 783.8 | 2D | Player X | 0.03 | ok (clean exit) |
| 16 | PASS | **Deep 3D: Submarine Odyssey** | 3D_MascotCapsule | 2244.6 | Mascot3D | Unknown | 0.05 | ok (clean exit) |
| 17 | PASS | **Treasure Towers 3D** | 3D_MascotCapsule | 307.6 | Mascot3D | Sony Ericsson | 0.21 | ok (clean exit) |
| 18 | PASS | **Gothic 3: The Beginning** | 3D_MascotCapsule | 409.5 | 2D | www.phc.zz.mu :D | 0.07 | ok (clean exit) |
| 19 | PASS | **Ancient Ruins** | 3D_MascotCapsule | 81.9 | 2D | www.handy-games.com  | 0.04 | ok (clean exit) |
| 20 | PASS | **Cyberpunk: Arasakas Plot** | 3D_MascotCapsule | 402.0 | 2D | mobigama.ru | 0.12 | ok (clean exit) |
| 21 | PASS | **Prince of Persia: Forgotten Sands** | Gameloft | 764.6 | 2D | Gameloft SA wapxx.or | 0.07 | ok (clean exit) |
| 22 | PASS | **Prince of Persia: Classic** | Gameloft | 619.4 | 2D | DK | 0.05 | ok (clean exit) |
| 23 | PASS | **Prince of Persia (2008)** | Gameloft | 650.5 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 24 | PASS | **Assassin's Creed** | Gameloft | 578.1 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 25 | PASS | **Assassin's Creed II** | Gameloft | 279.5 | 2D | Gameloft SA by Stox | 0.04 | ok (clean exit) |
| 26 | PASS | **Assassin's Creed: Brotherhood** | Gameloft | 1798.9 | 2D | Gameloft SA http://s | 0.09 | ok (clean exit) |
| 27 | PASS | **Real Football 2008** | Gameloft | 1021.0 | M3G | Gameloft SA | 0.08 | ok (clean exit) |
| 28 | PASS | **Real Football 2010** | Gameloft | 719.6 | 2D | Gameloft SA | 0.11 | ok (clean exit) |
| 29 | PASS | **Asphalt 3: Street Rules** | Gameloft | 1106.5 | M3G | Gameloft SA / mobers | 0.07 | ok (clean exit) |
| 30 | PASS | **Asphalt 4: Elite Racing** | Gameloft | 375.0 | 2D | Gameloft SA MobPorta | 0.06 | ok (clean exit) |
| 31 | PASS | **Asphalt 6: Adrenaline** | Gameloft | 1521.7 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 32 | PASS | **Gangstar: Crime City** | Gameloft | 342.4 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 33 | PASS | **Gangstar 2: Kings of LA** | Gameloft | 682.4 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 34 | PASS | **Gangstar: Miami Vindication** | Gameloft | 460.3 | 2D | Gameloft SA by Stox | 0.07 | ok (clean exit) |
| 35 | PASS | **Dungeon Hunter** | Gameloft | 580.1 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 36 | PASS | **Zombie Infection** | Gameloft | 1233.7 | 2D | Gameloft SA | 0.12 | ok (clean exit) |
| 37 | PASS | **Diamond Twister** | Gameloft | 594.7 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 38 | PASS | **Block Breaker Deluxe** | Gameloft | 649.1 | 2D | Gameloft SA / sensor | 0.05 | ok (clean exit) |
| 39 | PASS | **Soul of Darkness** | Gameloft | 629.7 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 40 | PASS | **Splinter Cell: Conviction** | Gameloft | 959.3 | 2D | Gameloft SA | 0.25 | ok (clean exit) |
| 41 | PASS | **Far Cry 2** | Gameloft | 632.4 | 2D | Gameloft SA | 0.07 | ok (clean exit) |
| 42 | PASS | **Midnight Pool** | Gameloft | 308.9 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 43 | PASS | **Midnight Bowling** | Gameloft | 1429.9 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 44 | PASS | **Modern Combat 2: Black Pegasus** | Gameloft | 1348.6 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 45 | PASS | **Guitar Rock Tour** | Gameloft | 574.6 | 2D | sensoru.net | 0.06 | ok (clean exit) |
| 46 | PASS | **Miami Nights: Single in the City** | Gameloft | 715.2 | 2D | Gameloft SA / sensor | 0.08 | ok (clean exit) |
| 47 | PASS | **New York Nights: Success in the City** | Gameloft | 619.0 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 48 | PASS | **Castle of Magic** | Gameloft | 945.4 | 2D | Gameloft SA | 0.23 | ok (clean exit) |
| 49 | PASS | **Hero of Sparta** | Gameloft | 996.5 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 50 | PASS | **The Sims 3** | EA_Mobile | 914.9 | M3G | Electronic Arts Inc. | 0.05 | ok (clean exit) |
| 51 | PASS | **The Sims 3: World Adventures** | EA_Mobile | 733.6 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 52 | PASS | **Plants vs. Zombies** | PopCap | 1633.5 | 2D | Electronic Arts Inc. | 0.07 | ok (clean exit) |
| 53 | PASS | **Bejeweled Twist** | PopCap | 658.1 | 2D | PopCap by BlackWaltz | 5.11 | ok (clean exit) |
| 54 | PASS | **Zuma's Revenge** | PopCap | 1095.3 | 2D | Electronic Arts | 0.08 | ok (clean exit) |
| 55 | PASS | **Peggle** | PopCap | 625.0 | 2D | PopCap | 30.23 | ok (clean exit) |
| 56 | PASS | **Chuzzle** | PopCap | 278.6 | 2D | Electronic Arts Inc. | 5.01 | ok (clean exit) |
| 57 | PASS | **Bookworm** | PopCap | 668.9 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 58 | PASS | **Monopoly Deal** | EA_Mobile | 690.8 | 2D | Electronic Arts / Be | 0.16 | ok (clean exit) |
| 59 | PASS | **Tetris Mania** | EA_Mobile | 244.3 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 60 | PASS | **Tetris Revolution** | EA_Mobile | 510.9 | 2D | Electronic Arts | 0.09 | ok (clean exit) |
| 61 | PASS | **Spore Creatures** | EA_Mobile | 763.5 | 2D | Electronic Arts | 0.05 | ok (clean exit) |
| 62 | PASS | **Worms 2008** | EA_Mobile | 567.1 | 2D | THQ | 0.09 | ok (clean exit) |
| 63 | PASS | **Worms 2011 Armageddon** | EA_Mobile | 711.0 | 2D | Electronic Arts Inc. | 0.05 | ok (clean exit) |
| 64 | PASS | **Medal of Honor** | EA_Mobile | 1119.8 | 2D | Electronic Arts BERO | 0.08 | ok (clean exit) |
| 65 | PASS | **Command & Conquer 4** | EA_Mobile | 723.2 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 66 | PASS | **SimCity Deluxe** | EA_Mobile | 971.6 | 2D | Electronic Arts by S | 0.08 | ok (clean exit) |
| 67 | PASS | **FIFA 10** | EA_Mobile | 893.7 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 68 | PASS | **Fight Night Round 4** | EA_Mobile | 403.8 | 2D | Electronic Arts MobP | 0.22 | ok (clean exit) |
| 69 | PASS | **Gravity Defied: Trial Racing** | Classics | 198.9 | 2D | TEGOS.RU | 0.02 | ok (clean exit) |
| 70 | PASS | **Bobby Carrot 4: Flower Power** | Classics | 152.2 | 2D | FDGSoft | 0.03 | ok (clean exit) |
| 71 | PASS | **Bobby Carrot 5: Level Up** | Classics | 365.2 | 2D | FDGSoft | 0.04 | ok (clean exit) |
| 72 | PASS | **Bounce Tales** | Classics | 368.8 | 2D | Nokia | 0.03 | ok (clean exit) |
| 73 | PASS | **EDGE** | Classics | 383.7 | 2D | Connect2Media | 1.26 | ok (clean exit) |
| 74 | PASS | **Gish Reloaded** | Classics | 485.1 | 2D | hardwire | 0.08 | ok (clean exit) |
| 75 | PASS | **Mafia II Mobile** | Classics | 790.8 | 2D | sensoru.net | 0.04 | ok (clean exit) |
| 76 | PASS | **Age of Heroes III** | Classics | 343.5 | 2D | Qplaze/Serviak | 0.04 | ok (clean exit) |
| 77 | PASS | **Age of Heroes Online** | Classics | 696.5 | 2D | Gear Games | 0.03 | ok (clean exit) |
| 78 | PASS | **Townsmen 6** | Classics | 554.9 | 2D | www.handy-games.com  | 0.1 | ok (clean exit) |
| 79 | PASS | **Tower Bloxx: New York** | Classics | 968.1 | 2D | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 80 | PASS | **City Bloxx** | Classics | 290.2 | M3G | Nokia | 0.11 | ok (clean exit) |
| 81 | PASS | **Left 2 Die** | Classics | 711.9 | 2D | Left of Die 3D | 0.05 | ok (clean exit) |
| 82 | PASS | **Darkest Fear** | Classics | 213.5 | 2D | Rovio | 0.02 | ok (clean exit) |
| 83 | PASS | **Silent Hill Mobile** | Classics | 476.6 | 2D | Konami | 0.18 | ok (clean exit) |
| 84 | PASS | **Resident Evil: Uprising** | Classics | 768.3 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 85 | PASS | **Tomb Raider: Underworld** | Classics | 1084.0 | M3G | Electronic Arts | 0.07 | ok (clean exit) |
| 86 | PASS | **Doodle Jump** | Classics | 325.8 | 2D | Mr. Goodliving Ltd b | 0.21 | ok (clean exit) |
| 87 | PASS | **Cut the Rope** | Classics | 1006.0 | 2D | MadiyarM | 0.07 | ok (clean exit) |
| 88 | PASS | **Angry Birds** | Classics | 394.5 | 2D | Seen123@spaces.ru | 0.05 | ok (clean exit) |
| 89 | **TIMEOUT** | **Fruit Ninja** | Classics | 462.2 | 2D | Roman Pivtso | 45.01 | Process timeout > 45s |
| 90 | PASS | **Contra 4** | Classics | 556.1 | 2D | Connect 2 Media/BiNP | 0.29 | ok (clean exit) |
| 91 | PASS | **Metal Slug 4** | Classics | 748.3 | 2D | I-play | 0.05 | ok (clean exit) |
| 92 | PASS | **Sonic the Hedgehog** | Classics | 718.2 | 2D | Glu Mobile Ltd | 0.54 | ok (clean exit) |
| 93 | PASS | **Sonic Advance** | Classics | 1065.9 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 94 | PASS | **Castlevania: Order of Shadows** | Classics | 1048.5 | 2D | CWA | 0.05 | ok (clean exit) |
| 95 | PASS | **Mega Man** | Classics | 669.1 | 2D | Capcom | 0.05 | ok (clean exit) |
| 96 | PASS | **Bomberman Deluxe** | Classics | 171.9 | 2D | ODESSEY | 0.07 | ok (clean exit) |
| 97 | PASS | **PAC-MAN Party** | Classics | 823.8 | 2D | Electronic Arts | 0.05 | ok (clean exit) |
| 98 | PASS | **Crash Bandicoot: Mutant Island** | Classics | 893.5 | 2D | wap.fonzo.game-java. | 0.05 | ok (clean exit) |
| 99 | PASS | **JBenchmark 1** | Benchmark | 62.5 | 2D | Kishonti LP | 0.03 | ok (clean exit) |
| 100 | PASS | **JBenchmark 2** | Benchmark | 63.3 | 2D | Kishonti LP | 0.03 | ok (clean exit) |
| 101 | PASS | **Galaxy on Fire 1 3D** | 3D_M3G | 791.0 | M3G | www.fishlabs.net | 0.03 | ok (clean exit) |
| 102 | PASS | **Burning Tires 3D** | 3D_M3G | 502.4 | Mascot3D | I-play TEGOS | 0.22 | ok (clean exit) |
| 103 | PASS | **Heli Strike 3D** | 3D_M3G | 409.5 | M3G | www.fishlabs.net / 6 | 0.03 | ok (clean exit) |
| 104 | PASS | **Gladiator 3D** | 3D_M3G | 1031.7 | Mascot3D | fishlabs | 0.03 | ok (clean exit) |
| 105 | PASS | **Colin McRae Rally 3D** | 3D_M3G | 776.6 | M3G | Glu Mobile | 0.03 | ok (clean exit) |
| 106 | PASS | **V-Rally 3D** | 3D_M3G | 444.5 | M3G | Nokia | 3.18 | ok (clean exit) |
| 107 | PASS | **Dakar 3D** | 3D_M3G | 851.9 | Mascot3D | Electronic Arts, Inc | 0.21 | ok (clean exit) |
| 108 | PASS | **Fast & Furious 3D** | 3D_M3G | 702.6 | M3G | mob.ua | 0.22 | ok (clean exit) |
| 109 | PASS | **Ducati 3D** | 3D_M3G | 280.0 | M3G | Superscape_Retail_SE | 0.17 | ok (clean exit) |
| 110 | PASS | **MotoGP 3D** | 3D_M3G | 143.5 | 2D | mob.ua | 0.55 | ok (clean exit) |
| 111 | PASS | **Star Wars: The Force Unleashed 3D** | 3D_M3G | 346.1 | 2D | THQ | 0.05 | ok (clean exit) |
| 112 | PASS | **Iron Man 3D** | 3D_M3G | 772.4 | 2D | Gameloft SA | 0.07 | ok (clean exit) |
| 113 | PASS | **Terminator Salvation 3D** | 3D_M3G | 524.3 | 2D | Gameloft SA by konon | 0.05 | ok (clean exit) |
| 114 | PASS | **Resident Evil 3D** | 3D_M3G | 900.4 | M3G | CAPCOM | 0.09 | ok (clean exit) |
| 115 | **TIMEOUT** | **Time Crisis 3D** | 3D_M3G | 432.1 | Mascot3D | namco / BerON | 45.01 | Process timeout > 45s |
| 116 | PASS | **Orcs & Elves** | 3D_M3G | 478.8 | 2D | Electronic Arts serv | 0.07 | ok (clean exit) |
| 117 | PASS | **Orcs & Elves II** | 3D_M3G | 290.3 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 118 | PASS | **K-Rally** | 3D_M3G | 1251.5 | Mascot3D | NBGI. | 0.73 | ok (clean exit) |
| 119 | PASS | **Metal Gear Solid Mobile 3D** | 3D_M3G | 935.7 | 2D | Bjorn Carlin, www.ar | 0.06 | ok (clean exit) |
| 120 | PASS | **Micro Counter Strike 3D** | 3D_M3G | 2697.2 | M3G | TEGOS.RU | 2.8 | ok (clean exit) |
| 121 | PASS | **Project Gotham Racing Mobile 3D** | 3D_M3G | 952.0 | M3G | Glu Mobile / sensoru | 0.04 | ok (clean exit) |
| 122 | PASS | **Crash Arena 3D** | 3D_M3G | 348.6 | M3G | MobileLeap | 0.06 | ok (clean exit) |
| 123 | PASS | **3D Autobahn Raser** | 3D_M3G | 235.2 | M3G | Living Mobile | 3.17 | ok (clean exit) |
| 124 | PASS | **3D Snowboard** | 3D_M3G | 701.2 | M3G | fishlabs/BiNPDA | 0.03 | ok (clean exit) |
| 125 | PASS | **3D Street Racing** | 3D_M3G | 1118.3 | M3G | Gameloft SA | 0.05 | ok (clean exit) |
| 126 | PASS | **3D Urban Attack** | 3D_M3G | 1117.7 | 2D | VGM | 0.08 | ok (clean exit) |
| 127 | PASS | **3D Minigolf** | 3D_M3G | 116.4 | 2D | Synergenix Interacti | 0.07 | ok (clean exit) |
| 128 | PASS | **3D Pool** | 3D_M3G | 1116.3 | M3G | Gameloft SA | 0.06 | ok (clean exit) |
| 129 | PASS | **3D Rollercoaster** | 3D_M3G | 368.3 | M3G | Digital Chocolate, I | 0.04 | ok (clean exit) |
| 130 | PASS | **Prince of Persia: Warrior Within** | Gameloft | 202.1 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 131 | PASS | **Prince of Persia: The Two Thrones** | Gameloft | 348.6 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 132 | PASS | **Prince of Persia: Harem Adventures** | Gameloft | 63.0 | 2D | Gameloft | 0.02 | ok (clean exit) |
| 133 | PASS | **Prince of Persia: Sands of Time** | Gameloft | 196.9 | 2D | Gameloft SA | 0.03 | ok (clean exit) |
| 134 | PASS | **Assassin's Creed: Revelations** | Gameloft | 976.2 | 2D | Gameloft SA by DUAL1 | 0.08 | ok (clean exit) |
| 135 | PASS | **Assassin's Creed III** | Gameloft | 996.4 | 2D | Gameloft SA wapxx.or | 0.07 | ok (clean exit) |
| 136 | PASS | **Asphalt: Urban GT** | Gameloft | 339.5 | M3G | Gameloft | 0.03 | ok (clean exit) |
| 137 | PASS | **Asphalt: Urban GT 2** | Gameloft | 364.4 | M3G | Gameloft SA | 0.03 | ok (clean exit) |
| 138 | PASS | **Asphalt 2** | Gameloft | 326.3 | 2D | Gameloft SA | 0.03 | ok (clean exit) |
| 139 | PASS | **Asphalt 5** | Gameloft | 357.0 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 140 | PASS | **Gangstar Rio: City of Saints** | Gameloft | 890.1 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 141 | PASS | **Splinter Cell: Pandora Tomorrow** | Gameloft | 223.7 | 2D | Gameloft SA | 0.03 | ok (clean exit) |
| 142 | **TIMEOUT** | **Splinter Cell: Chaos Theory** | Gameloft | 226.5 | 2D | Gameloft SA | 45.02 | Process timeout > 45s |
| 143 | PASS | **Splinter Cell: Double Agent** | Gameloft | 341.7 | 2D | Gameloft SA | 0.86 | ok (clean exit) |
| 144 | PASS | **Real Football 2006** | Gameloft | 578.2 | M3G | Gameloft SA | 0.04 | ok (clean exit) |
| 145 | PASS | **Real Football 2007** | Gameloft | 1004.8 | M3G | Gameloft SA | 0.07 | ok (clean exit) |
| 146 | PASS | **Real Football 2009** | Gameloft | 683.9 | 2D | http://blackcat.bal- | 0.08 | ok (clean exit) |
| 147 | PASS | **Real Football 2011** | Gameloft | 1080.3 | 2D | sensoru.net | 0.1 | ok (clean exit) |
| 148 | PASS | **Real Football 2012** | Gameloft | 378.6 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 149 | PASS | **Real Football 2013** | Gameloft | 997.7 | 2D | Gameloft SA | 0.11 | ok (clean exit) |
| 150 | PASS | **Dungeon Hunter 2** | Gameloft | 828.0 | 2D | Gameloft SA by DUAL1 | 0.08 | ok (clean exit) |
| 151 | PASS | **Dungeon Hunter 3** | Gameloft | 941.7 | 2D | Gameloft SA by Stox | 0.19 | ok (clean exit) |
| 152 | PASS | **Zombie Infection 2** | Gameloft | 658.1 | 2D | Gameloft SA , GoD_Sm | 0.1 | ok (clean exit) |
| 153 | PASS | **Diamond Twister 2** | Gameloft | 1009.1 | 2D | Gameloft SA by Stox | 0.07 | ok (clean exit) |
| 154 | PASS | **Block Breaker Deluxe 2** | Gameloft | 1096.1 | 2D | Gameloft SA | 0.07 | ok (clean exit) |
| 155 | PASS | **Block Breaker 3 Unlimited** | Gameloft | 395.7 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 156 | PASS | **Paris Nights** | Gameloft | 1009.4 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 157 | PASS | **Las Vegas Nights** | Gameloft | 584.1 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 158 | PASS | **Driver: L.A. Undercover** | Gameloft | 543.7 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 159 | PASS | **Driver: San Francisco** | Gameloft | 1268.0 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 160 | PASS | **Driver: Vegas** | Gameloft | 322.8 | 2D | Glu Mobile by Stox | 0.03 | ok (clean exit) |
| 161 | PASS | **Shrek the Third** | Gameloft | 338.4 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 162 | PASS | **Shrek Forever After** | Gameloft | 756.6 | M3G | Gameloft SA | 0.08 | ok (clean exit) |
| 163 | PASS | **Spider-Man: Toxic City** | Gameloft | 627.3 | 2D | Gameloft SA by konon | 0.04 | ok (clean exit) |
| 164 | PASS | **Spider-Man 3** | Gameloft | 648.6 | 2D | Gameloft SA | 0.11 | ok (clean exit) |
| 165 | PASS | **The Amazing Spider-Man** | Gameloft | 1017.6 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 166 | PASS | **Ultimate Spider-Man** | Gameloft | 339.5 | 2D | Mforma_fix_by_BlackW | 0.04 | ok (clean exit) |
| 167 | PASS | **The Dark Knight Rises** | Gameloft | 1013.2 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 168 | PASS | **Batman Begins** | Gameloft | 64.1 | 2D | WarnerBros | 0.02 | ok (clean exit) |
| 169 | PASS | **Iron Man 2** | Gameloft | 783.3 | 2D | Gameloft SA / sensor | 0.07 | ok (clean exit) |
| 170 | PASS | **Avatar** | Gameloft | 624.4 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 171 | PASS | **Wild West Guns** | Gameloft | 911.9 | 2D | Gameloft SA by Stox | 0.1 | ok (clean exit) |
| 172 | PASS | **Chuck Norris: Bring on the Pain** | Gameloft | 646.3 | 2D | Gameloft SA by konon | 0.05 | ok (clean exit) |
| 173 | PASS | **Brain Challenge** | Gameloft | 774.8 | 2D | Gameloft SA by DUAL1 | 0.11 | ok (clean exit) |
| 174 | PASS | **Brain Challenge 2** | Gameloft | 802.5 | 2D | Gameloft SA | 0.11 | ok (clean exit) |
| 175 | PASS | **Million Dollar Poker** | Gameloft | 585.6 | 2D | Gameloft SA / suppli | 0.05 | ok (clean exit) |
| 176 | PASS | **Platinum Solitaire** | Gameloft | 1282.1 | 2D | [url=http://ruwapa.n | 0.06 | ok (clean exit) |
| 177 | PASS | **Platinum Solitaire 2** | Gameloft | 792.4 | 2D | Gameloft SA by konon | 0.08 | ok (clean exit) |
| 178 | PASS | **Bubble Bash** | Gameloft | 902.4 | 2D | Gameloft SA by DUAL1 | 0.08 | ok (clean exit) |
| 179 | PASS | **Bubble Bash 2** | Gameloft | 1226.8 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 180 | PASS | **Abracadaball** | Gameloft | 590.0 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 181 | PASS | **Wonder Blocks** | Gameloft | 600.2 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 182 | PASS | **CSI: Miami** | Gameloft | 924.3 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 183 | PASS | **CSI: New York** | Gameloft | 609.3 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 184 | PASS | **Lost** | Gameloft | 395.1 | 2D | pux.su | 0.05 | ok (clean exit) |
| 185 | PASS | **Heroes** | Gameloft | 570.4 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 186 | PASS | **Desperate Housewives** | Gameloft | 368.4 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 187 | PASS | **Brothers in Arms: Earned in Blood** | Gameloft | 258.8 | 2D | Gameloft SA | 0.03 | ok (clean exit) |
| 188 | PASS | **Brothers in Arms: Art of War** | Gameloft | 598.3 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 189 | PASS | **Brothers in Arms 3D** | Gameloft | 992.1 | 2D | Falcon Mobile Inc/Mo | 0.06 | ok (clean exit) |
| 190 | PASS | **Rayman Raving Rabbids** | Gameloft | 602.3 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 191 | PASS | **Rayman 3** | Gameloft | 344.3 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 192 | PASS | **Nightmare Creatures** | Gameloft | 159.9 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 193 | PASS | **Medieval Combat: Age of Glory** | Gameloft | 249.2 | 2D | mob.ua | 0.05 | ok (clean exit) |
| 194 | PASS | **Die Hard 4.0** | Gameloft | 1598.6 | 2D | Gameloft SA RuGame.m | 0.11 | ok (clean exit) |
| 195 | PASS | **Alien vs Predator 2** | Gameloft | 246.2 | M3G | Glu Mobile | 0.19 | ok (clean exit) |
| 196 | PASS | **Men in Black 3** | Gameloft | 347.0 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 197 | PASS | **Ice Age: Dawn of the Dinosaurs** | Gameloft | 1014.0 | 2D | pux.su | 0.05 | ok (clean exit) |
| 198 | PASS | **Ice Age: Continental Drift** | Gameloft | 1094.4 | 2D | Gameloft SA | 0.06 | ok (clean exit) |
| 199 | PASS | **Green Farm** | Gameloft | 620.8 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 200 | PASS | **Green Farm 2** | Gameloft | 664.6 | 2D | javabank.ru | 0.26 | ok (clean exit) |
| 201 | PASS | **Green Farm 3** | Gameloft | 612.2 | 2D | Gameloft SA by Stox | 0.07 | ok (clean exit) |
| 202 | PASS | **Little Big City** | Gameloft | 904.7 | 2D | Gameloft SA | 0.21 | ok (clean exit) |
| 203 | PASS | **Danger Dash** | Gameloft | 776.1 | 2D | Gameloft SA | 0.16 | ok (clean exit) |
| 204 | PASS | **Ninja Up!** | Gameloft | 368.4 | 2D | MrRap | 0.04 | ok (clean exit) |
| 205 | PASS | **The Sims 2** | EA_Mobile | 711.5 | 2D | Electronic Arts | 0.02 | ok (clean exit) |
| 206 | PASS | **The Sims 2: Castaway** | EA_Mobile | 711.6 | 2D | Electronic Arts, ser | 0.03 | ok (clean exit) |
| 207 | PASS | **The Sims 2: Pets** | EA_Mobile | 568.1 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 208 | PASS | **The Sims Medieval** | EA_Mobile | 344.1 | 2D | Electronic Arts Inc. | 0.05 | ok (clean exit) |
| 209 | PASS | **The Sims 3: Ambitions** | EA_Mobile | 779.6 | 2D | Electronic Arts by S | 0.03 | ok (clean exit) |
| 210 | PASS | **SimCity Societies** | EA_Mobile | 604.3 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 211 | PASS | **SimCity Metropolis** | EA_Mobile | 774.5 | 2D | Electronic Arts/BiNP | 0.1 | ok (clean exit) |
| 212 | PASS | **Need for Speed: Underground 2** | EA_Mobile | 1417.9 | M3G | FEARLESS | 0.05 | ok (clean exit) |
| 213 | PASS | **Need for Speed: ProStreet** | EA_Mobile | 738.5 | M3G | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 214 | PASS | **Need for Speed: Undercover** | EA_Mobile | 1001.8 | M3G | Electronic Arts | 0.16 | ok (clean exit) |
| 215 | PASS | **Need for Speed: Hot Pursuit** | EA_Mobile | 1176.6 | M3G | mob.ua | 0.46 | ok (clean exit) |
| 216 | PASS | **Need for Speed: The Run** | EA_Mobile | 1046.3 | M3G | Electronic Arts Inc. | 0.56 | ok (clean exit) |
| 217 | PASS | **FIFA 07** | EA_Mobile | 438.6 | M3G | Electronic Arts / Su | 0.05 | ok (clean exit) |
| 218 | PASS | **FIFA 08** | EA_Mobile | 866.8 | M3G | Electronic Arts Inc. | 0.22 | ok (clean exit) |
| 219 | PASS | **FIFA 09** | EA_Mobile | 516.1 | 2D | pux.su | 0.05 | ok (clean exit) |
| 220 | PASS | **FIFA 11** | EA_Mobile | 938.7 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 221 | PASS | **FIFA 12** | EA_Mobile | 602.8 | M3G | Electronic Arts Inc. | 0.28 | ok (clean exit) |
| 222 | PASS | **FIFA 13** | EA_Mobile | 597.4 | M3G | Electronic Arts Inc. | 0.13 | ok (clean exit) |
| 223 | PASS | **FIFA 14** | EA_Mobile | 1232.3 | M3G | Electronic Arts | 0.18 | ok (clean exit) |
| 224 | PASS | **Fight Night Round 3** | EA_Mobile | 286.2 | M3G | Electronic Arts | 0.09 | ok (clean exit) |
| 225 | PASS | **Medal of Honor: Airborne** | EA_Mobile | 559.8 | 2D | Electronic Arts | 0.5 | ok (clean exit) |
| 226 | PASS | **Command & Conquer 3: Tiberium Wars** | EA_Mobile | 466.4 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 227 | PASS | **Command & Conquer: Red Alert** | EA_Mobile | 784.8 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 228 | PASS | **Spore** | EA_Mobile | 648.7 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 229 | PASS | **Tetris (EA)** | EA_Mobile | 552.0 | M3G | [url=http://ruwapa.n | 0.05 | ok (clean exit) |
| 230 | PASS | **Monopoly World** | EA_Mobile | 510.4 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 231 | PASS | **Monopoly Classic** | EA_Mobile | 475.8 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 232 | PASS | **Scrabble** | EA_Mobile | 626.5 | 2D | Electronic Arts, Inc | 0.06 | ok (clean exit) |
| 233 | PASS | **Yahtzee Deluxe** | EA_Mobile | 157.7 | 2D | JAMDAT Mobile Inc. / | 0.03 | ok (clean exit) |
| 234 | PASS | **The Game of Life** | EA_Mobile | 435.0 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 235 | PASS | **Trivial Pursuit** | EA_Mobile | 565.3 | 2D | Electronic Arts, Inc | 0.08 | ok (clean exit) |
| 236 | PASS | **Lemonade Tycoon** | EA_Mobile | 161.0 | 2D | JAMDAT Mobile inc. / | 0.03 | ok (clean exit) |
| 237 | PASS | **NBA Live 08** | EA_Mobile | 488.0 | 2D | Electronic Arts/BiNP | 0.03 | ok (clean exit) |
| 238 | PASS | **NBA Live 10** | EA_Mobile | 491.3 | 2D | mobigama.ru | 0.41 | ok (clean exit) |
| 239 | PASS | **NHL 07** | EA_Mobile | 286.2 | 2D | THQ Wireless, Inc. | 0.04 | ok (clean exit) |
| 240 | PASS | **Tiger Woods PGA Tour 09** | EA_Mobile | 643.2 | 2D | Electronic Arts, Inc | 0.19 | ok (clean exit) |
| 241 | PASS | **Mass Effect Infiltrator** | EA_Mobile | 951.1 | 2D | Falcon Mobile/Mordor | 0.05 | ok (clean exit) |
| 242 | **TIMEOUT** | **Battlefield: Bad Company 2** | EA_Mobile | 1331.8 | M3G | Vendor | 45.01 | Process timeout > 45s |
| 243 | PASS | **Burnout** | EA_Mobile | 527.7 | M3G | Electronic Arts/BiNP | 0.03 | ok (clean exit) |
| 244 | PASS | **Worms 2007** | EA_Mobile | 227.3 | M3G | THQ Wireless Inc | 0.61 | ok (clean exit) |
| 245 | **MISSING_API** | **Worms Forts: Under Siege** | EA_Mobile | 271.2 | Mascot3D | THQ Wireless | 0.04 | Java Exception: java/lang/NoClassDefFoundError: co |
| 246 | PASS | **Worms: Reloaded** | EA_Mobile | 842.1 | 2D | Electronic Arts Inc. | 0.23 | ok (clean exit) |
| 247 | PASS | **Bejeweled 2** | PopCap | 362.7 | 2D | Electronic Arts, Inc | 0.05 | ok (clean exit) |
| 248 | PASS | **Bejeweled** | PopCap | 369.3 | 2D | EA Mobile | 0.05 | ok (clean exit) |
| 249 | PASS | **Peggle Mobile** | PopCap | 685.8 | 2D | Electronic Arts Inc. | 21.97 | ok (clean exit) |
| 250 | PASS | **Silent Hill Mobile 2** | SEGA_Konami_Capcom | 473.9 | 2D | Konami | 0.26 | ok (clean exit) |
| 251 | PASS | **Silent Hill Mobile 3** | SEGA_Konami_Capcom | 492.1 | 2D | Konami | 0.27 | ok (clean exit) |
| 252 | PASS | **Silent Hill: Orphan** | SEGA_Konami_Capcom | 485.7 | 2D | Konami n polick11 | 0.17 | ok (clean exit) |
| 253 | PASS | **Castlevania: Aria of Sorrow** | SEGA_Konami_Capcom | 422.7 | 2D | Konami | 0.15 | ok (clean exit) |
| 254 | PASS | **Castlevania: Dawn of Sorrow** | SEGA_Konami_Capcom | 293.7 | 2D | Konami | 0.04 | ok (clean exit) |
| 255 | PASS | **Resident Evil: Genesis** | SEGA_Konami_Capcom | 268.5 | 2D | Capcom Interactive,  | 0.15 | ok (clean exit) |
| 256 | PASS | **Resident Evil: Degeneration** | SEGA_Konami_Capcom | 917.2 | M3G | CAPCOM/BiNPDA | 0.06 | ok (clean exit) |
| 257 | PASS | **Resident Evil: The Missions** | SEGA_Konami_Capcom | 963.7 | M3G | CAPCOM | 0.08 | ok (clean exit) |
| 258 | PASS | **Devil May Cry** | SEGA_Konami_Capcom | 460.3 | 2D | YURIK | 0.04 | ok (clean exit) |
| 259 | PASS | **Devil May Cry 3** | SEGA_Konami_Capcom | 947.5 | 2D | Finger Charm | 0.06 | ok (clean exit) |
| 260 | PASS | **Devil May Cry 4** | SEGA_Konami_Capcom | 944.4 | 2D | Capcom [by TonCat23  | 0.06 | ok (clean exit) |
| 261 | PASS | **Sonic the Hedgehog 2** | SEGA_Konami_Capcom | 278.7 | 2D | Beijing palm full pa | 0.1 | ok (clean exit) |
| 262 | PASS | **Sonic Jump** | SEGA_Konami_Capcom | 331.3 | 2D | Glu Mobile/BiNPDA | 0.5 | ok (clean exit) |
| 263 | PASS | **Sonic at the Olympic Games** | SEGA_Konami_Capcom | 347.9 | 2D | SEGA | 0.1 | ok (clean exit) |
| 264 | PASS | **Sonic Racing** | SEGA_Konami_Capcom | 823.5 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 265 | PASS | **Sonic Unleashed** | SEGA_Konami_Capcom | 648.3 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 266 | PASS | **Super Monkey Ball** | SEGA_Konami_Capcom | 355.8 | 2D | Glu Mobile/daddyfats | 0.03 | ok (clean exit) |
| 267 | PASS | **Virtua Tennis Mobile** | SEGA_Konami_Capcom | 202.6 | 2D | giu | 0.09 | ok (clean exit) |
| 268 | PASS | **After Burner** | SEGA_Konami_Capcom | 63.7 | 2D | Stuart2773 | 0.04 | ok (clean exit) |
| 269 | PASS | **Golden Axe** | SEGA_Konami_Capcom | 764.7 | 2D | Necrophilia | 0.04 | ok (clean exit) |
| 270 | PASS | **Streets of Rage** | SEGA_Konami_Capcom | 731.4 | 2D | SEGA WOW INC. | 0.05 | ok (clean exit) |
| 271 | PASS | **Metal Slug 1** | SEGA_Konami_Capcom | 149.9 | 2D | TEGOS.ru | 0.05 | ok (clean exit) |
| 272 | PASS | **Metal Slug 2** | SEGA_Konami_Capcom | 626.1 | 2D | I-play | 0.06 | ok (clean exit) |
| 273 | PASS | **Metal Slug 3** | SEGA_Konami_Capcom | 518.8 | 2D | noname | 8.03 | running (game loop active) |
| 274 | PASS | **Frogger** | SEGA_Konami_Capcom | 217.0 | 2D | sizka.net / sizka.ne | 0.03 | ok (clean exit) |
| 275 | PASS | **Pro Evolution Soccer 2008** | SEGA_Konami_Capcom | 329.7 | 2D | Konami | 0.04 | ok (clean exit) |
| 276 | PASS | **Pro Evolution Soccer 2009** | SEGA_Konami_Capcom | 379.2 | 2D | Konami | 0.04 | ok (clean exit) |
| 277 | PASS | **Pro Evolution Soccer 2010** | SEGA_Konami_Capcom | 502.7 | 2D | Connect2Media. | 0.04 | ok (clean exit) |
| 278 | PASS | **Pro Evolution Soccer 2011** | SEGA_Konami_Capcom | 602.5 | 2D | Konami | 0.22 | ok (clean exit) |
| 279 | PASS | **Pro Evolution Soccer 2012** | SEGA_Konami_Capcom | 711.5 | 2D | Konami | 0.04 | ok (clean exit) |
| 280 | PASS | **Mega Man II** | SEGA_Konami_Capcom | 607.9 | 2D | Capcom | 0.06 | ok (clean exit) |
| 281 | PASS | **Mega Man III** | SEGA_Konami_Capcom | 212.4 | 2D | Capcom Interactive,  | 0.07 | ok (clean exit) |
| 282 | PASS | **Street Fighter Alpha** | SEGA_Konami_Capcom | 538.3 | 2D | Gameloft SA | 0.22 | ok (clean exit) |
| 283 | PASS | **Street Fighter II** | SEGA_Konami_Capcom | 963.6 | 2D | gameCho | 0.1 | ok (clean exit) |
| 284 | PASS | **1942 (Capcom)** | SEGA_Konami_Capcom | 944.6 | 2D | www.minisoyo.com | 0.03 | ok (clean exit) |
| 285 | PASS | **Ghosts 'n Goblins** | SEGA_Konami_Capcom | 289.1 | 2D | CAPCOM | 0.06 | ok (clean exit) |
| 286 | PASS | **Phoenix Wright: Ace Attorney** | SEGA_Konami_Capcom | 254.6 | 2D | CAPCOM/BiNPDA | 8.03 | running (game loop active) |
| 287 | PASS | **Pac-Man (Namco)** | SEGA_Konami_Capcom | 697.9 | 2D | Namco serviak | 0.05 | ok (clean exit) |
| 288 | PASS | **Ms. Pac-Man** | SEGA_Konami_Capcom | 259.9 | 2D | Namco | 0.06 | ok (clean exit) |
| 289 | PASS | **Galaga** | SEGA_Konami_Capcom | 792.9 | 2D | NBNE/ | 0.06 | ok (clean exit) |
| 290 | PASS | **Tekken Mobile** | SEGA_Konami_Capcom | 477.6 | M3G | mod by Danchyk | 0.84 | ok (clean exit) |
| 291 | PASS | **Soulcalibur Mobile** | SEGA_Konami_Capcom | 846.9 | 2D | mob.ua | 0.08 | ok (clean exit) |
| 292 | PASS | **Ace Combat Mobile** | SEGA_Konami_Capcom | 848.3 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 293 | PASS | **Townsmen 1** | HandyGames_HeroCraft | 85.7 | 2D | www.handy-games.com  | 0.06 | ok (clean exit) |
| 294 | PASS | **Townsmen 2** | HandyGames_HeroCraft | 184.0 | 2D | www.handy-games.com  | 0.06 | ok (clean exit) |
| 295 | PASS | **Townsmen 3** | HandyGames_HeroCraft | 194.0 | 2D | www.handy-games.com  | 0.06 | ok (clean exit) |
| 296 | PASS | **Townsmen 4** | HandyGames_HeroCraft | 159.6 | 2D | www.handy-games.com  | 0.58 | ok (clean exit) |
| 297 | PASS | **Townsmen 5** | HandyGames_HeroCraft | 232.8 | 2D | mob.ua | 0.06 | ok (clean exit) |
| 298 | PASS | **Cyberlords: Arcology** | HandyGames_HeroCraft | 625.6 | 2D | [url=http://ruwapa.n | 8.03 | running (game loop active) |
| 299 | PASS | **Guns'n'Glory** | HandyGames_HeroCraft | 578.4 | 2D | Handy-Games | 0.07 | ok (clean exit) |
| 300 | PASS | **Devils & Demons** | HandyGames_HeroCraft | 504.6 | 2D | www.handy-games.com  | 0.05 | ok (clean exit) |
| 301 | PASS | **Aces of the Luftwaffe** | HandyGames_HeroCraft | 628.4 | 2D | www.handy-games.com  | 0.04 | ok (clean exit) |
| 302 | PASS | **Farm Invasion USA** | HandyGames_HeroCraft | 612.0 | 2D | WapBox.net | 0.09 | ok (clean exit) |
| 303 | PASS | **Aporkalypse** | HandyGames_HeroCraft | 556.3 | 2D | www.handy-games.com  | 0.08 | ok (clean exit) |
| 304 | PASS | **Dynamite Fishing** | HandyGames_HeroCraft | 449.5 | 2D | handy-games_retail_B | 0.04 | ok (clean exit) |
| 305 | PASS | **Shark or Die** | HandyGames_HeroCraft | 504.7 | 2D | www.handy-games.com  | 0.1 | ok (clean exit) |
| 306 | PASS | **Tattoo Tycoon** | HandyGames_HeroCraft | 636.4 | 2D | www.handy-games.com  | 0.15 | ok (clean exit) |
| 307 | PASS | **Vegas Hustler** | HandyGames_HeroCraft | 499.5 | 2D | Glu Mobile | 0.03 | ok (clean exit) |
| 308 | PASS | **Revival** | HandyGames_HeroCraft | 169.3 | 2D | V&V, Herocraft / Kon | 0.02 | ok (clean exit) |
| 309 | PASS | **Revival 2** | HandyGames_HeroCraft | 346.4 | 2D | V&V, HeroCraft | 0.23 | ok (clean exit) |
| 310 | PASS | **Art of War** | HandyGames_HeroCraft | 838.8 | 2D | Gear Games | 0.03 | ok (clean exit) |
| 311 | PASS | **Art of War 2: Global Confederation** | HandyGames_HeroCraft | 608.6 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 312 | PASS | **Majesty: The Fantasy Kingdom Sim** | HandyGames_HeroCraft | 916.0 | 2D | HeroCraft | 0.86 | ok (clean exit) |
| 313 | PASS | **Robo 2** | HandyGames_HeroCraft | 633.8 | 2D | HeroCraft | 0.22 | ok (clean exit) |
| 314 | PASS | **Dragon & Dracula** | HandyGames_HeroCraft | 546.2 | M3G | HeroCraft | 0.21 | ok (clean exit) |
| 315 | PASS | **Elven Chronicles** | HandyGames_HeroCraft | 235.3 | 2D | BigBlueBubble | 0.06 | ok (clean exit) |
| 316 | PASS | **Bobby Carrot 1** | Classics | 282.6 | 2D | FDGSoft | 0.04 | ok (clean exit) |
| 317 | PASS | **Bobby Carrot 2** | Classics | 169.2 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 318 | PASS | **Bobby Carrot 3: Evolution** | Classics | 284.1 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 319 | PASS | **Bounce** | Classics | 55.0 | 2D | Nokia | 0.04 | ok (clean exit) |
| 320 | PASS | **Gravity Defied: Pro** | Classics | 103.1 | 2D | Codebrew Software &  | 0.02 | ok (clean exit) |
| 321 | PASS | **Mafia Mobile** | Classics | 639.2 | 2D | Connect2Media | 0.04 | ok (clean exit) |
| 322 | PASS | **Age of Heroes I** | Classics | 288.2 | 2D | Qplaze /supplied by  | 0.04 | ok (clean exit) |
| 323 | PASS | **Age of Heroes II: Underground Horror** | Classics | 147.6 | 2D | Qplaze | 0.09 | ok (clean exit) |
| 324 | PASS | **Age of Heroes IV: Blood and Twilight** | Classics | 444.5 | 2D | mod by Danchyk | 0.05 | ok (clean exit) |
| 325 | PASS | **Age of Heroes V: Chimaera's Heart** | Classics | 297.6 | 2D | Spirit61 | 0.04 | ok (clean exit) |
| 326 | PASS | **Darkest Fear 2: Grim Oak** | Classics | 179.7 | 2D | Rovio | 0.03 | ok (clean exit) |
| 327 | PASS | **Darkest Fear 3: Nightmare** | Classics | 213.6 | 2D | mob.ua | 0.02 | ok (clean exit) |
| 328 | PASS | **Tower Bloxx** | Classics | 419.5 | M3G | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 329 | PASS | **3D Rollercoaster Rush** | Classics | 756.2 | M3G | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 330 | PASS | **Rollercoaster Rush 99 Tracks** | Classics | 303.8 | 2D | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 331 | PASS | **Rollercoaster Rush** | Classics | 369.2 | M3G | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 332 | PASS | **Crazy Penguin Catapult** | Classics | 378.4 | 2D | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 333 | PASS | **Crazy Penguin Catapult 2** | Classics | 398.9 | M3G | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 334 | PASS | **Diamond Islands** | Classics | 395.1 | 2D | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 335 | PASS | **Diamond Islands 2** | Classics | 640.3 | 2D | Digital Chocolate, I | 0.03 | ok (clean exit) |
| 336 | PASS | **S.T.A.L.K.E.R. Mobile** | Classics | 302.0 | M3G | Qplaze | 3.37 | ok (clean exit) |
| 337 | PASS | **Fallout Mobile** | Classics | 3026.3 | 2D | Almemusic | 7.59 | ok (clean exit) |
| 338 | PASS | **Subway Surfers (Java)** | Classics | 2218.8 | 2D | mob.ua | 0.06 | ok (clean exit) |
| 339 | PASS | **Temple Run (Java)** | Classics | 643.0 | 2D | mob.ua | 0.07 | ok (clean exit) |
| 340 | PASS | **Flappy Bird (Java)** | Classics | 174.5 | 2D | pux.su | 0.06 | ok (clean exit) |
| 341 | PASS | **Minecraft 2D** | Classics | 65.9 | 2D | Tedium | 0.03 | ok (clean exit) |
| 342 | PASS | **Minecraft 3D (Java)** | Classics | 875.3 | 2D | Aperture | 0.04 | ok (clean exit) |
| 343 | PASS | **Doodle Jump Deluxe** | Classics | 188.0 | 2D | Mr. Goodliving Ltd b | 0.03 | ok (clean exit) |
| 344 | PASS | **Fruit Ninja 2** | Classics | 272.6 | 2D | Vendor | 0.03 | ok (clean exit) |
| 345 | PASS | **Angry Birds Seasons** | Classics | 394.5 | 2D | Перевод от Fonzo Tea | 0.04 | ok (clean exit) |
| 346 | PASS | **Angry Birds Rio** | Classics | 1025.8 | 2D | Angry Birds / Mobile | 0.23 | ok (clean exit) |
| 347 | PASS | **Angry Birds Space** | Classics | 4416.1 | 2D | TEGOS | 0.52 | ok (clean exit) |
| 348 | PASS | **Cut the Rope 2** | Classics | 2007.1 | 2D | MadiyarM | 0.08 | ok (clean exit) |
| 349 | PASS | **Kamikaze** | Classics | 240.2 | 2D | HeroCraft | 0.2 | ok (clean exit) |
| 350 | PASS | **Kamikaze 2: The Way of Ninja** | Classics | 210.5 | 2D | HeroCraft | 0.21 | ok (clean exit) |
| 351 | PASS | **Bumer (Бумер)** | Classics | 152.3 | 2D | Qplaze | 0.03 | ok (clean exit) |
| 352 | PASS | **Bumer 2 (Бумер 2)** | Classics | 152.3 | 2D | siza.ru | 0.03 | ok (clean exit) |
| 353 | PASS | **Brigada (Бригада)** | Classics | 176.7 | 2D | RME | 0.03 | ok (clean exit) |
| 354 | PASS | **Russian Fishing (Русская рыбалка)** | Classics | 1450.9 | 2D | killa_bee | 0.06 | ok (clean exit) |
| 355 | PASS | **Parkour** | Classics | 320.2 | 2D | Sina Mobile | 0.02 | ok (clean exit) |
| 356 | PASS | **Pimp My Ride** | Classics | 208.0 | 2D | Infospace | 0.04 | ok (clean exit) |
| 357 | PASS | **Tank-o-box** | Classics | 360.4 | 2D | Reactive Phone Solut | 0.05 | ok (clean exit) |
| 358 | PASS | **Lode Runner** | Classics | 116.5 | 2D | pux.su | 0.03 | ok (clean exit) |
| 359 | PASS | **Durak (Дурак)** | Classics | 680.3 | 2D | MobiLeap | 0.04 | ok (clean exit) |
| 360 | PASS | **Nu Pogodi (Ну погоди)** | Classics | 674.7 | 2D | NET Lizard | 0.03 | ok (clean exit) |
| 361 | PASS | **Pirates of the Caribbean: At World's End** | Disney_Pixar | 185.0 | 2D | WapBox.net | 4.43 | ok (clean exit) |
| 362 | PASS | **Pirates of the Caribbean: Dead Man's Chest** | Disney_Pixar | 446.5 | 2D | Indiagames Ltd. | 0.04 | ok (clean exit) |
| 363 | PASS | **Cars (Тачки)** | Disney_Pixar | 464.2 | 2D | HeroCraft | 0.05 | ok (clean exit) |
| 364 | PASS | **Cars 2** | Disney_Pixar | 463.9 | 2D | sensoru.netMIDlet-Ve | 0.06 | ok (clean exit) |
| 365 | PASS | **TRON: Legacy** | Disney_Pixar | 768.1 | 2D | DisneyBERON | 0.04 | ok (clean exit) |
| 366 | PASS | **Toy Story 3** | Disney_Pixar | 385.8 | 2D | Living Mobile | 0.03 | ok (clean exit) |
| 367 | PASS | **WALL-E** | Disney_Pixar | 347.3 | 2D | CAPCOM | 0.1 | ok (clean exit) |
| 368 | PASS | **Ratatouille** | Disney_Pixar | 176.0 | 2D | THQ Wireless | 0.05 | ok (clean exit) |
| 369 | PASS | **Aladdin** | Disney_Pixar | 540.8 | 2D | Disney Mobile | 0.06 | ok (clean exit) |
| 370 | PASS | **The Lion King** | Disney_Pixar | 199.9 | 2D | Walt Disney company  | 0.03 | ok (clean exit) |
| 371 | PASS | **Tarzan** | Disney_Pixar | 204.6 | 2D | Mr. Goodliving Ltd | 0.25 | ok (clean exit) |
| 372 | PASS | **Hercules** | Disney_Pixar | 260.5 | 2D | Disney Mobile Studio | 0.03 | ok (clean exit) |
| 373 | PASS | **Chip 'n Dale** | Disney_Pixar | 630.7 | 2D | Dynamic Pixels | 0.04 | ok (clean exit) |
| 374 | PASS | **Darkwing Duck** | Disney_Pixar | 235.1 | 2D | DWC | 6.11 | ok (clean exit) |
| 375 | PASS | **Хроники нарнии 3 1 mbS by gameloft** | Gameloft | 1234.5 | 2D | Gameloft SA | 0.16 | ok (clean exit) |
| 376 | PASS | **Real Football 2011 отGameloft 2010** | Gameloft | 781.6 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 377 | PASS | **Gameloft OsTitans** | Gameloft | 552.3 | 2D | Gameloft | 0.04 | ok (clean exit) |
| 378 | PASS | **Gameloft Soul Of Darkness** | Gameloft | 275.5 | 2D | Gameloft | 0.05 | ok (clean exit) |
| 379 | PASS | **gameloft676** | Gameloft | 670.7 | 2D | Gameloft SA | 0.07 | ok (clean exit) |
| 380 | PASS | **Gamelofts BackgammonS** | Gameloft | 178.4 | 2D | Gameloft SA | 1.35 | ok (clean exit) |
| 381 | PASS | **Gamelofts Backgammon Русская версия** | Gameloft | 178.5 | 2D | Gameloft SA | 1.35 | ok (clean exit) |
| 382 | PASS | **Wild West Guns Gameloft** | Gameloft | 2206.1 | 2D | Gameloft SA | 0.11 | ok (clean exit) |
| 383 | PASS | **Driver San Francisco Gameloft 128х160** | Gameloft | 256.0 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 384 | PASS | **megasity empire by gameloft** | Gameloft | 545.8 | 2D | Gameloft SA | 0.35 | ok (clean exit) |
| 385 | PASS | **Asphalt 6 Adrenaline от Gameloft** | Gameloft | 365.3 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 386 | PASS | **Gameloft Brain Challenge 4 Breaking Limi** | Gameloft | 765.0 | 2D | Gameloft SA | 0.13 | ok (clean exit) |
| 387 | PASS | **Gameloft Cannon Rats** | Gameloft | 787.8 | 2D | Gameloft SA by Stox | 0.05 | ok (clean exit) |
| 388 | PASS | **Gameloft Gangstar 3 Miami Vindication v** | Gameloft | 1096.8 | 2D | Gameloft SA | 0.1 | ok (clean exit) |
| 389 | PASS | **LegoBatman2 new gameloft** | Gameloft | 321.6 | 2D | Gameloft SA by Stox | 0.04 | ok (clean exit) |
| 390 | PASS | **Big Range Hunting 2 New Gameloft HIT** | Gameloft | 902.4 | 2D | Gameloft SA | 0.59 | ok (clean exit) |
| 391 | PASS | **Нарды Gamelofts Backgammon 128x128** | Gameloft | 63.7 | 2D | Gameloft SA | 1.12 | ok (clean exit) |
| 392 | PASS | **Нарды Gamelofts Backgammon 176x208** | Gameloft | 140.2 | 2D | Gameloft SA | 1.35 | ok (clean exit) |
| 393 | PASS | **Нарды Gamelofts backgammonS** | Gameloft | 178.3 | 2D | Gameloft SA | 1.35 | ok (clean exit) |
| 394 | PASS | **Gamelofts Backgammon** | Gameloft | 178.4 | 2D | Gameloft SA | 1.35 | ok (clean exit) |
| 395 | PASS | **Gamelofts Backgammon 128x128** | Gameloft | 63.7 | 2D | Gameloft SA | 1.12 | ok (clean exit) |
| 396 | PASS | **A Good Day To Die Hard 2013 Gameloft** | Gameloft | 1008.7 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 397 | PASS | **CHESSMASTER GAMELOFT** | Gameloft | 206.2 | 2D | Gameloft SA | 0.08 | ok (clean exit) |
| 398 | PASS | **horse riding academy gameloft** | Gameloft | 263.3 | 2D | mob.ua | 0.03 | ok (clean exit) |
| 399 | PASS | **DangerDash Nokia Gameloft** | Gameloft | 756.2 | 2D | Gameloft SA by Celsi | 0.16 | ok (clean exit) |
| 400 | PASS | **Snake by Gameloft 240 320** | Gameloft | 456.7 | 2D | Gameloft SA | 0.04 | ok (clean exit) |
| 401 | PASS | **FIFA 2011 На Русском** | EA_Mobile | 1004.0 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 402 | PASS | **FIFA 11 Русская версия 240320** | EA_Mobile | 938.7 | 2D | Electronic Arts Inc. | 0.13 | ok (clean exit) |
| 403 | PASS | **x400 FIFA 11 Рус** | EA_Mobile | 940.2 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 404 | PASS | **59 TheSims3** | EA_Mobile | 543.7 | M3G | Electronic Arts | 0.93 | ok (clean exit) |
| 405 | PASS | **x320 fifa 2011** | EA_Mobile | 1068.8 | 2D | mobigama.ru | 0.04 | ok (clean exit) |
| 406 | PASS | **x208 need for speed carbon** | EA_Mobile | 294.2 | M3G | mobigama.ru | 0.02 | ok (clean exit) |
| 407 | PASS | **x320 need for speed carbon** | EA_Mobile | 387.3 | M3G | mobigama.ru | 0.02 | ok (clean exit) |
| 408 | PASS | **x320 need for speed carbon s60** | EA_Mobile | 390.8 | M3G | mobigama.ru | 0.02 | ok (clean exit) |
| 409 | PASS | **x208 need for speed undercover** | EA_Mobile | 870.6 | M3G | mobigama.ru | 0.17 | ok (clean exit) |
| 410 | PASS | **FIFA 2011 сенсорные экраны** | EA_Mobile | 1027.1 | 2D | Electronic Arts | 0.14 | ok (clean exit) |
| 411 | PASS | **TheSims3Dream n70** | EA_Mobile | 257.2 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 412 | PASS | **Need for Speed HotPursuit3d 360** | EA_Mobile | 1165.8 | M3G | Electronic Arts | 0.22 | ok (clean exit) |
| 413 | PASS | **FIFA 2010s** | EA_Mobile | 819.8 | 2D | Electronic Arts, ser | 0.21 | ok (clean exit) |
| 414 | PASS | **Sims 3 AmbitionS Nokia** | EA_Mobile | 543.7 | 2D | Electronic Arts by S | 0.04 | ok (clean exit) |
| 415 | PASS | **need for speed** | EA_Mobile | 1115.5 | Mascot3D | mob.ua | 0.42 | ok (clean exit) |
| 416 | PASS | **Need for speed shift 3D** | EA_Mobile | 706.8 | M3G | Electronic Arts by s | 0.33 | ok (clean exit) |
| 417 | PASS | **Sims 3 AmbS s60 240х320 N95** | EA_Mobile | 777.1 | 2D | Electronic Arts by S | 0.04 | ok (clean exit) |
| 418 | PASS | **The sims 3 ambitions** | EA_Mobile | 860.3 | 2D | mob.ua | 0.05 | ok (clean exit) |
| 419 | PASS | **Need for Speed Hot Pursuit** | EA_Mobile | 581.3 | 2D | Electronic Arts Inc. | 0.57 | ok (clean exit) |
| 420 | PASS | **Need for Speed Hot Pursuit sam** | EA_Mobile | 581.3 | 2D | Electronic Arts Inc. | 0.55 | ok (clean exit) |
| 421 | PASS | **Need For Speed Hot PursuitS 240х320** | EA_Mobile | 568.3 | 2D | Electronic Arts Inc. | 0.55 | ok (clean exit) |
| 422 | PASS | **The Sims Pool** | EA_Mobile | 126.9 | 2D | Electronic Arts | 0.05 | ok (clean exit) |
| 423 | PASS | **the sims3 128x128s jar** | EA_Mobile | 249.0 | 2D | Electronic Arts | 0.24 | ok (clean exit) |
| 424 | PASS | **Need for Speed Hot Pursuit s60** | EA_Mobile | 1203.1 | Mascot3D | mob.ua | 0.4 | ok (clean exit) |
| 425 | PASS | **x320 the sims 2 castaway mobile** | EA_Mobile | 711.6 | 2D | mobigama.ru | 0.03 | ok (clean exit) |
| 426 | PASS | **Need For Speed Carbon** | EA_Mobile | 399.4 | M3G | mob.ua | 0.03 | ok (clean exit) |
| 427 | PASS | **NEED FOR SPEED HOT PURSUIT 2010 ауди мод** | EA_Mobile | 577.1 | 2D | Electronic Arts | 0.54 | ok (clean exit) |
| 428 | PASS | **need for speed hot pursuit 3d 176х220** | EA_Mobile | 433.8 | 2D | mob.ua | 0.82 | ok (clean exit) |
| 429 | PASS | **need for speed hot pursuit 3d 360х640** | EA_Mobile | 1165.8 | M3G | mob.ua | 0.22 | ok (clean exit) |
| 430 | PASS | **mysims 360x640 nokia s60v5** | EA_Mobile | 651.2 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 431 | PASS | **The Sims 3   World Adventures** | EA_Mobile | 732.0 | 2D | EA | 0.03 | ok (clean exit) |
| 432 | PASS | **Need for Speed Hot PursuitS** | EA_Mobile | 1157.3 | Mascot3D | Electronic Arts BERO | 0.4 | ok (clean exit) |
| 433 | PASS | **FIFA 2011 160** | EA_Mobile | 339.6 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 434 | PASS | **Sims 3 Dream Ambitions** | EA_Mobile | 793.8 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 435 | PASS | **the sims3** | EA_Mobile | 255.4 | 2D | mobigama.ru | 0.25 | ok (clean exit) |
| 436 | PASS | **Sims 3 Dream Ambitions 240** | EA_Mobile | 796.6 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 437 | PASS | **Sims 3 Dream Ambitions 240 nok** | EA_Mobile | 559.1 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 438 | PASS | **Need For Speed   Most Wanted** | EA_Mobile | 626.9 | M3G | P3N15*Laci | 0.09 | ok (clean exit) |
| 439 | PASS | **x320 fifa 2010** | EA_Mobile | 871.8 | 2D | mobigama.ru | 1.23 | ok (clean exit) |
| 440 | PASS | **x320 fifa 2010 n40** | EA_Mobile | 677.4 | 2D | mobigama.ru | 0.29 | ok (clean exit) |
| 441 | PASS | **x128 fifa 2010** | EA_Mobile | 345.2 | 2D | mobigama.ru | 0.18 | ok (clean exit) |
| 442 | PASS | **x160 fifa 2010** | EA_Mobile | 351.1 | 2D | mobigama.ru | 0.17 | ok (clean exit) |
| 443 | PASS | **x208 fifa 2010** | EA_Mobile | 487.3 | 2D | mobigama.ru | 0.15 | ok (clean exit) |
| 444 | PASS | **x220 fifa 2010** | EA_Mobile | 656.0 | 2D | mobigama.ru | 0.64 | ok (clean exit) |
| 445 | PASS | **x320 fifa 2010 n60** | EA_Mobile | 853.6 | 2D | mobigama.ru | 1.11 | ok (clean exit) |
| 446 | PASS | **The Sims Pool 3D** | EA_Mobile | 772.6 | M3G | Electronic Arts | 0.08 | ok (clean exit) |
| 447 | PASS | **FIFA 2010 рус вер** | EA_Mobile | 877.5 | 2D | Electronic Arts | 1.23 | ok (clean exit) |
| 448 | PASS | **fifa 2011 s60** | EA_Mobile | 1253.5 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 449 | PASS | **FIFA 2011 рус вер** | EA_Mobile | 1094.6 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 450 | PASS | **Sims 3 Dream Ambitions 160 nok** | EA_Mobile | 114.1 | 2D | Electronic Arts | 0.02 | ok (clean exit) |
| 451 | PASS | **Need for Speed Underground Rivals 240x32** | EA_Mobile | 799.4 | M3G | FEARLESS | 0.04 | ok (clean exit) |
| 452 | PASS | **need for speed hot pursuit 2D** | EA_Mobile | 572.0 | 2D | Electronic Arts Inc. | 0.54 | ok (clean exit) |
| 453 | PASS | **need for speed hot pursuit 3d nokia** | EA_Mobile | 1175.7 | M3G | Electronic Arts Inc. | 0.47 | ok (clean exit) |
| 454 | PASS | **3D Need For Speed Pro Street** | EA_Mobile | 638.9 | Mascot3D | Electronic Arts | 0.16 | ok (clean exit) |
| 455 | PASS | **need for speed pro street 3D nokia** | EA_Mobile | 403.4 | M3G | Electronic Arts/BiNP | 0.35 | ok (clean exit) |
| 456 | PASS | **Sims 3 КарьераS 176х208** | EA_Mobile | 225.8 | 2D | Electronic Arts | 0.03 | ok (clean exit) |
| 457 | PASS | **FIFA 2011** | EA_Mobile | 969.7 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 458 | PASS | **FIFA 2010** | EA_Mobile | 813.0 | 2D | wap.fonzo.game-java. | 1.13 | ok (clean exit) |
| 459 | PASS | **need for speed hot pursuit nokia** | EA_Mobile | 711.0 | 2D | wap.fonzo.game-java. | 0.53 | ok (clean exit) |
| 460 | PASS | **need for speed hot pursuit 3d nokia s60** | EA_Mobile | 1315.0 | M3G | wap.fonzo.game-java. | 0.48 | ok (clean exit) |
| 461 | PASS | **need for speed hot pursuit 3ds** | EA_Mobile | 1254.7 | Mascot3D | wap.fonzo.game-java. | 0.43 | ok (clean exit) |
| 462 | PASS | **Need for Speed HotPursuit3d 176** | EA_Mobile | 408.2 | 2D | Electronic Arts | 0.26 | ok (clean exit) |
| 463 | PASS | **Need for Speed Shift** | EA_Mobile | 731.0 | M3G | Electronic Arts | 0.63 | ok (clean exit) |
| 464 | PASS | **SIMS 3 DJ ROMAN KRIVOIs** | EA_Mobile | 531.7 | 2D | Electronic Arts | 0.86 | ok (clean exit) |
| 465 | PASS | **Sims 3 Dream Ambitions 400** | EA_Mobile | 843.1 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 466 | PASS | **Need for Speed Hot Pursuit 3D v 4 3 4** | EA_Mobile | 1127.5 | Mascot3D | Electronic Arts BERO | 0.44 | ok (clean exit) |
| 467 | PASS | **the sims 3 world adventures** | EA_Mobile | 118.9 | 2D | mob.ua | 0.06 | ok (clean exit) |
| 468 | PASS | **fifa 2011 nokias** | EA_Mobile | 559.2 | 2D | Electronic Arts Inc. | 0.16 | ok (clean exit) |
| 469 | PASS | **need for speed PROstreet** | EA_Mobile | 437.2 | M3G | Electronic Arts | 0.42 | ok (clean exit) |
| 470 | PASS | **Need For Speed Undercover 3Ds** | EA_Mobile | 974.4 | Mascot3D | [url=http://ruwapa.n | 0.17 | ok (clean exit) |
| 471 | PASS | **sims4 mod** | EA_Mobile | 691.5 | M3G | Electronic Arts | 0.9 | ok (clean exit) |
| 472 | PASS | **fifa 2010 world cup africa 240** | EA_Mobile | 951.6 | 2D | Electronic Arts serv | 0.04 | ok (clean exit) |
| 473 | PASS | **FIFA Manager 2010 240 w2** | EA_Mobile | 596.8 | 2D | Electronic Arts | 0.1 | ok (clean exit) |
| 474 | PASS | **mysims** | EA_Mobile | 611.8 | 2D | mob.ua | 0.13 | ok (clean exit) |
| 475 | PASS | **Need For Speed World China** | EA_Mobile | 772.9 | 2D | 北京动物 | 0.04 | ok (clean exit) |
| 476 | PASS | **The Sims 3 Dream Ambitions sams240400** | EA_Mobile | 837.8 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 477 | PASS | **The Sims 4 mod** | EA_Mobile | 688.9 | M3G | Electronic Arts | 0.88 | ok (clean exit) |
| 478 | PASS | **x320 need for speed hot pursuit3D** | EA_Mobile | 1115.4 | Mascot3D | mobigama.ru | 0.43 | ok (clean exit) |
| 479 | PASS | **need for speed of drift racing** | EA_Mobile | 266.2 | 2D | mob.ua | 1.14 | ok (clean exit) |
| 480 | PASS | **EA Mobile SimCity Metropolis** | EA_Mobile | 789.3 | 2D | [url=http://ruwapa.n | 0.1 | ok (clean exit) |
| 481 | PASS | **FIFA 11sskaja versija 240320** | EA_Mobile | 939.4 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 482 | PASS | **need for speed undercover** | EA_Mobile | 960.7 | M3G | Electronic Arts | 0.21 | ok (clean exit) |
| 483 | PASS | **FIFA World Cup South Africa** | EA_Mobile | 531.5 | 2D | Electronic Arts | 0.15 | ok (clean exit) |
| 484 | PASS | **Need for Speed HotPursuit3d** | EA_Mobile | 580.8 | 2D | Electronic Arts | 0.56 | ok (clean exit) |
| 485 | PASS | **Need For Speed World** | EA_Mobile | 773.0 | 2D | 北京动物 | 0.04 | ok (clean exit) |
| 486 | PASS | **The Sims 3 Русская** | EA_Mobile | 255.4 | 2D | Electronic Arts | 0.25 | ok (clean exit) |
| 487 | PASS | **3D Need for Speed Hot Pursuit** | EA_Mobile | 1128.3 | Mascot3D | Electronic Arts BERO | 0.41 | ok (clean exit) |
| 488 | PASS | **need for speed most wanted** | EA_Mobile | 691.0 | M3G | mob.ua | 0.09 | ok (clean exit) |
| 489 | PASS | **fifa 2011 480x800** | EA_Mobile | 1036.0 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 490 | PASS | **sims 2 castaway k500s** | EA_Mobile | 124.3 | 2D | Electronic Arts | 0.02 | ok (clean exit) |
| 491 | PASS | **need for speed underground 2** | EA_Mobile | 1141.9 | M3G | FEARLESS | 0.05 | ok (clean exit) |
| 492 | PASS | **Need For Speed Undercoverv3** | EA_Mobile | 486.2 | 2D | Electronic Arts/BiNP | 0.26 | ok (clean exit) |
| 493 | PASS | **The Sims Poolv3** | EA_Mobile | 407.1 | 2D | Electronic Arts | 0.06 | ok (clean exit) |
| 494 | PASS | **The Sims Pool 208x208** | EA_Mobile | 279.7 | 2D | Electronic Arts | 0.06 | ok (clean exit) |
| 495 | PASS | **The Sims Pool 128x160** | EA_Mobile | 126.9 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 496 | PASS | **Sims DJ 240 moto** | EA_Mobile | 306.0 | 2D | Electronic Arts | 0.05 | ok (clean exit) |
| 497 | PASS | **FIFA 2011 240 nok** | EA_Mobile | 856.3 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 498 | PASS | **Need For Speed Pro Street** | EA_Mobile | 324.5 | 2D | Electronic Arts | 1.22 | ok (clean exit) |
| 499 | PASS | **x320 need for speed shift s60** | EA_Mobile | 730.2 | M3G | mobigama.ru | 0.62 | ok (clean exit) |
| 500 | PASS | **EASPORTSFIFA11** | EA_Mobile | 444.8 | 2D | Electronic Arts Inc. | 0.04 | ok (clean exit) |
| 501 | PASS | **Gravity Defied: Classic** | Classics | 455.1 | 2D | TEGOS.RU | 0.02 | ok (clean exit) |
| 502 | PASS | **Castlevania: Order of Shadows** | SEGA_Konami_Capcom | 523.5 | 2D | Connect2Media | 0.29 | ok (clean exit) |
| 503 | PASS | **Real Football 2010** | Gameloft | 963.0 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 504 | PASS | **S.T.A.L.K.E.R.: Shadow of Chernobyl** | Classics | 1645.0 | M3G | Vendor | 8.08 | running (game loop active) |
| 505 | PASS | **Medal of Honor Mobile** | EA_Mobile | 579.4 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 506 | PASS | **Gangstar 2: Kings of L.A.** | Gameloft | 845.0 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 507 | PASS | **Art of War 2: Liberation of Peru** | HandyGames_HeroCraft | 848.7 | 2D | Gear Games | 0.03 | ok (clean exit) |
| 508 | PASS | **Worms 2008** | EA_Mobile | 532.4 | 2D | THQ/BiNPDA | 0.09 | ok (clean exit) |
| 509 | PASS | **Devil May Cry 3 Mobile** | SEGA_Konami_Capcom | 288.9 | 2D | CAPCOM /supplied by  | 0.03 | ok (clean exit) |
| 510 | PASS | **Command & Conquer 4: Tiberian Twilight** | EA_Mobile | 1145.7 | 2D | Electronic Arts | 0.04 | ok (clean exit) |
| 511 | PASS | **Tekken Mobile** | Classics | 845.6 | 2D | Namco | 0.06 | ok (clean exit) |
| 512 | PASS | **Dynamite Fishing** | HandyGames_HeroCraft | 437.9 | 2D | www.handy-games.com  | 0.28 | ok (clean exit) |
| 513 | PASS | **Worms 2010** | EA_Mobile | 991.7 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 514 | PASS | **Asphalt 4: Elite Racing 3D** | 3D_M3G | 266.4 | 2D | Gameloft SA | 0.03 | ok (clean exit) |
| 515 | PASS | **Guns 'n' Glory** | HandyGames_HeroCraft | 547.8 | 2D | WapBox.net | 0.1 | ok (clean exit) |
| 516 | PASS | **Farm Frenzy 3** | HandyGames_HeroCraft | 706.7 | 2D | HeroCraft | 0.23 | ok (clean exit) |
| 517 | PASS | **Ridge Racer Mobile** | Classics | 1309.7 | 2D | THQ | 0.22 | ok (clean exit) |
| 518 | PASS | **Postal Mobile** | HandyGames_HeroCraft | 328.4 | 2D | HeroCraft | 0.04 | ok (clean exit) |
| 519 | PASS | **Tron: Legacy Mobile** | Disney_Pixar | 637.8 | 2D | Disney | 0.04 | ok (clean exit) |
| 520 | PASS | **Iron Man 2 Mobile** | Gameloft | 1773.3 | 2D | Gameloft SA by Stox | 0.05 | ok (clean exit) |
| 521 | PASS | **Aces of the Luftwaffe 2** | HandyGames_HeroCraft | 524.2 | 2D | www.handy-games.com  | 0.15 | ok (clean exit) |
| 522 | PASS | **Doodle Jump Deluxe** | Classics | 435.7 | 2D | Mr. Goodliving Ltd | 0.16 | ok (clean exit) |
| 523 | PASS | **Galaga Mobile** | Classics | 311.8 | 2D | NBNE | 0.05 | ok (clean exit) |
| 524 | PASS | **Pro Evolution Soccer 2011** | SEGA_Konami_Capcom | 550.2 | 2D | Konami | 0.05 | ok (clean exit) |
| 525 | PASS | **Worms 2011** | EA_Mobile | 675.0 | 2D | Electronic Arts | 0.08 | ok (clean exit) |
| 526 | PASS | **Bumer: Ssorvannye Bashni** | Classics | 597.9 | M3G | www.m3gworks.com | 2.79 | ok (clean exit) |
| 527 | PASS | **Chuzzle** | PopCap | 239.3 | 2D | mob.ua | 5.01 | ok (clean exit) |
| 528 | PASS | **Streets of Rage** | SEGA_Konami_Capcom | 489.8 | 2D | 动力创想科技 | 0.04 | ok (clean exit) |
| 529 | PASS | **Bobby Carrot 5: Level Up** | Classics | 281.0 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 530 | PASS | **Kozaki Mobile** | Classics | 760.6 | 2D | Fenix-Soft | 0.17 | ok (clean exit) |
| 531 | PASS | **Nu, Pogodi! Pogodi, Volk!** | Classics | 529.4 | 2D | NET Lizard | 0.03 | ok (clean exit) |
| 532 | PASS | **Silent Hill Mobile 3** | SEGA_Konami_Capcom | 416.2 | 2D | Connect2Media | 0.04 | ok (clean exit) |
| 533 | PASS | **Zuma's Revenge** | PopCap | 1161.7 | 2D | Electronic Arts Inc. | 0.05 | ok (clean exit) |
| 534 | PASS | **Spider-Man: Toxic City** | Gameloft | 943.9 | 2D | Gameloft SA by konon | 0.04 | ok (clean exit) |
| 535 | PASS | **Battlefield: Bad Company 2** | EA_Mobile | 262.8 | 2D | rebenoj | 0.03 | ok (clean exit) |
| 536 | PASS | **Townsmen 2** | HandyGames_HeroCraft | 243.4 | 2D | www.handy-games.com  | 0.06 | ok (clean exit) |
| 537 | PASS | **Aladdin Mobile** | Disney_Pixar | 551.6 | 2D | Disney Mobile Studio | 0.05 | ok (clean exit) |
| 538 | PASS | **Revival 2** | HandyGames_HeroCraft | 227.8 | 2D | V&V, HeroCraft | 0.2 | ok (clean exit) |
| 539 | PASS | **Plants vs Zombies 240x320** | PopCap | 1012.7 | 2D | Game | 0.05 | ok (clean exit) |
| 540 | PASS | **The Lion King Mobile** | Disney_Pixar | 199.9 | 2D | Walt Disney company  | 0.03 | ok (clean exit) |
| 541 | PASS | **Contra 4** | SEGA_Konami_Capcom | 433.5 | 2D | KONAMI | 0.06 | ok (clean exit) |
| 542 | PASS | **Monopoly World** | EA_Mobile | 480.5 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 543 | PASS | **Shinobi** | SEGA_Konami_Capcom | 1005.7 | 2D | [url=http://ruwapa.n | 0.04 | ok (clean exit) |
| 544 | PASS | **Cyberlords: Arcology** | HandyGames_HeroCraft | 797.9 | 2D | [url=http://ruwapa.n | 8.03 | running (game loop active) |
| 545 | PASS | **SimCity Societies** | EA_Mobile | 610.1 | 2D | Electronic Arts, Inc | 0.07 | ok (clean exit) |
| 546 | PASS | **Frogger Beats** | SEGA_Konami_Capcom | 216.9 | 2D | Konami_by_BlackWaltz | 0.03 | ok (clean exit) |
| 547 | PASS | **Street Fighter II Mobile** | SEGA_Konami_Capcom | 767.2 | 2D | Gameloft SA | 0.22 | ok (clean exit) |
| 548 | PASS | **Allods Mobile** | Classics | 345.0 | 2D | Nival Interactive | 0.11 | ok (clean exit) |
| 549 | PASS | **SimCity Deluxe** | EA_Mobile | 1034.4 | 2D | Electronic Arts | 0.07 | ok (clean exit) |
| 550 | PASS | **Ghosts 'n Goblins Mobile** | SEGA_Konami_Capcom | 221.7 | 2D | CAPCOM | 0.05 | ok (clean exit) |
| 551 | PASS | **Golden Axe** | SEGA_Konami_Capcom | 764.8 | 2D | Necrophilia | 0.04 | ok (clean exit) |
| 552 | PASS | **Sonic Jump** | SEGA_Konami_Capcom | 334.5 | 2D | mob.ua | 0.04 | ok (clean exit) |
| 553 | PASS | **Pirates of the Caribbean: Dead Man's Chest** | Disney_Pixar | 185.0 | 2D | Living Mobile | 4.43 | ok (clean exit) |
| 554 | PASS | **Farm Frenzy 2** | HandyGames_HeroCraft | 419.1 | 2D | HeroCraft | 0.22 | ok (clean exit) |
| 555 | PASS | **Bookworm** | PopCap | 268.0 | 2D | PopCap_Retail_Etty | 5.44 | ok (clean exit) |
| 556 | PASS | **Sonic the Hedgehog 2** | SEGA_Konami_Capcom | 242.1 | 2D | iFone_OrangeSA | 0.04 | ok (clean exit) |
| 557 | PASS | **Silent Hill Mobile 2** | SEGA_Konami_Capcom | 492.1 | 2D | Konami | 0.27 | ok (clean exit) |
| 558 | PASS | **Metro 2033 Mobile** | Classics | 814.1 | 2D | pux.su | 0.07 | ok (clean exit) |
| 559 | PASS | **Phineas and Ferb** | Disney_Pixar | 355.8 | 2D | Disney | 0.12 | ok (clean exit) |
| 560 | PASS | **Townsmen 3** | HandyGames_HeroCraft | 568.4 | 2D | www.handy-games.com  | 0.07 | ok (clean exit) |
| 561 | PASS | **Pac-Man Championship Edition** | Classics | 697.9 | 2D | Namco serviak | 0.05 | ok (clean exit) |
| 562 | PASS | **Gangstar City** | Gameloft | 2040.0 | 2D | Gameloft SA | 0.12 | ok (clean exit) |
| 563 | PASS | **Bejeweled Twist** | PopCap | 496.8 | 2D | Electronic Arts Inc. | 5.11 | ok (clean exit) |
| 564 | PASS | **Real Football 2008** | Gameloft | 1056.2 | M3G | Gameloft/Tommy_M | 0.07 | ok (clean exit) |
| 565 | PASS | **Sonic Unleashed** | SEGA_Konami_Capcom | 989.6 | M3G | Gameloft SA | 0.05 | ok (clean exit) |
| 566 | PASS | **Angry Birds Mobile** | Classics | 598.9 | 2D | Touch-Games.RU | 0.33 | ok (clean exit) |
| 567 | PASS | **Trivial Pursuit Mobile** | EA_Mobile | 533.2 | 2D | Electronic Arts, Inc | 0.06 | ok (clean exit) |
| 568 | PASS | **Townsmen 6: Revolution** | HandyGames_HeroCraft | 474.3 | 2D | handy-games GmbH | 0.09 | ok (clean exit) |
| 569 | PASS | **RISK Mobile** | EA_Mobile | 990.2 | 2D | Electronic Arts | 0.06 | ok (clean exit) |
| 570 | PASS | **Green Farm 3** | Gameloft | 694.5 | 2D | Gameloft SA by Stox | 0.07 | ok (clean exit) |
| 571 | PASS | **Sonic Spinball** | SEGA_Konami_Capcom | 573.1 | 2D | Electronic Arts | 0.06 | ok (clean exit) |
| 572 | PASS | **Toy Story 3** | Disney_Pixar | 325.4 | 2D | Disney/Ismayeel | 0.03 | ok (clean exit) |
| 573 | PASS | **The Avengers Mobile** | Gameloft | 955.6 | 2D | Gameloft SA | 0.09 | ok (clean exit) |
| 574 | PASS | **Dead Space Mobile** | EA_Mobile | 1233.8 | M3G | m3gworks | 4.69 | ok (clean exit) |
| 575 | PASS | **Need for Speed Carbon 3D** | 3D_M3G | 397.2 | M3G | Electronic Arts | 0.03 | ok (clean exit) |
| 576 | PASS | **Townsmen 4** | HandyGames_HeroCraft | 239.0 | 2D | www.handy-games.com  | 0.08 | ok (clean exit) |
| 577 | PASS | **Modern Combat 2: Black Pegasus** | Gameloft | 888.5 | 2D | Gameloft SA | 0.26 | ok (clean exit) |
| 578 | PASS | **Cars 2** | Disney_Pixar | 464.2 | 2D | namobilu.com | 0.05 | ok (clean exit) |
| 579 | PASS | **Asphalt 3: Street Rules 3D** | 3D_M3G | 1013.6 | M3G | Gameloft SA/BiNPDA | 0.04 | ok (clean exit) |
| 580 | PASS | **Monopoly Here & Now** | EA_Mobile | 547.3 | 2D | Electronic Arts Inc. | 0.03 | ok (clean exit) |
| 581 | PASS | **Fruit Ninja Mobile** | Classics | 276.0 | 2D | Vendor | 0.03 | ok (clean exit) |
| 582 | PASS | **S.T.A.L.K.E.R.: Clear Sky** | Classics | 805.5 | M3G | M3GWORKS Team | 4.14 | ok (clean exit) |
| 583 | PASS | **Need for Speed Most Wanted 3D** | 3D_M3G | 1496.6 | M3G | Electronic Arts | 0.45 | ok (clean exit) |
| 584 | PASS | **Townsmen 1** | HandyGames_HeroCraft | 633.9 | 2D | www.handy-games.com  | 0.08 | ok (clean exit) |
| 585 | PASS | **Townsmen 5** | HandyGames_HeroCraft | 234.1 | 2D | Handy-Games | 0.06 | ok (clean exit) |
| 586 | PASS | **Virtua Tennis Mobile** | SEGA_Konami_Capcom | 299.6 | Mascot3D | DDJ | 0.05 | ok (clean exit) |
| 587 | PASS | **Captain America: Sentinel of Liberty** | Gameloft | 781.3 | 2D | Disney by Stox | 0.03 | ok (clean exit) |
| 588 | PASS | **Castlevania: Aria of Sorrow** | SEGA_Konami_Capcom | 404.6 | 2D | Konami | 0.14 | ok (clean exit) |
| 589 | PASS | **Bobby Carrot 4: Flower Company** | Classics | 303.5 | 2D | FDGSoft | 0.04 | ok (clean exit) |
| 590 | PASS | **Pro Evolution Soccer 2012** | SEGA_Konami_Capcom | 803.4 | 2D | Konami/Tommy_M | 0.08 | ok (clean exit) |
| 591 | PASS | **Need for Speed The Run** | EA_Mobile | 495.5 | 2D | Electronic Arts Inc. | 0.62 | ok (clean exit) |
| 592 | PASS | **Super Monkey Ball** | SEGA_Konami_Capcom | 323.0 | 2D | Glu Mobile | 0.04 | ok (clean exit) |
| 593 | PASS | **Soulcalibur Mobile** | Classics | 936.1 | 2D | NAMCO BANDAI Games A | 0.04 | ok (clean exit) |
| 594 | PASS | **Men in Black 3** | Gameloft | 1081.9 | 2D | Gameloft SA | 0.05 | ok (clean exit) |
| 595 | PASS | **Bobby Carrot 3: Evolution** | Classics | 228.1 | 2D | FDGSoft | 0.03 | ok (clean exit) |
| 596 | PASS | **Little Big City** | Gameloft | 324.8 | 2D | Gameloft SA | 0.72 | ok (clean exit) |
| 597 | PASS | **Wonder Zoo** | Gameloft | 617.2 | 2D | Gameloft SA by Stox | 0.42 | ok (clean exit) |
| 598 | PASS | **Scrabble Mobile** | EA_Mobile | 553.6 | 2D | Electronic Arts, Inc | 0.06 | ok (clean exit) |
| 599 | PASS | **The Amazing Spider-Man** | Gameloft | 1102.7 | 2D | Gameloft SA | 0.07 | ok (clean exit) |
| 600 | PASS | **Mega Man III** | SEGA_Konami_Capcom | 210.3 | 2D | Capcom Interactive,  | 0.07 | ok (clean exit) |

## 5. Выводы и архитектурные наблюдения

- Объединённый набор Топ-600 полностью включает все игры из предыдущих Топ-100 и Топ-500, а также расширенный каталог культовых тайтлов (SEGA Sonic, Konami Castlevania/Silent Hill, Capcom DMC/MegaMan, FDG Bobby Carrot, HandyGames Townsmen, HeroCraft, Disney и др.).
- Исправлена критическая проблема декодирования UTF-8/кириллицы в именах JAR-архивов и URL (`percent_decode`), предотвращена бесконечная рекурсия `looks_like_classpath_resource`.
- Для Windows увеличен размер стека основного рабочего потока эмулятора до 16 МБ, устраняя переполнение стека при глубоких вложенных вызовах JVM.
- Высокая параллельность (8 потоков на многоядерном CPU) позволила протестировать 600 коммерческих игр менее чем за 2 минуты.