{\rtf1\ansi\ansicpg1252\cocoartf1504\cocoasubrtf760
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
{\colortbl;\red255\green255\blue255;}
{\*\expandedcolortbl;;}
\paperw11900\paperh16840\margl1440\margr1440\vieww10800\viewh8400\viewkind0
\pard\tx566\tx1133\tx1700\tx2267\tx2834\tx3401\tx3968\tx4535\tx5102\tx5669\tx6236\tx6803\pardirnatural\partightenfactor0

\f0\fs24 \cf0 fn factorial(val: usize) -> usize \{\
    if val > 1 \{ return val * factorial(val - 1); \}\
    1\
\}\
\
\
fn main() \{\
    let number_digits = 10;\
    let target = 1000000;\
\
    let mut digits_to_pick = Vec::new();\
    for i in 0..number_digits \{ digits_to_pick.push(i); \}\
\
    let mut digits = Vec::new();\
    let mut num = target;\
    for i in (2..number_digits).rev() \{ \
        let fac = factorial(i);\
        println!("\{\}", fac);\
        let K = num / fac;\
        println!("\{\}", K);\
        num = num - K * fac;\
\
        let picked_digit = digits_to_pick[K];\
        digits.push(picked_digit);\
        digits_to_pick.retain(|&x| x != picked_digit);\
\
    \}\
\
    if target % 2 == 0 \{\
        println!("0");\
        digits.push(digits_to_pick[0]);\
        digits.push(digits_to_pick[1]);\
    \} else \{\
        println!("1");\
        digits.push(digits_to_pick[1]);\
        digits.push(digits_to_pick[0]);\
    \}\
\
    println!("\{:?\}", digits);\
\}}