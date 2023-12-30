# Welcome to my NASA hunch web tool 
> The Site is hosted here: [Cosmic Calendars](https://zone.shuttleapp.rs)

## Table of Contents

  * [Introduction](#what-is-this-tool)
  * [How do we calculate Timezones?](#timezone-chart)
  * [How do we calculate solor system dates?](#how-do-we-calculate-solar-system-dates)
  * [Can you trust us?](#can-you-trust-us)
  * [Supported Timezones](#supported-timezones)
  * [Cross Platform Compatability](#cross-platform-compatibility)
  * [Story](#story)
  * [Credits](#credits-+-inspiration)
  * [Summary](#summary)


## What is this tool?
* This is an application that removes the complexity of caluclating timezones and communication delays across the solar system.

## Timezone Chart
> We adjust the offsets depending if your more west or east on the globe.
![standard-time-zone-chart-of-world-2021-03](https://github.com/ethanAthompson/cosmic_calendar/assets/140981795/f1b35bfa-a0f2-4f37-91f4-cbe4f5e547c5)

## How do we calculate solar system dates? 
* Question: If I were to ask you what time will it be tomorrow on saturn, what is your response? 
* Guess: idk...
* Answer: [Rust Solar](https://github.com/ethanAthompson/rust_solar)

## Supported Timezones

### Timezones (Calculated from Coordinated Universal Time)
  - Major (Most commonly used)
    - [ ] Coordinated Universal Time (UTC)
    - [ ] Baker Island Time (BIT)
    - [x] Niue Time (NUT)
    - [ ] Samoa Standard Time (SST)
    - [ ] Hawaii-Aleutian Standard Time (HST)
    - [ ] Alaska Standard Time (AKST)
    - [x] Pacific Standard Time (PST)
    - [ ] Philippine Standard Time (PSTP)
    - [ ] Mountain Standard Time (MST)
    - [ ] Central Standard Time (CST)
    - [x] Eastern Standard Time (EST)
    - [x] Alantic Standard Time (AST)
    - [ ] Argentina Time (ART)
    - [ ] Brasília Time (BRT)
    - [ ] South Georgia Time (GST)
    - [ ] Azores Time (AZOT)
    - [ ] Cape Verde Time (CVT)
    - [x] Greenwhich Mean Time (GMT)
    - [x] Central European Time (CET)
    - [ ] West Africa Time (WAT)
    - [ ] Eastern European Time (EET)
    - [ ] Central Africa Time (CAT)
    - [ ] Moscow Time (MSK)
    - [ ] Arabia Standard Time (AST)
    - [ ] Azerbaijan Time (AZT)
    - [ ] Samara Time (SAMT)
    - [ ] Pakistan Standard Time (PKT)
    - [ ] Yekaterinburg Time (YEKT)
    - [ ] Indian Standard Time (IST)
    - [ ] Sri Lanka Time (SLT)
    - [ ] Bangladesh Time (BST)
    - [ ] Bhutan Time (BTT)
    - [x] Omsk Time (OMST)
    - [ ] Cocos Islands Time (CCT)
    - [ ] Mynanmar Time (MMT)
    - [ ] Indochina Time (ICT)
    - [ ] Novosibirsk Time (NOVT)
    - [ ] China Standard Time (CST)
    - [ ] Australian Western Standard Time (AWST)
    - [ ] Central Western Standard Time (CWST)
    - [x] Japan Standard Time (JST)
    - [ ] Korea Standard Time (KST)
    - [ ] Yakutsk Time (YAKT)
    - [ ] Australian Eastern Standard Time (AEST)
    - [ ] Vladivostok Time (VLAT)
    - [ ] Solomon Islands Time (SBT)
    - [ ] Magadan Time (MAGT)
    - [ ] Norfolk Island Time (NFT)
    - [ ] New Zealand Standard Time (NZST)
    - [ ] Fiji Time (FJT)
    - [ ] Tonga Time (TOT)
    - [ ] Line Islands Time (LINT)

  - Minor (Less commonly used)
    - [ ] Newfoundland Standard Time (NST)
    - [ ] Iran Standard Time (IRST)
    - [ ] Afghanistan Time (AFT)
    - [ ] Nepal Time (NPT)
    - [ ] Central Western Standard Time (CWST)
    - [ ] Australian Central Standard Time (ACST)
    - [ ] Lord Howe Standard Time (LHST)
    - [ ] Chatham Islands Time (CHAST)
    

## Cross Platform Compatibility

- Web Browsers
  - [ ] Google Chrome
  - [ ] Firefox
  - [ ] Vivaldi
  - [ ] ....

- Desktop Devices
  - [ ] Windows 10/11
  <!-- You need to build on app on windows for window devices; linker, mscv, etc.. -->
  - [ ] Ubuntu 20.04 LTS
  - [ ] MacOS Montery
  - [ ] Chrome OS
  - [ ] ....

- Mobile Devices
  - [ ] Samsung Galaxy
  - [ ] Google Pixel
  - [ ] OnePlus
  - [ ] LG
  - [ ] Asus ROG
  - [ ] ....

- Tablet Devices
  - [ ] Apple
  - [ ] Lenovo
  - [ ] ....

- Watch Devices
  - [ ] Apple
  - [ ] Samsung Galaxy
  - [ ] Amazfit
  - [ ] ....

## Story
> The server folder hosts the directory I have for production onto the web
> The src-tarui publishes my web app as a desktop app.
> The src is the core of the two workspaces, withoout src **server** and **src-tauri** fails.

## Credits + Inspiration
* [Leptos](https://leptos.dev/) 
* [Tauri](https://tauri.app/)
* [Leptos-Csr-Tailwind-Netlify-App](https://leptos-csr-tailwind.netlify.app/)
* [Time.Is](https://time.is/)
* [UTC Time](https://www.utctime.net/)

## Summary
*  Thank you for reading this.
