# rustBahtText ![main branch](https://github.com/tstxni97/rustBahtText/actions/workflows/ci.yml/badge.svg?branch=main)

rust library that converts numerical variables to thai text based on https://github.com/tstxni97/goBahttext

รัสต์ไลบรารี่สำหรับแปลงตัวเลขเป็นข้อความภาษาไทยและมีหน่วยสตางค์ที่ถูกต้อง โดยแปลงโค้ดมาจาก https://github.com/tstxni97/goBahttext

# Installation

```bash
cargo add rust_baht_text
```

# Usage

```rust

use rust_baht_text::baht_text;

fn main() {
    println!("{}", baht_text(2.21));
    // สองบาทยี่สิบเอ็ดสตางค์
}

```