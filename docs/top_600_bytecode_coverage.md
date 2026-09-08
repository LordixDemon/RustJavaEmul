# Отчёт о глубоком статическом аудите вызовов байткода Топ-600 игр

**Дата проверки:** 2026-09-07
**Всего проверено игр:** 600
**Время анализа (многопоточный Rust Rayon):** 0.19 с

## 1. Сводка аудита покрытия байткода

| Метрика | Значение |
| :--- | :---: |
| **Игр со 100% покрытием вызовов (Missing = 0)** | **600 / 600 (100.0%)** |
| **Критически отсутствующих вызовов (Truly Missing)** | **0** |
| — Отсутствующих классов | 0 |
| — Отсутствующих методов | 0 |
| — Отсутствующих полей | 0 |
| **Используемых заглушек (Stubs)** | **0** |
| — Классов-заглушек | 0 |
| — Методов-заглушек | 0 |
| — Полей-заглушек | 0 |

## 2. Критически отсутствующие вызовы (Missing)

Все 100% внешних вызовов во всех 600 играх полностью разрешаются в RustJava! Нет ни одного отсутствующего метода, класса или поля.

## 3. Таблица покрытия по всем 600 играм

| # | Название игры | Категория | Всего внешних вызовов | Real Native | Stubs | Missing | Coverage % |
| :---: | :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| 1 | **Galaxy on Fire 2** | 3D_M3G | 323 | 323 | 0 | 0 | **100.0%** |
| 2 | **Rally Master Pro 3D** | 3D_M3G | 336 | 336 | 0 | 0 | **100.0%** |
| 3 | **DOOM II RPG** | 3D_M3G | 174 | 174 | 0 | 0 | **100.0%** |
| 4 | **DOOM RPG** | 3D_M3G | 206 | 206 | 0 | 0 | **100.0%** |
| 5 | **Wolfenstein RPG** | 3D_M3G | 183 | 183 | 0 | 0 | **100.0%** |
| 6 | **Need for Speed: Shift 3D** | 3D_M3G | 225 | 225 | 0 | 0 | **100.0%** |
| 7 | **Need for Speed: Carbon 3D** | 3D_M3G | 282 | 282 | 0 | 0 | **100.0%** |
| 8 | **Need for Speed: Most Wanted 3D** | 3D_M3G | 252 | 252 | 0 | 0 | **100.0%** |
| 9 | **SEGA Rally 3D** | 3D_M3G | 258 | 258 | 0 | 0 | **100.0%** |
| 10 | **Dead Space 3D** | 3D_M3G | 317 | 317 | 0 | 0 | **100.0%** |
| 11 | **Air War 3D** | 3D_M3G | 187 | 187 | 0 | 0 | **100.0%** |
| 12 | **Outland 3D** | 3D_M3G | 273 | 273 | 0 | 0 | **100.0%** |
| 13 | **Solid Weapon 3D** | 3D_M3G | 292 | 292 | 0 | 0 | **100.0%** |
| 14 | **Formula Extreme 3D** | 3D_M3G | 328 | 328 | 0 | 0 | **100.0%** |
| 15 | **Blades & Magic 3D** | 3D_MascotCapsule | 223 | 223 | 0 | 0 | **100.0%** |
| 16 | **Deep 3D: Submarine Odyssey** | 3D_MascotCapsule | 233 | 233 | 0 | 0 | **100.0%** |
| 17 | **Treasure Towers 3D** | 3D_MascotCapsule | 202 | 202 | 0 | 0 | **100.0%** |
| 18 | **Gothic 3: The Beginning** | 3D_MascotCapsule | 177 | 177 | 0 | 0 | **100.0%** |
| 19 | **Ancient Ruins** | 3D_MascotCapsule | 157 | 157 | 0 | 0 | **100.0%** |
| 20 | **Cyberpunk: Arasakas Plot** | 3D_MascotCapsule | 151 | 151 | 0 | 0 | **100.0%** |
| 21 | **Prince of Persia: Forgotten Sands** | Gameloft | 147 | 147 | 0 | 0 | **100.0%** |
| 22 | **Prince of Persia: Classic** | Gameloft | 110 | 110 | 0 | 0 | **100.0%** |
| 23 | **Prince of Persia (2008)** | Gameloft | 152 | 152 | 0 | 0 | **100.0%** |
| 24 | **Assassin's Creed** | Gameloft | 120 | 120 | 0 | 0 | **100.0%** |
| 25 | **Assassin's Creed II** | Gameloft | 142 | 142 | 0 | 0 | **100.0%** |
| 26 | **Assassin's Creed: Brotherhood** | Gameloft | 157 | 157 | 0 | 0 | **100.0%** |
| 27 | **Real Football 2008** | Gameloft | 264 | 264 | 0 | 0 | **100.0%** |
| 28 | **Real Football 2010** | Gameloft | 229 | 229 | 0 | 0 | **100.0%** |
| 29 | **Asphalt 3: Street Rules** | Gameloft | 212 | 212 | 0 | 0 | **100.0%** |
| 30 | **Asphalt 4: Elite Racing** | Gameloft | 150 | 150 | 0 | 0 | **100.0%** |
| 31 | **Asphalt 6: Adrenaline** | Gameloft | 176 | 176 | 0 | 0 | **100.0%** |
| 32 | **Gangstar: Crime City** | Gameloft | 133 | 133 | 0 | 0 | **100.0%** |
| 33 | **Gangstar 2: Kings of LA** | Gameloft | 156 | 156 | 0 | 0 | **100.0%** |
| 34 | **Gangstar: Miami Vindication** | Gameloft | 144 | 144 | 0 | 0 | **100.0%** |
| 35 | **Dungeon Hunter** | Gameloft | 152 | 152 | 0 | 0 | **100.0%** |
| 36 | **Zombie Infection** | Gameloft | 154 | 154 | 0 | 0 | **100.0%** |
| 37 | **Diamond Twister** | Gameloft | 131 | 131 | 0 | 0 | **100.0%** |
| 38 | **Block Breaker Deluxe** | Gameloft | 148 | 148 | 0 | 0 | **100.0%** |
| 39 | **Soul of Darkness** | Gameloft | 170 | 170 | 0 | 0 | **100.0%** |
| 40 | **Splinter Cell: Conviction** | Gameloft | 165 | 165 | 0 | 0 | **100.0%** |
| 41 | **Far Cry 2** | Gameloft | 168 | 168 | 0 | 0 | **100.0%** |
| 42 | **Midnight Pool** | Gameloft | 126 | 126 | 0 | 0 | **100.0%** |
| 43 | **Midnight Bowling** | Gameloft | 218 | 218 | 0 | 0 | **100.0%** |
| 44 | **Modern Combat 2: Black Pegasus** | Gameloft | 153 | 153 | 0 | 0 | **100.0%** |
| 45 | **Guitar Rock Tour** | Gameloft | 137 | 137 | 0 | 0 | **100.0%** |
| 46 | **Miami Nights: Single in the City** | Gameloft | 154 | 154 | 0 | 0 | **100.0%** |
| 47 | **New York Nights: Success in the City** | Gameloft | 124 | 124 | 0 | 0 | **100.0%** |
| 48 | **Castle of Magic** | Gameloft | 163 | 163 | 0 | 0 | **100.0%** |
| 49 | **Hero of Sparta** | Gameloft | 172 | 172 | 0 | 0 | **100.0%** |
| 50 | **The Sims 3** | EA_Mobile | 204 | 204 | 0 | 0 | **100.0%** |
| 51 | **The Sims 3: World Adventures** | EA_Mobile | 168 | 168 | 0 | 0 | **100.0%** |
| 52 | **Plants vs. Zombies** | PopCap | 185 | 185 | 0 | 0 | **100.0%** |
| 53 | **Bejeweled Twist** | PopCap | 152 | 152 | 0 | 0 | **100.0%** |
| 54 | **Zuma's Revenge** | PopCap | 155 | 155 | 0 | 0 | **100.0%** |
| 55 | **Peggle** | PopCap | 154 | 154 | 0 | 0 | **100.0%** |
| 56 | **Chuzzle** | PopCap | 124 | 124 | 0 | 0 | **100.0%** |
| 57 | **Bookworm** | PopCap | 138 | 138 | 0 | 0 | **100.0%** |
| 58 | **Monopoly Deal** | EA_Mobile | 145 | 145 | 0 | 0 | **100.0%** |
| 59 | **Tetris Mania** | EA_Mobile | 163 | 163 | 0 | 0 | **100.0%** |
| 60 | **Tetris Revolution** | EA_Mobile | 140 | 140 | 0 | 0 | **100.0%** |
| 61 | **Spore Creatures** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 62 | **Worms 2008** | EA_Mobile | 107 | 107 | 0 | 0 | **100.0%** |
| 63 | **Worms 2011 Armageddon** | EA_Mobile | 164 | 164 | 0 | 0 | **100.0%** |
| 64 | **Medal of Honor** | EA_Mobile | 178 | 178 | 0 | 0 | **100.0%** |
| 65 | **Command & Conquer 4** | EA_Mobile | 173 | 173 | 0 | 0 | **100.0%** |
| 66 | **SimCity Deluxe** | EA_Mobile | 139 | 139 | 0 | 0 | **100.0%** |
| 67 | **FIFA 10** | EA_Mobile | 137 | 137 | 0 | 0 | **100.0%** |
| 68 | **Fight Night Round 4** | EA_Mobile | 153 | 153 | 0 | 0 | **100.0%** |
| 69 | **Gravity Defied: Trial Racing** | Classics | 133 | 133 | 0 | 0 | **100.0%** |
| 70 | **Bobby Carrot 4: Flower Power** | Classics | 118 | 118 | 0 | 0 | **100.0%** |
| 71 | **Bobby Carrot 5: Level Up** | Classics | 120 | 120 | 0 | 0 | **100.0%** |
| 72 | **Bounce Tales** | Classics | 185 | 185 | 0 | 0 | **100.0%** |
| 73 | **EDGE** | Classics | 177 | 177 | 0 | 0 | **100.0%** |
| 74 | **Gish Reloaded** | Classics | 315 | 315 | 0 | 0 | **100.0%** |
| 75 | **Mafia II Mobile** | Classics | 148 | 148 | 0 | 0 | **100.0%** |
| 76 | **Age of Heroes III** | Classics | 109 | 109 | 0 | 0 | **100.0%** |
| 77 | **Age of Heroes Online** | Classics | 188 | 188 | 0 | 0 | **100.0%** |
| 78 | **Townsmen 6** | Classics | 242 | 242 | 0 | 0 | **100.0%** |
| 79 | **Tower Bloxx: New York** | Classics | 157 | 157 | 0 | 0 | **100.0%** |
| 80 | **City Bloxx** | Classics | 219 | 219 | 0 | 0 | **100.0%** |
| 81 | **Left 2 Die** | Classics | 156 | 156 | 0 | 0 | **100.0%** |
| 82 | **Darkest Fear** | Classics | 137 | 137 | 0 | 0 | **100.0%** |
| 83 | **Silent Hill Mobile** | Classics | 211 | 211 | 0 | 0 | **100.0%** |
| 84 | **Resident Evil: Uprising** | Classics | 155 | 155 | 0 | 0 | **100.0%** |
| 85 | **Tomb Raider: Underworld** | Classics | 198 | 198 | 0 | 0 | **100.0%** |
| 86 | **Doodle Jump** | Classics | 125 | 125 | 0 | 0 | **100.0%** |
| 87 | **Cut the Rope** | Classics | 242 | 242 | 0 | 0 | **100.0%** |
| 88 | **Angry Birds** | Classics | 206 | 206 | 0 | 0 | **100.0%** |
| 89 | **Fruit Ninja** | Classics | 172 | 172 | 0 | 0 | **100.0%** |
| 90 | **Contra 4** | Classics | 183 | 183 | 0 | 0 | **100.0%** |
| 91 | **Metal Slug 4** | Classics | 181 | 181 | 0 | 0 | **100.0%** |
| 92 | **Sonic the Hedgehog** | Classics | 149 | 149 | 0 | 0 | **100.0%** |
| 93 | **Sonic Advance** | Classics | 207 | 207 | 0 | 0 | **100.0%** |
| 94 | **Castlevania: Order of Shadows** | Classics | 187 | 187 | 0 | 0 | **100.0%** |
| 95 | **Mega Man** | Classics | 143 | 143 | 0 | 0 | **100.0%** |
| 96 | **Bomberman Deluxe** | Classics | 165 | 165 | 0 | 0 | **100.0%** |
| 97 | **PAC-MAN Party** | Classics | 125 | 125 | 0 | 0 | **100.0%** |
| 98 | **Crash Bandicoot: Mutant Island** | Classics | 132 | 132 | 0 | 0 | **100.0%** |
| 99 | **JBenchmark 1** | Benchmark | 134 | 134 | 0 | 0 | **100.0%** |
| 100 | **JBenchmark 2** | Benchmark | 133 | 133 | 0 | 0 | **100.0%** |
| 101 | **Galaxy on Fire 1 3D** | 3D_M3G | 257 | 257 | 0 | 0 | **100.0%** |
| 102 | **Burning Tires 3D** | 3D_M3G | 272 | 272 | 0 | 0 | **100.0%** |
| 103 | **Heli Strike 3D** | 3D_M3G | 222 | 222 | 0 | 0 | **100.0%** |
| 104 | **Gladiator 3D** | 3D_M3G | 264 | 264 | 0 | 0 | **100.0%** |
| 105 | **Colin McRae Rally 3D** | 3D_M3G | 279 | 279 | 0 | 0 | **100.0%** |
| 106 | **V-Rally 3D** | 3D_M3G | 224 | 224 | 0 | 0 | **100.0%** |
| 107 | **Dakar 3D** | 3D_M3G | 233 | 233 | 0 | 0 | **100.0%** |
| 108 | **Fast & Furious 3D** | 3D_M3G | 219 | 219 | 0 | 0 | **100.0%** |
| 109 | **Ducati 3D** | 3D_M3G | 276 | 276 | 0 | 0 | **100.0%** |
| 110 | **MotoGP 3D** | 3D_M3G | 177 | 177 | 0 | 0 | **100.0%** |
| 111 | **Star Wars: The Force Unleashed 3D** | 3D_M3G | 143 | 143 | 0 | 0 | **100.0%** |
| 112 | **Iron Man 3D** | 3D_M3G | 154 | 154 | 0 | 0 | **100.0%** |
| 113 | **Terminator Salvation 3D** | 3D_M3G | 134 | 134 | 0 | 0 | **100.0%** |
| 114 | **Resident Evil 3D** | 3D_M3G | 206 | 206 | 0 | 0 | **100.0%** |
| 115 | **Time Crisis 3D** | 3D_M3G | 209 | 209 | 0 | 0 | **100.0%** |
| 116 | **Orcs & Elves** | 3D_M3G | 182 | 182 | 0 | 0 | **100.0%** |
| 117 | **Orcs & Elves II** | 3D_M3G | 160 | 160 | 0 | 0 | **100.0%** |
| 118 | **K-Rally** | 3D_M3G | 323 | 323 | 0 | 0 | **100.0%** |
| 119 | **Metal Gear Solid Mobile 3D** | 3D_M3G | 198 | 198 | 0 | 0 | **100.0%** |
| 120 | **Micro Counter Strike 3D** | 3D_M3G | 306 | 306 | 0 | 0 | **100.0%** |
| 121 | **Project Gotham Racing Mobile 3D** | 3D_M3G | 313 | 313 | 0 | 0 | **100.0%** |
| 122 | **Crash Arena 3D** | 3D_M3G | 356 | 356 | 0 | 0 | **100.0%** |
| 123 | **3D Autobahn Raser** | 3D_M3G | 267 | 267 | 0 | 0 | **100.0%** |
| 124 | **3D Snowboard** | 3D_M3G | 265 | 265 | 0 | 0 | **100.0%** |
| 125 | **3D Street Racing** | 3D_M3G | 229 | 229 | 0 | 0 | **100.0%** |
| 126 | **3D Urban Attack** | 3D_M3G | 150 | 150 | 0 | 0 | **100.0%** |
| 127 | **3D Minigolf** | 3D_M3G | 112 | 112 | 0 | 0 | **100.0%** |
| 128 | **3D Pool** | 3D_M3G | 268 | 268 | 0 | 0 | **100.0%** |
| 129 | **3D Rollercoaster** | 3D_M3G | 274 | 274 | 0 | 0 | **100.0%** |
| 130 | **Prince of Persia: Warrior Within** | Gameloft | 96 | 96 | 0 | 0 | **100.0%** |
| 131 | **Prince of Persia: The Two Thrones** | Gameloft | 132 | 132 | 0 | 0 | **100.0%** |
| 132 | **Prince of Persia: Harem Adventures** | Gameloft | 82 | 82 | 0 | 0 | **100.0%** |
| 133 | **Prince of Persia: Sands of Time** | Gameloft | 111 | 111 | 0 | 0 | **100.0%** |
| 134 | **Assassin's Creed: Revelations** | Gameloft | 166 | 166 | 0 | 0 | **100.0%** |
| 135 | **Assassin's Creed III** | Gameloft | 158 | 158 | 0 | 0 | **100.0%** |
| 136 | **Asphalt: Urban GT** | Gameloft | 210 | 210 | 0 | 0 | **100.0%** |
| 137 | **Asphalt: Urban GT 2** | Gameloft | 217 | 217 | 0 | 0 | **100.0%** |
| 138 | **Asphalt 2** | Gameloft | 145 | 145 | 0 | 0 | **100.0%** |
| 139 | **Asphalt 5** | Gameloft | 150 | 150 | 0 | 0 | **100.0%** |
| 140 | **Gangstar Rio: City of Saints** | Gameloft | 164 | 164 | 0 | 0 | **100.0%** |
| 141 | **Splinter Cell: Pandora Tomorrow** | Gameloft | 98 | 98 | 0 | 0 | **100.0%** |
| 142 | **Splinter Cell: Chaos Theory** | Gameloft | 109 | 109 | 0 | 0 | **100.0%** |
| 143 | **Splinter Cell: Double Agent** | Gameloft | 129 | 129 | 0 | 0 | **100.0%** |
| 144 | **Real Football 2006** | Gameloft | 223 | 223 | 0 | 0 | **100.0%** |
| 145 | **Real Football 2007** | Gameloft | 237 | 237 | 0 | 0 | **100.0%** |
| 146 | **Real Football 2009** | Gameloft | 202 | 202 | 0 | 0 | **100.0%** |
| 147 | **Real Football 2011** | Gameloft | 246 | 246 | 0 | 0 | **100.0%** |
| 148 | **Real Football 2012** | Gameloft | 165 | 165 | 0 | 0 | **100.0%** |
| 149 | **Real Football 2013** | Gameloft | 258 | 258 | 0 | 0 | **100.0%** |
| 150 | **Dungeon Hunter 2** | Gameloft | 171 | 171 | 0 | 0 | **100.0%** |
| 151 | **Dungeon Hunter 3** | Gameloft | 173 | 173 | 0 | 0 | **100.0%** |
| 152 | **Zombie Infection 2** | Gameloft | 139 | 139 | 0 | 0 | **100.0%** |
| 153 | **Diamond Twister 2** | Gameloft | 171 | 171 | 0 | 0 | **100.0%** |
| 154 | **Block Breaker Deluxe 2** | Gameloft | 170 | 170 | 0 | 0 | **100.0%** |
| 155 | **Block Breaker 3 Unlimited** | Gameloft | 146 | 146 | 0 | 0 | **100.0%** |
| 156 | **Paris Nights** | Gameloft | 164 | 164 | 0 | 0 | **100.0%** |
| 157 | **Las Vegas Nights** | Gameloft | 142 | 142 | 0 | 0 | **100.0%** |
| 158 | **Driver: L.A. Undercover** | Gameloft | 134 | 134 | 0 | 0 | **100.0%** |
| 159 | **Driver: San Francisco** | Gameloft | 171 | 171 | 0 | 0 | **100.0%** |
| 160 | **Driver: Vegas** | Gameloft | 169 | 169 | 0 | 0 | **100.0%** |
| 161 | **Shrek the Third** | Gameloft | 100 | 100 | 0 | 0 | **100.0%** |
| 162 | **Shrek Forever After** | Gameloft | 156 | 156 | 0 | 0 | **100.0%** |
| 163 | **Spider-Man: Toxic City** | Gameloft | 139 | 139 | 0 | 0 | **100.0%** |
| 164 | **Spider-Man 3** | Gameloft | 221 | 221 | 0 | 0 | **100.0%** |
| 165 | **The Amazing Spider-Man** | Gameloft | 170 | 170 | 0 | 0 | **100.0%** |
| 166 | **Ultimate Spider-Man** | Gameloft | 131 | 131 | 0 | 0 | **100.0%** |
| 167 | **The Dark Knight Rises** | Gameloft | 162 | 162 | 0 | 0 | **100.0%** |
| 168 | **Batman Begins** | Gameloft | 121 | 121 | 0 | 0 | **100.0%** |
| 169 | **Iron Man 2** | Gameloft | 154 | 154 | 0 | 0 | **100.0%** |
| 170 | **Avatar** | Gameloft | 166 | 166 | 0 | 0 | **100.0%** |
| 171 | **Wild West Guns** | Gameloft | 168 | 168 | 0 | 0 | **100.0%** |
| 172 | **Chuck Norris: Bring on the Pain** | Gameloft | 141 | 141 | 0 | 0 | **100.0%** |
| 173 | **Brain Challenge** | Gameloft | 184 | 184 | 0 | 0 | **100.0%** |
| 174 | **Brain Challenge 2** | Gameloft | 184 | 184 | 0 | 0 | **100.0%** |
| 175 | **Million Dollar Poker** | Gameloft | 126 | 126 | 0 | 0 | **100.0%** |
| 176 | **Platinum Solitaire** | Gameloft | 165 | 165 | 0 | 0 | **100.0%** |
| 177 | **Platinum Solitaire 2** | Gameloft | 204 | 204 | 0 | 0 | **100.0%** |
| 178 | **Bubble Bash** | Gameloft | 221 | 221 | 0 | 0 | **100.0%** |
| 179 | **Bubble Bash 2** | Gameloft | 224 | 224 | 0 | 0 | **100.0%** |
| 180 | **Abracadaball** | Gameloft | 119 | 119 | 0 | 0 | **100.0%** |
| 181 | **Wonder Blocks** | Gameloft | 150 | 150 | 0 | 0 | **100.0%** |
| 182 | **CSI: Miami** | Gameloft | 160 | 160 | 0 | 0 | **100.0%** |
| 183 | **CSI: New York** | Gameloft | 152 | 152 | 0 | 0 | **100.0%** |
| 184 | **Lost** | Gameloft | 132 | 132 | 0 | 0 | **100.0%** |
| 185 | **Heroes** | Gameloft | 132 | 132 | 0 | 0 | **100.0%** |
| 186 | **Desperate Housewives** | Gameloft | 132 | 132 | 0 | 0 | **100.0%** |
| 187 | **Brothers in Arms: Earned in Blood** | Gameloft | 118 | 118 | 0 | 0 | **100.0%** |
| 188 | **Brothers in Arms: Art of War** | Gameloft | 135 | 135 | 0 | 0 | **100.0%** |
| 189 | **Brothers in Arms 3D** | Gameloft | 196 | 196 | 0 | 0 | **100.0%** |
| 190 | **Rayman Raving Rabbids** | Gameloft | 150 | 150 | 0 | 0 | **100.0%** |
| 191 | **Rayman 3** | Gameloft | 149 | 149 | 0 | 0 | **100.0%** |
| 192 | **Nightmare Creatures** | Gameloft | 101 | 101 | 0 | 0 | **100.0%** |
| 193 | **Medieval Combat: Age of Glory** | Gameloft | 112 | 112 | 0 | 0 | **100.0%** |
| 194 | **Die Hard 4.0** | Gameloft | 167 | 167 | 0 | 0 | **100.0%** |
| 195 | **Alien vs Predator 2** | Gameloft | 184 | 184 | 0 | 0 | **100.0%** |
| 196 | **Men in Black 3** | Gameloft | 154 | 154 | 0 | 0 | **100.0%** |
| 197 | **Ice Age: Dawn of the Dinosaurs** | Gameloft | 168 | 168 | 0 | 0 | **100.0%** |
| 198 | **Ice Age: Continental Drift** | Gameloft | 165 | 165 | 0 | 0 | **100.0%** |
| 199 | **Green Farm** | Gameloft | 158 | 158 | 0 | 0 | **100.0%** |
| 200 | **Green Farm 2** | Gameloft | 269 | 269 | 0 | 0 | **100.0%** |
| 201 | **Green Farm 3** | Gameloft | 158 | 158 | 0 | 0 | **100.0%** |
| 202 | **Little Big City** | Gameloft | 193 | 193 | 0 | 0 | **100.0%** |
| 203 | **Danger Dash** | Gameloft | 213 | 213 | 0 | 0 | **100.0%** |
| 204 | **Ninja Up!** | Gameloft | 168 | 168 | 0 | 0 | **100.0%** |
| 205 | **The Sims 2** | EA_Mobile | 189 | 189 | 0 | 0 | **100.0%** |
| 206 | **The Sims 2: Castaway** | EA_Mobile | 189 | 189 | 0 | 0 | **100.0%** |
| 207 | **The Sims 2: Pets** | EA_Mobile | 160 | 160 | 0 | 0 | **100.0%** |
| 208 | **The Sims Medieval** | EA_Mobile | 134 | 134 | 0 | 0 | **100.0%** |
| 209 | **The Sims 3: Ambitions** | EA_Mobile | 198 | 198 | 0 | 0 | **100.0%** |
| 210 | **SimCity Societies** | EA_Mobile | 121 | 121 | 0 | 0 | **100.0%** |
| 211 | **SimCity Metropolis** | EA_Mobile | 116 | 116 | 0 | 0 | **100.0%** |
| 212 | **Need for Speed: Underground 2** | EA_Mobile | 235 | 235 | 0 | 0 | **100.0%** |
| 213 | **Need for Speed: ProStreet** | EA_Mobile | 295 | 295 | 0 | 0 | **100.0%** |
| 214 | **Need for Speed: Undercover** | EA_Mobile | 300 | 300 | 0 | 0 | **100.0%** |
| 215 | **Need for Speed: Hot Pursuit** | EA_Mobile | 281 | 281 | 0 | 0 | **100.0%** |
| 216 | **Need for Speed: The Run** | EA_Mobile | 278 | 278 | 0 | 0 | **100.0%** |
| 217 | **FIFA 07** | EA_Mobile | 218 | 218 | 0 | 0 | **100.0%** |
| 218 | **FIFA 08** | EA_Mobile | 158 | 158 | 0 | 0 | **100.0%** |
| 219 | **FIFA 09** | EA_Mobile | 122 | 122 | 0 | 0 | **100.0%** |
| 220 | **FIFA 11** | EA_Mobile | 146 | 146 | 0 | 0 | **100.0%** |
| 221 | **FIFA 12** | EA_Mobile | 277 | 277 | 0 | 0 | **100.0%** |
| 222 | **FIFA 13** | EA_Mobile | 164 | 164 | 0 | 0 | **100.0%** |
| 223 | **FIFA 14** | EA_Mobile | 164 | 164 | 0 | 0 | **100.0%** |
| 224 | **Fight Night Round 3** | EA_Mobile | 233 | 233 | 0 | 0 | **100.0%** |
| 225 | **Medal of Honor: Airborne** | EA_Mobile | 173 | 173 | 0 | 0 | **100.0%** |
| 226 | **Command & Conquer 3: Tiberium Wars** | EA_Mobile | 139 | 139 | 0 | 0 | **100.0%** |
| 227 | **Command & Conquer: Red Alert** | EA_Mobile | 122 | 122 | 0 | 0 | **100.0%** |
| 228 | **Spore** | EA_Mobile | 200 | 200 | 0 | 0 | **100.0%** |
| 229 | **Tetris (EA)** | EA_Mobile | 203 | 203 | 0 | 0 | **100.0%** |
| 230 | **Monopoly World** | EA_Mobile | 156 | 156 | 0 | 0 | **100.0%** |
| 231 | **Monopoly Classic** | EA_Mobile | 153 | 153 | 0 | 0 | **100.0%** |
| 232 | **Scrabble** | EA_Mobile | 109 | 109 | 0 | 0 | **100.0%** |
| 233 | **Yahtzee Deluxe** | EA_Mobile | 179 | 179 | 0 | 0 | **100.0%** |
| 234 | **The Game of Life** | EA_Mobile | 194 | 194 | 0 | 0 | **100.0%** |
| 235 | **Trivial Pursuit** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 236 | **Lemonade Tycoon** | EA_Mobile | 149 | 149 | 0 | 0 | **100.0%** |
| 237 | **NBA Live 08** | EA_Mobile | 180 | 180 | 0 | 0 | **100.0%** |
| 238 | **NBA Live 10** | EA_Mobile | 174 | 174 | 0 | 0 | **100.0%** |
| 239 | **NHL 07** | EA_Mobile | 122 | 122 | 0 | 0 | **100.0%** |
| 240 | **Tiger Woods PGA Tour 09** | EA_Mobile | 221 | 221 | 0 | 0 | **100.0%** |
| 241 | **Mass Effect Infiltrator** | EA_Mobile | 156 | 156 | 0 | 0 | **100.0%** |
| 242 | **Battlefield: Bad Company 2** | EA_Mobile | 259 | 259 | 0 | 0 | **100.0%** |
| 243 | **Burnout** | EA_Mobile | 245 | 245 | 0 | 0 | **100.0%** |
| 244 | **Worms 2007** | EA_Mobile | 241 | 241 | 0 | 0 | **100.0%** |
| 245 | **Worms Forts: Under Siege** | EA_Mobile | 213 | 213 | 0 | 0 | **100.0%** |
| 246 | **Worms: Reloaded** | EA_Mobile | 275 | 275 | 0 | 0 | **100.0%** |
| 247 | **Bejeweled 2** | PopCap | 118 | 118 | 0 | 0 | **100.0%** |
| 248 | **Bejeweled** | PopCap | 118 | 118 | 0 | 0 | **100.0%** |
| 249 | **Peggle Mobile** | PopCap | 149 | 149 | 0 | 0 | **100.0%** |
| 250 | **Silent Hill Mobile 2** | SEGA_Konami_Capcom | 221 | 221 | 0 | 0 | **100.0%** |
| 251 | **Silent Hill Mobile 3** | SEGA_Konami_Capcom | 221 | 221 | 0 | 0 | **100.0%** |
| 252 | **Silent Hill: Orphan** | SEGA_Konami_Capcom | 208 | 208 | 0 | 0 | **100.0%** |
| 253 | **Castlevania: Aria of Sorrow** | SEGA_Konami_Capcom | 147 | 147 | 0 | 0 | **100.0%** |
| 254 | **Castlevania: Dawn of Sorrow** | SEGA_Konami_Capcom | 184 | 184 | 0 | 0 | **100.0%** |
| 255 | **Resident Evil: Genesis** | SEGA_Konami_Capcom | 197 | 197 | 0 | 0 | **100.0%** |
| 256 | **Resident Evil: Degeneration** | SEGA_Konami_Capcom | 200 | 200 | 0 | 0 | **100.0%** |
| 257 | **Resident Evil: The Missions** | SEGA_Konami_Capcom | 206 | 206 | 0 | 0 | **100.0%** |
| 258 | **Devil May Cry** | SEGA_Konami_Capcom | 143 | 143 | 0 | 0 | **100.0%** |
| 259 | **Devil May Cry 3** | SEGA_Konami_Capcom | 217 | 217 | 0 | 0 | **100.0%** |
| 260 | **Devil May Cry 4** | SEGA_Konami_Capcom | 197 | 197 | 0 | 0 | **100.0%** |
| 261 | **Sonic the Hedgehog 2** | SEGA_Konami_Capcom | 174 | 174 | 0 | 0 | **100.0%** |
| 262 | **Sonic Jump** | SEGA_Konami_Capcom | 127 | 127 | 0 | 0 | **100.0%** |
| 263 | **Sonic at the Olympic Games** | SEGA_Konami_Capcom | 146 | 146 | 0 | 0 | **100.0%** |
| 264 | **Sonic Racing** | SEGA_Konami_Capcom | 170 | 170 | 0 | 0 | **100.0%** |
| 265 | **Sonic Unleashed** | SEGA_Konami_Capcom | 160 | 160 | 0 | 0 | **100.0%** |
| 266 | **Super Monkey Ball** | SEGA_Konami_Capcom | 181 | 181 | 0 | 0 | **100.0%** |
| 267 | **Virtua Tennis Mobile** | SEGA_Konami_Capcom | 141 | 141 | 0 | 0 | **100.0%** |
| 268 | **After Burner** | SEGA_Konami_Capcom | 97 | 97 | 0 | 0 | **100.0%** |
| 269 | **Golden Axe** | SEGA_Konami_Capcom | 157 | 157 | 0 | 0 | **100.0%** |
| 270 | **Streets of Rage** | SEGA_Konami_Capcom | 156 | 156 | 0 | 0 | **100.0%** |
| 271 | **Metal Slug 1** | SEGA_Konami_Capcom | 147 | 147 | 0 | 0 | **100.0%** |
| 272 | **Metal Slug 2** | SEGA_Konami_Capcom | 218 | 218 | 0 | 0 | **100.0%** |
| 273 | **Metal Slug 3** | SEGA_Konami_Capcom | 229 | 229 | 0 | 0 | **100.0%** |
| 274 | **Frogger** | SEGA_Konami_Capcom | 73 | 73 | 0 | 0 | **100.0%** |
| 275 | **Pro Evolution Soccer 2008** | SEGA_Konami_Capcom | 118 | 118 | 0 | 0 | **100.0%** |
| 276 | **Pro Evolution Soccer 2009** | SEGA_Konami_Capcom | 152 | 152 | 0 | 0 | **100.0%** |
| 277 | **Pro Evolution Soccer 2010** | SEGA_Konami_Capcom | 171 | 171 | 0 | 0 | **100.0%** |
| 278 | **Pro Evolution Soccer 2011** | SEGA_Konami_Capcom | 275 | 275 | 0 | 0 | **100.0%** |
| 279 | **Pro Evolution Soccer 2012** | SEGA_Konami_Capcom | 158 | 158 | 0 | 0 | **100.0%** |
| 280 | **Mega Man II** | SEGA_Konami_Capcom | 143 | 143 | 0 | 0 | **100.0%** |
| 281 | **Mega Man III** | SEGA_Konami_Capcom | 124 | 124 | 0 | 0 | **100.0%** |
| 282 | **Street Fighter Alpha** | SEGA_Konami_Capcom | 146 | 146 | 0 | 0 | **100.0%** |
| 283 | **Street Fighter II** | SEGA_Konami_Capcom | 194 | 194 | 0 | 0 | **100.0%** |
| 284 | **1942 (Capcom)** | SEGA_Konami_Capcom | 129 | 129 | 0 | 0 | **100.0%** |
| 285 | **Ghosts 'n Goblins** | SEGA_Konami_Capcom | 122 | 122 | 0 | 0 | **100.0%** |
| 286 | **Phoenix Wright: Ace Attorney** | SEGA_Konami_Capcom | 116 | 116 | 0 | 0 | **100.0%** |
| 287 | **Pac-Man (Namco)** | SEGA_Konami_Capcom | 255 | 255 | 0 | 0 | **100.0%** |
| 288 | **Ms. Pac-Man** | SEGA_Konami_Capcom | 330 | 330 | 0 | 0 | **100.0%** |
| 289 | **Galaga** | SEGA_Konami_Capcom | 196 | 196 | 0 | 0 | **100.0%** |
| 290 | **Tekken Mobile** | SEGA_Konami_Capcom | 260 | 260 | 0 | 0 | **100.0%** |
| 291 | **Soulcalibur Mobile** | SEGA_Konami_Capcom | 178 | 178 | 0 | 0 | **100.0%** |
| 292 | **Ace Combat Mobile** | SEGA_Konami_Capcom | 124 | 124 | 0 | 0 | **100.0%** |
| 293 | **Townsmen 1** | HandyGames_HeroCraft | 132 | 132 | 0 | 0 | **100.0%** |
| 294 | **Townsmen 2** | HandyGames_HeroCraft | 176 | 176 | 0 | 0 | **100.0%** |
| 295 | **Townsmen 3** | HandyGames_HeroCraft | 208 | 208 | 0 | 0 | **100.0%** |
| 296 | **Townsmen 4** | HandyGames_HeroCraft | 171 | 171 | 0 | 0 | **100.0%** |
| 297 | **Townsmen 5** | HandyGames_HeroCraft | 179 | 179 | 0 | 0 | **100.0%** |
| 298 | **Cyberlords: Arcology** | HandyGames_HeroCraft | 195 | 195 | 0 | 0 | **100.0%** |
| 299 | **Guns'n'Glory** | HandyGames_HeroCraft | 192 | 192 | 0 | 0 | **100.0%** |
| 300 | **Devils & Demons** | HandyGames_HeroCraft | 169 | 169 | 0 | 0 | **100.0%** |
| 301 | **Aces of the Luftwaffe** | HandyGames_HeroCraft | 171 | 171 | 0 | 0 | **100.0%** |
| 302 | **Farm Invasion USA** | HandyGames_HeroCraft | 240 | 240 | 0 | 0 | **100.0%** |
| 303 | **Aporkalypse** | HandyGames_HeroCraft | 211 | 211 | 0 | 0 | **100.0%** |
| 304 | **Dynamite Fishing** | HandyGames_HeroCraft | 170 | 170 | 0 | 0 | **100.0%** |
| 305 | **Shark or Die** | HandyGames_HeroCraft | 194 | 194 | 0 | 0 | **100.0%** |
| 306 | **Tattoo Tycoon** | HandyGames_HeroCraft | 179 | 179 | 0 | 0 | **100.0%** |
| 307 | **Vegas Hustler** | HandyGames_HeroCraft | 197 | 197 | 0 | 0 | **100.0%** |
| 308 | **Revival** | HandyGames_HeroCraft | 179 | 179 | 0 | 0 | **100.0%** |
| 309 | **Revival 2** | HandyGames_HeroCraft | 315 | 315 | 0 | 0 | **100.0%** |
| 310 | **Art of War** | HandyGames_HeroCraft | 211 | 211 | 0 | 0 | **100.0%** |
| 311 | **Art of War 2: Global Confederation** | HandyGames_HeroCraft | 135 | 135 | 0 | 0 | **100.0%** |
| 312 | **Majesty: The Fantasy Kingdom Sim** | HandyGames_HeroCraft | 322 | 322 | 0 | 0 | **100.0%** |
| 313 | **Robo 2** | HandyGames_HeroCraft | 322 | 322 | 0 | 0 | **100.0%** |
| 314 | **Dragon & Dracula** | HandyGames_HeroCraft | 331 | 331 | 0 | 0 | **100.0%** |
| 315 | **Elven Chronicles** | HandyGames_HeroCraft | 182 | 182 | 0 | 0 | **100.0%** |
| 316 | **Bobby Carrot 1** | Classics | 124 | 124 | 0 | 0 | **100.0%** |
| 317 | **Bobby Carrot 2** | Classics | 123 | 123 | 0 | 0 | **100.0%** |
| 318 | **Bobby Carrot 3: Evolution** | Classics | 120 | 120 | 0 | 0 | **100.0%** |
| 319 | **Bounce** | Classics | 129 | 129 | 0 | 0 | **100.0%** |
| 320 | **Gravity Defied: Pro** | Classics | 137 | 137 | 0 | 0 | **100.0%** |
| 321 | **Mafia Mobile** | Classics | 148 | 148 | 0 | 0 | **100.0%** |
| 322 | **Age of Heroes I** | Classics | 116 | 116 | 0 | 0 | **100.0%** |
| 323 | **Age of Heroes II: Underground Horror** | Classics | 261 | 261 | 0 | 0 | **100.0%** |
| 324 | **Age of Heroes IV: Blood and Twilight** | Classics | 154 | 154 | 0 | 0 | **100.0%** |
| 325 | **Age of Heroes V: Chimaera's Heart** | Classics | 109 | 109 | 0 | 0 | **100.0%** |
| 326 | **Darkest Fear 2: Grim Oak** | Classics | 166 | 166 | 0 | 0 | **100.0%** |
| 327 | **Darkest Fear 3: Nightmare** | Classics | 137 | 137 | 0 | 0 | **100.0%** |
| 328 | **Tower Bloxx** | Classics | 267 | 267 | 0 | 0 | **100.0%** |
| 329 | **3D Rollercoaster Rush** | Classics | 238 | 238 | 0 | 0 | **100.0%** |
| 330 | **Rollercoaster Rush 99 Tracks** | Classics | 145 | 145 | 0 | 0 | **100.0%** |
| 331 | **Rollercoaster Rush** | Classics | 276 | 276 | 0 | 0 | **100.0%** |
| 332 | **Crazy Penguin Catapult** | Classics | 150 | 150 | 0 | 0 | **100.0%** |
| 333 | **Crazy Penguin Catapult 2** | Classics | 155 | 155 | 0 | 0 | **100.0%** |
| 334 | **Diamond Islands** | Classics | 156 | 156 | 0 | 0 | **100.0%** |
| 335 | **Diamond Islands 2** | Classics | 155 | 155 | 0 | 0 | **100.0%** |
| 336 | **S.T.A.L.K.E.R. Mobile** | Classics | 349 | 349 | 0 | 0 | **100.0%** |
| 337 | **Fallout Mobile** | Classics | 118 | 118 | 0 | 0 | **100.0%** |
| 338 | **Subway Surfers (Java)** | Classics | 85 | 85 | 0 | 0 | **100.0%** |
| 339 | **Temple Run (Java)** | Classics | 234 | 234 | 0 | 0 | **100.0%** |
| 340 | **Flappy Bird (Java)** | Classics | 98 | 98 | 0 | 0 | **100.0%** |
| 341 | **Minecraft 2D** | Classics | 129 | 129 | 0 | 0 | **100.0%** |
| 342 | **Minecraft 3D (Java)** | Classics | 156 | 156 | 0 | 0 | **100.0%** |
| 343 | **Doodle Jump Deluxe** | Classics | 114 | 114 | 0 | 0 | **100.0%** |
| 344 | **Fruit Ninja 2** | Classics | 98 | 98 | 0 | 0 | **100.0%** |
| 345 | **Angry Birds Seasons** | Classics | 206 | 206 | 0 | 0 | **100.0%** |
| 346 | **Angry Birds Rio** | Classics | 237 | 237 | 0 | 0 | **100.0%** |
| 347 | **Angry Birds Space** | Classics | 212 | 212 | 0 | 0 | **100.0%** |
| 348 | **Cut the Rope 2** | Classics | 242 | 242 | 0 | 0 | **100.0%** |
| 349 | **Kamikaze** | Classics | 243 | 243 | 0 | 0 | **100.0%** |
| 350 | **Kamikaze 2: The Way of Ninja** | Classics | 229 | 229 | 0 | 0 | **100.0%** |
| 351 | **Bumer (Бумер)** | Classics | 119 | 119 | 0 | 0 | **100.0%** |
| 352 | **Bumer 2 (Бумер 2)** | Classics | 119 | 119 | 0 | 0 | **100.0%** |
| 353 | **Brigada (Бригада)** | Classics | 111 | 111 | 0 | 0 | **100.0%** |
| 354 | **Russian Fishing (Русская рыбалка)** | Classics | 139 | 139 | 0 | 0 | **100.0%** |
| 355 | **Parkour** | Classics | 133 | 133 | 0 | 0 | **100.0%** |
| 356 | **Pimp My Ride** | Classics | 169 | 169 | 0 | 0 | **100.0%** |
| 357 | **Tank-o-box** | Classics | 293 | 293 | 0 | 0 | **100.0%** |
| 358 | **Lode Runner** | Classics | 172 | 172 | 0 | 0 | **100.0%** |
| 359 | **Durak (Дурак)** | Classics | 397 | 397 | 0 | 0 | **100.0%** |
| 360 | **Nu Pogodi (Ну погоди)** | Classics | 229 | 229 | 0 | 0 | **100.0%** |
| 361 | **Pirates of the Caribbean: At World's End** | Disney_Pixar | 173 | 173 | 0 | 0 | **100.0%** |
| 362 | **Pirates of the Caribbean: Dead Man's Chest** | Disney_Pixar | 156 | 156 | 0 | 0 | **100.0%** |
| 363 | **Cars (Тачки)** | Disney_Pixar | 208 | 208 | 0 | 0 | **100.0%** |
| 364 | **Cars 2** | Disney_Pixar | 208 | 208 | 0 | 0 | **100.0%** |
| 365 | **TRON: Legacy** | Disney_Pixar | 143 | 143 | 0 | 0 | **100.0%** |
| 366 | **Toy Story 3** | Disney_Pixar | 164 | 164 | 0 | 0 | **100.0%** |
| 367 | **WALL-E** | Disney_Pixar | 132 | 132 | 0 | 0 | **100.0%** |
| 368 | **Ratatouille** | Disney_Pixar | 143 | 143 | 0 | 0 | **100.0%** |
| 369 | **Aladdin** | Disney_Pixar | 135 | 135 | 0 | 0 | **100.0%** |
| 370 | **The Lion King** | Disney_Pixar | 116 | 116 | 0 | 0 | **100.0%** |
| 371 | **Tarzan** | Disney_Pixar | 122 | 122 | 0 | 0 | **100.0%** |
| 372 | **Hercules** | Disney_Pixar | 118 | 118 | 0 | 0 | **100.0%** |
| 373 | **Chip 'n Dale** | Disney_Pixar | 202 | 202 | 0 | 0 | **100.0%** |
| 374 | **Darkwing Duck** | Disney_Pixar | 185 | 185 | 0 | 0 | **100.0%** |
| 375 | **Хроники нарнии 3 1 mbS by gameloft** | Gameloft | 162 | 162 | 0 | 0 | **100.0%** |
| 376 | **Real Football 2011 отGameloft 2010** | Gameloft | 235 | 235 | 0 | 0 | **100.0%** |
| 377 | **Gameloft OsTitans** | Gameloft | 117 | 117 | 0 | 0 | **100.0%** |
| 378 | **Gameloft Soul Of Darkness** | Gameloft | 155 | 155 | 0 | 0 | **100.0%** |
| 379 | **gameloft676** | Gameloft | 147 | 147 | 0 | 0 | **100.0%** |
| 380 | **Gamelofts BackgammonS** | Gameloft | 90 | 90 | 0 | 0 | **100.0%** |
| 381 | **Gamelofts Backgammon Русская версия** | Gameloft | 90 | 90 | 0 | 0 | **100.0%** |
| 382 | **Wild West Guns Gameloft** | Gameloft | 162 | 162 | 0 | 0 | **100.0%** |
| 383 | **Driver San Francisco Gameloft 128х160** | Gameloft | 151 | 151 | 0 | 0 | **100.0%** |
| 384 | **megasity empire by gameloft** | Gameloft | 116 | 116 | 0 | 0 | **100.0%** |
| 385 | **Asphalt 6 Adrenaline от Gameloft** | Gameloft | 130 | 130 | 0 | 0 | **100.0%** |
| 386 | **Gameloft Brain Challenge 4 Breaking Limi** | Gameloft | 184 | 184 | 0 | 0 | **100.0%** |
| 387 | **Gameloft Cannon Rats** | Gameloft | 166 | 166 | 0 | 0 | **100.0%** |
| 388 | **Gameloft Gangstar 3 Miami Vindication v** | Gameloft | 152 | 152 | 0 | 0 | **100.0%** |
| 389 | **LegoBatman2 new gameloft** | Gameloft | 146 | 146 | 0 | 0 | **100.0%** |
| 390 | **Big Range Hunting 2 New Gameloft HIT** | Gameloft | 141 | 141 | 0 | 0 | **100.0%** |
| 391 | **Нарды Gamelofts Backgammon 128x128** | Gameloft | 85 | 85 | 0 | 0 | **100.0%** |
| 392 | **Нарды Gamelofts Backgammon 176x208** | Gameloft | 88 | 88 | 0 | 0 | **100.0%** |
| 393 | **Нарды Gamelofts backgammonS** | Gameloft | 90 | 90 | 0 | 0 | **100.0%** |
| 394 | **Gamelofts Backgammon** | Gameloft | 90 | 90 | 0 | 0 | **100.0%** |
| 395 | **Gamelofts Backgammon 128x128** | Gameloft | 85 | 85 | 0 | 0 | **100.0%** |
| 396 | **A Good Day To Die Hard 2013 Gameloft** | Gameloft | 157 | 157 | 0 | 0 | **100.0%** |
| 397 | **CHESSMASTER GAMELOFT** | Gameloft | 124 | 124 | 0 | 0 | **100.0%** |
| 398 | **horse riding academy gameloft** | Gameloft | 130 | 130 | 0 | 0 | **100.0%** |
| 399 | **DangerDash Nokia Gameloft** | Gameloft | 213 | 213 | 0 | 0 | **100.0%** |
| 400 | **Snake by Gameloft 240 320** | Gameloft | 115 | 115 | 0 | 0 | **100.0%** |
| 401 | **FIFA 2011 На Русском** | EA_Mobile | 147 | 147 | 0 | 0 | **100.0%** |
| 402 | **FIFA 11 Русская версия 240320** | EA_Mobile | 146 | 146 | 0 | 0 | **100.0%** |
| 403 | **x400 FIFA 11 Рус** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 404 | **59 TheSims3** | EA_Mobile | 218 | 218 | 0 | 0 | **100.0%** |
| 405 | **x320 fifa 2011** | EA_Mobile | 184 | 184 | 0 | 0 | **100.0%** |
| 406 | **x208 need for speed carbon** | EA_Mobile | 280 | 280 | 0 | 0 | **100.0%** |
| 407 | **x320 need for speed carbon** | EA_Mobile | 284 | 284 | 0 | 0 | **100.0%** |
| 408 | **x320 need for speed carbon s60** | EA_Mobile | 282 | 282 | 0 | 0 | **100.0%** |
| 409 | **x208 need for speed undercover** | EA_Mobile | 252 | 252 | 0 | 0 | **100.0%** |
| 410 | **FIFA 2011 сенсорные экраны** | EA_Mobile | 151 | 151 | 0 | 0 | **100.0%** |
| 411 | **TheSims3Dream n70** | EA_Mobile | 165 | 165 | 0 | 0 | **100.0%** |
| 412 | **Need for Speed HotPursuit3d 360** | EA_Mobile | 280 | 280 | 0 | 0 | **100.0%** |
| 413 | **FIFA 2010s** | EA_Mobile | 131 | 131 | 0 | 0 | **100.0%** |
| 414 | **Sims 3 AmbitionS Nokia** | EA_Mobile | 202 | 202 | 0 | 0 | **100.0%** |
| 415 | **need for speed** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 416 | **Need for speed shift 3D** | EA_Mobile | 255 | 255 | 0 | 0 | **100.0%** |
| 417 | **Sims 3 AmbS s60 240х320 N95** | EA_Mobile | 180 | 180 | 0 | 0 | **100.0%** |
| 418 | **The sims 3 ambitions** | EA_Mobile | 172 | 172 | 0 | 0 | **100.0%** |
| 419 | **Need for Speed Hot Pursuit** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 420 | **Need for Speed Hot Pursuit sam** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 421 | **Need For Speed Hot PursuitS 240х320** | EA_Mobile | 164 | 164 | 0 | 0 | **100.0%** |
| 422 | **The Sims Pool** | EA_Mobile | 114 | 114 | 0 | 0 | **100.0%** |
| 423 | **the sims3 128x128s jar** | EA_Mobile | 165 | 165 | 0 | 0 | **100.0%** |
| 424 | **Need for Speed Hot Pursuit s60** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 425 | **x320 the sims 2 castaway mobile** | EA_Mobile | 189 | 189 | 0 | 0 | **100.0%** |
| 426 | **Need For Speed Carbon** | EA_Mobile | 281 | 281 | 0 | 0 | **100.0%** |
| 427 | **NEED FOR SPEED HOT PURSUIT 2010 ауди мод** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 428 | **need for speed hot pursuit 3d 176х220** | EA_Mobile | 165 | 165 | 0 | 0 | **100.0%** |
| 429 | **need for speed hot pursuit 3d 360х640** | EA_Mobile | 280 | 280 | 0 | 0 | **100.0%** |
| 430 | **mysims 360x640 nokia s60v5** | EA_Mobile | 169 | 169 | 0 | 0 | **100.0%** |
| 431 | **The Sims 3   World Adventures** | EA_Mobile | 167 | 167 | 0 | 0 | **100.0%** |
| 432 | **Need for Speed Hot PursuitS** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 433 | **FIFA 2011 160** | EA_Mobile | 150 | 150 | 0 | 0 | **100.0%** |
| 434 | **Sims 3 Dream Ambitions** | EA_Mobile | 180 | 180 | 0 | 0 | **100.0%** |
| 435 | **the sims3** | EA_Mobile | 161 | 161 | 0 | 0 | **100.0%** |
| 436 | **Sims 3 Dream Ambitions 240** | EA_Mobile | 198 | 198 | 0 | 0 | **100.0%** |
| 437 | **Sims 3 Dream Ambitions 240 nok** | EA_Mobile | 202 | 202 | 0 | 0 | **100.0%** |
| 438 | **Need For Speed   Most Wanted** | EA_Mobile | 252 | 252 | 0 | 0 | **100.0%** |
| 439 | **x320 fifa 2010** | EA_Mobile | 181 | 181 | 0 | 0 | **100.0%** |
| 440 | **x320 fifa 2010 n40** | EA_Mobile | 131 | 131 | 0 | 0 | **100.0%** |
| 441 | **x128 fifa 2010** | EA_Mobile | 129 | 129 | 0 | 0 | **100.0%** |
| 442 | **x160 fifa 2010** | EA_Mobile | 134 | 134 | 0 | 0 | **100.0%** |
| 443 | **x208 fifa 2010** | EA_Mobile | 137 | 137 | 0 | 0 | **100.0%** |
| 444 | **x220 fifa 2010** | EA_Mobile | 140 | 140 | 0 | 0 | **100.0%** |
| 445 | **x320 fifa 2010 n60** | EA_Mobile | 191 | 191 | 0 | 0 | **100.0%** |
| 446 | **The Sims Pool 3D** | EA_Mobile | 256 | 256 | 0 | 0 | **100.0%** |
| 447 | **FIFA 2010 рус вер** | EA_Mobile | 181 | 181 | 0 | 0 | **100.0%** |
| 448 | **fifa 2011 s60** | EA_Mobile | 151 | 151 | 0 | 0 | **100.0%** |
| 449 | **FIFA 2011 рус вер** | EA_Mobile | 184 | 184 | 0 | 0 | **100.0%** |
| 450 | **Sims 3 Dream Ambitions 160 nok** | EA_Mobile | 157 | 157 | 0 | 0 | **100.0%** |
| 451 | **Need for Speed Underground Rivals 240x32** | EA_Mobile | 316 | 316 | 0 | 0 | **100.0%** |
| 452 | **need for speed hot pursuit 2D** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 453 | **need for speed hot pursuit 3d nokia** | EA_Mobile | 281 | 281 | 0 | 0 | **100.0%** |
| 454 | **3D Need For Speed Pro Street** | EA_Mobile | 226 | 226 | 0 | 0 | **100.0%** |
| 455 | **need for speed pro street 3D nokia** | EA_Mobile | 263 | 263 | 0 | 0 | **100.0%** |
| 456 | **Sims 3 КарьераS 176х208** | EA_Mobile | 167 | 167 | 0 | 0 | **100.0%** |
| 457 | **FIFA 2011** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 458 | **FIFA 2010** | EA_Mobile | 137 | 137 | 0 | 0 | **100.0%** |
| 459 | **need for speed hot pursuit nokia** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 460 | **need for speed hot pursuit 3d nokia s60** | EA_Mobile | 281 | 281 | 0 | 0 | **100.0%** |
| 461 | **need for speed hot pursuit 3ds** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 462 | **Need for Speed HotPursuit3d 176** | EA_Mobile | 175 | 175 | 0 | 0 | **100.0%** |
| 463 | **Need for Speed Shift** | EA_Mobile | 268 | 268 | 0 | 0 | **100.0%** |
| 464 | **SIMS 3 DJ ROMAN KRIVOIs** | EA_Mobile | 177 | 177 | 0 | 0 | **100.0%** |
| 465 | **Sims 3 Dream Ambitions 400** | EA_Mobile | 198 | 198 | 0 | 0 | **100.0%** |
| 466 | **Need for Speed Hot Pursuit 3D v 4 3 4** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 467 | **the sims 3 world adventures** | EA_Mobile | 156 | 156 | 0 | 0 | **100.0%** |
| 468 | **fifa 2011 nokias** | EA_Mobile | 149 | 149 | 0 | 0 | **100.0%** |
| 469 | **need for speed PROstreet** | EA_Mobile | 258 | 258 | 0 | 0 | **100.0%** |
| 470 | **Need For Speed Undercover 3Ds** | EA_Mobile | 227 | 227 | 0 | 0 | **100.0%** |
| 471 | **sims4 mod** | EA_Mobile | 221 | 221 | 0 | 0 | **100.0%** |
| 472 | **fifa 2010 world cup africa 240** | EA_Mobile | 147 | 147 | 0 | 0 | **100.0%** |
| 473 | **FIFA Manager 2010 240 w2** | EA_Mobile | 136 | 136 | 0 | 0 | **100.0%** |
| 474 | **mysims** | EA_Mobile | 170 | 170 | 0 | 0 | **100.0%** |
| 475 | **Need For Speed World China** | EA_Mobile | 130 | 130 | 0 | 0 | **100.0%** |
| 476 | **The Sims 3 Dream Ambitions sams240400** | EA_Mobile | 168 | 168 | 0 | 0 | **100.0%** |
| 477 | **The Sims 4 mod** | EA_Mobile | 221 | 221 | 0 | 0 | **100.0%** |
| 478 | **x320 need for speed hot pursuit3D** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 479 | **need for speed of drift racing** | EA_Mobile | 175 | 175 | 0 | 0 | **100.0%** |
| 480 | **EA Mobile SimCity Metropolis** | EA_Mobile | 116 | 116 | 0 | 0 | **100.0%** |
| 481 | **FIFA 11sskaja versija 240320** | EA_Mobile | 146 | 146 | 0 | 0 | **100.0%** |
| 482 | **need for speed undercover** | EA_Mobile | 257 | 257 | 0 | 0 | **100.0%** |
| 483 | **FIFA World Cup South Africa** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 484 | **Need for Speed HotPursuit3d** | EA_Mobile | 171 | 171 | 0 | 0 | **100.0%** |
| 485 | **Need For Speed World** | EA_Mobile | 130 | 130 | 0 | 0 | **100.0%** |
| 486 | **The Sims 3 Русская** | EA_Mobile | 161 | 161 | 0 | 0 | **100.0%** |
| 487 | **3D Need for Speed Hot Pursuit** | EA_Mobile | 237 | 237 | 0 | 0 | **100.0%** |
| 488 | **need for speed most wanted** | EA_Mobile | 246 | 246 | 0 | 0 | **100.0%** |
| 489 | **fifa 2011 480x800** | EA_Mobile | 149 | 149 | 0 | 0 | **100.0%** |
| 490 | **sims 2 castaway k500s** | EA_Mobile | 166 | 166 | 0 | 0 | **100.0%** |
| 491 | **need for speed underground 2** | EA_Mobile | 235 | 235 | 0 | 0 | **100.0%** |
| 492 | **Need For Speed Undercoverv3** | EA_Mobile | 145 | 145 | 0 | 0 | **100.0%** |
| 493 | **The Sims Poolv3** | EA_Mobile | 115 | 115 | 0 | 0 | **100.0%** |
| 494 | **The Sims Pool 208x208** | EA_Mobile | 105 | 105 | 0 | 0 | **100.0%** |
| 495 | **The Sims Pool 128x160** | EA_Mobile | 114 | 114 | 0 | 0 | **100.0%** |
| 496 | **Sims DJ 240 moto** | EA_Mobile | 111 | 111 | 0 | 0 | **100.0%** |
| 497 | **FIFA 2011 240 nok** | EA_Mobile | 144 | 144 | 0 | 0 | **100.0%** |
| 498 | **Need For Speed Pro Street** | EA_Mobile | 176 | 176 | 0 | 0 | **100.0%** |
| 499 | **x320 need for speed shift s60** | EA_Mobile | 259 | 259 | 0 | 0 | **100.0%** |
| 500 | **EASPORTSFIFA11** | EA_Mobile | 147 | 147 | 0 | 0 | **100.0%** |
| 501 | **Gravity Defied: Classic** | Classics | 134 | 134 | 0 | 0 | **100.0%** |
| 502 | **Castlevania: Order of Shadows** | SEGA_Konami_Capcom | 214 | 214 | 0 | 0 | **100.0%** |
| 503 | **Real Football 2010** | Gameloft | 237 | 237 | 0 | 0 | **100.0%** |
| 504 | **S.T.A.L.K.E.R.: Shadow of Chernobyl** | Classics | 208 | 208 | 0 | 0 | **100.0%** |
| 505 | **Medal of Honor Mobile** | EA_Mobile | 152 | 152 | 0 | 0 | **100.0%** |
| 506 | **Gangstar 2: Kings of L.A.** | Gameloft | 155 | 155 | 0 | 0 | **100.0%** |
| 507 | **Art of War 2: Liberation of Peru** | HandyGames_HeroCraft | 170 | 170 | 0 | 0 | **100.0%** |
| 508 | **Worms 2008** | EA_Mobile | 103 | 103 | 0 | 0 | **100.0%** |
| 509 | **Devil May Cry 3 Mobile** | SEGA_Konami_Capcom | 126 | 126 | 0 | 0 | **100.0%** |
| 510 | **Command & Conquer 4: Tiberian Twilight** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 511 | **Tekken Mobile** | Classics | 300 | 300 | 0 | 0 | **100.0%** |
| 512 | **Dynamite Fishing** | HandyGames_HeroCraft | 276 | 276 | 0 | 0 | **100.0%** |
| 513 | **Worms 2010** | EA_Mobile | 136 | 136 | 0 | 0 | **100.0%** |
| 514 | **Asphalt 4: Elite Racing 3D** | 3D_M3G | 149 | 149 | 0 | 0 | **100.0%** |
| 515 | **Guns 'n' Glory** | HandyGames_HeroCraft | 191 | 191 | 0 | 0 | **100.0%** |
| 516 | **Farm Frenzy 3** | HandyGames_HeroCraft | 287 | 287 | 0 | 0 | **100.0%** |
| 517 | **Ridge Racer Mobile** | Classics | 324 | 324 | 0 | 0 | **100.0%** |
| 518 | **Postal Mobile** | HandyGames_HeroCraft | 187 | 187 | 0 | 0 | **100.0%** |
| 519 | **Tron: Legacy Mobile** | Disney_Pixar | 147 | 147 | 0 | 0 | **100.0%** |
| 520 | **Iron Man 2 Mobile** | Gameloft | 200 | 200 | 0 | 0 | **100.0%** |
| 521 | **Aces of the Luftwaffe 2** | HandyGames_HeroCraft | 279 | 279 | 0 | 0 | **100.0%** |
| 522 | **Doodle Jump Deluxe** | Classics | 116 | 116 | 0 | 0 | **100.0%** |
| 523 | **Galaga Mobile** | Classics | 185 | 185 | 0 | 0 | **100.0%** |
| 524 | **Pro Evolution Soccer 2011** | SEGA_Konami_Capcom | 157 | 157 | 0 | 0 | **100.0%** |
| 525 | **Worms 2011** | EA_Mobile | 148 | 148 | 0 | 0 | **100.0%** |
| 526 | **Bumer: Ssorvannye Bashni** | Classics | 306 | 306 | 0 | 0 | **100.0%** |
| 527 | **Chuzzle** | PopCap | 121 | 121 | 0 | 0 | **100.0%** |
| 528 | **Streets of Rage** | SEGA_Konami_Capcom | 149 | 149 | 0 | 0 | **100.0%** |
| 529 | **Bobby Carrot 5: Level Up** | Classics | 120 | 120 | 0 | 0 | **100.0%** |
| 530 | **Kozaki Mobile** | Classics | 186 | 186 | 0 | 0 | **100.0%** |
| 531 | **Nu, Pogodi! Pogodi, Volk!** | Classics | 229 | 229 | 0 | 0 | **100.0%** |
| 532 | **Silent Hill Mobile 3** | SEGA_Konami_Capcom | 127 | 127 | 0 | 0 | **100.0%** |
| 533 | **Zuma's Revenge** | PopCap | 163 | 163 | 0 | 0 | **100.0%** |
| 534 | **Spider-Man: Toxic City** | Gameloft | 142 | 142 | 0 | 0 | **100.0%** |
| 535 | **Battlefield: Bad Company 2** | EA_Mobile | 148 | 148 | 0 | 0 | **100.0%** |
| 536 | **Townsmen 2** | HandyGames_HeroCraft | 182 | 182 | 0 | 0 | **100.0%** |
| 537 | **Aladdin Mobile** | Disney_Pixar | 122 | 122 | 0 | 0 | **100.0%** |
| 538 | **Revival 2** | HandyGames_HeroCraft | 280 | 280 | 0 | 0 | **100.0%** |
| 539 | **Plants vs Zombies 240x320** | PopCap | 191 | 191 | 0 | 0 | **100.0%** |
| 540 | **The Lion King Mobile** | Disney_Pixar | 116 | 116 | 0 | 0 | **100.0%** |
| 541 | **Contra 4** | SEGA_Konami_Capcom | 90 | 90 | 0 | 0 | **100.0%** |
| 542 | **Monopoly World** | EA_Mobile | 158 | 158 | 0 | 0 | **100.0%** |
| 543 | **Shinobi** | SEGA_Konami_Capcom | 188 | 188 | 0 | 0 | **100.0%** |
| 544 | **Cyberlords: Arcology** | HandyGames_HeroCraft | 193 | 193 | 0 | 0 | **100.0%** |
| 545 | **SimCity Societies** | EA_Mobile | 114 | 114 | 0 | 0 | **100.0%** |
| 546 | **Frogger Beats** | SEGA_Konami_Capcom | 73 | 73 | 0 | 0 | **100.0%** |
| 547 | **Street Fighter II Mobile** | SEGA_Konami_Capcom | 144 | 144 | 0 | 0 | **100.0%** |
| 548 | **Allods Mobile** | Classics | 151 | 151 | 0 | 0 | **100.0%** |
| 549 | **SimCity Deluxe** | EA_Mobile | 140 | 140 | 0 | 0 | **100.0%** |
| 550 | **Ghosts 'n Goblins Mobile** | SEGA_Konami_Capcom | 118 | 118 | 0 | 0 | **100.0%** |
| 551 | **Golden Axe** | SEGA_Konami_Capcom | 157 | 157 | 0 | 0 | **100.0%** |
| 552 | **Sonic Jump** | SEGA_Konami_Capcom | 134 | 134 | 0 | 0 | **100.0%** |
| 553 | **Pirates of the Caribbean: Dead Man's Chest** | Disney_Pixar | 173 | 173 | 0 | 0 | **100.0%** |
| 554 | **Farm Frenzy 2** | HandyGames_HeroCraft | 288 | 288 | 0 | 0 | **100.0%** |
| 555 | **Bookworm** | PopCap | 134 | 134 | 0 | 0 | **100.0%** |
| 556 | **Sonic the Hedgehog 2** | SEGA_Konami_Capcom | 131 | 131 | 0 | 0 | **100.0%** |
| 557 | **Silent Hill Mobile 2** | SEGA_Konami_Capcom | 221 | 221 | 0 | 0 | **100.0%** |
| 558 | **Metro 2033 Mobile** | Classics | 220 | 220 | 0 | 0 | **100.0%** |
| 559 | **Phineas and Ferb** | Disney_Pixar | 254 | 254 | 0 | 0 | **100.0%** |
| 560 | **Townsmen 3** | HandyGames_HeroCraft | 179 | 179 | 0 | 0 | **100.0%** |
| 561 | **Pac-Man Championship Edition** | Classics | 255 | 255 | 0 | 0 | **100.0%** |
| 562 | **Gangstar City** | Gameloft | 172 | 172 | 0 | 0 | **100.0%** |
| 563 | **Bejeweled Twist** | PopCap | 139 | 139 | 0 | 0 | **100.0%** |
| 564 | **Real Football 2008** | Gameloft | 264 | 264 | 0 | 0 | **100.0%** |
| 565 | **Sonic Unleashed** | SEGA_Konami_Capcom | 213 | 213 | 0 | 0 | **100.0%** |
| 566 | **Angry Birds Mobile** | Classics | 191 | 191 | 0 | 0 | **100.0%** |
| 567 | **Trivial Pursuit Mobile** | EA_Mobile | 143 | 143 | 0 | 0 | **100.0%** |
| 568 | **Townsmen 6: Revolution** | HandyGames_HeroCraft | 203 | 203 | 0 | 0 | **100.0%** |
| 569 | **RISK Mobile** | EA_Mobile | 117 | 117 | 0 | 0 | **100.0%** |
| 570 | **Green Farm 3** | Gameloft | 164 | 164 | 0 | 0 | **100.0%** |
| 571 | **Sonic Spinball** | SEGA_Konami_Capcom | 121 | 121 | 0 | 0 | **100.0%** |
| 572 | **Toy Story 3** | Disney_Pixar | 169 | 169 | 0 | 0 | **100.0%** |
| 573 | **The Avengers Mobile** | Gameloft | 181 | 181 | 0 | 0 | **100.0%** |
| 574 | **Dead Space Mobile** | EA_Mobile | 378 | 378 | 0 | 0 | **100.0%** |
| 575 | **Need for Speed Carbon 3D** | 3D_M3G | 283 | 283 | 0 | 0 | **100.0%** |
| 576 | **Townsmen 4** | HandyGames_HeroCraft | 172 | 172 | 0 | 0 | **100.0%** |
| 577 | **Modern Combat 2: Black Pegasus** | Gameloft | 267 | 267 | 0 | 0 | **100.0%** |
| 578 | **Cars 2** | Disney_Pixar | 208 | 208 | 0 | 0 | **100.0%** |
| 579 | **Asphalt 3: Street Rules 3D** | 3D_M3G | 211 | 211 | 0 | 0 | **100.0%** |
| 580 | **Monopoly Here & Now** | EA_Mobile | 168 | 168 | 0 | 0 | **100.0%** |
| 581 | **Fruit Ninja Mobile** | Classics | 98 | 98 | 0 | 0 | **100.0%** |
| 582 | **S.T.A.L.K.E.R.: Clear Sky** | Classics | 249 | 249 | 0 | 0 | **100.0%** |
| 583 | **Need for Speed Most Wanted 3D** | 3D_M3G | 262 | 262 | 0 | 0 | **100.0%** |
| 584 | **Townsmen 1** | HandyGames_HeroCraft | 202 | 202 | 0 | 0 | **100.0%** |
| 585 | **Townsmen 5** | HandyGames_HeroCraft | 179 | 179 | 0 | 0 | **100.0%** |
| 586 | **Virtua Tennis Mobile** | SEGA_Konami_Capcom | 204 | 204 | 0 | 0 | **100.0%** |
| 587 | **Captain America: Sentinel of Liberty** | Gameloft | 140 | 140 | 0 | 0 | **100.0%** |
| 588 | **Castlevania: Aria of Sorrow** | SEGA_Konami_Capcom | 102 | 102 | 0 | 0 | **100.0%** |
| 589 | **Bobby Carrot 4: Flower Company** | Classics | 120 | 120 | 0 | 0 | **100.0%** |
| 590 | **Pro Evolution Soccer 2012** | SEGA_Konami_Capcom | 171 | 171 | 0 | 0 | **100.0%** |
| 591 | **Need for Speed The Run** | EA_Mobile | 199 | 199 | 0 | 0 | **100.0%** |
| 592 | **Super Monkey Ball** | SEGA_Konami_Capcom | 181 | 181 | 0 | 0 | **100.0%** |
| 593 | **Soulcalibur Mobile** | Classics | 128 | 128 | 0 | 0 | **100.0%** |
| 594 | **Men in Black 3** | Gameloft | 157 | 157 | 0 | 0 | **100.0%** |
| 595 | **Bobby Carrot 3: Evolution** | Classics | 124 | 124 | 0 | 0 | **100.0%** |
| 596 | **Little Big City** | Gameloft | 220 | 220 | 0 | 0 | **100.0%** |
| 597 | **Wonder Zoo** | Gameloft | 220 | 220 | 0 | 0 | **100.0%** |
| 598 | **Scrabble Mobile** | EA_Mobile | 109 | 109 | 0 | 0 | **100.0%** |
| 599 | **The Amazing Spider-Man** | Gameloft | 159 | 159 | 0 | 0 | **100.0%** |
| 600 | **Mega Man III** | SEGA_Konami_Capcom | 124 | 124 | 0 | 0 | **100.0%** |
